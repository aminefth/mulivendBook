<template>
  <div class="space-y-6">
    <!-- Filters and Sort -->
    <div class="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between">
      <div class="flex flex-wrap gap-2">
        <USelectMenu
          v-model="selectedCategory"
          :options="categoryOptions"
          placeholder="All Categories"
          class="w-48"
        />
        <USelectMenu
          v-model="selectedLanguage"
          :options="languageOptions"
          placeholder="All Languages"
          class="w-32"
        />
        <URange
          v-model="priceRange"
          :min="0"
          :max="1000"
          :step="10"
          class="w-48"
        />
      </div>
      
      <USelectMenu
        v-model="sortBy"
        :options="sortOptions"
        class="w-48"
      />
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
      <USkeleton
        v-for="i in 8"
        :key="i"
        class="h-96 w-full"
      />
    </div>

    <!-- Products Grid -->
    <div v-else-if="products.length > 0" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
      <ProductCard
        v-for="product in products"
        :key="product.id"
        :product="product"
        @click="navigateToProduct(product.id)"
        class="cursor-pointer"
      />
    </div>

    <!-- Empty State -->
    <UCard v-else class="text-center py-12">
      <div class="space-y-4">
        <UIcon name="i-heroicons-book-open" class="w-16 h-16 mx-auto text-gray-400" />
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {{ $t('product.no_products') }}
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            {{ $t('product.no_products_desc') }}
          </p>
        </div>
        <UButton @click="clearFilters">
          {{ $t('product.clear_filters') }}
        </UButton>
      </div>
    </UCard>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="flex justify-center">
      <UPagination
        v-model="currentPage"
        :page-count="pageSize"
        :total="totalItems"
        :ui="{
          wrapper: 'flex items-center gap-1',
          rounded: '!rounded-full min-w-[32px] justify-center',
          default: {
            activeButton: {
              variant: 'outline'
            }
          }
        }"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
interface Product {
  id: string
  name: string
  author: string
  price: number
  stock_quantity: number
  images?: string[]
  average_rating?: number
  review_count?: number
  category: string
  language: string
  vendor?: {
    business_name: string
  }
}

interface Props {
  initialProducts?: Product[]
  categoryFilter?: string
  searchQuery?: string
}

const props = withDefaults(defineProps<Props>(), {
  initialProducts: () => [],
  categoryFilter: '',
  searchQuery: ''
})

const { $t } = useI18n()
const config = useRuntimeConfig()

// Reactive state
const products = ref<Product[]>(props.initialProducts)
const loading = ref(false)
const currentPage = ref(1)
const pageSize = ref(20)
const totalItems = ref(0)
const totalPages = computed(() => Math.ceil(totalItems.value / pageSize.value))

// Filters
const selectedCategory = ref(props.categoryFilter)
const selectedLanguage = ref('')
const priceRange = ref([0, 1000])
const sortBy = ref('newest')

// Options
const categoryOptions = [
  { label: $t('home.categories.islamic_studies'), value: 'islamic_studies' },
  { label: $t('home.categories.arabic_literature'), value: 'arabic_literature' },
  { label: $t('home.categories.french_literature'), value: 'french_literature' },
  { label: $t('home.categories.moroccan_culture'), value: 'moroccan_culture' },
  { label: $t('home.categories.academic'), value: 'academic' },
  { label: $t('home.categories.business'), value: 'business' },
  { label: $t('home.categories.technology'), value: 'technology' },
  { label: $t('home.categories.children'), value: 'children' }
]

const languageOptions = [
  { label: 'العربية', value: 'Arabic' },
  { label: 'Français', value: 'French' },
  { label: 'English', value: 'English' }
]

const sortOptions = [
  { label: $t('product.sort.newest'), value: 'newest' },
  { label: $t('product.sort.price_low'), value: 'price_asc' },
  { label: $t('product.sort.price_high'), value: 'price_desc' },
  { label: $t('product.sort.rating'), value: 'rating' },
  { label: $t('product.sort.popular'), value: 'popular' }
]

// Watch for filter changes
watch([selectedCategory, selectedLanguage, priceRange, sortBy, currentPage], () => {
  fetchProducts()
}, { deep: true })

// Methods
const fetchProducts = async () => {
  loading.value = true
  try {
    const params = new URLSearchParams({
      page: currentPage.value.toString(),
      per_page: pageSize.value.toString(),
      sort: sortBy.value,
      min_price: priceRange.value[0].toString(),
      max_price: priceRange.value[1].toString()
    })

    if (selectedCategory.value) {
      params.append('category', selectedCategory.value)
    }
    if (selectedLanguage.value) {
      params.append('language', selectedLanguage.value)
    }
    if (props.searchQuery) {
      params.append('q', props.searchQuery)
    }

    const response = await $fetch<{
      products: Product[]
      total: number
      page: number
      per_page: number
    }>(`/products?${params.toString()}`, {
      baseURL: config.public.catalogServiceUrl
    })

    products.value = response.products
    totalItems.value = response.total
  } catch (error) {
    console.error('Failed to fetch products:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('product.fetch_error'),
      color: 'red'
    })
  } finally {
    loading.value = false
  }
}

const navigateToProduct = (productId: string) => {
  navigateTo(`/products/${productId}`)
}

const clearFilters = () => {
  selectedCategory.value = ''
  selectedLanguage.value = ''
  priceRange.value = [0, 1000]
  sortBy.value = 'newest'
  currentPage.value = 1
}

// Initialize
onMounted(() => {
  if (props.initialProducts.length === 0) {
    fetchProducts()
  }
})
</script>
