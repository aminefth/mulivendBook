<template>
  <div class="container mx-auto px-4 py-8">
    <div v-if="loading" class="space-y-8">
      <USkeleton class="h-8 w-64" />
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <USkeleton class="aspect-[3/4] w-full" />
        <div class="space-y-4">
          <USkeleton class="h-8 w-full" />
          <USkeleton class="h-6 w-32" />
          <USkeleton class="h-12 w-24" />
          <USkeleton class="h-10 w-full" />
        </div>
      </div>
    </div>

    <div v-else-if="product" class="space-y-8">
      <!-- Breadcrumb -->
      <UBreadcrumb :links="breadcrumbLinks" />

      <!-- Product Details -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- Product Images -->
        <div class="space-y-4">
          <div class="aspect-[3/4] bg-gray-100 dark:bg-gray-800 rounded-lg overflow-hidden">
            <img
              :src="selectedImage || '/placeholder-book.jpg'"
              :alt="product.name"
              class="w-full h-full object-cover"
            />
          </div>
          <div v-if="product.images && product.images.length > 1" class="flex gap-2 overflow-x-auto">
            <button
              v-for="(image, index) in product.images"
              :key="index"
              @click="selectedImage = image"
              class="flex-shrink-0 w-20 h-24 bg-gray-100 dark:bg-gray-800 rounded-lg overflow-hidden border-2"
              :class="selectedImage === image ? 'border-primary-500' : 'border-transparent'"
            >
              <img :src="image" :alt="`${product.name} ${index + 1}`" class="w-full h-full object-cover" />
            </button>
          </div>
        </div>

        <!-- Product Info -->
        <div class="space-y-6">
          <div>
            <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
              {{ product.name }}
            </h1>
            <p class="text-lg text-gray-600 dark:text-gray-400">
              {{ $t('product.author') }}: {{ product.author }}
            </p>
            <p class="text-sm text-gray-500">
              {{ $t('product.publisher') }}: {{ product.publisher }}
            </p>
          </div>

          <!-- Rating -->
          <div class="flex items-center gap-4">
            <div class="flex items-center gap-2">
              <StarRating
                :rating="product.average_rating || 0"
                :star-size="20"
                :show-rating="false"
                :read-only="true"
                active-color="#fbbf24"
                inactive-color="#e5e7eb"
              />
              <span class="text-sm text-gray-600 dark:text-gray-400">
                ({{ product.review_count || 0 }} {{ $t('product.reviews') }})
              </span>
            </div>
            <UButton variant="ghost" size="xs" @click="scrollToReviews">
              {{ $t('product.read_reviews') }}
            </UButton>
          </div>

          <!-- Price -->
          <div class="space-y-2">
            <div class="flex items-center gap-4">
              <span class="text-3xl font-bold text-primary-600">
                {{ formatPrice(product.price) }}
              </span>
              <span v-if="product.original_price && product.original_price > product.price" 
                    class="text-lg text-gray-500 line-through">
                {{ formatPrice(product.original_price) }}
              </span>
            </div>
            <div v-if="product.stock_quantity <= 5 && product.stock_quantity > 0" class="text-sm text-orange-600">
              {{ $t('product.only_left', { count: product.stock_quantity }) }}
            </div>
          </div>

          <!-- Product Details -->
          <div class="grid grid-cols-2 gap-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
            <div>
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ $t('product.language') }}</span>
              <p class="font-medium">{{ product.language }}</p>
            </div>
            <div>
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ $t('product.pages') }}</span>
              <p class="font-medium">{{ product.pages }}</p>
            </div>
            <div>
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ $t('product.isbn') }}</span>
              <p class="font-medium">{{ product.isbn }}</p>
            </div>
            <div>
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ $t('common.vendor') }}</span>
              <p class="font-medium">{{ product.vendor?.business_name }}</p>
            </div>
          </div>

          <!-- Actions -->
          <div class="space-y-4">
            <div class="flex items-center gap-4">
              <UInput
                v-model.number="quantity"
                type="number"
                :min="1"
                :max="product.stock_quantity"
                class="w-20"
              />
              <UButton
                size="lg"
                :disabled="product.stock_quantity === 0"
                :loading="addingToCart"
                @click="addToCart"
                class="flex-1"
              >
                <UIcon name="i-heroicons-shopping-cart" class="w-5 h-5 mr-2" />
                {{ product.stock_quantity === 0 ? $t('product.out_of_stock') : $t('product.add_to_cart') }}
              </UButton>
            </div>
            
            <div class="flex gap-2">
              <UButton variant="outline" size="lg" class="flex-1" @click="addToWishlist">
                <UIcon name="i-heroicons-heart" class="w-5 h-5 mr-2" />
                {{ $t('product.add_to_wishlist') }}
              </UButton>
              <UButton variant="outline" size="lg" @click="shareProduct">
                <UIcon name="i-heroicons-share" class="w-5 h-5" />
              </UButton>
            </div>
          </div>

          <!-- Vendor Info -->
          <UCard class="p-4">
            <div class="flex items-center gap-4">
              <div class="w-12 h-12 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center">
                <UIcon name="i-heroicons-building-storefront" class="w-6 h-6 text-primary-600" />
              </div>
              <div class="flex-1">
                <h3 class="font-semibold text-gray-900 dark:text-white">
                  {{ product.vendor?.business_name }}
                </h3>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  {{ product.vendor?.description }}
                </p>
              </div>
              <UButton variant="outline" size="sm" :to="`/vendors/${product.vendor_id}`">
                {{ $t('common.view_store') }}
              </UButton>
            </div>
          </UCard>
        </div>
      </div>

      <!-- Product Description -->
      <UCard>
        <template #header>
          <h2 class="text-xl font-semibold">{{ $t('product.description') }}</h2>
        </template>
        <div class="prose dark:prose-invert max-w-none">
          <p>{{ product.description }}</p>
        </div>
      </UCard>

      <!-- Reviews Section -->
      <div ref="reviewsSection">
        <ProductReviews :product-id="product.id" :reviews="product.reviews" />
      </div>

      <!-- Related Products -->
      <div v-if="relatedProducts.length > 0">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
          {{ $t('product.related') }}
        </h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <ProductCard
            v-for="relatedProduct in relatedProducts"
            :key="relatedProduct.id"
            :product="relatedProduct"
            @click="navigateToProduct(relatedProduct.id)"
            class="cursor-pointer"
          />
        </div>
      </div>
    </div>

    <div v-else class="text-center py-12">
      <UIcon name="i-heroicons-exclamation-triangle" class="w-16 h-16 mx-auto text-gray-400 mb-4" />
      <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
        {{ $t('product.not_found') }}
      </h2>
      <p class="text-gray-600 dark:text-gray-400 mb-4">
        {{ $t('product.not_found_desc') }}
      </p>
      <UButton to="/products">
        {{ $t('product.browse_products') }}
      </UButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import StarRating from 'vue-star-rating'

