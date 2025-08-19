<template>
  <div class="container mx-auto px-4 py-8">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-8">
        {{ $t('cart.title') }}
      </h1>

      <!-- Empty Cart -->
      <div v-if="cartStore.isEmpty" class="text-center py-12">
        <UIcon name="i-heroicons-shopping-cart" class="w-24 h-24 mx-auto text-gray-400 mb-6" />
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-4">
          {{ $t('cart.empty') }}
        </h2>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          {{ $t('cart.empty_desc') }}
        </p>
        <UButton to="/products" size="lg">
          {{ $t('cart.continue_shopping') }}
        </UButton>
      </div>

      <!-- Cart Items -->
      <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <!-- Items List -->
        <div class="lg:col-span-2 space-y-4">
          <UCard
            v-for="item in cartStore.items"
            :key="item.id"
            class="p-4"
          >
            <div class="flex items-start gap-4">
              <!-- Product Image -->
              <div class="w-20 h-24 bg-gray-100 dark:bg-gray-800 rounded-lg overflow-hidden flex-shrink-0">
                <img
                  :src="item.image || '/placeholder-book.jpg'"
                  :alt="item.name"
                  class="w-full h-full object-cover"
                />
              </div>

              <!-- Product Details -->
              <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between">
                  <div class="flex-1">
                    <h3 class="font-semibold text-gray-900 dark:text-white line-clamp-2">
                      <NuxtLink :to="`/products/${item.product_id}`" class="hover:text-primary-600">
                        {{ item.name }}
                      </NuxtLink>
                    </h3>
                    <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                      {{ $t('product.author') }}: {{ item.author }}
                    </p>
                    <p class="text-xs text-gray-500 mt-1">
                      {{ item.vendor_name }}
                    </p>
                  </div>
                  
                  <UButton
                    variant="ghost"
                    size="xs"
                    icon="i-heroicons-x-mark"
                    @click="removeItem(item.product_id)"
                  />
                </div>

                <!-- Quantity and Price -->
                <div class="flex items-center justify-between mt-4">
                  <div class="flex items-center gap-2">
                    <UButton
                      variant="outline"
                      size="xs"
                      icon="i-heroicons-minus"
                      :disabled="item.quantity <= 1"
                      @click="updateQuantity(item.product_id, item.quantity - 1)"
                    />
                    <span class="w-12 text-center font-medium">{{ item.quantity }}</span>
                    <UButton
                      variant="outline"
                      size="xs"
                      icon="i-heroicons-plus"
                      :disabled="item.quantity >= item.stock_quantity"
                      @click="updateQuantity(item.product_id, item.quantity + 1)"
                    />
                  </div>
                  
                  <div class="text-right">
                    <p class="font-semibold text-primary-600">
                      {{ formatPrice(item.price * item.quantity) }}
                    </p>
                    <p class="text-sm text-gray-500">
                      {{ formatPrice(item.price) }} {{ $t('cart.each') }}
                    </p>
                  </div>
                </div>

                <!-- Stock Warning -->
                <div v-if="item.quantity > item.stock_quantity" class="mt-2">
                  <UAlert
                    color="orange"
                    variant="soft"
                    :title="$t('cart.stock_warning')"
                    :description="$t('cart.stock_warning_desc', { available: item.stock_quantity })"
                  />
                </div>
              </div>
            </div>
          </UCard>
        </div>

        <!-- Order Summary -->
        <div class="lg:col-span-1">
          <UCard class="sticky top-8">
            <template #header>
              <h2 class="text-lg font-semibold">{{ $t('cart.order_summary') }}</h2>
            </template>

            <div class="space-y-4">
              <div class="flex justify-between">
                <span>{{ $t('cart.subtotal') }}</span>
                <span class="font-medium">{{ formatPrice(cartStore.totalAmount) }}</span>
              </div>
              
              <div class="flex justify-between">
                <span>{{ $t('cart.shipping') }}</span>
                <span class="font-medium">{{ formatPrice(shippingCost) }}</span>
              </div>
              
              <div class="flex justify-between">
                <span>{{ $t('cart.tax') }}</span>
                <span class="font-medium">{{ formatPrice(taxAmount) }}</span>
              </div>
              
              <UDivider />
              
              <div class="flex justify-between text-lg font-semibold">
                <span>{{ $t('cart.total') }}</span>
                <span class="text-primary-600">{{ formatPrice(totalWithTaxAndShipping) }}</span>
              </div>

              <!-- Promo Code -->
              <div class="space-y-2">
                <UInput
                  v-model="promoCode"
                  :placeholder="$t('cart.promo_code')"
                  size="sm"
                />
                <UButton
                  variant="outline"
                  size="sm"
                  block
                  :loading="applyingPromo"
                  @click="applyPromoCode"
                >
                  {{ $t('cart.apply_promo') }}
                </UButton>
              </div>

              <UButton
                size="lg"
                block
                :disabled="!canCheckout"
                @click="proceedToCheckout"
              >
                {{ $t('cart.checkout') }}
              </UButton>

              <UButton
                variant="outline"
                size="lg"
                block
                to="/products"
              >
                {{ $t('cart.continue_shopping') }}
              </UButton>
            </div>
          </UCard>

          <!-- Security Features -->
          <UCard class="mt-4 p-4">
            <div class="space-y-3 text-sm">
              <div class="flex items-center gap-2 text-green-600">
                <UIcon name="i-heroicons-shield-check" class="w-4 h-4" />
                <span>{{ $t('cart.secure_checkout') }}</span>
              </div>
              <div class="flex items-center gap-2 text-blue-600">
                <UIcon name="i-heroicons-truck" class="w-4 h-4" />
                <span>{{ $t('cart.free_shipping') }}</span>
              </div>
              <div class="flex items-center gap-2 text-purple-600">
                <UIcon name="i-heroicons-arrow-path" class="w-4 h-4" />
                <span>{{ $t('cart.easy_returns') }}</span>
              </div>
            </div>
          </UCard>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { $t } = useI18n()
