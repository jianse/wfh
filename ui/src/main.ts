import './assets/main.css'
import 'primeicons/primeicons.css'
// import 'virtual:uno.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

import PrimeVue from 'primevue/config'
import Aura from '@primeuix/themes/aura'
import ConfirmationService from 'primevue/confirmationservice'
import DialogService from 'primevue/dialogservice'
import ToastService from 'primevue/toastservice'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import { loadLocaleMessages } from './i18n'
import { createI18n } from 'vue-i18n'

const app = createApp(App)

const i18n = createI18n({
  legacy: false,
  locale: 'zh',
})
app.use(i18n)

await loadLocaleMessages(i18n, 'zh')

app.use(ToastService)
app.use(ConfirmationService)
app.use(DialogService)

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(pinia)
app.use(router)
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      prefix: 'p',
      cssLayer: {
        name: 'primevue',
        order: 'theme, base, primevue',
      },
    },
  },
})

app.mount('#app')
