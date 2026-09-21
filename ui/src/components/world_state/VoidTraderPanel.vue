<script setup lang="ts">
import { computed } from 'vue'
import { useTimestamp } from '@vueuse/core'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const now = useTimestamp({ interval: 1000 })
const traders = computed(() => props.rawWorldState?.VoidTraders ?? [])

function isActive(activation?: string, expiry?: string): boolean {
  if (!activation || !expiry) return false
  return now.value >= new Date(activation).getTime() && now.value <= new Date(expiry).getTime()
}
</script>

<template>
  <WorldStatePanel :title="'虚空商人'" :icon="props.icon">
    <div v-if="!traders.length" class="ws-empty">虚空商人暂无行程</div>
    <template v-for="(v, idx) in traders" :key="idx">
      <template v-if="isActive(v.Activation, v.Expiry)">
        <div class="mb-xs">
          <span class="ws-tag on">已到来</span> <span class="fw-500">{{ v.Node || '' }}</span>
        </div>
        <div class="mb-sm"><CountDown :time="v.Expiry" @hit-zero="emit('timeUp')" />离开</div>
        <div v-if="v.Manifest?.length" class="ws-scroll">
          <table class="ds-table">
            <thead>
              <tr><th>物品</th><th class="num-col">杜卡德金币</th><th class="num-col">星币</th></tr>
            </thead>
            <tbody>
              <tr v-for="(m, i) in v.Manifest" :key="i">
                <td>{{ m.ItemName }}</td>
                <td class="num-col">{{ m.PrimePrice != null ? m.PrimePrice : '—' }}</td>
                <td class="num-col">{{ m.RegularPrice != null ? m.RegularPrice : '—' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div v-else class="ws-empty">货品清单尚未公布</div>
      </template>
      <template v-else>
        <div class="mb-xs">
          <span class="tag">未到来</span> <span class="fw-500">{{ v.Node || '' }}</span>
        </div>
        <div><CountDown :time="v.Activation" @hit-zero="emit('timeUp')" />到达</div>
      </template>
    </template>
  </WorldStatePanel>
</template>
