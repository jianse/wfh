<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-swords"></i>
        <span>入侵任务</span>
      </div>
    </template>
    <template #content>
      <div class="gap-2 max-h-120 grid grid-cols-1 lg:grid-cols-2 overflow-y-auto px-2">
        <div
          v-for="invasion in invasions"
          :key="invasion._id"
          class="rounded-md border p-3 border-gray-600 bg-surface-50 dark:bg-surface-900"
        >
          <div class="font-bold text-lg">{{ invasion.Node }}</div>

          <div class="flex flex-row justify-between items-center gap-4 mt-2">
            <div class="flex flex-col flex-1 items-start">
              <div class="font-medium">{{ invasion.DefenderMissionInfo.faction }}</div>
              <div class="text-sm">{{ rewardAsString(invasion.AttackerReward) || '-' }}</div>
            </div>
            <div class="text-center flex-1 font-bold text-lg">
              {{ Math.abs(invasion.Count) }} / {{ invasion.Goal }}
            </div>
            <div class="flex flex-col flex-1 items-end">
              <div class="font-medium">{{ invasion.AttackerMissionInfo.faction }}</div>
              <div class="text-sm">{{ rewardAsString(invasion.DefenderReward) }}</div>
            </div>
          </div>

          <pv-progress-bar
            :value="calcInvasionProgress(invasion)"
            class="mt-2"
            :show-value="false"
          />
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { Invasion, RawWorldState, Reward } from '@/api/world_state'
import { computed } from 'vue'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()

const invasions = computed(
  () => props.rawWorldState?.Invasions.filter((i) => !(Math.abs(i.Count) >= i.Goal)) ?? [],
)

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