const cartStore = useCartStore()
const authStore = useAuthStore()

// SEO
useHead({
  title: $t('cart.title')
})

// Reactive state
const promoCode = ref('')
const applyingPromo = ref(false)

// Computed
const shippingCost = computed(() => {
  return cartStore.totalAmount >= 500 ? 0 : 30 // Free shipping over 500 MAD
})

const taxAmount = computed(() => {
  return cartStore.totalAmount * 0.2 // 20% VAT
})

const totalWithTaxAndShipping = computed(() => {
  return cartStore.totalAmount + shippingCost.value + taxAmount.value
})

const canCheckout = computed(() => {
  return cartStore.items.length > 0 && 
         cartStore.items.every(item => item.quantity <= item.stock_quantity)
})

// Methods
const formatPrice = (price: number) => {
  return new Intl.NumberFormat('ar-MA', {
    style: 'currency',
    currency: 'MAD',
    minimumFractionDigits: 0
  }).format(price)
}

const removeItem = (productId: string) => {
  cartStore.removeItem(productId)
  
  const toast = useToast()
  toast.add({
    title: $t('cart.item_removed'),
    color: 'green'
  })
}

const updateQuantity = (productId: string, quantity: number) => {
  cartStore.updateQuantity(productId, quantity)
}

const applyPromoCode = async () => {
  if (!promoCode.value.trim()) return
  
  applyingPromo.value = true
  try {
    // Mock promo code validation
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    const toast = useToast()
    if (promoCode.value.toLowerCase() === 'welcome10') {
      toast.add({
        title: $t('cart.promo_applied'),
        description: $t('cart.promo_discount', { discount: '10%' }),
        color: 'green'
      })
    } else {
      toast.add({
        title: $t('cart.promo_invalid'),
        color: 'red'
      })
    }
  } catch (error) {
    console.error('Failed to apply promo code:', error)
  } finally {
    applyingPromo.value = false
  }
}

const proceedToCheckout = () => {
  if (!authStore.isAuthenticated) {
    navigateTo('/auth/login?redirect=/checkout')
  } else {
    navigateTo('/checkout')
  }
}

// Initialize
onMounted(() => {
  cartStore.loadFromStorage()
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
