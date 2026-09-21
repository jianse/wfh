<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-box"></i>
        <span>圣装重生</span>
      </div>
    </template>
    <template #content>
      <div v-for="pvTrader in props.rawWorldState?.PrimeVaultTraders" :key="pvTrader._id">
        <p class="text-sm text-muted-color mb-4"><CountDownTo :time="pvTrader.Expiry" /> 轮换</p>
        <pv-tabs value="rotation">
          <pv-tab-list>
            <pv-tab value="rotation">轮换</pv-tab>
            <pv-tab value="evergreen">常驻</pv-tab>
          </pv-tab-list>
          <pv-tab-panels>
            <pv-tab-panel value="rotation">
              <pv-data-table
                :value="pvTrader.Manifest"
                scrollable
                scroll-height="400px"
                striped-rows
              >
                <pv-column field="ItemName" header="物品"></pv-column>
                <pv-column field="PrimePrice" header="御品阿耶精华" style="width: 30%"></pv-column>
                <pv-column field="RegularPrice" header="阿耶精华" style="width: 25%"></pv-column>
              </pv-data-table>
            </pv-tab-panel>
            <pv-tab-panel value="evergreen">
              <pv-data-table
                :value="pvTrader.EvergreenManifest"
                scrollable
                scroll-height="400px"
                striped-rows
              >
                <pv-column field="ItemName" header="物品"></pv-column>
                <pv-column field="PrimePrice" header="御品阿耶精华" style="width: 30%"></pv-column>
                <pv-column field="RegularPrice" header="阿耶精华" style="width: 25%"></pv-column>
              </pv-data-table>
            </pv-tab-panel>
          </pv-tab-panels>
        </pv-tabs>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()
</script>
