<script setup lang="ts">
import { computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { wfhName } from '@/router/wfh'

const auth = useAuthStore()

const dbItems = [
  {
    view: 'warframes',
    label: '战甲',
    count: '125',
    icon: '<path d="M12 3a4 4 0 0 0-4 4v2H6a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h1v4h10v-4h1a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2h-2V7a4 4 0 0 0-4-4z"/>'
  },
  { view: 'weapons', label: '武器', count: '952', icon: '<path d="M4 20L16 8l-4-4L4 12zM16 8l4 4"/>' },
  {
    view: 'mods',
    label: '振幅晶体',
    count: '1.8k',
    icon: '<rect x="3" y="3" width="18" height="18" rx="3"/><path d="M3 9h18M9 9v12"/>'
  },
  {
    view: 'arcane',
    label: '赋能',
    count: '168',
    icon: '<circle cx="12" cy="12" r="8"/><path d="M12 8l2 4-2 4-2-4z"/>'
  },
  { view: 'relics', label: '遗物', count: '5.4k', icon: '<path d="M12 3l8 5-8 5-8-5z"/><path d="M4 12l8 5 8-5M4 16l8 5 8-5"/>' },
  {
    view: 'resources',
    label: '资源',
    count: '3.5k',
    icon: '<path d="M12 3l7 4v5c0 5-3.5 8-7 9-3.5-1-7-4-7-9V7z"/>'
  },
  { view: 'recipes', label: '配方', count: '1.9k', icon: '<path d="M4 7h16M7 7v12h10V7M9 7V4h6v3"/>' }
]

const systemItems = [
  {
    view: 'world-state',
    label: '事件追踪',
    icon: '<circle cx="12" cy="12" r="9"/><path d="M3 12h18M12 3a15 15 0 0 1 0 18M12 3a15 15 0 0 0 0 18"/>'
  },
  {
    view: 'about',
    label: '关于',
    icon: '<circle cx="12" cy="12" r="9"/><path d="M12 11v5M12 8h.01"/>'
  },
  {
    view: 'sync-status',
    label: '数据同步',
    icon: '<ellipse cx="12" cy="6" rx="7" ry="3"/><path d="M5 6v6c0 1.66 3.13 3 7 3s7-1.34 7-3V6"/><path d="M5 12v6c0 1.66 3.13 3 7 3s7-1.34 7-3v-6"/>'
  }
]

const adminItems = computed(() => {
  const items = []
  if (auth.canMaintain) {
    items.push(
      {
        view: 'sync-records',
        label: '同步记录',
        icon: '<path d="M4 12a8 8 0 0 1 13.7-5.7L20 8"/><path d="M20 4v4h-4"/><path d="M20 12a8 8 0 0 1-13.7 5.7L4 16"/><path d="M4 20v-4h4"/>'
      },
      {
        view: 'rewrite',
        label: '重写数据',
        icon: '<path d="M4 7h9M19 7h1M4 17h1M11 17h9"/><circle cx="16" cy="7" r="2.2"/><circle cx="8" cy="17" r="2.2"/>'
      }
    )
  }
  if (auth.canAdmin) {
    items.push(
      {
        view: 'users',
        label: '用户管理',
        icon: '<circle cx="9" cy="8" r="3"/><path d="M3 20a6 6 0 0 1 12 0"/><path d="M16 5.5a3 3 0 0 1 0 5.8M18.5 20a6 6 0 0 0-3-5.2"/>'
      },
      {
        view: 'languages',
        label: '语言管理',
        icon: '<circle cx="12" cy="12" r="9"/><path d="M3 12h18M12 3a15 15 0 0 1 0 18M12 3a15 15 0 0 0 0 18"/>'
      }
    )
  }
  return items
})
</script>

<template>
  <aside class="sidebar" data-od-id="sidebar">
    <div class="sidebar-brand">
      <span class="brand-mark">仓</span>
      <div>
        <div class="brand-name">仓鼠助手</div>
        <div class="brand-sub">Warframe 数据百科</div>
      </div>
    </div>

    <nav class="nav" id="nav">
      <RouterLink class="nav-item" :to="{ name: 'wfh-home' }" active-class="active" exact-active-class="active">
        <span class="ni"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M3 11l9-7 9 7" /><path d="M5 10v10h14V10" /></svg></span>
        <span>主页</span>
      </RouterLink>

      <div class="nav-label">数据库</div>
      <RouterLink
        v-for="item in dbItems"
        :key="item.view"
        class="nav-item"
        :to="{ name: wfhName(item.view) }"
        active-class="active"
      >
        <span class="ni"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" v-html="item.icon" /></span>
        <span>{{ item.label }}</span>
        <span class="count">{{ item.count }}</span>
      </RouterLink>

      <div class="nav-label">世界状态</div>
      <RouterLink class="nav-item" :to="{ name: 'wfh-world-state' }" active-class="active">
        <span class="ni"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" v-html="systemItems[0].icon" /></span>
        <span>事件追踪</span>
      </RouterLink>

      <div class="nav-label">系统</div>
      <RouterLink
        v-for="item in systemItems.slice(1)"
        :key="item.view"
        class="nav-item"
        :to="{ name: wfhName(item.view) }"
        active-class="active"
      >
        <span class="ni"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" v-html="item.icon" /></span>
        <span>{{ item.label }}</span>
      </RouterLink>

      <template v-if="adminItems.length">
        <div class="nav-label nav-gate">管理后台</div>
        <RouterLink
          v-for="item in adminItems"
          :key="item.view"
          class="nav-item nav-gate"
          :to="{ name: wfhName(item.view) }"
          active-class="active"
        >
          <span class="ni"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" v-html="item.icon" /></span>
          <span>{{ item.label }}</span>
        </RouterLink>
      </template>
    </nav>

    <div class="sidebar-foot">
      <!-- <RouterLink class="nav-item" :to="{ name: 'home' }">
        <span class="ni"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M10 6l-6 6 6 6" /><path d="M20 12H4" /></svg></span>
        <span>返回主应用</span>
      </RouterLink> -->
      <div class="sync-line"><span class="status-dot"></span> 数据已同步</div>
      <div class="meta">版本 2026.09 · 官方导出</div>
    </div>
  </aside>
</template>
