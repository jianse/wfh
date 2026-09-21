<script setup lang="ts">
import { computed } from 'vue'
import type { Conquest, ConquestMission, RawWorldState } from '@/api/world_state'
import { useWorldStatePrefsStore } from '@/stores/world_state_prefs'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'
import StringWithIcon from '@/components/StringWithIcon.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const store = useWorldStatePrefsStore()
const conquests = computed(() => props.rawWorldState?.Conquests ?? [])

const name = (t: string) => (t === 'CT_LAB' ? '深层科研' : t === 'CT_HEX' ? '时光科研' : t)

function difficulty(m: ConquestMission, hard: boolean) {
  const type = hard ? 'CD_HARD' : 'CD_NORMAL'
  return m.difficulties?.find((d) => d.type === type)
}

function toggleHard(c: Conquest, checked: boolean) {
  store.setConquestHard(c.Type, checked)
}
</script>

<template>
  <WorldStatePanel :title="'每周科研'" :icon="props.icon">
    <div v-if="!conquests.length" class="ws-empty">暂无每周科研</div>
    <div v-else class="ws-cq-grid">
      <div v-for="c in conquests" :key="c.Type" class="ws-cq">
        <div class="ws-cq-head">
          <span class="ws-cq-name">{{ name(c.Type) }}</span>
          <span class="ct"><CountDown :time="c.Expiry" @hit-zero="emit('timeUp')" />轮换</span>
        </div>
        <label class="ws-cq-hard">
          <input
            type="checkbox"
            :checked="store.getConquestHard(c.Type)"
            @change="toggleHard(c, ($event.target as HTMLInputElement).checked)"
          />精英科研
        </label>

        <div class="ws-cq-h">任务</div>
        <div class="ws-cq-missions">
          <div v-for="(m, i) in c.Missions || []" :key="i" class="ws-cq-mission">
            <div class="ws-cq-mtype">{{ m.missionType }}</div>
            <div class="ws-cq-line">偏差：{{ difficulty(m, store.getConquestHard(c.Type))?.deviation || '—' }}</div>
            <div
              v-for="(r, ri) in difficulty(m, store.getConquestHard(c.Type))?.tRisks || []"
              :key="ri"
              class="ws-cq-line"
            >
              风险变量：{{ r.name }}
            </div>
            <div class="ws-cq-tip">
              <div class="t-h">{{ difficulty(m, store.getConquestHard(c.Type))?.deviation || '' }}</div>
              <div class="t-b">
                <StringWithIcon :text="difficulty(m, store.getConquestHard(c.Type))?.deviationDesc || ''" />
              </div>
              <div
                v-for="(r, ri) in difficulty(m, store.getConquestHard(c.Type))?.tRisks || []"
                :key="ri"
                class="t-b"
              >
                <span class="t-h">{{ r.name }}</span><br /><StringWithIcon :text="r.desc || ''" />
              </div>
            </div>
          </div>
        </div>

        <div class="ws-cq-h">可选改造</div>
        <ul class="ws-ul">
          <li v-for="(v, i) in c.TVariables || []" :key="i">
            <div class="fw-500">{{ v.name }}</div>
            <div class="ws-cq-line"><StringWithIcon :text="v.desc || ''" /></div>
          </li>
          <li v-if="!(c.TVariables || []).length" class="ws-empty">无</li>
        </ul>
      </div>
    </div>
  </WorldStatePanel>
</template>
