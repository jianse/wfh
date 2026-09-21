import router from '@/router'
import { useAuthStore } from '@/stores/auth'
import { useUiStore } from '@/stores/ui'

/**
 * 退出登录。
 * 退出后如果当前页面不再需要登录/更高权限，则停留在当前页；否则回到主页。
 */
export function logoutAndRedirect() {
  const auth = useAuthStore()
  const ui = useUiStore()

  const name = auth.displayName
  const required = router.currentRoute.value.meta.role as string | undefined

  auth.logout()
  ui.toast('info', '已退出登录', name ? `${name} 已安全退出。` : '')

  if (required && !auth.hasRole(required)) {
    router.push({ name: 'wfh-home' })
  }
}
