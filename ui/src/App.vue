<script setup lang="ts">
import { onBeforeUnmount, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import AppSidebar from '@/components/AppSidebar.vue'
import AppTopbar from '@/components/AppTopbar.vue'
import ToastHost from '@/components/ToastHost.vue'
import DialogHost from '@/components/DialogHost.vue'
import { useUiStore } from '@/stores/ui'
import { useThemeStore } from '@/stores/theme'
import { useAuthStore } from '@/stores/auth'

const ui = useUiStore()
const route = useRoute()

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    ui.closeTopDialog()
    ui.cancelConfirm()
    if (ui.navOpen) ui.setNavOpen(false)
  }
}

onMounted(() => {
  useThemeStore().init()
  useAuthStore().restore()
  document.addEventListener('keydown', onKeydown)
})
onBeforeUnmount(() => document.removeEventListener('keydown', onKeydown))

watch(
  () => route.fullPath,
  () => {
    if (ui.navOpen) ui.setNavOpen(false)
  }
)
</script>

<template>
  <div class="wfh-root">
    <div class="app">
      <AppSidebar />
      <div class="main">
        <AppTopbar />
        <main class="content">
          <RouterView v-slot="{ Component }">
            <component :is="Component" />
          </RouterView>
        </main>
      </div>
    </div>

    <div class="nav-backdrop" id="nav-backdrop" @click="ui.setNavOpen(false)"></div>

    <ToastHost />
    <DialogHost />
  </div>
</template>
