<script setup lang="ts">
import { computed } from 'vue'
import { useTimestamp } from '@vueuse/core'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
defineEmits<{ timeUp: [] }>()

const now = useTimestamp({ interval: 1000 })

const cetus = computed(() => {
  const act = props.rawWorldState?.SyndicateMissions?.find((m) => m.Tag === 'CetusSyndicate')
    ?.Activation
  if (!act) return { state: '未知', expiry: 0 }
  const a = new Date(act).getTime()
  const overlap = (((now.value - a) % 9000000) + 9000000) % 9000000
  const day = overlap < 6000000
  return {
    state: day ? '白昼' : '黑夜',
    expiry: day ? now.value + (6000000 - overlap) : now.value + (9000000 - overlap)
  }
})

const vallis = computed(() => {
  const lStart = Date.UTC(2018, 10, 10, 8, 13, 48)
  const off = (((now.value - lStart) % 1600000) + 1600000) % 1600000
  return off < 1200000
    ? { state: '寒冷', expiry: now.value - off + 1200000 }
    : { state: '温暖', expiry: now.value - off + 1600000 }
})

const earth = computed(() => {
  const eoff = (now.value / 1000) % 28800
  return eoff < 14400
    ? { state: '黑夜', expiry: now.value - eoff * 1000 + 14400000 }
    : { state: '白天', expiry: now.value - eoff * 1000 + 28800000 }
})

const cambion = computed(() => ({
  state: earth.value.state === '白天' ? '秩序' : '混乱',
  expiry: earth.value.expiry
}))

const zariman = computed(() => {
  const zoff = (((now.value - 1655182800000) % 18000000) + 18000000) % 18000000
  return zoff < 9000000
    ? { state: '克隆尼', expiry: now.value - zoff + 9000000 }
    : { state: '科普斯', expiry: now.value - zoff + 18000000 }
})

const cards = computed(() => [
  { key: 'cetus', name: '夜灵平野', ...cetus.value, tail: '切换' },
  { key: 'vallis', name: '奥布山谷', ...vallis.value, tail: '结束' },
  { key: 'cambion', name: '魔胎之境', ...cambion.value, tail: '结束' },
  { key: 'zariman', name: '扎里曼号', ...zariman.value, tail: '结束' },
  { key: 'earth', name: '地球', ...earth.value, tail: '结束' }
])
</script>

<template>
  <WorldStatePanel :title="'周期循环'" :icon="props.icon">
    <div class="ws-cycles">
      <div v-for="c in cards" :key="c.key" class="ws-cycle">
        <div class="ws-cycle-name">{{ c.name }}</div>
        <div class="ws-cycle-state">{{ c.state }}</div>
        <div class="mt-xs"><CountDown :time="c.expiry" />{{ c.tail }}</div>
      </div>
    </div>
  </WorldStatePanel>
</template>
