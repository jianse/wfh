<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-bell"></i>
        <span>警报</span>
      </div>
    </template>
    <template #content>
      <div class="flex flex-row gap-2 flex-wrap">
        <div
          class="border rounded-md p-3 border-gray-600 min-w-64"
          v-for="alert in props.rawWorldState?.Alerts"
          :key="alert._id"
        >
          <div class="font-bold">{{ alert.MissionInfo.location }}</div>
          <div class="text-sm text-muted-color">{{ alert.MissionInfo.missionType }}</div>
          <p class="text-sm mt-1">{{ alert.MissionInfo.descText }}</p>
          <p class="text-sm"><CountDownTo :time="alert.Expiry" />结束</p>
          <StringWithIcon :text="rewardAsString(alert.MissionInfo.missionReward)" />
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState, Reward } from '@/api/world_state'
import StringWithIcon from '@/components/StringWithIcon.vue'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()

function rewardAsString(reward: Reward): string {
  if (!reward) return ''
  const l = []
  if (reward.credits) l.push(`${reward.credits} 星币`)
  if (reward.xp) l.push(`${reward.xp} 经验值`)
  if (reward.itemNames) l.push(...reward.itemNames.map((i) => `${i}`))
  if (reward.countedItems)
    l.push(...reward.countedItems.map((i) => `${i.ItemCountSum} X ${i.ItemName}`))
  return l.join(' + ')
}
</script>
