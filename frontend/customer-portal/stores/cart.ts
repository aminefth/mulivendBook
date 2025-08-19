import { defineStore } from 'pinia'

interface CartItem {
  id: string
  product_id: string
  name: string
  author: string
  price: number
  quantity: number
  image?: string
  vendor_name: string
  stock_quantity: number
}

interface CartState {
  items: CartItem[]
  loading: boolean
}

export const useCartStore = defineStore('cart', {
  state: (): CartState => ({
    items: [],
    loading: false
  }),

  getters: {
    itemCount: (state) => state.items.reduce((total, item) => total + item.quantity, 0),
    
    totalAmount: (state) => state.items.reduce((total, item) => total + (item.price * item.quantity), 0),
    
    isEmpty: (state) => state.items.length === 0,
    
    getItemById: (state) => (productId: string) => 
      state.items.find(item => item.product_id === productId)
  },

  actions: {
    addItem(product: any, quantity: number = 1) {
      const existingItem = this.getItemById(product.id)
      
      if (existingItem) {
        const newQuantity = existingItem.quantity + quantity
        if (newQuantity <= product.stock_quantity) {
          existingItem.quantity = newQuantity
        }
      } else {
        const cartItem: CartItem = {
          id: `cart_${Date.now()}_${product.id}`,
          product_id: product.id,
          name: product.name,
          author: product.author,
          price: product.price,
          quantity: Math.min(quantity, product.stock_quantity),
          image: product.images?.[0],
          vendor_name: product.vendor?.business_name || '',
          stock_quantity: product.stock_quantity
        }
        this.items.push(cartItem)
      }
      
      this.saveToStorage()
    },

    removeItem(productId: string) {
      const index = this.items.findIndex(item => item.product_id === productId)
      if (index > -1) {
        this.items.splice(index, 1)
        this.saveToStorage()
      }
    },

    updateQuantity(productId: string, quantity: number) {
      const item = this.getItemById(productId)
      if (item && quantity > 0 && quantity <= item.stock_quantity) {
        item.quantity = quantity
        this.saveToStorage()
      }
    },

    clearCart() {
      this.items = []
      this.saveToStorage()
    },

    saveToStorage() {
      if (process.client) {
        localStorage.setItem('cart_items', JSON.stringify(this.items))
      }
    },

    loadFromStorage() {
      if (process.client) {
        const stored = localStorage.getItem('cart_items')
        if (stored) {
          try {
            this.items = JSON.parse(stored)
          } catch (error) {
            console.error('Failed to load cart from storage:', error)
            this.items = []
          }
        }
      }
    },

    async syncWithServer() {
      const authStore = useAuthStore()
      if (!authStore.isAuthenticated) return

      try {
        this.loading = true
        const config = useRuntimeConfig()
        
        // Get server cart
        const serverCart = await $fetch<CartItem[]>('/cart', {
          baseURL: config.public.orderServiceUrl,
          headers: authStore.getAuthHeaders()
        })

        // Merge local and server carts
        this.items = this.mergeCartItems(this.items, serverCart)
        this.saveToStorage()

        // Update server with merged cart
        await $fetch('/cart/sync', {
          baseURL: config.public.orderServiceUrl,
          method: 'POST',
          headers: authStore.getAuthHeaders(),
          body: { items: this.items }
        })
      } catch (error) {
        console.error('Failed to sync cart:', error)
      } finally {
        this.loading = false
      }
    },

    mergeCartItems(localItems: CartItem[], serverItems: CartItem[]): CartItem[] {
      const merged = [...localItems]
      
      serverItems.forEach(serverItem => {
        const existingIndex = merged.findIndex(item => item.product_id === serverItem.product_id)
        if (existingIndex > -1) {
          // Keep the higher quantity
          merged[existingIndex].quantity = Math.max(merged[existingIndex].quantity, serverItem.quantity)
        } else {
          merged.push(serverItem)
        }
      })
      
      return merged
    }
  }
})
