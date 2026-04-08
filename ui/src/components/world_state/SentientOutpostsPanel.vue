<template>
  <pv-card v-if="props.rawWorldState?.SentientOutpost" class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-exclamation-triangle"></i>
        <span>灵煞异常</span>
      </div>
    </template>
    <template #content>
      <p class="text-sm text-muted-color mb-2">
        <CountDownTo :time="props.rawWorldState?.SentientOutpost.expiry" />轮换
      </p>
      <div class="font-medium">
        {{
          props.rawWorldState?.SentientOutpost.node_name ||
          props.rawWorldState?.SentientOutpost.node
        }}
      </div>
      <div class="text-sm">{{ props.rawWorldState?.SentientOutpost.mission_type }}</div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()
</script>
