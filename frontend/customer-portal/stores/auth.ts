import { defineStore } from 'pinia'
import { jwtDecode } from 'jwt-decode'

interface User {
  id: string
  email: string
  full_name: string
  role: string
  is_verified: boolean
}

interface AuthState {
  user: User | null
  token: string | null
  isAuthenticated: boolean
  loading: boolean
}

interface JWTPayload {
  sub: string
  email: string
  full_name: string
  role: string
  is_verified: boolean
  exp: number
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    user: null,
    token: null,
    isAuthenticated: false,
    loading: false
  }),

  getters: {
    isCustomer: (state) => state.user?.role === 'customer',
    isVendor: (state) => state.user?.role === 'vendor',
    isAdmin: (state) => state.user?.role === 'admin',
    userName: (state) => state.user?.full_name || state.user?.email || ''
  },

  actions: {
    async login(email: string, password: string) {
      this.loading = true
      try {
        const config = useRuntimeConfig()
        const { data } = await $fetch<{ token: string, user: User }>('/auth/login', {
          baseURL: config.public.authServiceUrl,
          method: 'POST',
          body: { email, password }
        })

        this.setAuth(data.token, data.user)
        
        // Store in localStorage for persistence
        if (process.client) {
          localStorage.setItem('auth_token', data.token)
        }

        return { success: true }
      } catch (error: any) {
        console.error('Login error:', error)
        return { 
          success: false, 
          error: error.data?.message || 'فشل في تسجيل الدخول' 
        }
      } finally {
        this.loading = false
      }
    },

    async register(userData: {
      email: string
      password: string
      full_name: string
      phone?: string
    }) {
      this.loading = true
      try {
        const config = useRuntimeConfig()
        const { data } = await $fetch<{ token: string, user: User }>('/auth/register', {
          baseURL: config.public.authServiceUrl,
          method: 'POST',
          body: { ...userData, role: 'customer' }
        })

        this.setAuth(data.token, data.user)
        
        if (process.client) {
          localStorage.setItem('auth_token', data.token)
        }

        return { success: true }
      } catch (error: any) {
        console.error('Registration error:', error)
        return { 
          success: false, 
          error: error.data?.message || 'فشل في إنشاء الحساب' 
        }
      } finally {
        this.loading = false
      }
    },

    async logout() {
      this.user = null
      this.token = null
      this.isAuthenticated = false
      
      if (process.client) {
        localStorage.removeItem('auth_token')
      }

      await navigateTo('/auth/login')
    },

    async refreshUser() {
      if (!this.token) return

      try {
        const config = useRuntimeConfig()
        const user = await $fetch<User>('/auth/me', {
          baseURL: config.public.authServiceUrl,
          headers: {
            Authorization: `Bearer ${this.token}`
          }
        })

        this.user = user
      } catch (error) {
        console.error('Failed to refresh user:', error)
        await this.logout()
      }
    },

    setAuth(token: string, user: User) {
      this.token = token
      this.user = user
      this.isAuthenticated = true
    },

    initializeAuth() {
      if (!process.client) return

      const token = localStorage.getItem('auth_token')
      if (!token) return

      try {
        const decoded = jwtDecode<JWTPayload>(token)
        
        // Check if token is expired
        if (decoded.exp * 1000 < Date.now()) {
          localStorage.removeItem('auth_token')
          return
        }

        const user: User = {
          id: decoded.sub,
          email: decoded.email,
          full_name: decoded.full_name,
          role: decoded.role,
          is_verified: decoded.is_verified
        }

        this.setAuth(token, user)
      } catch (error) {
        console.error('Invalid token:', error)
        localStorage.removeItem('auth_token')
      }
    },

    getAuthHeaders() {
      return this.token ? { Authorization: `Bearer ${this.token}` } : {}
    }
  }
})
