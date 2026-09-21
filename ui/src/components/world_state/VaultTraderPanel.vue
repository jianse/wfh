<script setup lang="ts">
import { computed, ref } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const tab = ref<'rotation' | 'evergreen'>('rotation')
const traders = computed(() => props.rawWorldState?.PrimeVaultTraders ?? [])
</script>

<template>
  <WorldStatePanel :title="'圣装重生'" :icon="props.icon">
    <div v-if="!traders.length" class="ws-empty">圣装重生当前未开启</div>
    <template v-for="(v, idx) in traders" :key="idx">
      <div class="mb-xs"><CountDown :time="v.Expiry" @hit-zero="emit('timeUp')" />轮换</div>
      <div class="ws-tabs">
        <button :class="{ on: tab === 'rotation' }" @click="tab = 'rotation'">轮换</button>
        <button :class="{ on: tab === 'evergreen' }" @click="tab = 'evergreen'">常驻</button>
      </div>
      <div class="ws-scroll">
        <table class="ds-table">
          <thead>
            <tr>
              <th>物品</th>
              <th class="num-col">御品阿耶精华</th>
              <th class="num-col">阿耶精华</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="(m, i) in (tab === 'rotation' ? v.Manifest : v.EvergreenManifest) || []"
              :key="i"
            >
              <td>{{ m.ItemName }}</td>
              <td class="num-col">{{ m.PrimePrice != null ? m.PrimePrice : '—' }}</td>
              <td class="num-col">{{ m.RegularPrice != null ? m.RegularPrice : '—' }}</td>
            </tr>
            <tr v-if="!((tab === 'rotation' ? v.Manifest : v.EvergreenManifest) || []).length">
              <td colspan="3" class="ws-empty">暂无</td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </WorldStatePanel>
</template>
