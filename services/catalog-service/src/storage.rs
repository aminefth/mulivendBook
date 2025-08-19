use anyhow::Result;
use aws_sdk_s3::{Client, Config, Region};
use aws_config::BehaviorVersion;
use uuid::Uuid;
use actix_web::web;
use futures::StreamExt;
use crate::config::Config as AppConfig;
use crate::errors::AppError;

pub struct StorageService {
    client: Client,
    bucket: String,
}

impl StorageService {
    pub async fn new(config: &AppConfig) -> Result<Self> {
        let aws_config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(config.aws_region.clone()))
            .load()
            .await;

        let client = Client::new(&aws_config);

        Ok(StorageService {
            client,
            bucket: config.aws_bucket.clone(),
        })
    }

    pub async fn upload_product_images(
        &self,
        product_id: Uuid,
        mut payload: web::Payload,
    ) -> Result<Vec<String>, AppError> {
        let mut image_urls = Vec::new();
        let mut file_count = 0;
        const MAX_FILES: usize = 5;

        while let Some(chunk) = payload.next().await {
            if file_count >= MAX_FILES {
                break;
            }

            let data = chunk.map_err(|e| AppError::Storage(e.to_string()))?;
            
            // Generate unique filename
            let file_id = Uuid::new_v4();
            let key = format!("products/{}/images/{}.jpg", product_id, file_id);

            // Upload to S3
            let _result = self.client
                .put_object()
                .bucket(&self.bucket)
                .key(&key)
                .body(data.into())
                .content_type("image/jpeg")
                .send()
                .await
                .map_err(|e| AppError::Storage(e.to_string()))?;

            let image_url = format!("https://{}.s3.amazonaws.com/{}", self.bucket, key);
            image_urls.push(image_url);
            file_count += 1;
        }

        Ok(image_urls)
    }

    pub async fn delete_product_images(&self, product_id: Uuid) -> Result<(), AppError> {
        let prefix = format!("products/{}/images/", product_id);
        
        // List objects with prefix
        let objects = self.client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(&prefix)
            .send()
            .await
            .map_err(|e| AppError::Storage(e.to_string()))?;

        // Delete each object
        if let Some(contents) = objects.contents() {
            for object in contents {
                if let Some(key) = object.key() {
                    self.client
                        .delete_object()
                        .bucket(&self.bucket)
                        .key(key)
                        .send()
                        .await
                        .map_err(|e| AppError::Storage(e.to_string()))?;
                }
            }
        }

        Ok(())
    }
}
