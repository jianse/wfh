<template>
  <div>
    <button
      class="box-border border border-neutral-200 rounded-sm px-2 hover:border-neutral-400 hover:border-2 h-8 w-36"
      @click="open = true"
    >
      <i class="pi pi-search ps-1 pe-2"></i><span class="pe-1">搜索</span><kbd>Ctrl</kbd>+<kbd
        >K</kbd
      >
    </button>
    <pv-dialog v-model:visible="open" modal position="top" :dismissable-mask="true"> </pv-dialog>
  </div>
</template>
<script lang="ts" setup>
import { useMagicKeys } from '@vueuse/core'
import { ref, watch } from 'vue'

const open = ref(false)

const { Ctrl_K } = useMagicKeys({
  passive: false,
  onEventFired(e) {
    if (e.ctrlKey && e.key.toLowerCase() === 'k' && e.type === 'keydown') e.preventDefault()
  },
})

watch(Ctrl_K, (v) => {
  if (v) {
    open.value = true
  }
})
</script>
