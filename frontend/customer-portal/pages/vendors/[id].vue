<template>
  <div v-if="loading" class="container mx-auto px-4 py-8">
    <USkeleton class="h-8 w-64 mb-4" />
    <USkeleton class="h-64 w-full mb-8" />
    <USkeleton class="h-96 w-full" />
  </div>

  <div v-else-if="vendor" class="container mx-auto px-4 py-8">
    <!-- Vendor Header -->
    <div class="bg-gradient-to-r from-primary-600 to-primary-800 rounded-xl p-8 mb-8 text-white">
      <div class="flex flex-col md:flex-row items-start md:items-center gap-6">
        <div class="w-24 h-24 bg-white/20 rounded-xl overflow-hidden flex-shrink-0">
          <img
            :src="vendor.logo || '/placeholder-vendor.jpg'"
            :alt="vendor.business_name"
            class="w-full h-full object-cover"
          />
        </div>
        
        <div class="flex-1">
          <h1 class="text-3xl font-bold mb-2">{{ vendor.business_name }}</h1>
          <p class="text-primary-100 mb-3">{{ vendor.description }}</p>
          
          <div class="flex flex-wrap items-center gap-4 text-sm">
            <div class="flex items-center gap-1">
              <UIcon name="i-heroicons-star-solid" class="w-4 h-4 text-yellow-400" />
              <span>{{ vendor.rating?.toFixed(1) || '0.0' }} ({{ vendor.total_reviews || 0 }} {{ $t('vendors.reviews') }})</span>
            </div>
            
            <div class="flex items-center gap-1">
              <UIcon name="i-heroicons-map-pin" class="w-4 h-4" />
              <span>{{ vendor.city }}, {{ vendor.country }}</span>
            </div>
            
            <div class="flex items-center gap-1">
              <UIcon name="i-heroicons-calendar" class="w-4 h-4" />
              <span>{{ $t('vendors.member_since') }} {{ formatDate(vendor.created_at) }}</span>
            </div>
          </div>
        </div>
        
        <div class="flex flex-col gap-2">
          <UButton
            variant="white"
            @click="toggleFollow"
            :loading="followLoading"
          >
            <UIcon :name="isFollowing ? 'i-heroicons-heart-solid' : 'i-heroicons-heart'" class="w-4 h-4 mr-2" />
            {{ isFollowing ? $t('vendors.following') : $t('vendors.follow') }}
          </UButton>
          
          <UButton
            variant="outline"
            color="white"
            @click="contactVendor"
          >
            <UIcon name="i-heroicons-envelope" class="w-4 h-4 mr-2" />
            {{ $t('vendors.contact') }}
          </UButton>
        </div>
      </div>
    </div>

    <!-- Stats Cards -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
      <UCard>
        <div class="text-center">
          <div class="text-2xl font-bold text-primary-600">{{ vendor.total_books || 0 }}</div>
          <div class="text-sm text-gray-600 dark:text-gray-400">{{ $t('vendors.total_books') }}</div>
        </div>
      </UCard>
      
      <UCard>
        <div class="text-center">
          <div class="text-2xl font-bold text-green-600">{{ vendor.total_sales || 0 }}</div>
          <div class="text-sm text-gray-600 dark:text-gray-400">{{ $t('vendors.total_sales') }}</div>
        </div>
      </UCard>
      
      <UCard>
        <div class="text-center">
          <div class="text-2xl font-bold text-blue-600">{{ vendor.followers_count || 0 }}</div>
          <div class="text-sm text-gray-600 dark:text-gray-400">{{ $t('vendors.followers') }}</div>
        </div>
      </UCard>
      
      <UCard>
        <div class="text-center">
          <div class="text-2xl font-bold text-purple-600">{{ vendor.years_active || 0 }}</div>
          <div class="text-sm text-gray-600 dark:text-gray-400">{{ $t('vendors.years_active') }}</div>
        </div>
      </UCard>
    </div>

    <!-- Tabs -->
    <UTabs v-model="activeTab" :items="tabItems" class="mb-8">
      <!-- Books Tab -->
      <template #books>
        <div class="space-y-6">
          <!-- Filters -->
          <div class="flex flex-col md:flex-row gap-4">
            <UInput
              v-model="bookSearch"
              icon="i-heroicons-magnifying-glass"
              :placeholder="$t('vendors.search_books')"
              class="flex-1"
              @input="debouncedBookSearch"
            />
            
            <USelectMenu
              v-model="bookCategory"
              :options="categoryOptions"
              :placeholder="$t('vendors.filter_category')"
              class="w-full md:w-48"
            />
            
            <USelectMenu
              v-model="bookSort"
              :options="sortOptions"
              class="w-full md:w-48"
            />
          </div>

          <!-- Books Grid -->
          <div v-if="booksLoading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            <USkeleton
              v-for="i in 8"
              :key="i"
              class="h-80 rounded-lg"
            />
          </div>
          
          <div v-else-if="books.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            <ProductCard
              v-for="book in books"
              :key="book.id"
              :product="book"
            />
          </div>
          
          <div v-else class="text-center py-16">
            <UIcon name="i-heroicons-book-open" class="w-16 h-16 text-gray-400 mx-auto mb-4" />
            <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
              {{ $t('vendors.no_books') }}
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
              {{ $t('vendors.no_books_desc') }}
            </p>
          </div>

          <!-- Pagination -->
          <div v-if="totalBooksPages > 1" class="flex justify-center">
            <UPagination
              v-model="currentBooksPage"
              :page-count="booksPageSize"
              :total="totalBooks"
              :max="5"
            />
          </div>
        </div>
      </template>

      <!-- About Tab -->
      <template #about>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <UCard>
            <template #header>
              <h3 class="text-lg font-semibold">{{ $t('vendors.about_business') }}</h3>
            </template>
            
            <div class="space-y-4">
              <div>
                <h4 class="font-medium text-gray-900 dark:text-white mb-2">{{ $t('vendors.description') }}</h4>
                <p class="text-gray-600 dark:text-gray-400">{{ vendor.description }}</p>
              </div>
              
              <div>
                <h4 class="font-medium text-gray-900 dark:text-white mb-2">{{ $t('vendors.specialties') }}</h4>
                <div class="flex flex-wrap gap-2">
                  <UBadge
                    v-for="category in vendor.categories"
                    :key="category"
                    :label="category"
                    variant="soft"
                  />
                </div>
              </div>
              
              <div>
                <h4 class="font-medium text-gray-900 dark:text-white mb-2">{{ $t('vendors.languages') }}</h4>
                <div class="flex flex-wrap gap-2">
                  <UBadge
                    v-for="lang in vendor.languages || ['Arabic', 'French']"
                    :key="lang"
                    :label="lang"
                    variant="outline"
                  />
                </div>
              </div>
            </div>
          </UCard>
          
          <UCard>
            <template #header>
              <h3 class="text-lg font-semibold">{{ $t('vendors.contact_info') }}</h3>
            </template>
            
            <div class="space-y-4">
              <div class="flex items-center gap-3">
                <UIcon name="i-heroicons-user" class="w-5 h-5 text-gray-400" />
                <div>
                  <div class="font-medium">{{ vendor.owner_name }}</div>
                  <div class="text-sm text-gray-600 dark:text-gray-400">{{ $t('vendors.owner') }}</div>
                </div>
              </div>
              
              <div v-if="vendor.phone" class="flex items-center gap-3">
                <UIcon name="i-heroicons-phone" class="w-5 h-5 text-gray-400" />
                <div>
                  <div class="font-medium">{{ vendor.phone }}</div>
                  <div class="text-sm text-gray-600 dark:text-gray-400">{{ $t('vendors.phone') }}</div>
                </div>
              </div>
              
              <div v-if="vendor.email" class="flex items-center gap-3">
                <UIcon name="i-heroicons-envelope" class="w-5 h-5 text-gray-400" />
                <div>
                  <div class="font-medium">{{ vendor.email }}</div>
                  <div class="text-sm text-gray-600 dark:text-gray-400">{{ $t('vendors.email') }}</div>
                </div>
              </div>
              
              <div class="flex items-center gap-3">
                <UIcon name="i-heroicons-map-pin" class="w-5 h-5 text-gray-400" />
                <div>
                  <div class="font-medium">{{ vendor.address }}</div>
                  <div class="text-sm text-gray-600 dark:text-gray-400">{{ vendor.city }}, {{ vendor.country }}</div>
                </div>
              </div>
            </div>
          </UCard>
        </div>
      </template>

      <!-- Reviews Tab -->
      <template #reviews>
        <div class="space-y-6">
          <!-- Reviews Summary -->
          <UCard>
            <div class="flex items-center gap-6">
              <div class="text-center">
                <div class="text-4xl font-bold text-gray-900 dark:text-white">
                  {{ vendor.rating?.toFixed(1) || '0.0' }}
                </div>
                <div class="flex justify-center mb-2">
                  <UIcon
                    v-for="i in 5"
                    :key="i"
                    name="i-heroicons-star-solid"
                    :class="i <= Math.floor(vendor.rating || 0) ? 'text-yellow-400' : 'text-gray-300'"
                    class="w-5 h-5"
                  />
                </div>
                <div class="text-sm text-gray-600 dark:text-gray-400">
                  {{ vendor.total_reviews || 0 }} {{ $t('vendors.reviews') }}
                </div>
              </div>
              
              <div class="flex-1">
                <div v-for="(count, rating) in ratingDistribution" :key="rating" class="flex items-center gap-2 mb-2">
                  <span class="text-sm w-8">{{ rating }}★</span>
                  <div class="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div
                      class="bg-yellow-400 h-2 rounded-full"
                      :style="{ width: `${(count / (vendor.total_reviews || 1)) * 100}%` }"
                    ></div>
                  </div>
                  <span class="text-sm text-gray-600 dark:text-gray-400 w-8">{{ count }}</span>
                </div>
              </div>
            </div>
          </UCard>

          <!-- Reviews List -->
          <div v-if="reviews.length > 0" class="space-y-4">
            <div
              v-for="review in reviews"
              :key="review.id"
              class="border border-gray-200 dark:border-gray-700 rounded-lg p-4"
            >
              <div class="flex items-start gap-4">
                <div class="w-10 h-10 bg-gray-100 dark:bg-gray-800 rounded-full flex items-center justify-center">
                  <span class="text-sm font-medium">{{ review.user_name?.charAt(0) }}</span>
                </div>
                
                <div class="flex-1">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="font-medium">{{ review.user_name }}</span>
                    <div class="flex">
                      <UIcon
                        v-for="i in 5"
                        :key="i"
                        name="i-heroicons-star-solid"
                        :class="i <= review.rating ? 'text-yellow-400' : 'text-gray-300'"
                        class="w-4 h-4"
                      />
                    </div>
                    <span class="text-sm text-gray-500">{{ formatDate(review.created_at) }}</span>
                  </div>
                  
                  <p class="text-gray-600 dark:text-gray-400">{{ review.comment }}</p>
                </div>
              </div>
            </div>
          </div>
          
          <div v-else class="text-center py-16">
            <UIcon name="i-heroicons-chat-bubble-left-ellipsis" class="w-16 h-16 text-gray-400 mx-auto mb-4" />
            <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
              {{ $t('vendors.no_reviews') }}
            </h3>
            <p class="text-gray-600 dark:text-gray-400">
              {{ $t('vendors.no_reviews_desc') }}
            </p>
          </div>
        </div>
      </template>
    </UTabs>
  </div>

  <!-- Error State -->
  <div v-else class="container mx-auto px-4 py-16 text-center">
    <UIcon name="i-heroicons-exclamation-triangle" class="w-16 h-16 text-red-400 mx-auto mb-4" />
    <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
      {{ $t('vendors.not_found') }}
    </h1>
    <p class="text-gray-600 dark:text-gray-400 mb-6">
      {{ $t('vendors.not_found_desc') }}
    </p>
    <UButton @click="navigateTo('/vendors')">
      {{ $t('vendors.back_to_vendors') }}
    </UButton>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const { $t } = useI18n()
