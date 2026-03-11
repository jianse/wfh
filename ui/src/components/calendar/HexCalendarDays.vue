<template>
  <div>
    <!-- 筛选六人组任务 -->
    <div class="card flex flex-wrap justify-center gap-4">
      <div class="flex items-center gap-2">
        <pv-checkbox v-model="filterEvents" inputId="hex1" name="hexFilter" value="CET_CHALLENGE" />
        <label for="hex1"> 待办事项 </label>
      </div>
      <div class="flex items-center gap-2">
        <pv-checkbox v-model="filterEvents" inputId="hex2" name="hexFilter" value="CET_REWARD" />
        <label for="hex2"> 大奖！ </label>
      </div>
      <div class="flex items-center gap-2">
        <pv-checkbox v-model="filterEvents" inputId="hex3" name="hexFilter" value="CET_UPGRADE" />
        <label for="hex3"> 覆写 </label>
      </div>
      <div class="flex items-center gap-2">
        <pv-checkbox v-model="filterEvents" inputId="hex4" name="hexFilter" value="CET_BIRTHDAY" />
        <label for="hex4"> 生日 </label>
      </div>
      <pv-button size="small" label="全选" @click="selectAll"></pv-button>
      <pv-button size="small" label="取消" @click="deselectAll"></pv-button>
    </div>
  </div>
  <div class="flex flex-col gap-2">
    <div v-for="day in computedDays" :key="day.day" class="flex flex-row gap-2">
      <div class="size-10 text-center font-bold text-lg">{{ day.day }}</div>
      <div class="w-18">{{ translateEventType(day.events[0].type) }}</div>
      <div>
        <div v-for="(event, index) in day.events" :key="index" class="flex flex-row gap-2">
          <div v-if="event.type == 'CET_BIRTHDAY'">
            <div>{{ event.title }}</div>

            <div>{{ event.desc }}</div>
          </div>
          <div v-if="event.type == 'CET_CHALLENGE'">
            <div>{{ event.title }}</div>
            <div>{{ event.desc }}</div>
          </div>
          <div v-if="event.type == 'CET_REWARD'">
            <!-- <div>{{ event.reward }}</div> -->
            <StringWithIcon :text="event.name"></StringWithIcon>
          </div>
          <div v-if="event.type == 'CET_UPGRADE'">
            <!-- <div>{{ event.upgrade }}</div> -->
            <div>{{ event.name }}</div>
            <StringWithIcon :text="event.desc"></StringWithIcon>
            <!-- <div>{{ event.desc }}</div> -->
          </div>
          <!-- <div>{{ event.type }}</div> -->
        </div>
      </div>
      <!-- {{ day }} -->
    </div>
  </div>
</template>
<script lang="ts" setup>
import type { HexDay } from '@/api/world_state'
import { computed, ref } from 'vue'

const props = defineProps<{
  days: HexDay[]
}>()

const filterEvents = ref(['CET_CHALLENGE', 'CET_REWARD', 'CET_BIRTHDAY', 'CET_UPGRADE'])

function selectAll() {
  filterEvents.value = ['CET_CHALLENGE', 'CET_REWARD', 'CET_BIRTHDAY', 'CET_UPGRADE']
}
function deselectAll() {
  filterEvents.value = []
}

const computedDays = computed(() => {
  return props.days.filter((day) => {
    return day.events.some((event) => {
      return filterEvents.value.includes(event.type)
    })
  })
})

function translateEventType(type: string) {
  switch (type) {
    case 'CET_BIRTHDAY':
      return '生日'
    case 'CET_CHALLENGE':
      return '待办事项'
    case 'CET_REWARD':
      return '大奖！'
    case 'CET_UPGRADE':
      return '覆写'

    default:
      return '未知'
  }
}
</script>
