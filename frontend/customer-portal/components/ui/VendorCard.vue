<template>
  <UCard class="vendor-card cursor-pointer hover:shadow-lg transition-all duration-300">
    <div class="space-y-4">
      <!-- Vendor Header -->
      <div class="flex items-start gap-4">
        <div class="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-lg overflow-hidden flex-shrink-0">
          <img
            :src="vendor.logo || '/placeholder-vendor.jpg'"
            :alt="vendor.business_name"
            class="w-full h-full object-cover"
          />
        </div>
        
        <div class="flex-1 min-w-0">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white truncate">
            {{ vendor.business_name }}
          </h3>
          <p class="text-sm text-gray-600 dark:text-gray-400 truncate">
            {{ vendor.owner_name }}
          </p>
          
          <!-- Rating -->
          <div class="flex items-center gap-1 mt-1">
            <div class="flex">
              <UIcon
                v-for="i in 5"
                :key="i"
                name="i-heroicons-star-solid"
                :class="i <= Math.floor(vendor.rating || 0) ? 'text-yellow-400' : 'text-gray-300'"
                class="w-4 h-4"
              />
            </div>
            <span class="text-sm text-gray-600 dark:text-gray-400 ml-1">
              ({{ vendor.total_reviews || 0 }})
            </span>
          </div>
        </div>
        
        <!-- Status Badge -->
        <UBadge
          :color="vendor.status === 'active' ? 'green' : 'gray'"
          :label="$t(`vendors.status.${vendor.status}`)"
          size="xs"
        />
      </div>

      <!-- Description -->
      <p class="text-sm text-gray-600 dark:text-gray-400 line-clamp-2">
        {{ vendor.description }}
      </p>

      <!-- Stats -->
      <div class="grid grid-cols-3 gap-4 py-3 border-t border-gray-200 dark:border-gray-700">
        <div class="text-center">
          <div class="text-lg font-semibold text-gray-900 dark:text-white">
            {{ vendor.total_books || 0 }}
          </div>
          <div class="text-xs text-gray-500">
            {{ $t('vendors.books') }}
          </div>
        </div>
        
        <div class="text-center">
          <div class="text-lg font-semibold text-gray-900 dark:text-white">
            {{ vendor.total_sales || 0 }}
          </div>
          <div class="text-xs text-gray-500">
            {{ $t('vendors.sales') }}
          </div>
        </div>
        
        <div class="text-center">
          <div class="text-lg font-semibold text-gray-900 dark:text-white">
            {{ formatJoinDate(vendor.created_at) }}
          </div>
          <div class="text-xs text-gray-500">
            {{ $t('vendors.joined') }}
          </div>
        </div>
      </div>

      <!-- Location -->
      <div class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
        <UIcon name="i-heroicons-map-pin" class="w-4 h-4" />
        <span>{{ vendor.city }}, {{ vendor.country }}</span>
      </div>

      <!-- Categories -->
      <div v-if="vendor.categories && vendor.categories.length > 0" class="flex flex-wrap gap-1">
        <UBadge
          v-for="category in vendor.categories.slice(0, 3)"
          :key="category"
          :label="category"
          variant="soft"
          size="xs"
        />
        <UBadge
          v-if="vendor.categories.length > 3"
          :label="`+${vendor.categories.length - 3}`"
          variant="soft"
          size="xs"
          color="gray"
        />
      </div>

      <!-- Action Buttons -->
      <div class="flex gap-2 pt-2">
        <UButton
          variant="outline"
          size="sm"
          class="flex-1"
          @click.stop="viewBooks"
        >
          {{ $t('vendors.view_books') }}
        </UButton>
        
        <UButton
          variant="ghost"
          size="sm"
          icon="i-heroicons-heart"
          @click.stop="toggleFavorite"
          :color="isFavorite ? 'red' : 'gray'"
        />
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
interface Vendor {
  id: string
  business_name: string
  owner_name: string
  description: string
  logo?: string
  rating?: number
  total_reviews?: number
  total_books?: number
  total_sales?: number
  status: 'active' | 'inactive' | 'suspended'
  city: string
  country: string
  categories?: string[]
  created_at: string
}

interface Props {
  vendor: Vendor
}

const props = defineProps<Props>()
const emit = defineEmits(['favorite-toggled'])

const { $t } = useI18n()

// Reactive state
const isFavorite = ref(false)

// Methods
const formatJoinDate = (dateString: string) => {
  const date = new Date(dateString)
  const now = new Date()
  const diffTime = Math.abs(now.getTime() - date.getTime())
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
  
  if (diffDays < 30) {
    return $t('vendors.days_ago', { days: diffDays })
  } else if (diffDays < 365) {
    const months = Math.floor(diffDays / 30)
    return $t('vendors.months_ago', { months })
  } else {
    const years = Math.floor(diffDays / 365)
    return $t('vendors.years_ago', { years })
  }
}

const viewBooks = () => {
  navigateTo(`/products?vendor=${props.vendor.id}`)
}

const toggleFavorite = () => {
  isFavorite.value = !isFavorite.value
  emit('favorite-toggled', {
    vendorId: props.vendor.id,
    isFavorite: isFavorite.value
  })
  
  const toast = useToast()
  toast.add({
    title: isFavorite.value ? $t('vendors.added_to_favorites') : $t('vendors.removed_from_favorites'),
    color: isFavorite.value ? 'green' : 'gray'
  })
}
</script>

<style scoped>
.vendor-card {
  @apply transform transition-all duration-300 hover:scale-[1.02];
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