const config = useRuntimeConfig()

// SEO
useHead({
  title: computed(() => vendor.value ? `${vendor.value.business_name} - ${$t('vendors.title')}` : $t('vendors.title'))
})

// Reactive state
const loading = ref(true)
const vendor = ref(null)
const isFollowing = ref(false)
const followLoading = ref(false)
const activeTab = ref(0)

// Books state
const books = ref([])
const booksLoading = ref(false)
const bookSearch = ref('')
const bookCategory = ref('')
const bookSort = ref('name')
const currentBooksPage = ref(1)
const booksPageSize = ref(12)
const totalBooks = ref(0)

// Reviews state
const reviews = ref([])
const ratingDistribution = ref({ 5: 0, 4: 0, 3: 0, 2: 0, 1: 0 })

// Computed
const totalBooksPages = computed(() => Math.ceil(totalBooks.value / booksPageSize.value))

const tabItems = computed(() => [
  { key: 'books', label: $t('vendors.books'), slot: 'books' },
  { key: 'about', label: $t('vendors.about'), slot: 'about' },
  { key: 'reviews', label: $t('vendors.reviews'), slot: 'reviews' }
])

const categoryOptions = computed(() => [
  { label: $t('vendors.all_categories'), value: '' },
  { label: $t('home.categories.islamic_studies'), value: 'islamic-studies' },
  { label: $t('home.categories.arabic_literature'), value: 'arabic-literature' },
  { label: $t('home.categories.french_literature'), value: 'french-literature' },
  { label: $t('home.categories.academic'), value: 'academic' },
  { label: $t('home.categories.business'), value: 'business' }
])

