<template>
  <p>{{ conquestName(conquest.Type) }}</p>
  <CountDownTo :time="conquest.Expiry"></CountDownTo>轮换
  <div class="flex items-center gap-2">
    <pv-checkbox :input-id="conquest.Type" v-model="isHard" binary></pv-checkbox>
    <label :for="conquest.Type">精英科研</label>
  </div>

  <h3 class="mt-2 text-lg font-bold">任务</h3>
  <div class="flex flex-col gap-2 mt-2">
    <ConquestMissionItem
      v-for="(mission, index) in conquest.Missions"
      :mission="mission"
      :hard="isHard"
      :key="index"
    />
  </div>
  <h3 class="mt-2 text-lg font-bold">可选改造</h3>
  <ul class="list-disc list-outside pl-4">
    <li v-for="variable in conquest.TVariables" :key="variable.name">
      <div>{{ variable.name }}</div>
      <div><StringWithIcon :text="variable.desc"></StringWithIcon></div>
    </li>
  </ul>
</template>

<script setup lang="ts">
import type { Conquest } from '@/api/world_state'
import { computed } from 'vue'
import { useWorldStatePrefsStore } from '@/stores/world_state_prefs'

const { conquest } = defineProps<{ conquest: Conquest }>()

const store = useWorldStatePrefsStore()

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

const isHard = computed({
  get: () => store.getConquestHard(conquest.Type),
  set: (value) => store.setConquestHard(conquest.Type, value),
})
</script>
