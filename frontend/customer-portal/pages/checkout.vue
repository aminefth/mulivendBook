<template>
  <div class="container mx-auto px-4 py-8">
    <div class="max-w-6xl mx-auto">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-8">
        {{ $t('checkout.title') }}
      </h1>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <!-- Checkout Form -->
        <div class="lg:col-span-2 space-y-6">
          <!-- Shipping Address -->
          <UCard>
            <template #header>
              <h2 class="text-xl font-semibold flex items-center gap-2">
                <UIcon name="i-heroicons-truck" class="w-5 h-5" />
                {{ $t('checkout.shipping_address') }}
              </h2>
            </template>

            <form class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <UFormGroup :label="$t('checkout.first_name')" required>
                <UInput v-model="shippingForm.first_name" required />
              </UFormGroup>
              
              <UFormGroup :label="$t('checkout.last_name')" required>
                <UInput v-model="shippingForm.last_name" required />
              </UFormGroup>
              
              <UFormGroup :label="$t('checkout.phone')" required class="md:col-span-2">
                <UInput v-model="shippingForm.phone" type="tel" required />
              </UFormGroup>
              
              <UFormGroup :label="$t('checkout.address')" required class="md:col-span-2">
                <UInput v-model="shippingForm.address" required />
              </UFormGroup>
              
              <UFormGroup :label="$t('checkout.city')" required>
                <USelectMenu
                  v-model="shippingForm.city"
                  :options="moroccanCities"
                  required
                />
              </UFormGroup>
              
              <UFormGroup :label="$t('checkout.postal_code')" required>
                <UInput v-model="shippingForm.postal_code" required />
              </UFormGroup>
            </form>
          </UCard>

          <!-- Payment Method -->
          <UCard>
            <template #header>
              <h2 class="text-xl font-semibold flex items-center gap-2">
                <UIcon name="i-heroicons-credit-card" class="w-5 h-5" />
                {{ $t('checkout.payment_method') }}
              </h2>
            </template>

            <div class="space-y-4">
              <URadioGroup v-model="paymentMethod" :options="paymentOptions" />
              
              <!-- Credit Card Form -->
              <div v-if="paymentMethod === 'credit_card'" class="space-y-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <UFormGroup :label="$t('checkout.card_number')" required class="md:col-span-2">
                    <UInput
                      v-model="cardForm.number"
                      placeholder="1234 5678 9012 3456"
                      required
                    />
                  </UFormGroup>
                  
                  <UFormGroup :label="$t('checkout.card_name')" required class="md:col-span-2">
                    <UInput v-model="cardForm.name" required />
                  </UFormGroup>
                  
                  <UFormGroup :label="$t('checkout.expiry')" required>
                    <UInput v-model="cardForm.expiry" placeholder="MM/YY" required />
                  </UFormGroup>
                  
                  <UFormGroup :label="$t('checkout.cvv')" required>
                    <UInput v-model="cardForm.cvv" placeholder="123" required />
                  </UFormGroup>
                </div>
              </div>

              <!-- Bank Transfer Info -->
              <div v-if="paymentMethod === 'bank_transfer'" class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                <h3 class="font-semibold text-blue-900 dark:text-blue-100 mb-2">
                  {{ $t('checkout.bank_details') }}
                </h3>
                <div class="text-sm text-blue-800 dark:text-blue-200 space-y-1">
                  <p>{{ $t('checkout.bank_name') }}: Attijariwafa Bank</p>
                  <p>{{ $t('checkout.account_number') }}: 007 640 0000123456789</p>
                  <p>{{ $t('checkout.swift_code') }}: BCMAMAMC</p>
                </div>
              </div>
            </div>
          </UCard>

          <!-- Order Notes -->
          <UCard>
            <template #header>
              <h2 class="text-xl font-semibold flex items-center gap-2">
                <UIcon name="i-heroicons-chat-bubble-left-ellipsis" class="w-5 h-5" />
                {{ $t('checkout.order_notes') }}
              </h2>
            </template>
            
            <UTextarea
              v-model="orderNotes"
              :placeholder="$t('checkout.order_notes_placeholder')"
              :rows="3"
            />
          </UCard>
        </div>

        <!-- Order Summary -->
        <div class="lg:col-span-1">
          <UCard class="sticky top-8">
            <template #header>
              <h2 class="text-xl font-semibold">{{ $t('checkout.order_summary') }}</h2>
            </template>

            <!-- Items -->
            <div class="space-y-4 mb-6">
              <div
                v-for="item in cartStore.items"
                :key="item.id"
                class="flex items-center gap-3"
              >
                <div class="w-12 h-16 bg-gray-100 dark:bg-gray-800 rounded overflow-hidden flex-shrink-0">
                  <img
                    :src="item.image || '/placeholder-book.jpg'"
                    :alt="item.name"
                    class="w-full h-full object-cover"
                  />
                </div>
                <div class="flex-1 min-w-0">
                  <h3 class="font-medium text-sm line-clamp-2">{{ item.name }}</h3>
                  <p class="text-xs text-gray-500">{{ $t('cart.quantity') }}: {{ item.quantity }}</p>
                </div>
                <div class="text-sm font-medium">
                  {{ formatPrice(item.price * item.quantity) }}
                </div>
              </div>
            </div>

            <!-- Totals -->
            <div class="space-y-3 border-t pt-4">
              <div class="flex justify-between">
                <span>{{ $t('cart.subtotal') }}</span>
                <span>{{ formatPrice(cartStore.totalAmount) }}</span>
              </div>
              
              <div class="flex justify-between">
                <span>{{ $t('cart.shipping') }}</span>
                <span>{{ formatPrice(shippingCost) }}</span>
              </div>
              
              <div class="flex justify-between">
                <span>{{ $t('cart.tax') }}</span>
                <span>{{ formatPrice(taxAmount) }}</span>
              </div>
              
              <UDivider />
              
              <div class="flex justify-between text-lg font-semibold">
                <span>{{ $t('cart.total') }}</span>
                <span class="text-primary-600">{{ formatPrice(totalAmount) }}</span>
              </div>
            </div>

            <!-- Place Order Button -->
            <UButton
              size="lg"
              block
              class="mt-6"
              :loading="placingOrder"
              :disabled="!canPlaceOrder"
              @click="placeOrder"
            >
              {{ $t('checkout.place_order') }}
            </UButton>

            <!-- Security Info -->
            <div class="mt-4 p-3 bg-green-50 dark:bg-green-900/20 rounded-lg">
              <div class="flex items-center gap-2 text-green-700 dark:text-green-300 text-sm">
                <UIcon name="i-heroicons-shield-check" class="w-4 h-4" />
                <span>{{ $t('checkout.secure_payment') }}</span>
              </div>
            </div>
          </UCard>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

