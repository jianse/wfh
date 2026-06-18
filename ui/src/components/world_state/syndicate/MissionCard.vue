<template>
  <pv-card class="w-90 border border-gray-400">
    <template #title>
      {{ t(bounty.name) }}
    </template>
    <template #content>
      <div class="flex justify-between border-b border-gray-500 mb-2 pb-1">
        <div class="flex items-center">
          <slot name="xpIcon"></slot>
          <!-- <img
            src="@/assets/icons/ReputationLarge128.png"
            alt="声望"
            class="w-2 h-4 object-contain mr-1"
          /> -->
          <div>
            {{ job.xpAmounts.reduce((acc, cur) => acc + cur, 0) }}
          </div>
        </div>
        <div>等级：{{ job.minEnemyLevel }} - {{ job.maxEnemyLevel }}</div>
      </div>
      <div class="mb-3">
        {{ t(bounty.description) }}
      </div>

      <ul v-if="combinedRewards != undefined">
        <li v-for="reward in combinedRewards" :key="reward.type">
          <ItemBox :unique-name="reward.type" :count="reward.itemCount" />
          <!-- {{ reward?.d?.value?.name || reward.type }} {{ reward.itemCount }} -->
        </li>
      </ul>
    </template>
  </pv-card>
</template>
<script setup lang="ts">
import { findBounties, findRewards, type RewardItem } from '@/util/localDB'
import { computedAsync } from '@vueuse/core'
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  job: {
    jobType: string
    rewards: string
    masteryReq: number
    minEnemyLevel: number
    maxEnemyLevel: number
    xpAmounts: number[]
  }
}>()

const { t } = useI18n()

const bounty = computedAsync(
  async () => {
    return (
      (await findBounties(props.job.jobType)) || {
        uniqueName: props.job.jobType,
        name: '',
        description: '',
        stages: [],
        icon: '',
      }
    )
  },
  { uniqueName: props.job.jobType, name: '', description: '', stages: [], icon: '' },
)

function rewardEqual(a: RewardItem, b: RewardItem) {
  return a.type === b.type && a.itemCount === b.itemCount
}

function reduceRewards(acc: RewardItem[], cur: RewardItem) {
  if (acc.findIndex((it) => rewardEqual(it, cur)) === -1) {
    acc.push(cur)
  }
  return acc
}

const combinedRewards = computedAsync(async () => {
  const rewards = (await findRewards(props.job.rewards)) || {
    uniqueName: props.job.rewards,
    value: [],
  }
  return rewards.value.flatMap((rt) => rt).reduce(reduceRewards, []) as RewardItem[]
}, [])
</script>
