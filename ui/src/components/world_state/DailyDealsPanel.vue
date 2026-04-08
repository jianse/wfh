<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-shopping-cart"></i>
        <span>达尔沃特惠</span>
      </div>
    </template>
    <template #content>
      <ul class="space-y-2">
        <li v-for="deal in props.rawWorldState?.DailyDeals" :key="deal.StoreItem" class="text-sm">
          <div class="font-medium">{{ deal.StoreItemName }}</div>
          <div class="text-primary font-bold">{{ deal.SalePrice }}P</div>
          <div class="text-xs text-muted-color"><CountDownTo :time="deal.Expiry" />轮换</div>
        </li>
      </ul>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()
</script>
