<template>
  <div class="container mx-auto px-4 py-8">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-8">
        {{ $t('orders.title') }}
      </h1>

      <!-- Filters -->
      <div class="flex flex-col md:flex-row gap-4 mb-6">
        <USelectMenu
          v-model="statusFilter"
          :options="statusOptions"
          :placeholder="$t('orders.filter_status')"
          class="w-full md:w-48"
        />
        
        <USelectMenu
          v-model="timeFilter"
          :options="timeOptions"
          :placeholder="$t('orders.filter_time')"
          class="w-full md:w-48"
        />
        
        <UInput
          v-model="searchQuery"
          icon="i-heroicons-magnifying-glass"
          :placeholder="$t('orders.search_placeholder')"
          class="flex-1"
        />
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="space-y-4">
        <USkeleton
          v-for="i in 3"
          :key="i"
          class="h-32 rounded-lg"
        />
      </div>

      <!-- Orders List -->
      <div v-else-if="orders.length > 0" class="space-y-4">
        <UCard
          v-for="order in orders"
          :key="order.id"
          class="cursor-pointer hover:shadow-md transition-shadow"
          @click="navigateTo(`/orders/${order.id}`)"
        >
          <div class="flex flex-col md:flex-row md:items-center gap-4">
            <!-- Order Info -->
            <div class="flex-1">
              <div class="flex items-center gap-3 mb-2">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                  {{ $t('orders.order_number') }} #{{ order.order_number }}
                </h3>
                <UBadge
                  :color="getStatusColor(order.status)"
                  :label="$t(`orders.status.${order.status}`)"
                />
              </div>
              
              <div class="grid grid-cols-1 md:grid-cols-3 gap-2 text-sm text-gray-600 dark:text-gray-400">
                <div class="flex items-center gap-1">
                  <UIcon name="i-heroicons-calendar" class="w-4 h-4" />
                  <span>{{ formatDate(order.created_at) }}</span>
                </div>
                
                <div class="flex items-center gap-1">
                  <UIcon name="i-heroicons-shopping-bag" class="w-4 h-4" />
                  <span>{{ order.total_items }} {{ $t('orders.items') }}</span>
                </div>
                
                <div class="flex items-center gap-1">
                  <UIcon name="i-heroicons-credit-card" class="w-4 h-4" />
                  <span>{{ $t(`orders.payment.${order.payment_method}`) }}</span>
                </div>
              </div>
            </div>

            <!-- Order Total -->
            <div class="text-right">
              <div class="text-2xl font-bold text-gray-900 dark:text-white">
                {{ formatPrice(order.total_amount) }}
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                {{ $t('orders.total') }}
              </div>
            </div>

            <!-- Actions -->
            <div class="flex gap-2">
              <UButton
                variant="outline"
                size="sm"
                @click.stop="viewOrder(order.id)"
              >
                {{ $t('orders.view_details') }}
              </UButton>
              
              <UDropdown
                v-if="canPerformActions(order.status)"
                :items="getOrderActions(order)"
                @click.stop
              >
                <UButton
                  variant="ghost"
                  size="sm"
                  icon="i-heroicons-ellipsis-vertical"
                />
              </UDropdown>
            </div>
          </div>

          <!-- Order Items Preview -->
          <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
            <div class="flex items-center gap-3 overflow-x-auto">
              <div
                v-for="item in order.items.slice(0, 3)"
                :key="item.id"
                class="flex items-center gap-2 flex-shrink-0"
              >
                <div class="w-10 h-12 bg-gray-100 dark:bg-gray-800 rounded overflow-hidden">
                  <img
                    :src="item.product_image || '/placeholder-book.jpg'"
                    :alt="item.product_name"
                    class="w-full h-full object-cover"
                  />
                </div>
                <div class="min-w-0">
                  <div class="text-sm font-medium truncate">{{ item.product_name }}</div>
                  <div class="text-xs text-gray-500">{{ $t('orders.qty') }}: {{ item.quantity }}</div>
                </div>
              </div>
              
              <div v-if="order.items.length > 3" class="text-sm text-gray-500 flex-shrink-0">
                +{{ order.items.length - 3 }} {{ $t('orders.more_items') }}
              </div>
            </div>
          </div>
        </UCard>
      </div>

      <!-- Empty State -->
      <div v-else class="text-center py-16">
        <UIcon name="i-heroicons-shopping-bag" class="w-16 h-16 text-gray-400 mx-auto mb-4" />
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
          {{ $t('orders.no_orders') }}
        </h3>
        <p class="text-gray-600 dark:text-gray-400 mb-6">
          {{ $t('orders.no_orders_desc') }}
        </p>
        <UButton @click="navigateTo('/products')">
          {{ $t('orders.start_shopping') }}
        </UButton>
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="flex justify-center mt-8">
        <UPagination
          v-model="currentPage"
          :page-count="pageSize"
          :total="totalOrders"
          :max="5"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

const { $t } = useI18n()
const authStore = useAuthStore()
const config = useRuntimeConfig()

