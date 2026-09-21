<script setup lang="ts">
import { computed } from 'vue'
import type { RawWorldState } from '@/api/world_state'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const deals = computed(() => props.rawWorldState?.DailyDeals ?? [])
</script>

<template>
  <WorldStatePanel :title="'达尔沃特惠'" :icon="props.icon">
    <div v-if="!deals.length" class="ws-empty">暂无特惠商品</div>
    <div v-else class="ws-rows">
      <div v-for="(d, i) in deals" :key="i" class="ws-row">
        <div class="ws-row-title">{{ d.StoreItemName }}</div>
        <div>
          <span class="ws-price">{{ d.SalePrice }}P</span>
          <span v-if="d.OriginalPrice" class="ws-strike">{{ d.OriginalPrice }}P</span>
          <span v-if="d.Discount" class="ws-off">-{{ d.Discount }}%</span>
        </div>
        <div><CountDown :time="d.Expiry" @hit-zero="emit('timeUp')" />轮换</div>
      </div>
    </div>
  </WorldStatePanel>
</template>
