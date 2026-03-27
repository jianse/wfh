<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-swords"></i>
        <span>入侵任务</span>
      </div>
    </template>
    <template #content>
      <div
        v-for="invasion in props.rawWorldState?.Invasions"
        :key="invasion._id"
        class="border rounded-md p-4 mb-4 border-gray-600 last:mb-0"
      >
        <div class="font-bold text-lg mb-3">{{ invasion.Node }}</div>

        <div class="flex flex-row place-content-between items-center gap-4">
          <div class="flex flex-col flex-1 items-start">
            <div class="font-medium">{{ invasion.DefenderMissionInfo.faction }}</div>
            <div class="text-sm">{{ rewardAsString(invasion.AttackerReward) }}</div>
          </div>
          <div class="text-center flex-1 font-bold text-lg">
            {{ Math.abs(invasion.Count) }} / {{ invasion.Goal }}
          </div>
          <div class="flex flex-col flex-1 items-end">
            <div class="font-medium">{{ invasion.AttackerMissionInfo.faction }}</div>
            <div class="text-sm">{{ rewardAsString(invasion.DefenderReward) }}</div>
          </div>
        </div>

        <pv-progress-bar :value="calcInvasionProgress(invasion)" class="mt-3" />
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { Invasion, RawWorldState, Reward } from '@/api/world_state'

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

function calcInvasionProgress(invasion: Invasion): number {
  const vsInfestation = invasion.AttackerMissionInfo.faction === '感染者'
  const pct = (1 + invasion.Count / invasion.Goal) * (vsInfestation ? 100 : 50)
  return Math.min(100, Math.max(0, Math.round(pct)))
}
</script>
