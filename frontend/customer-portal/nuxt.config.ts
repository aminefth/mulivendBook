export default defineNuxtConfig({
  devtools: { enabled: true },
  
  modules: [
    '@nuxt/ui',
    '@pinia/nuxt',
    '@nuxtjs/i18n',
    '@vueuse/nuxt',
    '@nuxtjs/color-mode'
  ],

  css: ['~/assets/css/main.css'],

  runtimeConfig: {
    public: {
      apiBase: process.env.API_BASE_URL || 'http://localhost:8080/api',
      authServiceUrl: process.env.AUTH_SERVICE_URL || 'http://localhost:3001',
      catalogServiceUrl: process.env.CATALOG_SERVICE_URL || 'http://localhost:3003',
      orderServiceUrl: process.env.ORDER_SERVICE_URL || 'http://localhost:4003'
    }
  },

  i18n: {
    locales: [
      { code: 'ar', iso: 'ar-MA', file: 'ar.json', name: 'العربية', dir: 'rtl' },
      { code: 'fr', iso: 'fr-FR', file: 'fr.json', name: 'Français', dir: 'ltr' },
      { code: 'en', iso: 'en-US', file: 'en.json', name: 'English', dir: 'ltr' }
    ],
    lazy: true,
    langDir: 'locales/',
    defaultLocale: 'ar',
    strategy: 'prefix_except_default',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root'
    }
  },

  tailwindcss: {
    config: {
      content: [
        './components/**/*.{js,vue,ts}',
        './layouts/**/*.vue',
        './pages/**/*.vue',
        './plugins/**/*.{js,ts}',
        './nuxt.config.{js,ts}',
        './app.vue'
      ],
      theme: {
        extend: {
          colors: {
            primary: {
              50: '#f0f9ff',
              500: '#3b82f6',
              600: '#2563eb',
              700: '#1d4ed8',
              900: '#1e3a8a'
            },
            secondary: {
              50: '#fdf4ff',
              500: '#a855f7',
              600: '#9333ea',
              700: '#7c3aed'
            }
          },
          fontFamily: {
            'arabic': ['Noto Sans Arabic', 'sans-serif'],
            'latin': ['Inter', 'sans-serif']
          }
        }
      },
      plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
        require('@tailwindcss/aspect-ratio')
      ]
    }
  },

  app: {
    head: {
      title: 'BookMarket Pro - أكبر سوق للكتب في المغرب',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'اكتشف أكبر مجموعة من الكتب العربية والفرنسية والإنجليزية في المغرب. كتب إسلامية، أدب، علوم، وأكثر.' }
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Noto+Sans+Arabic:wght@300;400;500;600;700&family=Inter:wght@300;400;500;600;700&display=swap' }
      ]
    }
  },

  ssr: true,
  nitro: {
    preset: 'node-server'
  }
})
