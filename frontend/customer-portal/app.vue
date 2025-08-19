<template>
  <div>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    
    <!-- Global Toast Container -->
    <UNotifications />
    
    <!-- Loading Indicator -->
    <UModal v-model="globalLoading" :ui="{ width: 'w-auto' }" prevent-close>
      <div class="p-8 text-center">
        <UIcon name="i-heroicons-arrow-path" class="w-8 h-8 animate-spin mx-auto mb-4 text-primary-600" />
        <p class="text-gray-600 dark:text-gray-400">{{ $t('common.loading') }}</p>
      </div>
    </UModal>
  </div>
</template>

<script setup lang="ts">
const { locale } = useI18n()

// Global loading state
const globalLoading = ref(false)

// Set document direction based on locale
watch(locale, (newLocale) => {
  if (process.client) {
    document.documentElement.dir = newLocale === 'ar' ? 'rtl' : 'ltr'
    document.documentElement.lang = newLocale
  }
}, { immediate: true })

// Initialize on mount
onMounted(() => {
  // Set initial direction
  if (process.client) {
    document.documentElement.dir = locale.value === 'ar' ? 'rtl' : 'ltr'
    document.documentElement.lang = locale.value
  }
})
</script>