const sortOptions = computed(() => [
  { label: $t('vendors.sort_name'), value: 'name' },
  { label: $t('vendors.sort_price_low'), value: 'price_asc' },
  { label: $t('vendors.sort_price_high'), value: 'price_desc' },
  { label: $t('vendors.sort_newest'), value: 'created_at' }
])

// Methods
const fetchVendor = async () => {
  try {
    const response = await $fetch(`/vendors/${route.params.id}`, {
      baseURL: config.public.vendorServiceUrl
    })
    vendor.value = response
  } catch (error) {
    console.error('Failed to fetch vendor:', error)
  } finally {
    loading.value = false
  }
}

const fetchBooks = async () => {
  booksLoading.value = true
  
  try {
    const params = new URLSearchParams({
      vendor_id: route.params.id as string,
      page: currentBooksPage.value.toString(),
      limit: booksPageSize.value.toString(),
      sort: bookSort.value
    })
    
    if (bookSearch.value.trim()) {
      params.append('search', bookSearch.value.trim())
    }
    
    if (bookCategory.value) {
      params.append('category', bookCategory.value)
    }

    const response = await $fetch(`/products?${params}`, {
      baseURL: config.public.catalogServiceUrl
    })

    books.value = response.products || []
    totalBooks.value = response.total || 0
  } catch (error) {
    console.error('Failed to fetch books:', error)
  } finally {
    booksLoading.value = false
  }
}

