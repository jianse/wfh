<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-clock"></i>
        <span>周期循环</span>
      </div>
    </template>
    <template #content>
      <div class="flex flex-row gap-2 flex-wrap">
        <pv-card class="min-w-48 grow">
          <template #title>夜灵平野</template>
          <template #content>
            <div class="text-xl font-bold mb-2">{{ cetusCycle.state }}</div>
            <p>
              <CountDownTo :time="cetusCycle.expiry" @hit-zero="$emit('timeUp')"></CountDownTo>切换
            </p>
          </template>
        </pv-card>
        <pv-card class="min-w-48 grow">
          <template #title>奥布山谷</template>
          <template #content>
            <div class="text-xl font-bold mb-2">{{ vallisCycle.state }}</div>
            <p>
              <CountDownTo :time="vallisCycle.expiry" @hit-zero="$emit('timeUp')"></CountDownTo>结束
            </p>
          </template>
        </pv-card>
        <pv-card class="min-w-48 grow">
          <template #title>魔胎之境</template>
          <template #content>
            <div class="text-xl font-bold mb-2">{{ cambionCycle.state }}</div>
            <p>
              <CountDownTo :time="cambionCycle.expiry" @hit-zero="$emit('timeUp')"></CountDownTo
              >结束
            </p>
          </template>
        </pv-card>
        <pv-card class="min-w-48 grow">
          <template #title>扎里曼号</template>
          <template #content>
            <div class="text-xl font-bold mb-2">{{ zarimanCycle.state }}</div>
            <p>
              <CountDownTo :time="zarimanCycle.expiry" @hit-zero="$emit('timeUp')"></CountDownTo
              >结束
            </p>
          </template>
        </pv-card>
        <pv-card class="min-w-48 grow">
          <template #title>地球</template>
          <template #content>
            <div class="text-xl font-bold mb-2">{{ earthCycle.state }}</div>
            <p>
              <CountDownTo :time="earthCycle.expiry" @hit-zero="$emit('timeUp')"></CountDownTo>结束
            </p>
          </template>
        </pv-card>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import dayjs from 'dayjs'
import type { RawWorldState } from '@/api/world_state'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
  now: number
}>()

defineEmits<{
  timeUp: []
}>()

const cetusCycle = computed(() => {
  if (props.rawWorldState) {
    const cetusMission = props.rawWorldState.SyndicateMissions.find(
      (m) => m.Tag == 'CetusSyndicate',
    )

    if (cetusMission) {
      const activation_time = dayjs(cetusMission.Activation)
      const overlap = (props.now - activation_time.valueOf()) % (150 * 60 * 1000)
      const state = overlap < 100 * 60 * 1000 ? '白昼' : '黑夜'
      let expiry = ''
      if (overlap < 100 * 60 * 1000) {
        expiry = dayjs(props.now).add(100, 'm').subtract(overlap, 'ms').toISOString()
      } else {
        expiry = dayjs(props.now).add(150, 'm').subtract(overlap, 'ms').toISOString()
      }
      return { state, expiry }
    }
  }
  return { state: 'unknown', expiry: '' }
})

const lStart = new Date('November 10, 2018 08:13:48 UTC')
const loopTime = 1600000
const coldTime = loopTime - 400000

const vallisCycle = computed(() => {
  const offset = (props.now - lStart.getUTCMilliseconds()) % loopTime
  if (offset < coldTime) {
    return { state: '寒冷', expiry: props.now - offset + coldTime }
  } else {
    return { state: '温暖', expiry: props.now - offset + loopTime }
  }
})

const earthCycle = computed(() => {
  if (props.rawWorldState) {
    const offset = (props.now / 1000) % (8 * 60 * 60)
    if (offset < 4 * 60 * 60) {
      const expiry = props.now - offset * 1000 + 4 * 60 * 60 * 1000
      return { state: '黑夜', expiry }
    } else {
      const expiry = props.now - offset * 1000 + 8 * 60 * 60 * 1000
      return { state: '白天', expiry }
    }
  }
  return { state: 'unknown', expiry: '' }
})

const cambionCycle = computed(() => {
  const earth = earthCycle.value
  return {
    state: earth.state === '白昼' ? '秩序' : '混乱',
    expiry: earth.expiry,
  }
})

const corpusTimeMillis = 1655182800000
const fullCycle = 18000000
const stateMaximum = 9000000

const zarimanCycle = computed(() => {
  const offset = (props.now - corpusTimeMillis) % fullCycle
  if (offset < stateMaximum) {
    return { state: '克隆尼', expiry: props.now - offset + stateMaximum }
  } else {
    return { state: '科普斯', expiry: props.now - offset + fullCycle }
  }
})
</script>
