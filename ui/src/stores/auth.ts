import { defineStore } from 'pinia'
import { computed } from 'vue'
import { get_token } from '@/api/token'
import { get_current_user } from '@/api/user'
import { is_success } from '@/api'
import { useTokenStore } from '@/stores/token'
import { useUserStore } from '@/stores/user'
import { roleLabel, roleRank } from '@/lib/utils'

export interface AuthResult {
  ok: boolean
  field?: 'user' | 'pass'
  msg?: string
}

/**
 * 仓鼠助手鉴权：复用主应用的 token / user store（真实后端 API）。
 * 保留原型阶段的同名 getter / action 表面，避免各视图改动。
 */
export const useAuthStore = defineStore('auth', () => {
  const tokenStore = useTokenStore()
  const userStore = useUserStore()

  const currentUser = computed(() => userStore.user ?? null)

  const isLoggedIn = computed(() => userStore.isLogin)
  const rank = computed(() => roleRank(currentUser.value && currentUser.value.role))
  const canMaintain = computed(() => rank.value >= 1)
  const canAdmin = computed(() => rank.value >= 2)
  const displayName = computed(() => (currentUser.value ? currentUser.value.username : ''))
  const initial = computed(() =>
    currentUser.value ? [...currentUser.value.username][0].toUpperCase() : ''
  )
  const roleText = computed(() => (currentUser.value ? roleLabel(currentUser.value.role) : ''))

  /** 恢复会话：pinia 持久化已还原用户；若仅有 token 则向服务端补拉一次。 */
  async function restore(): Promise<void> {
    if (!tokenStore.token || userStore.user) return
    try {
      const { data } = await get_current_user()
      userStore.setUser(data)
    } catch {
      /* token 失效等，保持未登录态 */
    }
  }

  /** 校验账号密码；返回 { ok, field, msg } 供表单展示错误。 */
  async function authenticate(username: string, password: string): Promise<AuthResult> {
    const u = (username || '').trim()
    if (!u) return { ok: false, field: 'user', msg: '请输入用户名' }
    if (!password) return { ok: false, field: 'pass', msg: '请输入密码' }
    try {
      const client_id = await tokenStore.getOrCreateClientId()
      const res = await get_token({ client_id, username: u, password })
      if (!is_success(res)) {
        return { ok: false, field: 'pass', msg: res.msg || '用户名或密码错误' }
      }
      await tokenStore.setToken(res.data)
      return { ok: true }
    } catch {
      return { ok: false, field: 'pass', msg: '用户名或密码错误' }
    }
  }

  function logout() {
    tokenStore.clearToken()
  }

  function hasRole(required: string): boolean {
    return rank.value >= roleRank(required)
  }

  return {
    currentUser,
    isLoggedIn,
    rank,
    canMaintain,
    canAdmin,
    displayName,
    initial,
    roleText,
    restore,
    authenticate,
    logout,
    hasRole,
  }
})
