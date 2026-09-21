<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState, SyndicateMission } from '@/api/world_state'
import { bountyName } from '@/lib/ws-format'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const GROUPS: [string, string, string][] = [
  ['CetusSyndicate', '希图斯', '夜灵平野'],
  ['SolarisSyndicate', '索拉里斯', '奥布山谷'],
  ['EntratiSyndicate', '英择谛', '魔胎之境']
]

const all = computed(() => props.rawWorldState?.SyndicateMissions ?? [])

function group(tag: string): SyndicateMission | undefined {
  return all.value.find((m) => m.Tag === tag)
}
function rep(job: { xpAmounts?: number[] }): number {
  return (job.xpAmounts ?? []).reduce((a, b) => a + b, 0)
}
function jobs(m: SyndicateMission) {
  return 'Jobs' in m ? (m.Jobs ?? []) : []
}
</script>

<template>
  <WorldStatePanel :title="'集团任务'" :icon="props.icon">
    <div v-if="!all.length" class="ws-empty">暂无集团任务</div>
    <template v-for="[tag, label, place] in GROUPS" :key="tag">
      <div v-if="group(tag)" class="ws-syn">
        <div class="ws-syn-head">
          <span class="ws-dot accent"></span>{{ label
          }}<span class="fw-400 c-muted fs-13">· {{ place }}</span
          ><CountDown :time="group(tag)?.Expiry" @hit-zero="emit('timeUp')" />
        </div>
        <div class="ws-syn-cards">
          <div v-for="(j, i) in jobs(group(tag)!)" :key="i" class="ws-syn-card">
            <div class="ws-syn-top">
              <span class="ws-syn-rep">★ {{ rep(j).toLocaleString('zh-CN') }}</span>
              <span>等级 {{ j.minEnemyLevel }} - {{ j.maxEnemyLevel }}</span>
            </div>
            <div class="ws-syn-jt fw-500 c-fg">{{ bountyName(j.jobType) }}</div>
            <div v-if="j.masteryReq" class="ws-syn-jt">精通段位要求：{{ j.masteryReq }}</div>
          </div>
        </div>
      </div>
    </template>
  </WorldStatePanel>
</template>
