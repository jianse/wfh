<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import { tierLabel } from '@/lib/ws-format'
import { useWorldStatePrefsStore } from '@/stores/world_state_prefs'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const store = useWorldStatePrefsStore()

const TIERS = ['VoidT1', 'VoidT2', 'VoidT3', 'VoidT4', 'VoidT5', 'VoidT6']

const isTier = (t: string) => store.fissureTiers.includes(t)

const cmp = (a: string, b: string) => (a === b ? 0 : a > b ? 1 : -1)

const normal = computed(() =>
  (props.rawWorldState?.ActiveMissions ?? [])
    .filter((f) => !f.Hard && isTier(f.Modifier))
    .sort((a, b) => cmp(a.Modifier, b.Modifier))
)
const hard = computed(() =>
  (props.rawWorldState?.ActiveMissions ?? [])
    .filter((f) => f.Hard && isTier(f.Modifier))
    .sort((a, b) => cmp(a.Modifier, b.Modifier))
)
const storms = computed(() =>
  (props.rawWorldState?.VoidStorms ?? [])
    .filter((f) => isTier(f.ActiveMissionTier))
    .sort((a, b) => cmp(a.ActiveMissionTier, b.ActiveMissionTier))
)

const meta = computed(() => normal.value.length + hard.value.length + storms.value.length + ' 条')
</script>

<template>
  <WorldStatePanel :title="'虚空裂缝'" :icon="props.icon" :meta="meta">
    <div class="ws-filters">
      <label v-for="t in TIERS" :key="t" class="ws-chk">
        <input
          type="checkbox"
          :checked="isTier(t)"
          @change="store.toggleFissureTier(t)"
        />{{ tierLabel(t) }}
      </label>
      <button class="btn btn-secondary ws-mini" @click="store.setFissureTiers([...TIERS])">全选</button>
      <button class="btn btn-secondary ws-mini" @click="store.setFissureTiers([])">取消</button>
    </div>

    <div class="ws-fis-cols">
      <div>
        <h4 class="ws-col-h"><span class="ws-dot accent"></span>普通</h4>
        <ul class="ws-ul">
          <li v-for="f in normal" :key="f._id">
            <span class="ws-tier">{{ tierLabel(f.Modifier) }}</span>{{ f.Node }} {{ f.MissionType }}
            <br /><CountDown :time="f.Expiry" @hit-zero="emit('timeUp')" />结束
          </li>
          <li v-if="!normal.length" class="ws-empty">无</li>
        </ul>
      </div>
      <div>
        <h4 class="ws-col-h"><span class="ws-dot danger"></span>钢铁之路</h4>
        <ul class="ws-ul">
          <li v-for="f in hard" :key="f._id">
            <span class="ws-tier">{{ tierLabel(f.Modifier) }}</span>{{ f.Node }} {{ f.MissionType }}
            <br /><CountDown :time="f.Expiry" @hit-zero="emit('timeUp')" />结束
          </li>
          <li v-if="!hard.length" class="ws-empty">无</li>
        </ul>
      </div>
      <div>
        <h4 class="ws-col-h"><span class="ws-dot warn"></span>虚空风暴</h4>
        <ul class="ws-ul">
          <li v-for="f in storms" :key="f._id">
            <span class="ws-tier">{{ tierLabel(f.ActiveMissionTier) }}</span>{{ f.Node }}
            <br /><CountDown :time="f.Expiry" @hit-zero="emit('timeUp')" />结束
          </li>
          <li v-if="!storms.length" class="ws-empty">无</li>
        </ul>
      </div>
    </div>
  </WorldStatePanel>
</template>
