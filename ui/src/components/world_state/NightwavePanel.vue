<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-radio"></i>
        <span>午夜电波</span>
      </div>
    </template>
    <template #content>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="c in props.rawWorldState?.SeasonInfo.ActiveChallenges"
          :key="c._id"
          class="border rounded-md p-3 border-gray-600"
        >
          <div class="flex justify-between items-center mb-1">
            <span class="font-bold">{{ c.Name }}</span>
            <pv-tag v-if="c.Daily" value="每日" size="small" severity="info" />
          </div>
          <div class="text-sm text-muted-color mb-1">
            <StringWithIcon :text="c.Desc"></StringWithIcon>
          </div>
          <div class="flex justify-between items-center text-sm">
            <span class="text-primary font-bold">{{ c.Xp }} XP</span>
            <span class="text-muted-color"><CountDownTo :time="c.Expiry" />结束</span>
          </div>
        </div>
      </div>
    </template>
  </pv-card>
</template>

<script setup lang="ts">
import type { RawWorldState } from '@/api/world_state'
import StringWithIcon from '@/components/StringWithIcon.vue'

const props = defineProps<{
  rawWorldState: RawWorldState | undefined
}>()
</script>
