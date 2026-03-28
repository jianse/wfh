import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import { useUserStore } from '@/stores/user'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue'),
    },
    {
      path: '/resources',
      name: 'resources',
      component: () => import('../views/ResourcesView.vue'),
    },
    {
      path: '/manifest',
      name: 'manifest',
      component: () => import('../views/ManifestView.vue'),
    },
    {
      path: '/recipes',
      name: 'recipes',
      component: () => import('../views/RecipesView.vue'),
    },
    {
      path: '/warframes',
      name: 'warframes',
      component: () => import('../views/WarframeView.vue'),
    },
    {
      path: '/mods',
      name: 'mods',
      component: () => import('../views/ModsView.vue'),
    },
    {
      path: '/weapons',
      name: 'weapons',
      component: () => import('../views/WeaponsView.vue'),
    },
    {
      path: '/relics',
      name: 'relics',
      component: () => import('../views/RelicsView.vue'),
    },
    {
      path: '/arcane',
      name: 'arcane',
      component: () => import('../views/ArcaneView.vue'),
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('@/views/AboutView.vue'),
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/LoginView.vue'),
    },
    {
      path: '/world_state',
      name: 'world-state',
      component: () => import('@/views/WorldStateView.vue'),
    },
    {
      path: '/world_state/rewrite',
      name: 'world-state-rewrite',
      component: () => import('@/views/manage/WorldStateRewriteView.vue'),
      meta: {
        requiresAdmin: true,
        requiresRole: 'maintainer',
      },
    },
    {
      path: '/manage/sync-records',
      name: 'sync-records',
      component: () => import('@/views/manage/SyncRecordsView.vue'),
      meta: {
        requiresAdmin: true,
        requiresRole: 'maintainer',
      },
    },
    {
      path: '/manage/users',
      name: 'user-management',
      component: () => import('@/views/manage/UserManagementView.vue'),
      meta: {
        requiresRole: 'admin',
      },
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

export default router
