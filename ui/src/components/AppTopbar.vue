<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { titleFor } from '@/router/wfh'
import { useAuthStore } from '@/stores/auth'
import { useThemeStore } from '@/stores/theme'
import { useUiStore } from '@/stores/ui'
import { logoutAndRedirect } from '@/lib/logout'
import SearchModal from '@/components/SearchModal.vue'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()
const theme = useThemeStore()
const ui = useUiStore()

const menuOpen = ref(false)
const acctRoot = ref<HTMLElement | null>(null)

const crumb = computed(() => titleFor(route.name))

function onDocClick(e: MouseEvent) {
  if (acctRoot.value && !acctRoot.value.contains(e.target as Node)) menuOpen.value = false
}

function openSearch() {
  ui.openDialog(SearchModal)
}

function onKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    openSearch()
  }
}

onMounted(() => {
  document.addEventListener('click', onDocClick)
  document.addEventListener('keydown', onKeydown)
})
onBeforeUnmount(() => {
  document.removeEventListener('click', onDocClick)
  document.removeEventListener('keydown', onKeydown)
})

function logout() {
  menuOpen.value = false
  logoutAndRedirect()
}

function goLogin() {
  router.push({ name: 'wfh-login', query: { redirect: route.fullPath } })
}
</script>

<template>
  <header class="topbar" data-od-id="topbar">
    <button
      class="hamburger"
      id="nav-toggle"
      type="button"
      aria-label="打开导航菜单"
      aria-controls="nav"
      :aria-expanded="ui.navOpen ? 'true' : 'false'"
      @click="ui.setNavOpen(!ui.navOpen)"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><path d="M4 7h16M4 12h16M4 17h16" /></svg>
    </button>
    <div class="crumb"><span id="crumb-parent">{{ crumb[0] }}</span><span class="sep">/</span><strong id="crumb-current">{{ crumb[1] }}</strong></div>

    <div class="searchbox" id="search-open" role="button" tabindex="0" @click="openSearch" @keydown.enter="openSearch">
      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="11" cy="11" r="7" /><path d="M20 20l-3.5-3.5" /></svg>
      <span>搜索战甲、武器、赋能…</span>
      <span class="kbd">Ctrl K</span>
    </div>

    <button
      class="icon-btn theme-toggle"
      id="theme-toggle"
      type="button"
      aria-label="切换深色模式"
      title="切换深色模式"
      :aria-pressed="theme.isDark ? 'true' : 'false'"
      data-od-id="theme-toggle"
      @click="theme.toggle()"
    >
      <svg class="ico-moon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M20.5 14.3A8.5 8.5 0 1 1 9.7 3.5a6.8 6.8 0 0 0 10.8 10.8z" /></svg>
      <svg class="ico-sun" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" aria-hidden="true"><circle cx="12" cy="12" r="4.2" /><path d="M12 2.6v2.2M12 19.2v2.2M2.6 12h2.2M19.2 12h2.2M5.3 5.3l1.6 1.6M17.1 17.1l1.6 1.6M18.7 5.3l-1.6 1.6M6.9 17.1l-1.6 1.6" /></svg>
    </button>

    <div class="acct" id="acct" ref="acctRoot">
      <button v-if="!auth.isLoggedIn" class="acct-btn" id="acct-login" type="button" @click="goLogin">
        <span class="acct-avatar"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><circle cx="12" cy="8" r="3.4" /><path d="M5.5 20a6.5 6.5 0 0 1 13 0" /></svg></span>
        <span class="acct-name">登录</span>
      </button>

      <template v-else>
        <button class="acct-btn" id="acct-toggle" type="button" @click.stop="menuOpen = !menuOpen">
          <span class="acct-avatar">{{ auth.initial }}</span>
          <span class="acct-name">{{ auth.displayName }}</span>
          <span class="role-tag" :class="`role-${auth.currentUser?.role}`">{{ auth.roleText }}</span>
        </button>
        <div class="acct-menu" id="acct-menu" :class="{ open: menuOpen }">
          <div class="menu-head">
            <div class="menu-user">{{ auth.displayName }}</div>
            <span class="meta">{{ auth.roleText }} · ID {{ auth.currentUser?.id }}</span>
          </div>
          <button type="button" data-menu="settings" @click="menuOpen = false; router.push({ name: 'wfh-settings' })">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><circle cx="12" cy="12" r="3" /><path d="M12 3v2.4M12 18.6V21M3 12h2.4M18.6 12H21M5.6 5.6l1.7 1.7M16.7 16.7l1.7 1.7M18.4 5.6l-1.7 1.7M7.3 16.7l-1.7 1.7" /></svg>账号设置
          </button>
          <button type="button" data-menu="logout" @click="logout">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M15 4h3a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-3" /><path d="M10 12h9M15 8l4 4-4 4" /></svg>退出登录
          </button>
        </div>
      </template>
    </div>
  </header>
</template>
