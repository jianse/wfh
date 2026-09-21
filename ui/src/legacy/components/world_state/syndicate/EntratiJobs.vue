<template>
  <!-- 希图斯 声望任务 -->
  <div class="px-2" v-if="syndicateMission != undefined">
    <div class="font-xl font-bold my-2 flex items-center">
      <img
        src="@/assets/icons/FactionSigilEntrati.png"
        alt="英择谛"
        class="size-6 object-contain mr-1"
      />
      <div>英择谛</div>
    </div>
    <div class="flex flex-wrap gap-2">
      <MissionCard v-for="job in syndicateMission.Jobs" :key="job.jobType" :job="job">
        <template #xpIcon>
          <img
            src="@/assets/icons/EntratiTasksMotherASmall.png"
            alt="母亲石印"
            class="min-w-4 h-4 object-contain mr-1"
          />
        </template>
      </MissionCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SyndicateMission as syndicateMission } from '@/api/world_state'

import type { RawWorldState } from '@/api/world_state'
import { computed } from 'vue'
const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()

const SYNDICATE_TAG = 'EntratiSyndicate'

type SolarisSyndicateMission = syndicateMission & {
  Tag: 'EntratiSyndicate'
  Seed: number
  Jobs: {
    jobType: string
    rewards: string
    masteryReq: number
    minEnemyLevel: number
    maxEnemyLevel: number
    xpAmounts: number[]
    isVault: boolean
  }[]
}

const syndicateMission = computed(() => {
  const mission = props.rawWorldState?.SyndicateMissions.find(
    (mission) => mission.Tag === SYNDICATE_TAG,
  ) as SolarisSyndicateMission | undefined

  return mission
})
</script>
