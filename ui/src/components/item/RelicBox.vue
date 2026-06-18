<template>
  <div>
    <img :src="'https://content.warframe.com/PublicExport/' + item.textureLocation" />
    <span>{{ item.name }}</span> <span v-if="count">X {{ count }}</span>
  </div>
</template>
<script setup lang="ts">
import type { Relic } from '@/util/localDB'

defineProps<{
  item: Relic
  count?: number
}>()
</script>
