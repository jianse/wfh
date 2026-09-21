<template>
  <pv-card class="col-span-full">
    <template #title>
      <div class="flex items-center gap-2">
        <i class="pi pi-flash"></i>
        <span>突击任务</span>
      </div>
    </template>
    <template #content>
      <div v-for="sortie in props.rawWorldState?.Sorties" :key="sortie._id">
        <p class="text-sm text-muted-color mb-4"><CountDownTo :time="sortie.Expiry" />轮换</p>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <pv-card
            class="border border-gray-600"
            v-for="(mission, index) in sortie.Variants"
            :key="index"
          >
            <template #title>{{ mission.missionType }}</template>
            <template #content>
              <div class="flex flex-col gap-2 text-sm">
                <div class="font-medium">{{ mission.node }}</div>
                <div class="font-bold text-primary">{{ mission.modifierName }}</div>
                <div class="text-muted-color">
                  <StringWithIcon :text="mission.modifierDescription || ''" />
                </div>
              </div>
            </template>
          </pv-card>
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
