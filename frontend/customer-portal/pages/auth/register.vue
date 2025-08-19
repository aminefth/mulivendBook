<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div class="text-center">
        <NuxtLink to="/" class="flex items-center justify-center gap-3 mb-6">
          <UIcon name="i-heroicons-book-open" class="w-10 h-10 text-primary-600" />
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">BookMarket</h1>
        </NuxtLink>
        <h2 class="text-3xl font-bold text-gray-900 dark:text-white">
          {{ $t('auth.register.title') }}
        </h2>
        <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
          {{ $t('auth.register.subtitle') }}
        </p>
      </div>

      <UCard class="p-8">
        <form @submit.prevent="handleRegister" class="space-y-6">
          <div class="grid grid-cols-2 gap-4">
            <UFormGroup :label="$t('auth.register.first_name')" required>
              <UInput
                v-model="form.first_name"
                size="lg"
                icon="i-heroicons-user"
                :placeholder="$t('auth.register.first_name')"
                required
                :disabled="loading"
              />
            </UFormGroup>
            
            <UFormGroup :label="$t('auth.register.last_name')" required>
              <UInput
                v-model="form.last_name"
                size="lg"
                icon="i-heroicons-user"
                :placeholder="$t('auth.register.last_name')"
                required
                :disabled="loading"
              />
            </UFormGroup>
          </div>

          <UFormGroup :label="$t('auth.register.email')" required>
            <UInput
              v-model="form.email"
              type="email"
              size="lg"
              icon="i-heroicons-envelope"
              :placeholder="$t('auth.register.email_placeholder')"
              required
              :disabled="loading"
            />
          </UFormGroup>

          <UFormGroup :label="$t('auth.register.phone')">
            <UInput
              v-model="form.phone"
              type="tel"
              size="lg"
              icon="i-heroicons-phone"
              :placeholder="+212 6XX-XXXXXX"
              :disabled="loading"
            />
          </UFormGroup>

          <UFormGroup :label="$t('auth.register.password')" required>
            <UInput
              v-model="form.password"
              :type="showPassword ? 'text' : 'password'"
              size="lg"
              icon="i-heroicons-lock-closed"
              :placeholder="$t('auth.register.password_placeholder')"
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

          <UFormGroup :label="$t('auth.register.confirm_password')" required>
            <UInput
              v-model="form.confirm_password"
              :type="showConfirmPassword ? 'text' : 'password'"
              size="lg"
              icon="i-heroicons-lock-closed"
              :placeholder="$t('auth.register.confirm_password')"
              required
              :disabled="loading"
            >
              <template #trailing>
                <UButton
                  variant="ghost"
                  size="xs"
                  :icon="showConfirmPassword ? 'i-heroicons-eye-slash' : 'i-heroicons-eye'"
                  @click="showConfirmPassword = !showConfirmPassword"
                />
              </template>
            </UInput>
          </UFormGroup>

          <!-- Password Strength Indicator -->
          <div v-if="form.password" class="space-y-2">
            <div class="flex justify-between text-sm">
              <span>{{ $t('auth.register.password_strength') }}</span>
              <span :class="passwordStrengthColor">{{ passwordStrengthText }}</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div 
                class="h-2 rounded-full transition-all duration-300"
                :class="passwordStrengthColor"
                :style="{ width: `${passwordStrength * 25}%` }"
              ></div>
            </div>
          </div>

          <!-- Validation Errors -->
          <div v-if="validationErrors.length > 0" class="space-y-1">
            <UAlert
              v-for="error in validationErrors"
              :key="error"
              color="red"
              variant="soft"
              :title="error"
            />
          </div>

          <UCheckbox
            v-model="form.agree_terms"
            required
          >
            <template #label>
              <span class="text-sm">
                {{ $t('auth.register.agree_terms_start') }}
                <NuxtLink to="/terms" class="text-primary-600 hover:text-primary-500">
                  {{ $t('auth.register.terms') }}
                </NuxtLink>
                {{ $t('auth.register.and') }}
                <NuxtLink to="/privacy" class="text-primary-600 hover:text-primary-500">
                  {{ $t('auth.register.privacy') }}
                </NuxtLink>
              </span>
            </template>
          </UCheckbox>

          <UButton
            type="submit"
            size="lg"
            block
            :loading="loading"
            :disabled="!isFormValid"
          >
            {{ $t('auth.register.submit') }}
          </UButton>
        </form>

        <UDivider :label="$t('common.or')" class="my-6" />

        <!-- Social Registration -->
        <div class="space-y-3">
          <UButton
            variant="outline"
            size="lg"
            block
            @click="registerWithGoogle"
          >
            <UIcon name="i-simple-icons-google" class="w-5 h-5 mr-2" />
            {{ $t('auth.register.google') }}
          </UButton>
          <UButton
            variant="outline"
            size="lg"
            block
            @click="registerWithFacebook"
          >
            <UIcon name="i-simple-icons-facebook" class="w-5 h-5 mr-2" />
            {{ $t('auth.register.facebook') }}
          </UButton>
        </div>

        <p class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          {{ $t('auth.register.have_account') }}
          <NuxtLink
            to="/auth/login"
            class="font-medium text-primary-600 hover:text-primary-500"
          >
            {{ $t('auth.register.login_here') }}
          </NuxtLink>
        </p>
      </UCard>
    </div>
  </div>
