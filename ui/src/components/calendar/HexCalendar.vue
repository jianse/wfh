<template>
  <div>TODO: ADD CALENDAR BODY</div>
  <div v-for="m in months" :key="m.name" class="mt-3">
    <h3>{{ m.name }}</h3>
    <table class="border-collapse border border-blue-600">
      <thead>
        <tr>
          <th v-for="w in computedWeekDays" :key="w">{{ w }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, idx1) in m.dayMat" :key="idx1">
          <td class="border border-blue-600 px-4 py-2" v-for="(d, idx2) in row" :key="idx2">
            {{ d }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue'
import { chunk } from 'lodash'

const props = defineProps({
  season: String,
  offset: {
    type: Number,
    default: 0,
  },
  days: {
    type: Array,
    default: () => [],
  },
})

const weekDays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']

const computedWeekDays = computed(() => {
  const offset = props.offset % 7
  return weekDays.slice(offset, weekDays.length).concat(weekDays.slice(0, offset))
})

function calcDayMat(startWeekDay: number, dayOfMonth: number) {
  const total = Math.ceil((startWeekDay + dayOfMonth) / 7) * 7
  console.log('--', total)
  const mat = new Array<number | undefined>(total)
  mat.fill(undefined)
  for (let i = 0; i < dayOfMonth; i++) {
    mat[startWeekDay + i] = i + 1
  }
  return chunk(mat, 7)
}

const months = computed(() => {
  switch (props.season) {
    case 'CST_SPRING':
      return []
    case 'CST_SUMMER':
      return []
    case 'CST_FALL':
      return [
        {
          name: '10',
          dayOfMonth: 31,
          startWeekDay: 5,
          dayMat: calcDayMat(5 - props.offset, 31),
        },
        {
          name: '11',
          dayOfMonth: 30,
          startWeekDay: 1,
          dayMat: calcDayMat(1 - props.offset, 30),
        },
        {
          name: '12',
          dayOfMonth: 31,
          startWeekDay: 3,
          dayMat: calcDayMat(3 - props.offset, 31),
        },
      ]
    case 'CST_WINTER':
      return [
        {
          name: '1',
          dayOfMonth: 31,
          startWeekDay: 5,
          dayMat: calcDayMat(5 - props.offset, 31),
        },
        {
          name: '2',
          dayOfMonth: 28,
          startWeekDay: 1,
          dayMat: calcDayMat(1 - props.offset, 28),
        },
        {
          name: '3',
          dayOfMonth: 31,
          startWeekDay: 1,
          dayMat: calcDayMat(1 - props.offset, 31),
        },
      ]
    default:
      return []
  }
})
</script>
