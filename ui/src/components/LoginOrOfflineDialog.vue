<template>
  <!-- <pv-dialog v-model:visible="visible" modal :style="{ width: '25rem' }"> -->
  <span>您的登陆状态已失效，是否重新登录？</span>
  <div class="flex justify-end gap-2 mt-6">
    <pv-button
      type="button"
      label="前往登录"
      size="small"
      severity="secondary"
      @click="goLogin"
    ></pv-button>
    <pv-button type="button" label="保持离线" size="small" @click="stayOffline"></pv-button>
  </div>
  <pv-dialog ref="dlg"></pv-dialog>
</template>
<script lang="ts" setup>
import { inject, type Ref } from 'vue'
import { useRouter } from 'vue-router'
import type { DynamicDialogInstance } from 'primevue/dynamicdialogoptions'
import { useTokenStore } from '@/stores/token'

const dialogRef = inject<Ref<DynamicDialogInstance>>('dialogRef')

function stayOffline() {
  const tokenStore = useTokenStore()
  tokenStore.clearToken()

  dialogRef?.value.close()
}

const router = useRouter()

function goLogin() {
  dialogRef?.value.close()
  router.push({ name: 'login' })
}

</script>
