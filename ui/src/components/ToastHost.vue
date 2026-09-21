<script setup lang="ts">
import { useUiStore } from '@/stores/ui'

const ui = useUiStore()
</script>

<template>
  <div class="toast-wrap" id="toasts">
    <div
      v-for="t in ui.toasts"
      :key="t.id"
      class="toast"
      :class="{ ok: t.kind === 'ok', err: t.kind === 'err' }"
      :style="t.leaving ? { transition: 'opacity .22s ease, transform .22s ease', opacity: 0, transform: 'translateY(6px)' } : null"
    >
      <span class="t-dot"></span>
      <div>
        <div class="t-title">{{ t.title }}</div>
        <div v-if="t.desc" class="t-desc">{{ t.desc }}</div>
      </div>
    </div>
  </div>
</template>