</template>

<script setup lang="ts">
const { $t } = useI18n()
const authStore = useAuthStore()

// SEO
useHead({
  title: $t('auth.register.title')
})

// Reactive state
const loading = ref(false)
const showPassword = ref(false)
const showConfirmPassword = ref(false)

const form = reactive({
  first_name: '',
  last_name: '',
  email: '',
  phone: '',
  password: '',
  confirm_password: '',
  agree_terms: false
})

// Computed
const passwordStrength = computed(() => {
  const password = form.password
  let strength = 0
  
  if (password.length >= 8) strength++
  if (/[a-z]/.test(password)) strength++
  if (/[A-Z]/.test(password)) strength++
  if (/[0-9]/.test(password)) strength++
  if (/[^A-Za-z0-9]/.test(password)) strength++
  
  return Math.min(strength, 4)
})

const passwordStrengthText = computed(() => {
  const texts = [
    $t('auth.register.password_very_weak'),
    $t('auth.register.password_weak'),
    $t('auth.register.password_fair'),
    $t('auth.register.password_good'),
    $t('auth.register.password_strong')
  ]
  return texts[passwordStrength.value]
})

const passwordStrengthColor = computed(() => {
  const colors = [
    'bg-red-500 text-red-600',
    'bg-red-400 text-red-600',
    'bg-yellow-400 text-yellow-600',
    'bg-blue-400 text-blue-600',
    'bg-green-500 text-green-600'
  ]
  return colors[passwordStrength.value]
})

const validationErrors = computed(() => {
  const errors = []
  
  if (form.password && form.confirm_password && form.password !== form.confirm_password) {
    errors.push($t('auth.register.password_mismatch'))
  }
  
  if (form.password && form.password.length < 8) {
    errors.push($t('auth.register.password_min_length'))
  }
  
  if (form.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email)) {
    errors.push($t('auth.register.email_invalid'))
  }
  
  return errors
})

const isFormValid = computed(() => {
  return form.first_name && 
         form.last_name && 
         form.email && 
         form.password && 
         form.confirm_password &&
         form.password === form.confirm_password &&
         form.password.length >= 8 &&
         form.agree_terms &&
         validationErrors.value.length === 0
})

// Methods
const handleRegister = async () => {
  if (!isFormValid.value) return
  
  loading.value = true
  
  try {
    const userData = {
      email: form.email,
      password: form.password,
      full_name: `${form.first_name} ${form.last_name}`,
      phone: form.phone || undefined
    }
    
    const result = await authStore.register(userData)
    
    if (result.success) {
      const toast = useToast()
      toast.add({
        title: $t('auth.register.success'),
        description: $t('auth.register.welcome'),
        color: 'green'
      })
      
      await navigateTo('/')
    } else {
      const toast = useToast()
      toast.add({
        title: $t('auth.register.error'),
        description: result.error,
        color: 'red'
      })
    }
  } catch (error) {
    console.error('Registration error:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('auth.register.unexpected_error'),
      color: 'red'
    })
  } finally {
    loading.value = false
  }
}

const registerWithGoogle = () => {
  // Implement Google OAuth
  console.log('Google registration')
}

const registerWithFacebook = () => {
  // Implement Facebook OAuth
  console.log('Facebook registration')
}

// Redirect if already authenticated
onMounted(() => {
  if (authStore.isAuthenticated) {
    navigateTo('/')
  }
})
</script>
