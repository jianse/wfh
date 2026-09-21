<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-store"></i>
        <span>虚空商人</span>
      </div>
    </template>
    <template #content>
      <div v-for="(voidTrader, index) in props.rawWorldState?.VoidTraders" :key="index">
        <div v-if="tsBetween(props.now, voidTrader.Activation, voidTrader.Expiry)">
          <div class="flex items-center gap-2 mb-2">
            <pv-tag value="已到来" severity="success" />
            <span class="font-medium">{{ voidTrader.Node }}</span>
          </div>
          <div class="text-sm text-muted-color mb-4">
            <CountDownTo :time="voidTrader.Expiry"></CountDownTo> 离开
          </div>
          <pv-data-table :value="voidTrader.Manifest" scrollable scroll-height="400px" striped-rows>
            <pv-column field="ItemName" header="物品" style="width: 50%"></pv-column>
            <pv-column field="PrimePrice" header="杜卡德金币" style="width: 25%"></pv-column>
            <pv-column field="RegularPrice" header="星币" style="width: 25%"></pv-column>
          </pv-data-table>
        </div>
        <div v-else>
          <div class="flex items-center gap-2 mb-2">
            <pv-tag value="未到来" severity="warn" />
            <span class="font-medium">{{ voidTrader.Node }}</span>
          </div>
          <div class="text-sm text-muted-color">
            <CountDownTo :time="voidTrader.Activation"></CountDownTo> 到达
          </div>
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import dayjs from 'dayjs'
import isBetween from 'dayjs/plugin/isBetween'
import type { RawWorldState } from '@/api/world_state'

dayjs.extend(isBetween)

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
  now: number
}>()

function tsBetween(t: number, a: string, b: string) {
  return dayjs(t).isBetween(dayjs(a), dayjs(b))
}
</script>
