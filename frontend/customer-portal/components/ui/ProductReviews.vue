<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">{{ $t('product.reviews') }}</h2>
        <UButton v-if="authStore.isAuthenticated" @click="showReviewModal = true">
          {{ $t('product.write_review') }}
        </UButton>
      </div>
    </template>

    <!-- Reviews Summary -->
    <div class="border-b border-gray-200 dark:border-gray-700 pb-6 mb-6">
      <div class="flex items-center gap-8">
        <div class="text-center">
          <div class="text-4xl font-bold text-gray-900 dark:text-white">
            {{ averageRating.toFixed(1) }}
          </div>
          <StarRating
            :rating="averageRating"
            :star-size="20"
            :show-rating="false"
            :read-only="true"
            active-color="#fbbf24"
            inactive-color="#e5e7eb"
          />
          <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
            {{ totalReviews }} {{ $t('product.reviews') }}
          </p>
        </div>
        
        <div class="flex-1 space-y-2">
          <div v-for="rating in [5, 4, 3, 2, 1]" :key="rating" class="flex items-center gap-2">
            <span class="text-sm w-8">{{ rating }}★</span>
            <div class="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
              <div 
                class="bg-yellow-400 h-2 rounded-full"
                :style="{ width: `${getRatingPercentage(rating)}%` }"
              ></div>
            </div>
            <span class="text-sm text-gray-600 dark:text-gray-400 w-8">
              {{ getRatingCount(rating) }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Reviews List -->
    <div class="space-y-6">
      <div v-for="review in displayedReviews" :key="review.id" class="border-b border-gray-200 dark:border-gray-700 pb-6 last:border-b-0">
        <div class="flex items-start gap-4">
          <UAvatar :alt="review.user?.full_name" size="md" />
          <div class="flex-1 space-y-2">
            <div class="flex items-center gap-2">
              <h4 class="font-medium text-gray-900 dark:text-white">
                {{ review.user?.full_name || 'Anonymous' }}
              </h4>
              <StarRating
                :rating="review.rating"
                :star-size="16"
                :show-rating="false"
                :read-only="true"
                active-color="#fbbf24"
                inactive-color="#e5e7eb"
              />
              <span class="text-sm text-gray-500">
                {{ formatDate(review.created_at) }}
              </span>
            </div>
            <p class="text-gray-700 dark:text-gray-300">
              {{ review.review_text }}
            </p>
            <div class="flex items-center gap-4 text-sm">
              <button 
                @click="toggleHelpful(review.id)"
                class="flex items-center gap-1 text-gray-500 hover:text-primary-600 transition-colors"
              >
                <UIcon name="i-heroicons-hand-thumb-up" class="w-4 h-4" />
                {{ $t('product.helpful') }} ({{ review.helpful_count || 0 }})
              </button>
              <button 
                v-if="authStore.user?.id === review.user_id"
                @click="editReview(review)"
                class="text-gray-500 hover:text-primary-600 transition-colors"
              >
                {{ $t('common.edit') }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Load More -->
    <div v-if="reviews.length > displayLimit" class="text-center pt-6">
      <UButton variant="outline" @click="loadMore">
        {{ $t('common.load_more') }}
      </UButton>
    </div>

    <!-- Review Modal -->
    <UModal v-model="showReviewModal">
      <UCard>
        <template #header>
          <h3 class="text-lg font-semibold">
            {{ editingReview ? $t('product.edit_review') : $t('product.write_review') }}
          </h3>
        </template>

        <form @submit.prevent="submitReview" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              {{ $t('product.rating') }}
            </label>
            <StarRating
              v-model:rating="reviewForm.rating"
              :star-size="24"
              :show-rating="false"
              active-color="#fbbf24"
              inactive-color="#e5e7eb"
            />
          </div>
          
          <UTextarea
            v-model="reviewForm.review_text"
            :placeholder="$t('product.review_placeholder')"
            :rows="4"
            required
          />
          
          <div class="flex justify-end gap-2">
            <UButton variant="ghost" @click="closeReviewModal">
              {{ $t('common.cancel') }}
            </UButton>
            <UButton type="submit" :loading="submittingReview">
              {{ editingReview ? $t('common.update') : $t('product.submit_review') }}
            </UButton>
          </div>
        </form>
      </UCard>
    </UModal>
  </UCard>
</template>

<script setup lang="ts">
import StarRating from 'vue-star-rating'

interface Review {
  id: string
  user_id: string
  rating: number
  review_text: string
  helpful_count: number
  created_at: string
  user?: {
    full_name: string
  }
}

interface Props {
  productId: string
  reviews: Review[]
}

const props = defineProps<Props>()
const { $t } = useI18n()
const config = useRuntimeConfig()
const authStore = useAuthStore()

// Reactive state
const showReviewModal = ref(false)
const submittingReview = ref(false)
const editingReview = ref<Review | null>(null)
const displayLimit = ref(5)

const reviewForm = reactive({
  rating: 5,
  review_text: ''
})

// Computed
const averageRating = computed(() => {
  if (props.reviews.length === 0) return 0
  const sum = props.reviews.reduce((acc, review) => acc + review.rating, 0)
  return sum / props.reviews.length
})

const totalReviews = computed(() => props.reviews.length)

const displayedReviews = computed(() => 
  props.reviews.slice(0, displayLimit.value)
)

// Methods
const getRatingCount = (rating: number) => {
  return props.reviews.filter(review => review.rating === rating).length
}

const getRatingPercentage = (rating: number) => {
  if (totalReviews.value === 0) return 0
  return (getRatingCount(rating) / totalReviews.value) * 100
}

const formatDate = (dateString: string) => {
  return new Intl.DateTimeFormat('ar-MA', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  }).format(new Date(dateString))
}

const loadMore = () => {
  displayLimit.value += 5
}

const submitReview = async () => {
  submittingReview.value = true
  try {
    const endpoint = editingReview.value 
      ? `/products/${props.productId}/reviews/${editingReview.value.id}`
      : `/products/${props.productId}/reviews`
    
    const method = editingReview.value ? 'PUT' : 'POST'
    
    await $fetch(endpoint, {
      baseURL: config.public.catalogServiceUrl,
      method,
      headers: authStore.getAuthHeaders(),
      body: reviewForm
    })

    const toast = useToast()
    toast.add({
      title: $t('common.success'),
      description: editingReview.value 
        ? $t('product.review_updated') 
        : $t('product.review_submitted'),
      color: 'green'
    })

    closeReviewModal()
    // Refresh reviews
    await refreshReviews()
  } catch (error) {
    console.error('Failed to submit review:', error)
    const toast = useToast()
    toast.add({
      title: $t('common.error'),
      description: $t('product.review_error'),
      color: 'red'
    })
  } finally {
    submittingReview.value = false
  }
}

const editReview = (review: Review) => {
  editingReview.value = review
  reviewForm.rating = review.rating
  reviewForm.review_text = review.review_text
  showReviewModal.value = true
}

const closeReviewModal = () => {
  showReviewModal.value = false
  editingReview.value = null
  reviewForm.rating = 5
  reviewForm.review_text = ''
}

const toggleHelpful = async (reviewId: string) => {
  if (!authStore.isAuthenticated) {
    const toast = useToast()
    toast.add({
      title: $t('auth.login_required'),
      description: $t('auth.login_to_continue'),
      color: 'yellow'
    })
    return
  }

  try {
    await $fetch(`/reviews/${reviewId}/helpful`, {
      baseURL: config.public.catalogServiceUrl,
      method: 'POST',
      headers: authStore.getAuthHeaders()
    })
    
    // Refresh reviews to update helpful count
    await refreshReviews()
  } catch (error) {
    console.error('Failed to mark as helpful:', error)
  }
}

const refreshReviews = async () => {
  // This would typically emit an event to parent component to refresh
  // For now, we'll just show a success message
}
</script>
