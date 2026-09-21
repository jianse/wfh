<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import { rewardText } from '@/lib/ws-format'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'
import StringWithIcon from '@/components/StringWithIcon.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const alerts = computed(() => props.rawWorldState?.Alerts ?? [])
</script>

<template>
  <WorldStatePanel :title="'警报'" :icon="props.icon">
    <div v-if="!alerts.length" class="ws-empty">当前没有警报</div>
    <div v-else class="ws-cards">
      <div v-for="a in alerts" :key="a._id" class="ws-card">
        <div class="ws-card-title">{{ a.MissionInfo.location || '未知地点' }}</div>
        <div class="ws-card-sub">
          {{ a.MissionInfo.missionType }}
          <template v-if="a.MissionInfo.minEnemyLevel != null">
            · 等级 {{ a.MissionInfo.minEnemyLevel }} - {{ a.MissionInfo.maxEnemyLevel }}
          </template>
        </div>
        <div class="ws-card-desc"><StringWithIcon :text="a.MissionInfo.descText" /></div>
        <div class="ws-card-ct"><CountDown :time="a.Expiry" @hit-zero="emit('timeUp')" />结束</div>
        <div class="ws-card-reward">
          <StringWithIcon :text="rewardText(a.MissionInfo.missionReward)" />
        </div>
      </div>
    </div>
  </WorldStatePanel>
</template>
