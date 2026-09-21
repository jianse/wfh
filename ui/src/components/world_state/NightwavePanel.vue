<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'
import StringWithIcon from '@/components/StringWithIcon.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const challenges = computed(() => props.rawWorldState?.SeasonInfo?.ActiveChallenges ?? [])
</script>

<template>
  <WorldStatePanel :title="'午夜电波'" :icon="props.icon">
    <div v-if="!challenges.length" class="ws-empty">午夜电波暂无挑战</div>
    <div v-else class="ws-cards">
      <div v-for="c in challenges" :key="c._id" class="ws-card">
        <div class="row-between gap-xs">
          <span class="fw-600">{{ c.Name || '' }}</span>
          <span v-if="c.Daily" class="tag">每日</span>
        </div>
        <div class="ws-card-desc c-muted"><StringWithIcon :text="c.Desc" /></div>
        <div class="row-between mt-xs">
          <span class="ws-price">{{ c.Xp || 0 }} XP</span>
          <span><CountDown :time="c.Expiry" @hit-zero="emit('timeUp')" />结束</span>
        </div>
      </div>
    </div>
  </WorldStatePanel>
</template>
