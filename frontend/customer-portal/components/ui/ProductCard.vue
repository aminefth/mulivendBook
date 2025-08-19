<template>
  <UCard class="group hover:shadow-lg transition-all duration-300 overflow-hidden">
    <div class="aspect-[3/4] relative overflow-hidden rounded-lg bg-gray-100">
      <img
        :src="product.images?.[0] || '/placeholder-book.jpg'"
        :alt="product.name"
        class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
      />
      <div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
        <UButton
          icon="i-heroicons-heart"
          size="sm"
          color="white"
          variant="solid"
          :ui="{ rounded: 'rounded-full' }"
          @click="toggleWishlist"
        />
      </div>
      <div v-if="product.stock_quantity <= 5" class="absolute top-2 left-2">
        <UBadge color="red" variant="solid">
          {{ $t('product.low_stock') }}
        </UBadge>
      </div>
    </div>

    <div class="p-4 space-y-3">
      <div class="space-y-1">
        <h3 class="font-semibold text-gray-900 dark:text-white line-clamp-2 group-hover:text-primary-600 transition-colors">
          {{ product.name }}
        </h3>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          {{ $t('product.author') }}: {{ product.author }}
        </p>
        <p class="text-xs text-gray-500">
          {{ product.vendor?.business_name }}
        </p>
      </div>

      <div class="flex items-center gap-2">
        <StarRating
          :rating="product.average_rating || 0"
          :star-size="16"
          :show-rating="false"
          :read-only="true"
          active-color="#fbbf24"
          inactive-color="#e5e7eb"
        />
        <span class="text-xs text-gray-500">
          ({{ product.review_count || 0 }})
        </span>
      </div>

      <div class="flex items-center justify-between">
        <div class="space-y-1">
          <p class="text-lg font-bold text-primary-600">
            {{ formatPrice(product.price) }}
          </p>
          <p v-if="product.original_price && product.original_price > product.price" 
             class="text-sm text-gray-500 line-through">
            {{ formatPrice(product.original_price) }}
          </p>
        </div>
        
        <UButton
          :disabled="product.stock_quantity === 0"
          size="sm"
          @click="addToCart"
          :loading="loading"
        >
          <template v-if="product.stock_quantity === 0">
            {{ $t('product.out_of_stock') }}
          </template>
          <template v-else>
            <UIcon name="i-heroicons-shopping-cart" class="w-4 h-4 mr-1" />
            {{ $t('product.add_to_cart') }}
          </template>
        </UButton>
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import StarRating from 'vue-star-rating'

interface Product {
  id: string
  name: string
  author: string
  price: number
  original_price?: number
  stock_quantity: number
  images?: string[]
  average_rating?: number
  review_count?: number
  vendor?: {
    business_name: string
  }
}

interface Props {
  product: Product
}

const props = defineProps<Props>()
const { $t } = useI18n()
const cartStore = useCartStore()

const loading = ref(false)

const formatPrice = (price: number) => {
  return new Intl.NumberFormat('ar-MA', {
    style: 'currency',
    currency: 'MAD',
    minimumFractionDigits: 0
  }).format(price)
}

const addToCart = async () => {
  loading.value = true
  try {
    cartStore.addItem(props.product, 1)
    // Show success toast
    const toast = useToast()
    toast.add({
      title: $t('cart.item_added'),
      description: $t('cart.item_added_desc', { name: props.product.name }),
      color: 'green'
    })
  } catch (error) {
    console.error('Failed to add to cart:', error)
  } finally {
    loading.value = false
  }
}

const toggleWishlist = () => {
  // Implement wishlist functionality
  console.log('Toggle wishlist for:', props.product.id)
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
