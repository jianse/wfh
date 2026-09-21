<script setup lang="ts">
import { computed } from 'vue'
import type { Invasion, RawWorldState, Reward } from '@/api/world_state'
import { rewardText } from '@/lib/ws-format'
import WorldStatePanel from './WorldStatePanel.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
defineEmits<{ timeUp: [] }>()

const invasions = computed(
  () => props.rawWorldState?.Invasions.filter((i) => !(Math.abs(i.Count) >= i.Goal)) ?? []
)
const meta = computed(() => invasions.value.length + ' 场')

function progress(i: Invasion): string {
  const vsInfestation = i.AttackerMissionInfo?.faction === '感染者'
  const pct = (1 + i.Count / i.Goal) * (vsInfestation ? 100 : 50)
  return `${Math.min(100, Math.max(0, Math.round(pct)))}%`
}

function rewardStr(r: Reward): string {
  return rewardText(r)
}
</script>

<template>
  <WorldStatePanel :title="'入侵任务'" :icon="props.icon" :meta="meta">
    <div v-if="!invasions.length" class="ws-empty">当前没有进行中的入侵</div>
    <div v-else class="ws-inv-grid">
      <div v-for="i in invasions" :key="i._id" class="ws-card">
        <div class="ws-card-title">{{ i.Node || '' }}</div>
        <div class="ws-inv-top">
          <div class="ws-inv-side">
            <div class="ws-inv-fac">{{ i.DefenderMissionInfo?.faction }}</div>
            <div class="ws-inv-rew">{{ rewardStr(i.AttackerReward) || '-' }}</div>
          </div>
          <div class="ws-inv-count">
            {{ Math.abs(i.Count).toLocaleString('zh-CN') }} / {{ i.Goal.toLocaleString('zh-CN') }}
          </div>
          <div class="ws-inv-side r">
            <div class="ws-inv-fac">{{ i.AttackerMissionInfo?.faction }}</div>
            <div class="ws-inv-rew">{{ rewardStr(i.DefenderReward) }}</div>
          </div>
        </div>
        <div class="ws-bar"><i :style="{ width: progress(i) }"></i></div>
      </div>
    </div>
  </WorldStatePanel>
</template>
