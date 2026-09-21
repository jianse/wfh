<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-bolt"></i>
        <span>执刑官猎杀</span>
      </div>
    </template>
    <template #content>
      <div v-for="hunt in props.rawWorldState?.LiteSorties" :key="hunt._id">
        <div class="font-bold text-lg mb-2">BOSS: {{ hunt.BossName }}</div>
        <p class="text-sm text-muted-color mb-2"><CountDownTo :time="hunt.Expiry" />轮换</p>
        <ul class="list-disc list-outside pl-4 text-sm space-y-1">
          <li v-for="(mission, index) in hunt.Missions" :key="index">
            {{ mission.node }} {{ mission.missionType }}
          </li>
        </ul>
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
