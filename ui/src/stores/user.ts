import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

interface UserInfo {
  id: number
  username: string
}

export const useUserStore = defineStore('user', () => {
  const user = ref<UserInfo | undefined>(undefined)

  const isLogin = computed(() => user.value !== undefined)

  function setUser(newUser: UserInfo | undefined) {
    user.value = newUser
  }

  function clearUser() {
    user.value = undefined
  }
  return { user, isLogin, setUser, clearUser }
})
