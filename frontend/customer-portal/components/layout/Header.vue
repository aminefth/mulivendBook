<template>
  <header class="sticky top-0 z-50 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800">
    <!-- Top Bar -->
    <div class="bg-primary-600 text-white py-1">
      <div class="container mx-auto px-4 flex justify-between items-center text-sm">
        <div class="flex items-center gap-4">
          <span>{{ $t('header.free_shipping') }}</span>
          <span>{{ $t('header.support') }}: +212 5XX-XXXXXX</span>
        </div>
        <div class="flex items-center gap-4">
          <LanguageSwitcher />
          <UButton
            v-if="!authStore.isAuthenticated"
            to="/auth/login"
            variant="ghost"
            size="xs"
            color="white"
          >
            {{ $t('nav.login') }}
          </UButton>
        </div>
      </div>
    </div>

    <!-- Main Header -->
    <div class="container mx-auto px-4 py-4">
      <div class="flex items-center justify-between gap-4">
        <!-- Logo -->
        <NuxtLink to="/" class="flex items-center gap-3">
          <UIcon name="i-heroicons-book-open" class="w-8 h-8 text-primary-600" />
          <div class="hidden sm:block">
            <h1 class="text-xl font-bold text-gray-900 dark:text-white">BookMarket</h1>
            <p class="text-xs text-gray-600 dark:text-gray-400">{{ $t('header.tagline') }}</p>
          </div>
        </NuxtLink>

        <!-- Search Bar -->
        <div class="flex-1 max-w-2xl mx-4">
          <UInput
            v-model="searchQuery"
            :placeholder="$t('home.hero.search_placeholder')"
            size="lg"
            icon="i-heroicons-magnifying-glass"
            :ui="{ icon: { trailing: { pointer: '' } } }"
            @keyup.enter="performSearch"
          >
            <template #trailing>
              <UButton
                v-show="searchQuery !== ''"
                color="gray"
                variant="link"
                icon="i-heroicons-x-mark-20-solid"
                :padded="false"
                @click="searchQuery = ''"
              />
            </template>
          </UInput>
        </div>

        <!-- Actions -->
        <div class="flex items-center gap-2">
          <!-- Cart -->
          <UButton
            to="/cart"
            variant="ghost"
            size="lg"
            class="relative"
          >
            <UIcon name="i-heroicons-shopping-cart" class="w-6 h-6" />
            <UBadge
              v-if="cartStore.itemCount > 0"
              :label="cartStore.itemCount.toString()"
              color="red"
              class="absolute -top-1 -right-1"
            />
          </UButton>

          <!-- User Menu -->
          <UDropdown
            v-if="authStore.isAuthenticated"
            :items="userMenuItems"
            :popper="{ placement: 'bottom-end' }"
          >
            <UAvatar
              :alt="authStore.userName"
              size="md"
              class="cursor-pointer"
            />
          </UDropdown>

          <!-- Mobile Menu -->
          <UButton
            variant="ghost"
            size="lg"
            class="lg:hidden"
            @click="mobileMenuOpen = !mobileMenuOpen"
          >
            <UIcon name="i-heroicons-bars-3" class="w-6 h-6" />
          </UButton>
        </div>
      </div>

      <!-- Navigation -->
      <nav class="hidden lg:flex items-center gap-8 mt-4 pt-4 border-t border-gray-200 dark:border-gray-800">
        <NuxtLink
          v-for="item in navigationItems"
          :key="item.to"
          :to="item.to"
          class="text-gray-700 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400 font-medium transition-colors"
          active-class="text-primary-600 dark:text-primary-400"
        >
          {{ item.label }}
        </NuxtLink>
      </nav>
    </div>

    <!-- Mobile Menu -->
    <USlideover v-model="mobileMenuOpen" side="right">
      <div class="p-4 space-y-4">
        <div class="flex items-center justify-between">
          <h2 class="text-lg font-semibold">{{ $t('nav.menu') }}</h2>
          <UButton
            variant="ghost"
            size="sm"
            icon="i-heroicons-x-mark"
            @click="mobileMenuOpen = false"
          />
        </div>
        
        <nav class="space-y-2">
          <NuxtLink
            v-for="item in navigationItems"
            :key="item.to"
            :to="item.to"
            class="block py-2 px-3 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800"
            @click="mobileMenuOpen = false"
          >
            {{ item.label }}
          </NuxtLink>
        </nav>

        <div v-if="!authStore.isAuthenticated" class="pt-4 border-t space-y-2">
          <UButton to="/auth/login" block @click="mobileMenuOpen = false">
            {{ $t('nav.login') }}
          </UButton>
          <UButton to="/auth/register" variant="outline" block @click="mobileMenuOpen = false">
            {{ $t('nav.register') }}
          </UButton>
        </div>
      </div>
    </USlideover>
  </header>
</template>

<script setup lang="ts">
const { $t } = useI18n()
const authStore = useAuthStore()
const cartStore = useCartStore()
const router = useRouter()

const searchQuery = ref('')
const mobileMenuOpen = ref(false)

const navigationItems = computed(() => [
  { label: $t('nav.home'), to: '/' },
  { label: $t('nav.books'), to: '/products' },
  { label: $t('nav.categories'), to: '/categories' },
  { label: $t('nav.vendors'), to: '/vendors' }
])

const userMenuItems = computed(() => [
  [{
    label: authStore.userName,
    slot: 'account',
    disabled: true
  }],
  [{
    label: $t('account.dashboard'),
    icon: 'i-heroicons-squares-2x2',
    to: '/account'
  }, {
    label: $t('account.orders'),
    icon: 'i-heroicons-shopping-bag',
    to: '/account/orders'
  }, {
    label: $t('account.profile'),
    icon: 'i-heroicons-user',
    to: '/account/profile'
  }],
  [{
    label: $t('nav.logout'),
    icon: 'i-heroicons-arrow-right-on-rectangle',
    click: () => authStore.logout()
  }]
])

const performSearch = () => {
  if (searchQuery.value.trim()) {
    router.push(`/search?q=${encodeURIComponent(searchQuery.value)}`)
  }
}

// Initialize stores
onMounted(() => {
  authStore.initializeAuth()
  cartStore.loadFromStorage()
  
  if (authStore.isAuthenticated) {
    cartStore.syncWithServer()
  }
})
</script>