// SEO
useHead({
  title: $t('orders.title')
})

// Reactive state
const loading = ref(true)
const orders = ref([])
const statusFilter = ref('')
const timeFilter = ref('')
const searchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(10)
const totalOrders = ref(0)

// Computed
const totalPages = computed(() => Math.ceil(totalOrders.value / pageSize.value))

const statusOptions = computed(() => [
  { label: $t('orders.all_statuses'), value: '' },
  { label: $t('orders.status.pending'), value: 'pending' },
  { label: $t('orders.status.confirmed'), value: 'confirmed' },
  { label: $t('orders.status.processing'), value: 'processing' },
  { label: $t('orders.status.shipped'), value: 'shipped' },
  { label: $t('orders.status.delivered'), value: 'delivered' },
  { label: $t('orders.status.cancelled'), value: 'cancelled' }
])

const timeOptions = computed(() => [
  { label: $t('orders.all_time'), value: '' },
  { label: $t('orders.last_30_days'), value: '30d' },
  { label: $t('orders.last_3_months'), value: '3m' },
  { label: $t('orders.last_6_months'), value: '6m' },
  { label: $t('orders.last_year'), value: '1y' }
])

// Methods
const fetchOrders = async () => {
  loading.value = true
  
  try {
    const params = new URLSearchParams({
      page: currentPage.value.toString(),
      limit: pageSize.value.toString()
    })
    
    if (statusFilter.value) {
      params.append('status', statusFilter.value)
    }
    
    if (timeFilter.value) {
      params.append('time_filter', timeFilter.value)
    }
    
    if (searchQuery.value.trim()) {
      params.append('search', searchQuery.value.trim())
    }

    const response = await $fetch(`/orders/user/${authStore.user.id}?${params}`, {
      baseURL: config.public.orderServiceUrl,
      headers: authStore.getAuthHeaders()
    })

    orders.value = response.orders || []
    totalOrders.value = response.total || 0
  } catch (error) {
    console.error('Failed to fetch orders:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('orders.fetch_error'),
      color: 'red'
    })
  } finally {
    loading.value = false
  }
}

const formatPrice = (price: number) => {
  return new Intl.NumberFormat('ar-MA', {
    style: 'currency',
    currency: 'MAD',
    minimumFractionDigits: 0
  }).format(price)
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString()
}

const getStatusColor = (status: string) => {
  const colors = {
    pending: 'yellow',
    confirmed: 'blue',
    processing: 'purple',
    shipped: 'indigo',
    delivered: 'green',
    cancelled: 'red'
  }
  return colors[status] || 'gray'
}

const canPerformActions = (status: string) => {
  return ['pending', 'confirmed', 'processing', 'shipped'].includes(status)
}

const getOrderActions = (order: any) => {
  const actions = []
  
  if (order.status === 'pending') {
    actions.push([
      {
        label: $t('orders.cancel_order'),
        icon: 'i-heroicons-x-mark',
        click: () => cancelOrder(order.id)
      }
    ])
  }
  
  if (['shipped', 'delivered'].includes(order.status)) {
    actions.push([
      {
        label: $t('orders.track_order'),
        icon: 'i-heroicons-truck',
        click: () => trackOrder(order.id)
      }
    ])
  }
  
  if (order.status === 'delivered') {
    actions.push([
      {
        label: $t('orders.reorder'),
        icon: 'i-heroicons-arrow-path',
        click: () => reorder(order.id)
      }
    ])
  }
  
  return actions
}

const viewOrder = (orderId: string) => {
  navigateTo(`/orders/${orderId}`)
}

const cancelOrder = async (orderId: string) => {
  try {
    await $fetch(`/orders/${orderId}/cancel`, {
      baseURL: config.public.orderServiceUrl,
      method: 'POST',
      headers: authStore.getAuthHeaders()
    })
    
    const toast = useToast()
    toast.add({
      title: $t('orders.order_cancelled'),
      color: 'green'
    })
    
    fetchOrders()
  } catch (error) {
    console.error('Failed to cancel order:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('orders.cancel_error'),
      color: 'red'
    })
  }
}

const trackOrder = (orderId: string) => {
  navigateTo(`/orders/${orderId}/tracking`)
}

const reorder = async (orderId: string) => {
  try {
    const response = await $fetch(`/orders/${orderId}/reorder`, {
      baseURL: config.public.orderServiceUrl,
      method: 'POST',
      headers: authStore.getAuthHeaders()
    })
    
    const toast = useToast()
    toast.add({
      title: $t('orders.items_added_to_cart'),
      color: 'green'
    })
    
    navigateTo('/cart')
  } catch (error) {
    console.error('Failed to reorder:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('orders.reorder_error'),
      color: 'red'
    })
  }
}

const debouncedSearch = useDebounceFn(() => {
  currentPage.value = 1
  fetchOrders()
}, 300)

// Watchers
watch([statusFilter, timeFilter, currentPage], () => {
  fetchOrders()
})

watch(searchQuery, debouncedSearch)

// Initialize
onMounted(() => {
  fetchOrders()
})
</script>
