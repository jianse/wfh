<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-shield"></i>
        <span>虚空裂缝</span>
      </div>
    </template>
    <template #content>
      <div>
        <div class="card flex flex-wrap justify-center gap-4 mb-4">
          <div class="flex items-center gap-2" v-for="tier in tierOptions" :key="tier.value">
            <pv-checkbox
              v-model="filterTiers"
              :input-id="tier.value"
              name="fissure"
              :value="tier.value"
            />
            <label :for="tier.value">{{ tier.label }}</label>
          </div>
          <pv-button label="全选" size="small" @click="selectAllTiers" />
          <pv-button label="取消" size="small" @click="deselectAllTiers" />
        </div>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <h3 class="text-lg font-bold mb-2 flex items-center gap-2">
            <i class="pi pi-circle-fill text-blue-500"></i>
            普通
          </h3>
          <ul class="list-disc list-outside pl-4 space-y-1">
            <li v-for="fissure in normalFissures" :key="fissure._id" class="text-sm">
              <span class="font-medium">{{ tierAsString(fissure.Modifier) }}</span>
              {{ fissure.Node }} {{ fissure.MissionType }}
              <br />
              <CountDownTo :time="fissure.Expiry" @hit-zero="$emit('timeUp')"></CountDownTo>结束
            </li>
          </ul>
        </div>
        <div>
          <h3 class="text-lg font-bold mb-2 flex items-center gap-2">
            <i class="pi pi-circle-fill text-red-500"></i>
            钢铁之路
          </h3>
          <ul class="list-disc list-outside pl-4 space-y-1">
            <li v-for="fissure in hardFissures" :key="fissure._id" class="text-sm">
              <span class="font-medium">{{ tierAsString(fissure.Modifier) }}</span>
              {{ fissure.Node }} {{ fissure.MissionType }}
              <br />
              <CountDownTo :time="fissure.Expiry" @hit-zero="$emit('timeUp')"></CountDownTo>结束
            </li>
          </ul>
        </div>
        <div>
          <h3 class="text-lg font-bold mb-2 flex items-center gap-2">
            <i class="pi pi-circle-fill text-purple-500"></i>
            虚空风暴
          </h3>
          <ul class="list-disc list-outside pl-4 space-y-1">
            <li v-for="fissure in voidStorm" :key="fissure._id" class="text-sm">
              <span class="font-medium">{{ tierAsString(fissure.ActiveMissionTier) }}</span>
              {{ fissure.Node }}
              <br />
              <CountDownTo :time="fissure.Expiry" @hit-zero="$emit('timeUp')"></CountDownTo>结束
            </li>
          </ul>
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { clone } from 'lodash'
import type { RawWorldState } from '@/api/world_state'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()

defineEmits<{
  timeUp: []
}>()

const tierOptions = [
  { value: 'VoidT1', label: '古纪' },
  { value: 'VoidT2', label: '前纪' },
  { value: 'VoidT3', label: '中纪' },
  { value: 'VoidT4', label: '后纪' },
  { value: 'VoidT5', label: '安魂' },
  { value: 'VoidT6', label: '全能' },
]

const filterTiers = ref(['VoidT1', 'VoidT2', 'VoidT3', 'VoidT4', 'VoidT5', 'VoidT6'])

function selectAllTiers() {
  filterTiers.value = ['VoidT1', 'VoidT2', 'VoidT3', 'VoidT4', 'VoidT5', 'VoidT6']
}

function deselectAllTiers() {
  filterTiers.value = []
}

function strCmp(a: string, b: string) {
  return a === b ? 0 : a > b ? 1 : -1
}

function tierAsString(t: string): string {
  const map: Record<string, string> = {
    VoidT1: '古纪',
    VoidT2: '前纪',
    VoidT3: '中纪',
    VoidT4: '后纪',
    VoidT5: '安魂',
    VoidT6: '全能',
  }
  return map[t] || t
}

const normalFissures = computed(() => {
  if (!props.rawWorldState) return []
  return props.rawWorldState.ActiveMissions.filter((f) => !f.Hard)
    .filter((f) => filterTiers.value.includes(f.Modifier))
    .sort((a, b) => strCmp(a.Modifier, b.Modifier))
})

const hardFissures = computed(() => {
  if (!props.rawWorldState) return []
  return props.rawWorldState.ActiveMissions.filter((f) => f.Hard)
    .filter((f) => filterTiers.value.includes(f.Modifier))
    .sort((a, b) => strCmp(a.Modifier, b.Modifier))
})

const voidStorm = computed(() => {
  if (!props.rawWorldState) return []
  return clone(props.rawWorldState.VoidStorms)
    .filter((f) => filterTiers.value.includes(f.ActiveMissionTier))
    .sort((a, b) => strCmp(a.ActiveMissionTier, b.ActiveMissionTier))
})
</script>
