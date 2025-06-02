import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

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
      component: () => import('@/views/AboutView.vue')
    }
  ],
})

export default router
