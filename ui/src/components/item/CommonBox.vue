<template>
  <img :src="'https://content.warframe.com/PublicExport/' + computedItem.textureLocation" />
  <span >{{ computedItem.name }}</span
  ><span v-if="count"> X {{ count }}</span>
</template>

<script lang="ts" setup>
const props = defineProps<{
  item: object
  count?: number
}>()

import { computed } from 'vue'

const computedItem = computed(() => {
  return props.item as {name?: string; textureLocation?: string}
})

</script>
