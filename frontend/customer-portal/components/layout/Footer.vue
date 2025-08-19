<template>
  <footer class="bg-gray-900 text-white">
    <!-- Main Footer -->
    <div class="container mx-auto px-4 py-12">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        <!-- Company Info -->
        <div class="space-y-4">
          <div class="flex items-center gap-3">
            <UIcon name="i-heroicons-book-open" class="w-8 h-8 text-primary-400" />
            <h3 class="text-xl font-bold">BookMarket Pro</h3>
          </div>
          <p class="text-gray-300 text-sm leading-relaxed">
            {{ $t('footer.description') }}
          </p>
          <div class="flex gap-4">
            <UButton
              variant="ghost"
              size="sm"
              icon="i-simple-icons-facebook"
              to="https://facebook.com/bookmarket"
              target="_blank"
            />
            <UButton
              variant="ghost"
              size="sm"
              icon="i-simple-icons-twitter"
              to="https://twitter.com/bookmarket"
              target="_blank"
            />
            <UButton
              variant="ghost"
              size="sm"
              icon="i-simple-icons-instagram"
              to="https://instagram.com/bookmarket"
              target="_blank"
            />
            <UButton
              variant="ghost"
              size="sm"
              icon="i-simple-icons-linkedin"
              to="https://linkedin.com/company/bookmarket"
              target="_blank"
            />
          </div>
        </div>

        <!-- Quick Links -->
        <div class="space-y-4">
          <h4 class="text-lg font-semibold">{{ $t('footer.quick_links') }}</h4>
          <nav class="space-y-2">
            <NuxtLink
              v-for="link in quickLinks"
              :key="link.to"
              :to="link.to"
              class="block text-gray-300 hover:text-white transition-colors text-sm"
            >
              {{ link.label }}
            </NuxtLink>
          </nav>
        </div>

        <!-- Categories -->
        <div class="space-y-4">
          <h4 class="text-lg font-semibold">{{ $t('footer.categories') }}</h4>
          <nav class="space-y-2">
            <NuxtLink
              v-for="category in popularCategories"
              :key="category.to"
              :to="category.to"
              class="block text-gray-300 hover:text-white transition-colors text-sm"
            >
              {{ category.label }}
            </NuxtLink>
          </nav>
        </div>

        <!-- Contact & Support -->
        <div class="space-y-4">
          <h4 class="text-lg font-semibold">{{ $t('footer.contact') }}</h4>
          <div class="space-y-3 text-sm">
            <div class="flex items-center gap-2 text-gray-300">
              <UIcon name="i-heroicons-phone" class="w-4 h-4" />
              <span>+212 5XX-XXXXXX</span>
            </div>
            <div class="flex items-center gap-2 text-gray-300">
              <UIcon name="i-heroicons-envelope" class="w-4 h-4" />
              <span>support@bookmarket.ma</span>
            </div>
            <div class="flex items-start gap-2 text-gray-300">
              <UIcon name="i-heroicons-map-pin" class="w-4 h-4 mt-0.5" />
              <span>Casablanca, Morocco</span>
            </div>
          </div>
          
          <!-- Newsletter Signup -->
          <div class="space-y-2">
            <h5 class="font-medium">{{ $t('footer.newsletter') }}</h5>
            <form @submit.prevent="subscribeNewsletter" class="flex gap-2">
              <UInput
                v-model="newsletterEmail"
                type="email"
                size="sm"
                :placeholder="$t('footer.newsletter_placeholder')"
                class="flex-1"
              />
              <UButton
                type="submit"
                size="sm"
                :loading="subscribing"
              >
                {{ $t('footer.subscribe') }}
              </UButton>
            </form>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom Bar -->
    <div class="border-t border-gray-800">
      <div class="container mx-auto px-4 py-6">
        <div class="flex flex-col md:flex-row items-center justify-between gap-4">
          <div class="text-sm text-gray-400">
            © {{ currentYear }} BookMarket Pro. {{ $t('footer.rights_reserved') }}
          </div>
          
          <div class="flex items-center gap-6 text-sm">
            <NuxtLink
              to="/privacy"
              class="text-gray-400 hover:text-white transition-colors"
            >
              {{ $t('footer.privacy') }}
            </NuxtLink>
            <NuxtLink
              to="/terms"
              class="text-gray-400 hover:text-white transition-colors"
            >
              {{ $t('footer.terms') }}
            </NuxtLink>
            <NuxtLink
              to="/cookies"
              class="text-gray-400 hover:text-white transition-colors"
            >
              {{ $t('footer.cookies') }}
            </NuxtLink>
            <NuxtLink
              to="/help"
              class="text-gray-400 hover:text-white transition-colors"
            >
              {{ $t('footer.help') }}
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
const { $t } = useI18n()

// Reactive state
const newsletterEmail = ref('')
const subscribing = ref(false)

// Computed
const currentYear = computed(() => new Date().getFullYear())

const quickLinks = computed(() => [
  { label: $t('nav.home'), to: '/' },
  { label: $t('nav.books'), to: '/products' },
  { label: $t('nav.vendors'), to: '/vendors' },
  { label: $t('footer.about'), to: '/about' },
  { label: $t('footer.blog'), to: '/blog' },
  { label: $t('footer.careers'), to: '/careers' }
])

const popularCategories = computed(() => [
  { label: $t('home.categories.islamic_studies'), to: '/categories/islamic-studies' },
  { label: $t('home.categories.arabic_literature'), to: '/categories/arabic-literature' },
  { label: $t('home.categories.french_literature'), to: '/categories/french-literature' },
  { label: $t('home.categories.academic'), to: '/categories/academic' },
  { label: $t('home.categories.business'), to: '/categories/business' },
  { label: $t('home.categories.children'), to: '/categories/children' }
])

// Methods
const subscribeNewsletter = async () => {
  if (!newsletterEmail.value.trim()) return
  
  subscribing.value = true
  try {
    // Mock newsletter subscription
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    const toast = useToast()
    toast.add({
      title: $t('common.success'),
      description: $t('footer.newsletter_success'),
      color: 'green'
    })
    
    newsletterEmail.value = ''
  } catch (error) {
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('footer.newsletter_error'),
      color: 'red'
    })
  } finally {
    subscribing.value = false
  }
}
</script>
