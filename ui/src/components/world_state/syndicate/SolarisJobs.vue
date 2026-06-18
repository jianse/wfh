<template>
  <!-- 希图斯 声望任务 -->
  <div class="px-2" v-if="syndicateMission != undefined">
    <div class="font-xl font-bold my-2 flex items-center">
      <img
        src="@/assets/icons/FactionSigilSolarisUnited.png"
        alt="索拉里斯"
        class="size-6 object-contain mr-1"
      />
      <div>索拉里斯</div>
    </div>
    <div class="flex flex-wrap gap-2">
      <MissionCard :job="job" v-for="job in syndicateMission.Jobs" :key="job.jobType">
        <template #xpIcon>
          <img
            src="@/assets/icons/ReputationLarge128.png"
            alt="声望"
            class="w-2 h-4 object-contain mr-1"
          />
        </template>
      </MissionCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SyndicateMission } from '@/api/world_state'

import type { RawWorldState } from '@/api/world_state'
import { computed } from 'vue'
const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()

const SYNDICATE_TAG = 'SolarisSyndicate'

type SolarisSyndicateMission = SyndicateMission & {
  Tag: 'SolarisSyndicate'
  Seed: number
  Jobs: {
    jobType: string
    rewards: string
    masteryReq: number
    minEnemyLevel: number
    maxEnemyLevel: number
    xpAmounts: number[]
  }[]
}

const syndicateMission = computed(() => {
  const mission = props.rawWorldState?.SyndicateMissions.find(
    (mission) => mission.Tag === SYNDICATE_TAG,
  ) as SolarisSyndicateMission | undefined
  return mission
})
</script>