const fetchReviews = async () => {
  try {
    const response = await $fetch(`/vendors/${route.params.id}/reviews`, {
      baseURL: config.public.vendorServiceUrl
    })
    reviews.value = response.reviews || []
    ratingDistribution.value = response.rating_distribution || { 5: 0, 4: 0, 3: 0, 2: 0, 1: 0 }
  } catch (error) {
    console.error('Failed to fetch reviews:', error)
  }
}

const toggleFollow = async () => {
  followLoading.value = true
  
  try {
    // Mock follow/unfollow
    await new Promise(resolve => setTimeout(resolve, 500))
    isFollowing.value = !isFollowing.value
    
    const toast = useToast()
    toast.add({
      title: isFollowing.value ? $t('vendors.now_following') : $t('vendors.unfollowed'),
      color: isFollowing.value ? 'green' : 'gray'
    })
  } catch (error) {
    console.error('Failed to toggle follow:', error)
  } finally {
    followLoading.value = false
  }
}

const contactVendor = () => {
  // Open contact modal or navigate to contact page
  console.log('Contact vendor')
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString()
}

const debouncedBookSearch = useDebounceFn(() => {
  currentBooksPage.value = 1
  fetchBooks()
}, 300)

// Watchers
watch([bookCategory, bookSort, currentBooksPage], () => {
  fetchBooks()
})

watch(activeTab, (newTab) => {
  if (newTab === 0 && books.value.length === 0) {
    fetchBooks()
  } else if (newTab === 2 && reviews.value.length === 0) {
    fetchReviews()
  }
})

// Initialize
onMounted(async () => {
  await fetchVendor()
  if (vendor.value) {
    fetchBooks()
  }
})
</script>
