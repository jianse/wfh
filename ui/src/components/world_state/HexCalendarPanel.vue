<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-calendar"></i>
        <span>1999 日历</span>
      </div>
    </template>
    <template #content>
      <div v-for="c in props.rawWorldState?.KnownCalendarSeasons" :key="c.Season">
        <div class="flex items-center gap-2 mb-2">
          <span class="font-bold text-lg">{{ c.Season }}</span>
          <span class="text-sm text-muted-color"> <CountDownTo :time="c.Expiry" />轮换 </span>
        </div>
        <HexCalendarDays :days="c.Days" />
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'
import HexCalendarDays from '@/components/calendar/HexCalendarDays.vue'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()
</script>
