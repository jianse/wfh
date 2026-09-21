<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const sp = computed(() => props.rawWorldState?.SteelPath)
</script>

<template>
  <WorldStatePanel :title="'钢铁之路每周'" :icon="props.icon">
    <div v-if="!sp" class="ws-empty">暂无钢铁之路轮换</div>
    <div v-else class="ws-row">
      <div class="ws-row-title fs-16 fw-600">{{ sp.current_reward.name }}</div>
      <div class="meta">{{ sp.current_reward.cost }} 钢铁精华</div>
      <div class="mt-xs"><CountDown :time="sp.Expiry" @hit-zero="emit('timeUp')" />轮换</div>
    </div>
  </WorldStatePanel>
</template>
