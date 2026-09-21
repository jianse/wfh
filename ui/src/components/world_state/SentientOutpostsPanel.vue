<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const outpost = computed(() => props.rawWorldState?.SentientOutpost)
</script>

<template>
  <WorldStatePanel :title="'灵煞异常'" :icon="props.icon">
    <div v-if="!outpost" class="ws-empty">当前没有灵煞异常</div>
    <template v-else>
      <div class="mb-xs"><CountDown :time="outpost.expiry" @hit-zero="emit('timeUp')" />轮换</div>
      <div class="fw-500">{{ outpost.node_name || outpost.node }}</div>
      <div class="meta">{{ outpost.mission_type || '' }}</div>
    </template>
  </WorldStatePanel>
</template>
