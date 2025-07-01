import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { type CurrentUser } from '@/api/user'

type UserInfo = CurrentUser

export const useUserStore = defineStore('user', () => {
  const user = ref<UserInfo | undefined>(undefined)

  const isLogin = computed(() => user.value !== undefined)

  function setUser(newUser: UserInfo | undefined) {
    user.value = newUser
  }

  function clearUser() {
    user.value = undefined
  }
  const isAdmin = computed(() => user.value?.role === 'admin')
  return { user, isLogin, isAdmin, setUser, clearUser }
})
