import { get_current_user } from '@/api/user'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useUserStore } from './user'

export const useTokenStore = defineStore(
  'token',
  () => {
    const token = ref<string | undefined>(undefined)

    async function setToken(newToken?: string) {
      token.value = newToken
      if (newToken) {
        const { setUser } = useUserStore()
        const nu = await get_current_user()
        setUser(nu.data)
      }
    }

    function clearToken() {
      const { clearUser } = useUserStore()
      clearUser()
      token.value = undefined
    }
    return { token, setToken, clearToken }
  },
  { persist: true },
)
