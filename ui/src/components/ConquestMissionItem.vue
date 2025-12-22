<template>
  <div
    class="border border-gray-600 cursor-pointer px-3 py-1 rounded-md"
    @click="toggle"
    @mouseenter="show"
    @mouseleave="hide"
  >
    <!-- <template #title> -->
    <div class="font-bold text-md">{{ mission.missionType }}</div>
    <!-- </template> -->

    <!-- <template #content> -->
    <div class="flex h-18 flex-col justify-center">
      <div class="w-fit">偏差：{{ difficulty?.deviation }}</div>
      <div v-for="risk in difficulty?.tRisks" :key="risk.name">风险变量：{{ risk.name }}</div>
    </div>
    <!-- </template> -->
  </div>
  <pv-popover
    ref="op"
    class="w-130 bg-slate-700"
    :pt="{
      arrow: { class: 'bg-slate-700' },
    }"
  >
    <div>
      <div class="font-bold">{{ difficulty?.deviation }}</div>
      <div>
        <string-with-icon :text="difficulty?.deviationDesc ?? ''"></string-with-icon>
      </div>
      <div class="mt-2" v-for="risk in difficulty?.tRisks" :key="risk.name">
        <div class="font-bold">{{ risk.name }}</div>
        <div>
          <string-with-icon :text="risk.desc"></string-with-icon>
        </div>
      </div>
    </div>
  </pv-popover>
</template>

<script lang="ts" setup>
import type { ConquestMission } from '@/api/world_state'
import { computed, ref, type PropType } from 'vue'
// import type StringWithIconVue from './StringWithIcon.vue'
// import vTooltip from 'primevue/tooltip'

const props = defineProps({
  mission: {
    type: Object as PropType<ConquestMission>,
    required: false,
    default: () => {
      return {
        missionType: '',
        difficulty: {
          deviation: '',
          tRisks: [],
        },
      }
    },
  },
  hard: {
    type: Boolean,
    required: false,
    default: false,
  },
})

const difficulty = computed(() => {
  const type = props.hard ? 'CD_HARD' : 'CD_NORMAL'
  return props.mission.difficulties.find((d) => d.type === type)
})
const op = ref()

function toggle(event: MouseEvent) {
  if (!op.value) return
  op.value.toggle(event)
}
function show(event: MouseEvent) {
  if (!op.value) return
  op.value.show(event)
}
function hide(event: MouseEvent) {
  if (!op.value) return
  op.value.hide(event)
}
</script>
