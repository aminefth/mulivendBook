use anyhow::Result;
use elasticsearch::{Elasticsearch, http::transport::Transport};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::models::{Product, SearchRequest, SearchResponse};

pub struct SearchEngine {
    client: Elasticsearch,
    index_name: String,
}

impl SearchEngine {
    pub async fn new(elasticsearch_url: &str) -> Result<Self> {
        let transport = Transport::single_node(elasticsearch_url)?;
        let client = Elasticsearch::new(transport);
        
        let search_engine = SearchEngine {
            client,
            index_name: "products".to_string(),
        };
        
        // Create index if it doesn't exist
        search_engine.create_index().await?;
        
        Ok(search_engine)
    }

    async fn create_index(&self) -> Result<()> {
        let mapping = json!({
            "mappings": {
                "properties": {
                    "id": { "type": "keyword" },
                    "vendor_id": { "type": "keyword" },
                    "title": { 
                        "type": "text",
                        "analyzer": "standard",
                        "fields": {
                            "keyword": { "type": "keyword" }
                        }
                    },
                    "description": { "type": "text", "analyzer": "standard" },
                    "author": { 
                        "type": "text",
                        "fields": {
                            "keyword": { "type": "keyword" }
                        }
                    },
                    "publisher": { 
                        "type": "text",
                        "fields": {
                            "keyword": { "type": "keyword" }
                        }
                    },
                    "isbn": { "type": "keyword" },
                    "language": { "type": "keyword" },
                    "category_id": { "type": "keyword" },
                    "price": { "type": "double" },
                    "discount_price": { "type": "double" },
                    "stock_quantity": { "type": "integer" },
                    "status": { "type": "keyword" },
                    "created_at": { "type": "date" },
                    "updated_at": { "type": "date" }
                }
            }
        });

        let _response = self.client
            .indices()
            .create(elasticsearch::indices::IndicesCreateParts::Index(&self.index_name))
            .body(mapping)
            .send()
            .await;

        Ok(())
    }

    pub async fn index_product(&self, product: &Product) -> Result<()> {
        let doc = json!({
            "id": product.id,
            "vendor_id": product.vendor_id,
            "title": product.title,
            "description": product.description,
            "author": product.author,
            "publisher": product.publisher,
            "isbn": product.isbn,
            "language": product.language,
            "category_id": product.category_id,
            "price": product.price,
            "discount_price": product.discount_price,
            "stock_quantity": product.stock_quantity,
            "status": product.status,
            "created_at": product.created_at,
            "updated_at": product.updated_at
        });

        self.client
            .index(elasticsearch::IndexParts::IndexId(&self.index_name, &product.id.to_string()))
            .body(doc)
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete_product(&self, product_id: Uuid) -> Result<()> {
        self.client
            .delete(elasticsearch::DeleteParts::IndexId(&self.index_name, &product_id.to_string()))
            .send()
            .await?;

        Ok(())
    }

    pub async fn search(&self, request: &SearchRequest) -> Result<SearchResponse> {
        let mut query = json!({
            "bool": {
                "must": [],
                "filter": []
            }
        });

        // Add text search
        if !request.query.is_empty() {
            query["bool"]["must"].as_array_mut().unwrap().push(json!({
                "multi_match": {
                    "query": request.query,
                    "fields": ["title^2", "description", "author^1.5", "publisher"]
                }
            }));
        }

        // Add filters
        if let Some(category_id) = request.category_id {
            query["bool"]["filter"].as_array_mut().unwrap().push(json!({
                "term": { "category_id": category_id }
            }));
        }

        if let Some(min_price) = request.min_price {
            query["bool"]["filter"].as_array_mut().unwrap().push(json!({
                "range": { "price": { "gte": min_price } }
            }));
        }

        if let Some(max_price) = request.max_price {
            query["bool"]["filter"].as_array_mut().unwrap().push(json!({
                "range": { "price": { "lte": max_price } }
            }));
        }

        if let Some(author) = &request.author {
            query["bool"]["filter"].as_array_mut().unwrap().push(json!({
                "term": { "author.keyword": author }
            }));
        }

        if let Some(publisher) = &request.publisher {
            query["bool"]["filter"].as_array_mut().unwrap().push(json!({
                "term": { "publisher.keyword": publisher }
            }));
        }

        if let Some(language) = &request.language {
            query["bool"]["filter"].as_array_mut().unwrap().push(json!({
                "term": { "language": language }
            }));
        }

        // Add status filter for active products
        query["bool"]["filter"].as_array_mut().unwrap().push(json!({
            "term": { "status": "active" }
        }));

        let page = request.page.unwrap_or(1);
        let limit = request.limit.unwrap_or(20);
        let from = (page - 1) * limit;

        let mut sort = vec![];
        if let Some(sort_by) = &request.sort_by {
            let order = request.sort_order.as_deref().unwrap_or("asc");
            sort.push(json!({ sort_by: { "order": order } }));
        } else {
            sort.push(json!({ "_score": { "order": "desc" } }));
            sort.push(json!({ "created_at": { "order": "desc" } }));
        }

        let search_body = json!({
            "query": query,
            "sort": sort,
            "from": from,
            "size": limit,
            "aggs": {
                "categories": {
                    "terms": { "field": "category_id", "size": 10 }
                },
                "authors": {
                    "terms": { "field": "author.keyword", "size": 10 }
                },
                "publishers": {
                    "terms": { "field": "publisher.keyword", "size": 10 }
                },
                "price_ranges": {
                    "range": {
                        "field": "price",
                        "ranges": [
                            { "to": 10 },
                            { "from": 10, "to": 25 },
                            { "from": 25, "to": 50 },
                            { "from": 50 }
                        ]
                    }
                }
            }
        });

        let response = self.client
            .search(elasticsearch::SearchParts::Index(&[&self.index_name]))
            .body(search_body)
            .send()
            .await?;

        let response_body: Value = response.json().await?;
        
        let hits = response_body["hits"]["hits"].as_array().unwrap_or(&vec![]);
        let total = response_body["hits"]["total"]["value"].as_i64().unwrap_or(0);
        
        let products: Vec<Product> = hits.iter()
            .filter_map(|hit| {
                serde_json::from_value(hit["_source"].clone()).ok()
            })
            .collect();

        let total_pages = (total as f64 / limit as f64).ceil() as i32;
        let facets = response_body["aggregations"].clone();

        Ok(SearchResponse {
            products,
            total,
            page,
            limit,
            total_pages,
            facets,
            suggestions: vec![], // TODO: Implement suggestions
        })
    }
}