const route = useRoute()
const { $t } = useI18n()
const config = useRuntimeConfig()
const cartStore = useCartStore()

// Reactive state
const product = ref(null)
const loading = ref(true)
const selectedImage = ref('')
const quantity = ref(1)
const addingToCart = ref(false)
const relatedProducts = ref([])
const reviewsSection = ref(null)

// Computed
const breadcrumbLinks = computed(() => [
  { label: $t('nav.home'), to: '/' },
  { label: $t('nav.books'), to: '/products' },
  { label: product.value?.category?.name || '', to: `/categories/${product.value?.category?.slug}` },
  { label: product.value?.name || '', to: route.path }
])

// Methods
const formatPrice = (price: number) => {
  return new Intl.NumberFormat('ar-MA', {
    style: 'currency',
    currency: 'MAD',
    minimumFractionDigits: 0
  }).format(price)
}

const fetchProduct = async () => {
  try {
    const response = await $fetch(`/products/${route.params.id}`, {
      baseURL: config.public.catalogServiceUrl
    })
    product.value = response
    selectedImage.value = response.images?.[0] || ''
    
    // Update page meta
    useHead({
      title: response.name,
      meta: [
        { name: 'description', content: response.description }
      ]
    })
    
    // Fetch related products
    fetchRelatedProducts()
  } catch (error) {
    console.error('Failed to fetch product:', error)
  } finally {
    loading.value = false
  }
}

const fetchRelatedProducts = async () => {
  try {
    const response = await $fetch(`/products/${route.params.id}/related`, {
      baseURL: config.public.catalogServiceUrl
    })
    relatedProducts.value = response.products || []
  } catch (error) {
    console.error('Failed to fetch related products:', error)
  }
}

const addToCart = async () => {
  addingToCart.value = true
  try {
    cartStore.addItem(product.value, quantity.value)
    
    const toast = useToast()
    toast.add({
      title: $t('cart.item_added'),
      description: $t('cart.item_added_desc', { name: product.value.name }),
      color: 'green'
    })
  } catch (error) {
    console.error('Failed to add to cart:', error)
  } finally {
    addingToCart.value = false
  }
}

const addToWishlist = () => {
  // Implement wishlist functionality
  const toast = useToast()
  toast.add({
    title: $t('common.success'),
    description: $t('product.added_to_wishlist'),
    color: 'green'
  })
}

const shareProduct = async () => {
  if (navigator.share) {
    try {
      await navigator.share({
        title: product.value.name,
        text: product.value.description,
        url: window.location.href
      })
    } catch (error) {
      console.error('Failed to share:', error)
    }
  } else {
    // Fallback: copy to clipboard
    await navigator.clipboard.writeText(window.location.href)
    const toast = useToast()
    toast.add({
      title: $t('common.success'),
      description: $t('product.link_copied'),
      color: 'green'
    })
  }
}

const scrollToReviews = () => {
  reviewsSection.value?.scrollIntoView({ behavior: 'smooth' })
}

const navigateToProduct = (productId: string) => {
  navigateTo(`/products/${productId}`)
}

// Initialize
onMounted(() => {
  fetchProduct()
})

// Watch for route changes
watch(() => route.params.id, () => {
  if (route.params.id) {
    loading.value = true
    fetchProduct()
  }
})
</script>
