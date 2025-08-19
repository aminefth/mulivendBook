<template>
  <div>
    <!-- Hero Section -->
    <section class="relative bg-gradient-to-br from-primary-600 via-primary-700 to-primary-800 text-white overflow-hidden">
      <div class="absolute inset-0 bg-black/20"></div>
      <div class="relative container mx-auto px-4 py-16 lg:py-24">
        <div class="max-w-4xl mx-auto text-center space-y-8">
          <h1 class="text-4xl lg:text-6xl font-bold leading-tight">
            {{ $t('home.hero.title') }}
          </h1>
          <p class="text-xl lg:text-2xl text-primary-100 max-w-2xl mx-auto">
            {{ $t('home.hero.subtitle') }}
          </p>
          
          <!-- Hero Search -->
          <div class="max-w-2xl mx-auto">
            <UInput
              v-model="heroSearchQuery"
              size="xl"
              :placeholder="$t('home.hero.search_placeholder')"
              icon="i-heroicons-magnifying-glass"
              class="shadow-2xl"
              @keyup.enter="performHeroSearch"
            >
              <template #trailing>
                <UButton
                  size="md"
                  @click="performHeroSearch"
                  :disabled="!heroSearchQuery.trim()"
                >
                  {{ $t('nav.search') }}
                </UButton>
              </template>
            </UInput>
          </div>

          <div class="flex flex-wrap justify-center gap-4">
            <UButton
              to="/products"
              size="lg"
              color="white"
              variant="solid"
            >
              {{ $t('home.hero.cta') }}
            </UButton>
            <UButton
              to="/categories"
              size="lg"
              color="white"
              variant="outline"
            >
              {{ $t('nav.categories') }}
            </UButton>
          </div>
        </div>
      </div>
    </section>

    <!-- Featured Categories -->
    <section class="py-16 bg-gray-50 dark:bg-gray-900">
      <div class="container mx-auto px-4">
        <div class="text-center mb-12">
          <h2 class="text-3xl lg:text-4xl font-bold text-gray-900 dark:text-white mb-4">
            {{ $t('home.categories.title') }}
          </h2>
          <p class="text-xl text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
            {{ $t('home.categories.subtitle') }}
          </p>
        </div>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
          <UCard
            v-for="category in featuredCategories"
            :key="category.slug"
            class="group hover:shadow-lg transition-all duration-300 cursor-pointer"
            @click="navigateToCategory(category.slug)"
          >
            <div class="text-center p-4 space-y-4">
              <div class="w-16 h-16 mx-auto bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center group-hover:bg-primary-200 dark:group-hover:bg-primary-800 transition-colors">
                <UIcon :name="category.icon" class="w-8 h-8 text-primary-600" />
              </div>
              <div>
                <h3 class="font-semibold text-gray-900 dark:text-white group-hover:text-primary-600 transition-colors">
                  {{ category.name }}
                </h3>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  {{ category.count }} {{ $t('nav.books') }}
                </p>
              </div>
            </div>
          </UCard>
        </div>
      </div>
    </section>

    <!-- Featured Books -->
    <section class="py-16">
      <div class="container mx-auto px-4">
        <div class="flex items-center justify-between mb-8">
          <div>
            <h2 class="text-3xl lg:text-4xl font-bold text-gray-900 dark:text-white mb-2">
              {{ $t('home.featured.title') }}
            </h2>
            <p class="text-xl text-gray-600 dark:text-gray-400">
              {{ $t('home.featured.subtitle') }}
            </p>
          </div>
          <UButton to="/products" variant="outline">
            {{ $t('common.view_all') }}
          </UButton>
        </div>

        <div v-if="loadingFeatured" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <USkeleton v-for="i in 8" :key="i" class="h-96 w-full" />
        </div>

        <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <ProductCard
            v-for="product in featuredProducts"
            :key="product.id"
            :product="product"
            @click="navigateToProduct(product.id)"
            class="cursor-pointer"
          />
        </div>
      </div>
    </section>

    <!-- Partner Vendors -->
    <section class="py-16 bg-gray-50 dark:bg-gray-900">
      <div class="container mx-auto px-4">
        <div class="text-center mb-12">
          <h2 class="text-3xl lg:text-4xl font-bold text-gray-900 dark:text-white mb-4">
            {{ $t('home.vendors.title') }}
          </h2>
          <p class="text-xl text-gray-600 dark:text-gray-400">
            {{ $t('home.vendors.subtitle') }}
          </p>
        </div>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
          <UCard
            v-for="vendor in featuredVendors"
            :key="vendor.id"
            class="group hover:shadow-lg transition-all duration-300 cursor-pointer"
            @click="navigateToVendor(vendor.id)"
          >
            <div class="text-center p-6 space-y-4">
              <div class="w-20 h-20 mx-auto bg-gray-100 dark:bg-gray-800 rounded-full flex items-center justify-center">
                <UIcon name="i-heroicons-building-storefront" class="w-10 h-10 text-primary-600" />
              </div>
              <div>
                <h3 class="font-semibold text-gray-900 dark:text-white group-hover:text-primary-600 transition-colors">
                  {{ vendor.business_name }}
                </h3>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  {{ vendor.book_count }} {{ $t('nav.books') }}
                </p>
                <UBadge v-if="vendor.featured" color="primary" variant="soft" size="xs" class="mt-2">
                  {{ $t('common.featured') }}
                </UBadge>
              </div>
            </div>
          </UCard>
        </div>
      </div>
    </section>

    <!-- Newsletter -->
    <section class="py-16 bg-primary-600 text-white">
      <div class="container mx-auto px-4 text-center">
        <div class="max-w-2xl mx-auto space-y-6">
          <h2 class="text-3xl lg:text-4xl font-bold">
            {{ $t('home.newsletter.title') }}
          </h2>
          <p class="text-xl text-primary-100">
            {{ $t('home.newsletter.subtitle') }}
          </p>
          
          <form @submit.prevent="subscribeNewsletter" class="flex flex-col sm:flex-row gap-4 max-w-md mx-auto">
            <UInput
              v-model="newsletterEmail"
              type="email"
              :placeholder="$t('auth.login.email')"
              size="lg"
              required
              class="flex-1"
            />
            <UButton
              type="submit"
              size="lg"
              color="white"
              :loading="subscribing"
            >
              {{ $t('home.newsletter.subscribe') }}
            </UButton>
          </form>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
