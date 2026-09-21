<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const sched = computed(() => props.rawWorldState?.EndlessXpSchedule?.[0])
const cats = computed(() => sched.value?.CategoryChoices ?? [])
</script>

<template>
  <WorldStatePanel :title="'无尽回廊'" :icon="props.icon">
    <div v-if="!cats.length" class="ws-empty">暂无无尽回廊轮换</div>
    <template v-else>
      <div class="mb-sm"><CountDown :time="sched?.Expiry" @hit-zero="emit('timeUp')" />轮换</div>
      <div class="ws-endless">
        <div v-for="c in cats" :key="c.Category">
          <div class="ws-endless-cat">{{ c.Category === 'EXC_NORMAL' ? '普通' : '困难' }}</div>
          <div class="ws-endless-grid">
            <span v-for="n in c.Choices || []" :key="n" class="ws-chip">{{ n }}</span>
          </div>
        </div>
      </div>
    </template>
  </WorldStatePanel>
</template>
