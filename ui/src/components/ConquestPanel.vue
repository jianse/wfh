<template>
  <p>{{ computedConquest.Name }}</p>
  <CountDownTo :time="computedConquest.Expiry"></CountDownTo>轮换
  <div class="flex items-center gap-2">
    <pv-checkbox :input-id="computedConquest.Type" v-model="isHard" binary></pv-checkbox>
    <label :for="computedConquest.Type">精英科研</label>
  </div>

  <h3 class="mt-2 text-lg font-bold">任务</h3>
  <div class="flex flex-column gap-2">
    <div v-for="mission in computedConquest.Missions" :key="mission.faction">
      <!-- <div>{{ mission.faction }}</div> -->
      <div>{{ mission.missionType }}</div>
      <div>偏差：{{ mission.difficulty?.deviation }}</div>
      <div>
        <div v-for="risk in mission.difficulty?.risks" :key="risk">风险变量：{{ risk }}</div>
      </div>
    </div>
  </div>
  <h3 class="mt-2 text-lg font-bold">可选改造</h3>
  <ul class="list-disc list-outside pl-4">
    <li v-for="variable in computedConquest.Variables" :key="variable">
      {{ variable }}
    </li>
  </ul>
</template>

<script setup lang="ts">
import type { Conquest, ConquestMission } from '@/api/world_state'
import { cloneDeep } from 'lodash'
import { computed, ref } from 'vue'

const { conquest } = defineProps<{ conquest: Conquest }>()

const computedConquest = computed(() => {
  if (conquest) {
    const c = cloneDeep(conquest)
    return {
      ...c,
      Name: conquestName(conquest.Type),
      Missions: c.Missions.map((m) => {
        return {
          ...m,
          difficulty: calcMissionDifficulties(m),
        }
      }),
    }
  }
  return {
    Type: '',
    Name: '--',
    Expiry: 0,
    Missions: [],
    Variables: [],
  }
})

function conquestName(type: string): string {
  switch (type) {
    case 'CT_LAB':
      return '深层科研'
    case 'CT_HEX':
      return '时光科研'
    default:
      return type
  }
}

function calcMissionDifficulties(mission: ConquestMission) {
  const type = isHard.value ? 'CD_HARD' : 'CD_NORMAL'
  return mission.difficulties.find((d) => d.type === type)
}

const isHard = ref(false)
</script>
