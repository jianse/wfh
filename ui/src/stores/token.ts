import { get_current_user } from '@/api/user'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useUserStore } from './user'
import { get_client_id } from '@/api/token'

export interface TokenRes {
  token: string
  token_valid_until: string
  refresh_token: string
  refresh_valid_until: string
}

export const useTokenStore = defineStore(
  'token',
  () => {
    const token = ref<string | undefined>(undefined)
    const token_valid_until = ref<string | undefined>(undefined)
    const refresh_token = ref<string | undefined>(undefined)
    const refresh_valid_until = ref<string | undefined>(undefined)
    const clientId = ref<string | undefined>(undefined)

    async function setToken(newToken?: TokenRes) {
      // token.value = newToken?.token ?? undefined

      if (newToken) {
        token.value = newToken.token
        token_valid_until.value = newToken.token_valid_until
        refresh_token.value = newToken.refresh_token
        refresh_valid_until.value = newToken.refresh_valid_until

        const { setUser } = useUserStore()
        const nu = await get_current_user()
        setUser(nu.data)
      } else {
        clearToken()
      }
    }

    function clearToken() {
      const { clearUser } = useUserStore()
      clearUser()
      token.value = undefined
      token_valid_until.value = undefined
      refresh_token.value = undefined
      refresh_valid_until.value = undefined
    }

    async function getOrCreateClientId(): Promise<string> {
      if (!clientId.value) {
        clientId.value = (await get_client_id()).data.nonce
      }
      console.log('clientId', clientId.value)
      return clientId.value
    }

    return {
      token,
      token_valid_until,
      refresh_token,
      refresh_valid_until,
      clientId,
      setToken,
      clearToken,
      getOrCreateClientId,
    }
  },
  { persist: true },
)
