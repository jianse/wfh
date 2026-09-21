import { createRouter, createWebHistory } from 'vue-router'
import type { Component } from 'vue'
import { useUserStore } from '@/stores/user'

type LazyView = () => Promise<Component>
const wfhViews = import.meta.glob('../views/*.vue') as unknown as Record<string, LazyView>
const wfhView = (name: string) => wfhViews[`../views/${name}.vue`]

type WfhAuthModule = {
  useAuthStore: () => { restore: () => void; hasRole: (role: string) => boolean }
}
type WfhUiModule = { useUiStore: () => { toast: (kind: string, title: string, desc?: string) => void } }
const wfhStores = import.meta.glob('../stores/*.ts') as unknown as Record<
  string,
  () => Promise<unknown>
>
const loadWfhAuth = wfhStores['../stores/auth.ts'] as () => Promise<WfhAuthModule>
const loadWfhUi = wfhStores['../stores/ui.ts'] as () => Promise<WfhUiModule>

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    // {
    //   path: '/',
    //   name: 'home',
    //   component: HomeView,
    // },
    // {
    //   path: '/about',
    //   name: 'about',
    //   // route level code-splitting
    //   // this generates a separate chunk (About.[hash].js) for this route
    //   // which is lazy-loaded when the route is visited.
    //   component: () => import('../views/AboutView.vue'),
    // },
    // {
    //   path: '/resources',
    //   name: 'resources',
    //   component: () => import('../views/ResourcesView.vue'),
    // },
    // {
    //   path: '/manifest',
    //   name: 'manifest',
    //   component: () => import('../views/ManifestView.vue'),
    // },
    // {
    //   path: '/recipes',
    //   name: 'recipes',
    //   component: () => import('../views/RecipesView.vue'),
    // },
    // {
    //   path: '/warframes',
    //   name: 'warframes',
    //   component: () => import('../views/WarframeView.vue'),
    // },
    // {
    //   path: '/mods',
    //   name: 'mods',
    //   component: () => import('../views/ModsView.vue'),
    // },
    // {
    //   path: '/weapons',
    //   name: 'weapons',
    //   component: () => import('../views/WeaponsView.vue'),
    // },
    // {
    //   path: '/relics',
    //   name: 'relics',
    //   component: () => import('../views/RelicsView.vue'),
    // },
    // {
    //   path: '/arcane',
    //   name: 'arcane',
    //   component: () => import('../views/ArcaneView.vue'),
    // },
    // {
    //   path: '/about',
    //   name: 'about',
    //   component: () => import('@/views/AboutView.vue'),
    // },
    // {
    //   path: '/login',
    //   name: 'login',
    //   component: () => import('@/views/LoginView.vue'),
    // },
    // {
    //   path: '/world_state',
    //   name: 'world-state',
    //   component: () => import('@/views/WorldStateView.vue'),
    // },
    // {
    //   path: '/world_state/rewrite',
    //   name: 'world-state-rewrite',
    //   component: () => import('@/views/manage/WorldStateRewriteView.vue'),
    //   meta: {
    //     requiresAdmin: true,
    //     requiresRole: 'maintainer',
    //   },
    // },
    // {
    //   path: '/manage/sync-records',
    //   name: 'sync-records',
    //   component: () => import('@/views/manage/SyncRecordsView.vue'),
    //   meta: {
    //     requiresAdmin: true,
    //     requiresRole: 'maintainer',
    //   },
    // },
    // {
    //   path: '/manage/users',
    //   name: 'user-management',
    //   component: () => import('@/views/manage/UserManagementView.vue'),
    //   meta: {
    //     requiresRole: 'admin',
    //   },
    // },
    // {
    //   path: '/settings',
    //   name: 'settings',
    //   component: () => import('@/views/SettingsView.vue'),
    // },
    // 仓鼠助手界面分组，统一挂在 / 前缀下。
    // 视图通过 import.meta.glob 懒加载，/wfh 仅作为路由名与登录页前缀保留。
    {
      path: '/',
      name: 'home',
      // component: wfhView('HomeView'),
      children: [
        { path: '', name: 'wfh-home', component: wfhView('HomeView') },
        { path: 'warframes', name: 'wfh-warframes', component: wfhView('WarframesView') },
        { path: 'weapons', name: 'wfh-weapons', component: wfhView('WeaponsView') },
        { path: 'mods', name: 'wfh-mods', component: wfhView('ModsView') },
        { path: 'arcane', name: 'wfh-arcane', component: wfhView('ArcaneView') },
        { path: 'relics', name: 'wfh-relics', component: wfhView('RelicsView') },
        { path: 'resources', name: 'wfh-resources', component: wfhView('ResourcesView') },
        { path: 'recipes', name: 'wfh-recipes', component: wfhView('RecipesView') },
        { path: 'world-state', name: 'wfh-world-state', component: wfhView('WorldStateView') },
        { path: 'about', name: 'wfh-about', component: wfhView('AboutView') },
        { path: 'login', name: 'wfh-login', component: wfhView('LoginView') },
        { path: 'settings', name: 'wfh-settings', component: wfhView('SettingsView') },
        { path: 'sync-status', name: 'wfh-sync-status', component: wfhView('SyncStatusView') },
        {
          path: 'sync-records',
          name: 'wfh-sync-records',
          meta: { role: 'maintainer' },
          component: wfhView('SyncRecordsView'),
        },
        {
          path: 'rewrite',
          name: 'wfh-rewrite',
          meta: { role: 'maintainer' },
          component: wfhView('RewriteView'),
        },
        {
          path: 'users',
          name: 'wfh-users',
          meta: { role: 'admin' },
          component: wfhView('UsersView'),
        },
        {
          path: 'languages',
          name: 'wfh-languages',
          meta: { role: 'admin' },
          component: wfhView('LanguagesView'),
        },
      ],
    },
    {
      path: '/404',
      name: 'not-found',
      component: () => import('@/views/NotFoundView.vue'),
    },
    {
      path: '/:pathMatch(.*)*',
      redirect: '/404',
    },
  ],
})

router.beforeEach((to, _from) => {
  const { user } = useUserStore()
  const role = user?.role

  const requiredRole = to.meta.requiresRole

  // admin 可以访问所有页面
  if (role === 'admin') {
    return
  }

  // maintainer 不能访问管理员专属页面
  if (role === 'maintainer' && requiredRole === 'admin') {
    // if (requiredRole === 'admin') {
    return { name: 'not-found' }
    // }
  }

  // normal 不能访问管理员和维护者页面
  if (role === 'normal' && (requiredRole === 'admin' || requiredRole === 'maintainer')) {
    return { name: 'not-found' }
  }
})

// 仓鼠助手沿用其自身的角色门禁（maintainer / admin），并回落到它自己的登录页。
router.beforeEach(async (to) => {
  const required = to.meta.role as string | undefined
  if (!required) return

  const [{ useAuthStore }, { useUiStore }] = await Promise.all([loadWfhAuth(), loadWfhUi()])

  const auth = useAuthStore()
  auth.restore()
  if (auth.hasRole(required)) return

  useUiStore().toast('info', '需要更高的权限', '请先登录维护者或管理员账号。')
  return { name: 'wfh-login', query: { redirect: to.fullPath } }
})

export default router
