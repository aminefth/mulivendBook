export default defineNuxtPlugin(() => {
  const authStore = useAuthStore()
  const cartStore = useCartStore()

  // Initialize authentication state
  authStore.initializeAuth()
  
  // Load cart from localStorage
  cartStore.loadFromStorage()
  
  // Sync cart with server if authenticated
  if (authStore.isAuthenticated) {
    cartStore.syncWithServer()
  }
})
