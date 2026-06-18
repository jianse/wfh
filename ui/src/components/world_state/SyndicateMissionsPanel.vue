<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-group"></i>
        <span>集团任务</span>
      </div>
    </template>
    <template #content>
      <CetusJobs :rawWorldState="props.rawWorldState" />
      <SolarisJobs :rawWorldState="props.rawWorldState" />
      <EntratiJobs :rawWorldState="props.rawWorldState" />
      <code>
        <pre>{{ ss }}</pre>
      </code>
    </template>
  </pv-card>
</template>
<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'

import { computed } from 'vue'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
  now: number
}>()

const ss = computed(
  () =>
    props.rawWorldState?.SyndicateMissions.filter(
      (mission) => mission.Tag === 'EntratiSyndicate',
    ) || [],
)
</script>
