<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-star"></i>
        <span>钢铁之路每周</span>
      </div>
    </template>
    <template #content>
      <div class="text-lg font-bold">
        {{ props.rawWorldState?.SteelPath.current_reward.name }}
      </div>
      <div class="text-sm text-muted-color">
        {{ props.rawWorldState?.SteelPath.current_reward.cost }} 钢铁精华
      </div>
      <p class="mt-2"><CountDownTo :time="props.rawWorldState?.SteelPath.Expiry" /> 轮换</p>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()
</script>