const { $t } = useI18n()
const config = useRuntimeConfig()
const router = useRouter()

// SEO
useHead({
  title: $t('home.hero.title'),
  meta: [
    { name: 'description', content: $t('home.hero.subtitle') }
  ]
})

// Reactive state
const heroSearchQuery = ref('')
const newsletterEmail = ref('')
const subscribing = ref(false)
const loadingFeatured = ref(true)
const featuredProducts = ref([])

// Featured categories with mock data
const featuredCategories = computed(() => [
  {
    name: $t('home.categories.islamic_studies'),
    slug: 'islamic-studies',
    icon: 'i-heroicons-book-open',
    count: 245
  },
  {
    name: $t('home.categories.arabic_literature'),
    slug: 'arabic-literature', 
    icon: 'i-heroicons-language',
    count: 189
  },
  {
    name: $t('home.categories.french_literature'),
    slug: 'french-literature',
    icon: 'i-heroicons-academic-cap',
    count: 156
  },
  {
    name: $t('home.categories.academic'),
    slug: 'academic',
    icon: 'i-heroicons-beaker',
    count: 298
  },
  {
    name: $t('home.categories.business'),
    slug: 'business',
    icon: 'i-heroicons-briefcase',
    count: 134
  },
  {
    name: $t('home.categories.technology'),
    slug: 'technology',
    icon: 'i-heroicons-computer-desktop',
    count: 167
  },
  {
    name: $t('home.categories.moroccan_culture'),
    slug: 'moroccan-culture',
    icon: 'i-heroicons-globe-africa',
    count: 89
  },
  {
    name: $t('home.categories.children'),
    slug: 'children',
    icon: 'i-heroicons-face-smile',
    count: 112
  }
])

// Featured vendors with mock data
const featuredVendors = ref([
  {
    id: '1',
    business_name: 'Dar Al Kitab',
    book_count: 245,
    featured: true
  },
  {
    id: '2', 
    business_name: 'Librairie Al Maghrib',
    book_count: 189,
    featured: true
  },
  {
    id: '3',
    business_name: 'Casablanca Books',
    book_count: 298,
    featured: false
  },
  {
    id: '4',
    business_name: 'Rabat Academic',
    book_count: 167,
    featured: false
  }
])

// Methods
const performHeroSearch = () => {
  if (heroSearchQuery.value.trim()) {
    router.push(`/search?q=${encodeURIComponent(heroSearchQuery.value)}`)
  }
}

const navigateToCategory = (slug: string) => {
  router.push(`/categories/${slug}`)
}

const navigateToProduct = (productId: string) => {
  router.push(`/products/${productId}`)
}

const navigateToVendor = (vendorId: string) => {
  router.push(`/vendors/${vendorId}`)
}

const subscribeNewsletter = async () => {
  subscribing.value = true
  try {
    // Mock newsletter subscription
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    const toast = useToast()
    toast.add({
      title: $t('common.success'),
      description: $t('home.newsletter.success'),
      color: 'green'
    })
    
    newsletterEmail.value = ''
  } catch (error) {
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('home.newsletter.error'),
      color: 'red'
    })
  } finally {
    subscribing.value = false
  }
}

const fetchFeaturedProducts = async () => {
  try {
    const response = await $fetch('/products/featured', {
      baseURL: config.public.catalogServiceUrl
    })
    featuredProducts.value = response.products || []
  } catch (error) {
    console.error('Failed to fetch featured products:', error)
  } finally {
    loadingFeatured.value = false
  }
}

// Initialize
onMounted(() => {
  fetchFeaturedProducts()
})
</script>
