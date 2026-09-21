<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-flag"></i>
        <span>活动</span>
      </div>
    </template>
    <template #content>
      <div class="flex flex-col gap-2">
        <div
          v-for="event in props.rawWorldState?.Goals"
          :key="event._id"
          class="rounded-md border p-3 border-gray-600 bg-surface-50 dark:bg-surface-900"
        >
          <div class="font-bold text-lg">{{ event.Desc }}</div>
          <div class="text-sm text-muted-color">{{ event.Node || event.VictimNode }}</div>
          <CountDownTo :time="event.Expiry" />结束

          <div class="mt-2 text-sm">{{ event.ToolTip }}</div>
          <pv-progress-bar
            class="mt-2"
            v-if="event.Community"
            :value="event.Count"
            :max="event.Goal"
          />

          <pv-progress-bar
            class="mt-2"
            v-if="event.Tag === 'GhoulEmergence'"
            :value="event.HealthPct * 100"
          />
          <div v-for="(inter, i) in event.InterimRewards" :key="i" class="text-sm">
            {{ rewardAsString(inter) }}
          </div>
          <div v-if="event.Reward" class="text-sm font-medium">
            奖励：{{ rewardAsString(event.Reward) }}
          </div>
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState, Reward } from '@/api/world_state'

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
