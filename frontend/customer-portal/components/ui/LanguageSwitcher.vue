<template>
  <UDropdown :items="languageItems" :popper="{ placement: 'bottom-end' }">
    <UButton variant="ghost" size="xs" color="white">
      <UIcon :name="currentLanguage.icon" class="w-4 h-4 mr-1" />
      {{ currentLanguage.code.toUpperCase() }}
      <UIcon name="i-heroicons-chevron-down" class="w-3 h-3 ml-1" />
    </UButton>
  </UDropdown>
</template>

<script setup lang="ts">
const { locale, locales, setLocale } = useI18n()

const currentLanguage = computed(() => {
  const current = (locales.value as any[]).find(l => l.code === locale.value)
  return {
    ...current,
    icon: current?.code === 'ar' ? 'i-twemoji-flag-morocco' : 
          current?.code === 'fr' ? 'i-twemoji-flag-france' : 
          'i-twemoji-flag-united-states'
  }
})

const languageItems = computed(() => [
  (locales.value as any[]).map(locale => ({
    label: locale.name,
    icon: locale.code === 'ar' ? 'i-twemoji-flag-morocco' : 
          locale.code === 'fr' ? 'i-twemoji-flag-france' : 
          'i-twemoji-flag-united-states',
    click: () => switchLanguage(locale.code)
  }))
])

const switchLanguage = async (newLocale: string) => {
  await setLocale(newLocale)
  
  // Update document direction for RTL
  if (process.client) {
    document.documentElement.dir = newLocale === 'ar' ? 'rtl' : 'ltr'
    document.documentElement.lang = newLocale
  }
}
</script>
