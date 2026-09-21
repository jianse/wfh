<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-flask"></i>
        <span>每周科研</span>
      </div>
    </template>
    <template #content>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div v-for="conquest in props.rawWorldState?.Conquests" :key="conquest.Type">
          <ConquestPanel :conquest="conquest"></ConquestPanel>
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'
import ConquestPanel from '@/components/ConquestPanel.vue'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()
</script>