const { $t } = useI18n()
const cartStore = useCartStore()
const authStore = useAuthStore()
const config = useRuntimeConfig()

// SEO
useHead({
  title: $t('checkout.title')
})

// Reactive state
const placingOrder = ref(false)
const paymentMethod = ref('credit_card')
const orderNotes = ref('')

const shippingForm = reactive({
  first_name: '',
  last_name: '',
  phone: '',
  address: '',
  city: '',
  postal_code: ''
})

const cardForm = reactive({
  number: '',
  name: '',
  expiry: '',
  cvv: ''
})

// Options
const paymentOptions = computed(() => [
  {
    value: 'credit_card',
    label: $t('checkout.credit_card'),
    description: $t('checkout.credit_card_desc')
  },
  {
    value: 'bank_transfer',
    label: $t('checkout.bank_transfer'),
    description: $t('checkout.bank_transfer_desc')
  },
  {
    value: 'cash_on_delivery',
    label: $t('checkout.cash_on_delivery'),
    description: $t('checkout.cash_on_delivery_desc')
  }
])

const moroccanCities = [
  'Casablanca', 'Rabat', 'Marrakech', 'Fès', 'Tangier', 'Agadir', 
  'Meknes', 'Oujda', 'Kenitra', 'Tetouan', 'Safi', 'Mohammedia'
]

// Computed
const shippingCost = computed(() => {
  return cartStore.totalAmount >= 500 ? 0 : 30
})

const taxAmount = computed(() => {
  return cartStore.totalAmount * 0.2
})

const totalAmount = computed(() => {
  return cartStore.totalAmount + shippingCost.value + taxAmount.value
})

const canPlaceOrder = computed(() => {
  const hasShippingInfo = shippingForm.first_name && 
                         shippingForm.last_name && 
                         shippingForm.phone && 
                         shippingForm.address && 
                         shippingForm.city && 
                         shippingForm.postal_code

  const hasPaymentInfo = paymentMethod.value === 'cash_on_delivery' || 
                        paymentMethod.value === 'bank_transfer' ||
                        (paymentMethod.value === 'credit_card' && 
                         cardForm.number && cardForm.name && cardForm.expiry && cardForm.cvv)

  return hasShippingInfo && hasPaymentInfo && cartStore.items.length > 0
})

// Methods
const formatPrice = (price: number) => {
  return new Intl.NumberFormat('ar-MA', {
    style: 'currency',
    currency: 'MAD',
    minimumFractionDigits: 0
  }).format(price)
}

const placeOrder = async () => {
  placingOrder.value = true
  
  try {
    const orderData = {
      items: cartStore.items.map(item => ({
        product_id: item.product_id,
        quantity: item.quantity,
        unit_price: item.price
      })),
      shipping_address: `${shippingForm.address}, ${shippingForm.city} ${shippingForm.postal_code}`,
      billing_address: `${shippingForm.address}, ${shippingForm.city} ${shippingForm.postal_code}`,
      payment_method: paymentMethod.value,
      payment_details: paymentMethod.value === 'credit_card' ? {
        card_last_four: cardForm.number.slice(-4),
        card_type: 'visa' // This would be detected from card number
      } : {},
      notes: orderNotes.value,
      total_amount: totalAmount.value
    }

    const response = await $fetch('/orders', {
      baseURL: config.public.orderServiceUrl,
      method: 'POST',
      headers: authStore.getAuthHeaders(),
      body: orderData
    })

    // Clear cart
    cartStore.clearCart()

    const toast = useToast()
    toast.add({
      title: $t('checkout.order_placed'),
      description: $t('checkout.order_confirmation', { orderNumber: response.order_number }),
      color: 'green'
    })

    // Redirect to order confirmation
    await navigateTo(`/orders/${response.id}`)
    
  } catch (error) {
    console.error('Failed to place order:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('checkout.order_error'),
      color: 'red'
    })
  } finally {
    placingOrder.value = false
  }
}

// Initialize form with user data
onMounted(() => {
  if (authStore.user) {
    const [firstName, ...lastNameParts] = authStore.user.full_name.split(' ')
    shippingForm.first_name = firstName
    shippingForm.last_name = lastNameParts.join(' ')
  }
  
  // Redirect if cart is empty
  if (cartStore.isEmpty) {
    navigateTo('/cart')
  }
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
