<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import { rewardText } from '@/lib/ws-format'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'
import StringWithIcon from '@/components/StringWithIcon.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const goals = computed(() => props.rawWorldState?.Goals ?? [])

function barWidth(value: number, max: number): string {
  const p = max ? (value / max) * 100 : value
  return `${Math.max(0, Math.min(100, Math.round(p || 0)))}%`
}
</script>

<template>
  <WorldStatePanel :title="'活动'" :icon="props.icon">
    <div v-if="!goals.length" class="ws-empty">当前没有进行中的活动</div>
    <div v-for="e in goals" :key="e._id" class="ws-ev">
      <div class="ws-ev-title">{{ e.Desc || '活动' }}</div>
      <div v-if="e.Node || e.VictimNode" class="ws-ev-node meta">
        {{ e.Node || e.VictimNode }}
      </div>
      <div class="mt-xs"><CountDown :time="e.Expiry" @hit-zero="emit('timeUp')" />结束</div>
      <div class="ws-ev-tip"><StringWithIcon :text="e.ToolTip" /></div>

      <template v-if="e.Goal">
        <div class="ws-bar accent"><i :style="{ width: barWidth(e.Count || 0, e.Goal) }"></i></div>
        <div class="ws-bar-label">
          {{ (e.Count || 0).toLocaleString('zh-CN') }} / {{ e.Goal.toLocaleString('zh-CN') }}
        </div>
      </template>
      <template v-if="e.Tag === 'GhoulEmergence' && e.HealthPct != null">
        <div class="ws-bar accent"><i :style="{ width: barWidth(e.HealthPct * 100, 100) }"></i></div>
        <div class="ws-bar-label">{{ Math.round(e.HealthPct * 100) }}%</div>
      </template>

      <div v-for="(r, i) in e.InterimRewards || []" :key="i" class="ws-ev-reward">
        <StringWithIcon :text="rewardText(r)" />
      </div>
      <div v-if="e.Reward" class="ws-ev-reward strong">
        奖励：<StringWithIcon :text="rewardText(e.Reward)" />
      </div>
    </div>
  </WorldStatePanel>
</template>
