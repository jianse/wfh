<template>
  <!-- 希图斯 声望任务 -->
  <div class="px-2" v-if="cetusSyndicateMission != undefined">
    <div class="font-xl font-bold my-2 flex items-center">
      <img
        src="@/assets/icons/FactionSigilEidolonCetusElder.png"
        alt="希图斯"
        class="size-6 object-contain mr-1"
      />
      <div>希图斯</div>
    </div>
    <div class="flex flex-wrap gap-2">
      <MissionCard v-for="job in cetusSyndicateMission.Jobs" :key="job.jobType" :job="job">
        <template #xpIcon>
          <img
            src="@/assets/icons/ReputationLarge128.png"
            alt="声望"
            class="w-2 h-4 object-contain mr-1"
          />
        </template>
      </MissionCard>
      <!-- <pv-card
        class="w-90 border border-gray-400"
        v-for="job in cetusSyndicateMission.Jobs"
        :key="job.jobType"
      >
        <template #title>
          {{ t(job.d?.name || '') || job.jobType }}
        </template>
        <template #content>
          <div class="flex justify-between border-b border-gray-500 mb-2 pb-1">
            <div class="flex items-center">
              <img
                src="@/assets/icons/ReputationLarge128.png"
                alt="声望"
                class="w-2 h-4 object-contain mr-1"
              />
              <div>
                {{ job.xpAmounts.reduce((acc, cur) => acc + cur, 0) }}
              </div>
            </div>
            <div>等级：{{ job.minEnemyLevel }} - {{ job.maxEnemyLevel }}</div>
          </div>
          <div class="mb-3">
            {{ t(job.d?.description ||'') }}
          </div>

          <ul v-if="job.combined_rewards != undefined">
            <li v-for="reward in job.combined_rewards" :key="reward.type">
              <ItemBox :unique-name="reward.type" :count="reward.itemCount" />
            </li>
          </ul>
        </template>
      </pv-card> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RawWorldState, SyndicateMissionWithJobs } from '@/api/world_state'
import { computed } from 'vue'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()

const cetusSyndicateMission = computed(() => {
  const cetusSM = props.rawWorldState?.SyndicateMissions.find(
    (mission) => mission.Tag === 'CetusSyndicate',
  ) as SyndicateMissionWithJobs | undefined

  return cetusSM
})
</script>
