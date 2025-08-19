<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div class="text-center">
        <NuxtLink to="/" class="flex items-center justify-center gap-3 mb-6">
          <UIcon name="i-heroicons-book-open" class="w-10 h-10 text-primary-600" />
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">BookMarket</h1>
        </NuxtLink>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white">
          {{ $t('auth.login.title') }}
        </h2>
        <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
          {{ $t('auth.login.subtitle') }}
        </p>
      </div>

      <UCard class="p-8">
        <form @submit.prevent="handleLogin" class="space-y-6">
          <div>
            <UFormGroup :label="$t('auth.login.email')" required>
              <UInput
                v-model="form.email"
                type="email"
                size="lg"
                icon="i-heroicons-envelope"
                :placeholder="$t('auth.login.email_placeholder')"
                required
                :disabled="loading"
              />
            </UFormGroup>
          </div>

          <div>
            <UFormGroup :label="$t('auth.login.password')" required>
              <UInput
                v-model="form.password"
                :type="showPassword ? 'text' : 'password'"
                size="lg"
                icon="i-heroicons-lock-closed"
                :placeholder="$t('auth.login.password_placeholder')"
                required
                :disabled="loading"
              >
                <template #trailing>
                  <UButton
                    variant="ghost"
                    size="xs"
                    :icon="showPassword ? 'i-heroicons-eye-slash' : 'i-heroicons-eye'"
                    @click="showPassword = !showPassword"
                  />
                </template>
              </UInput>
            </UFormGroup>
          </div>

          <div class="flex items-center justify-between">
            <UCheckbox
              v-model="form.remember"
              :label="$t('auth.login.remember')"
            />
            <NuxtLink
              to="/auth/forgot-password"
              class="text-sm text-primary-600 hover:text-primary-500"
            >
              {{ $t('auth.login.forgot_password') }}
            </NuxtLink>
          </div>

          <UButton
            type="submit"
            size="lg"
            block
            :loading="loading"
            :disabled="!form.email || !form.password"
          >
            {{ $t('auth.login.submit') }}
          </UButton>
        </form>

        <UDivider :label="$t('common.or')" class="my-6" />

        <!-- Social Login -->
        <div class="space-y-3">
          <UButton
            variant="outline"
            size="lg"
            block
            @click="loginWithGoogle"
          >
            <UIcon name="i-simple-icons-google" class="w-5 h-5 mr-2" />
            {{ $t('auth.login.google') }}
          </UButton>
          <UButton
            variant="outline"
            size="lg"
            block
            @click="loginWithFacebook"
          >
            <UIcon name="i-simple-icons-facebook" class="w-5 h-5 mr-2" />
            {{ $t('auth.login.facebook') }}
          </UButton>
        </div>

        <p class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          {{ $t('auth.login.no_account') }}
          <NuxtLink
            to="/auth/register"
            class="font-medium text-primary-600 hover:text-primary-500"
          >
            {{ $t('auth.login.create_account') }}
          </NuxtLink>
        </p>
      </UCard>
    </div>
  </div>
</template>

<script setup lang="ts">
const { $t } = useI18n()
const authStore = useAuthStore()
const route = useRoute()

// SEO
useHead({
  title: $t('auth.login.title')
})

// Reactive state
const loading = ref(false)
const showPassword = ref(false)

const form = reactive({
  email: '',
  password: '',
  remember: false
})

// Methods
const handleLogin = async () => {
  loading.value = true
  
  try {
    const result = await authStore.login(form.email, form.password)
    
    if (result.success) {
      const toast = useToast()
      toast.add({
        title: $t('auth.login.success'),
        description: $t('auth.login.welcome_back'),
        color: 'green'
      })
      
      // Redirect to intended page or home
      const redirectTo = route.query.redirect as string || '/'
      await navigateTo(redirectTo)
    } else {
      const toast = useToast()
      toast.add({
        title: $t('auth.login.error'),
        description: result.error,
        color: 'red'
      })
    }
  } catch (error) {
    console.error('Login error:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('auth.login.unexpected_error'),
      color: 'red'
    })
  } finally {
    loading.value = false
  }
}

const loginWithGoogle = () => {
  // Implement Google OAuth
  console.log('Google login')
}

const loginWithFacebook = () => {
  // Implement Facebook OAuth
  console.log('Facebook login')
}

// Redirect if already authenticated
onMounted(() => {
  if (authStore.isAuthenticated) {
    navigateTo('/')
  }
})
</script>
