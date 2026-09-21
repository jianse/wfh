<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'
import StringWithIcon from '@/components/StringWithIcon.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const sorties = computed(() => props.rawWorldState?.Sorties ?? [])
</script>

<template>
  <WorldStatePanel :title="'突击任务'" :icon="props.icon">
    <div v-if="!sorties.length" class="ws-empty">暂无突击任务</div>
    <template v-else>
      <div v-for="s in sorties" :key="s._id">
        <div class="mb-sm"><CountDown :time="s.Expiry" @hit-zero="emit('timeUp')" />轮换</div>
        <div class="ws-missions">
          <div v-for="(v, i) in s.Variants || []" :key="i" class="ws-mission">
            <div class="ws-mission-type">{{ v.missionType }}</div>
            <div class="ws-mission-node">{{ v.node }}</div>
            <div class="ws-mod">{{ v.modifierName }}</div>
            <div class="ws-mission-desc"><StringWithIcon :text="v.modifierDescription" /></div>
          </div>
        </div>
      </div>
    </template>
  </WorldStatePanel>
</template>
