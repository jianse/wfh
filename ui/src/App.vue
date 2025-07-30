<script setup lang="ts">
import type { MenuItem } from 'primevue/menuitem'
import { useRouter } from 'vue-router'
import { useUserStore } from './stores/user'
import { onMounted, ref } from 'vue'
import { useTokenStore } from './stores/token'
import { storeToRefs } from 'pinia'
import { get_current_user } from './api/user'

const navItems: MenuItem[] = [
  {
    label: '主页',
    route: { name: 'home' },
    icon: 'pi pi-home',
  },
  {
    label: '战甲',
    route: { name: 'warframes' },
  },
  // {
  //   label: '资源',
  //   route: { name: 'resources' },
  // },
  {
    label: '配方',
    route: { name: 'recipes' },
  },
  {
    label: '振幅晶体',
    route: { name: 'mods' },
  },
  {
    label: '武器',
    route: { name: 'weapons' },
  },
  {
    label: '遗物',
    route: { name: 'relics' },
  },
  // {
  //   label: '清单',
  //   route: { name: 'manifest' },
  // },
  {
    label: '赋能',
    route: { name: 'arcane' },
  },
  {
    label: '关于',
    route: { name: 'about' },
  },
]
const router = useRouter()

function toLogin() {
  router.push({ name: 'login' })
}

const { isLogin, user } = storeToRefs(useUserStore())

// user menu
const menu = ref()

const menuItems = [
  {
    label: '退出',
    icon: 'pi pi-sign-out',
    command: () => {
      const { clearToken } = useTokenStore()
      clearToken()
    },
  },
]

function toggleMenu(event: MouseEvent) {
  menu.value.toggle(event)
}

const tokenStore = useTokenStore()
const userStore = useUserStore()

onMounted(async () => {
  if (tokenStore.token) {
    const { data: cur_user } = await get_current_user()
    userStore.setUser(cur_user)
  }
})
</script>

<template>
  <pv-toast />
  <pv-dynamic-dialog />
  <div class="bg-slate-50 dark:bg-slate-800">
    <pv-menubar :model="navItems">
      <template #item="{ item, props, hasSubmenu }">
        <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
          <a v-ripple :href="href" v-bind="props.action" @click="navigate">
            <span v-if="item.icon" :class="item.icon" />
            <span>{{ item.label }}</span>
          </a>
        </router-link>
        <a v-else v-ripple :href="item.url" :target="item.target" v-bind="props.action">
          <!-- <span :class="item.icon" /> -->
          <span>{{ item.label }}</span>
          <span v-if="hasSubmenu" class="pi pi-fw pi-angle-down"></span>
        </a>
      </template>
      <template #end>
        <div class="flex items-center gap-2">
          <global-search />
          <div v-if="isLogin" class="flex items-center gap-2" @click="toggleMenu">
            <pv-avatar icon="pi pi-user" shape="circle" />
            <span>{{ user?.username }}</span>
            <pv-menu ref="menu" :model="menuItems" :popup="true"></pv-menu>
          </div>
          <div v-else class="flex items-center gap-2" @click="toLogin">
            <pv-button icon="pi pi-user" rounded></pv-button>
            <span>登录</span>
          </div>
        </div>
      </template>
    </pv-menubar>
    <div style="height: calc(100vh - 58px); overflow: auto">
      <router-view v-slot="{ Component }">
        <keep-alive :max="5">
          <component :is="Component" />
        </keep-alive>
      </router-view>
    </div>
  </div>
</template>

<style scoped>
.card {
  width: 500px;
  height: 200px;
  flex: initial;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 8px;
}

.sticky-header {
  position: sticky;
  top: 0;
  height: 5rem;
  background-color: #fff;
  border-bottom: 1px solid #333;
  display: flex;
  flex-direction: row;
  gap: 10px;
  padding: 10px;
}

.card img {
  object-fit: contain;
  height: 150px;
}
</style>
