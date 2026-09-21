<template>
  <span>{{ val }}</span>
</template>
<script setup lang="ts">
import { useTimestamp } from '@vueuse/core'
import { computed } from 'vue'
import dayjs from 'dayjs'
import duration, { type Duration } from 'dayjs/plugin/duration'
import relativeTime from 'dayjs/plugin/relativeTime'
import zhCN from 'dayjs/locale/zh-cn'

dayjs.extend(duration)
dayjs.extend(relativeTime)
dayjs.locale(zhCN)
const emits = defineEmits<{
  (e: 'hitZero'): void
}>()

const props = defineProps<{
  time: string | Date | undefined | number
}>()

const now = useTimestamp({ interval: 1000 })

const val = computed(() => {
  if (!props.time) {
    return '-'
  }
  const to = dayjs(props.time)
  const cur = dayjs(now.value)
  const dur = dayjs.duration(to.diff(cur))
  if (Math.floor(dur.asSeconds()) == 0) {
    console.log('hitZero')
    emits('hitZero')
  }
  return humanFormat(dur)
})

function humanFormat(dur: Duration) {
  // console.log(dur)
  let suffix = '后'
  if (dur.asSeconds() < 0) {
    suffix = '前'
  }
  const parts = []
  const seconds = Math.abs(dur.asSeconds())
  const values = [
    { unit: '天', value: Math.floor(seconds / 86400) },
    {
      unit: '时',
      value: Math.floor((seconds / 3600) % 24),
      vFormat: (v: number) => v.toString().padStart(2, '0'),
    },
    {
      unit: '分',
      value: Math.floor((seconds / 60) % 60),
      vFormat: (v: number) => v.toString().padStart(2, '0'),
    },
    {
      unit: '秒',
      value: Math.floor(seconds % 60),
      vFormat: (v: number) => v.toString().padStart(2, '0'),
    },
  ]
  let ignoreZero = true
  for (const { unit, value, vFormat } of values) {
    if (ignoreZero && value === 0) {
      continue
    } else {
      ignoreZero = false
    }
    let fv = value.toString()
    if (vFormat) {
      fv = vFormat(value)
    }

    parts.push(`${fv}${unit}`)
  }
  // parts.push(suffix)

  return parts.join(' ') + suffix
}
</script>
