<script setup lang="ts">
import { computed, watch } from 'vue'
import { useTimestamp } from '@vueuse/core'

const props = defineProps<{ time: string | Date | number | undefined | null }>()
const emit = defineEmits<{ 'hit-zero': [] }>()

const now = useTimestamp({ interval: 1000 })

function fmt(diffMs: number): string {
  const neg = diffMs < 0
  const s = Math.floor(Math.abs(diffMs) / 1000)
  if (s === 0) return '即将'
  const parts: string[] = []
  const arr: [number, string, number][] = [
    [Math.floor(s / 86400), '天', 0],
    [Math.floor(s / 3600) % 24, '时', 2],
    [Math.floor(s / 60) % 60, '分', 2],
    [s % 60, '秒', 2]
  ]
  let started = false
  for (const [v, u, p] of arr) {
    if (!started && v === 0) continue
    started = true
    parts.push((p ? String(v).padStart(p, '0') : String(v)) + u)
  }
  return parts.join(' ') + (neg ? '前' : '后')
}

const text = computed(() => {
  if (!props.time) return '-'
  const t = new Date(props.time).getTime()
  if (Number.isNaN(t)) return '-'
  return fmt(t - now.value)
})

watch(
  () => text.value,
  (v, old) => {
    if (old !== '即将' && v === '即将') emit('hit-zero')
  }
)

// 供面板内嵌展示（含“结束/轮换”后缀由调用方决定）
defineExpose({ text })
</script>

<template>
  <span class="ct">{{ text }}</span>
</template>
