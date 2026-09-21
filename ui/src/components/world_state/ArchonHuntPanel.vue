<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const hunts = computed(() => props.rawWorldState?.LiteSorties ?? [])
</script>

<template>
  <WorldStatePanel :title="'执刑官猎杀'" :icon="props.icon">
    <div v-if="!hunts.length" class="ws-empty">暂无执刑官猎杀</div>
    <template v-else>
      <div v-for="h in hunts" :key="h._id">
        <div class="ws-ev-title">BOSS：{{ h.BossName || '' }}</div>
        <div class="mt-xs mb-xs"><CountDown :time="h.Expiry" @hit-zero="emit('timeUp')" />轮换</div>
        <ul class="ws-ul">
          <li v-for="(m, i) in h.Missions || []" :key="i">{{ m.node }} {{ m.missionType }}</li>
        </ul>
      </div>
    </template>
  </WorldStatePanel>
</template>
