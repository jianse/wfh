<template>
  <pv-card>
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-infinity"></i>
        <span>无尽回廊</span>
      </div>
    </template>
    <template #content>
      <div class="flex flex-col gap-3">
        <div v-for="cat in props.rawWorldState?.EndlessXpChoices" :key="cat.Category">
          <div class="font-bold mb-2">{{ cat.Category === 'EXC_NORMAL' ? '普通' : '困难' }}</div>
          <ul class="list-disc list-outside pl-4 text-sm space-y-1">
            <li v-for="c in cat.Choices" :key="c">{{ c }}</li>
          </ul>
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()
</script>
