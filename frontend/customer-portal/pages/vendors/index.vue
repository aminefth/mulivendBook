<template>
  <div class="container mx-auto px-4 py-8">
    <!-- Header -->
    <div class="text-center mb-12">
      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
        {{ $t('vendors.title') }}
      </h1>
      <p class="text-lg text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
        {{ $t('vendors.subtitle') }}
      </p>
    </div>

    <!-- Filters -->
    <div class="flex flex-col md:flex-row gap-4 mb-8">
      <div class="flex-1">
        <UInput
          v-model="searchQuery"
          size="lg"
          icon="i-heroicons-magnifying-glass"
          :placeholder="$t('vendors.search_placeholder')"
          @input="debouncedSearch"
        />
      </div>
      
      <USelectMenu
        v-model="selectedCity"
        :options="cityOptions"
        size="lg"
        :placeholder="$t('vendors.filter_city')"
        class="w-full md:w-48"
      />
      
      <USelectMenu
        v-model="sortBy"
        :options="sortOptions"
        size="lg"
        class="w-full md:w-48"
      />
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <USkeleton
        v-for="i in 6"
        :key="i"
        class="h-80 rounded-lg"
      />
    </div>

    <!-- Vendors Grid -->
    <div v-else-if="vendors.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <VendorCard
        v-for="vendor in vendors"
        :key="vendor.id"
        :vendor="vendor"
        @click="navigateTo(`/vendors/${vendor.id}`)"
      />
    </div>

    <!-- Empty State -->
    <div v-else class="text-center py-16">
      <UIcon name="i-heroicons-building-storefront" class="w-16 h-16 text-gray-400 mx-auto mb-4" />
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
        {{ $t('vendors.no_vendors') }}
      </h3>
      <p class="text-gray-600 dark:text-gray-400">
        {{ $t('vendors.no_vendors_desc') }}
      </p>
    </div>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="flex justify-center mt-12">
      <UPagination
        v-model="currentPage"
        :page-count="pageSize"
        :total="totalVendors"
        :max="5"
        show-last
        show-first
      />
    </div>
  </div>
</template>

<script setup lang="ts">
const { $t } = useI18n()
const config = useRuntimeConfig()

// SEO
useHead({
  title: $t('vendors.title'),
  meta: [
    { name: 'description', content: $t('vendors.meta_description') }
  ]
})

// Reactive state
const loading = ref(true)
const vendors = ref([])
const searchQuery = ref('')
const selectedCity = ref('')
const sortBy = ref('name')
const currentPage = ref(1)
const pageSize = ref(12)
const totalVendors = ref(0)

// Computed
const totalPages = computed(() => Math.ceil(totalVendors.value / pageSize.value))

const cityOptions = computed(() => [
  { label: $t('vendors.all_cities'), value: '' },
  { label: 'Casablanca', value: 'casablanca' },
  { label: 'Rabat', value: 'rabat' },
  { label: 'Marrakech', value: 'marrakech' },
  { label: 'Fès', value: 'fes' },
  { label: 'Tangier', value: 'tangier' },
  { label: 'Agadir', value: 'agadir' }
])

const sortOptions = computed(() => [
  { label: $t('vendors.sort_name'), value: 'name' },
  { label: $t('vendors.sort_rating'), value: 'rating' },
  { label: $t('vendors.sort_books'), value: 'total_books' },
  { label: $t('vendors.sort_newest'), value: 'created_at' }
])

// Methods
const fetchVendors = async () => {
  loading.value = true
  
  try {
    const params = new URLSearchParams({
      page: currentPage.value.toString(),
      limit: pageSize.value.toString(),
      sort: sortBy.value
    })
    
    if (searchQuery.value.trim()) {
      params.append('search', searchQuery.value.trim())
    }
    
    if (selectedCity.value) {
      params.append('city', selectedCity.value)
    }

    const response = await $fetch(`/vendors?${params}`, {
      baseURL: config.public.vendorServiceUrl
    })

    vendors.value = response.vendors || []
    totalVendors.value = response.total || 0
  } catch (error) {
    console.error('Failed to fetch vendors:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('vendors.fetch_error'),
      color: 'red'
    })
  } finally {
    loading.value = false
  }
}

const debouncedSearch = useDebounceFn(() => {
  currentPage.value = 1
  fetchVendors()
}, 300)

// Watchers
watch([selectedCity, sortBy, currentPage], () => {
  fetchVendors()
})

// Initialize
onMounted(() => {
  fetchVendors()
})
</script>
