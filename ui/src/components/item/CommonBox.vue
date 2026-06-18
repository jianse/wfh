<template>
  <img :src="'https://content.warframe.com/PublicExport/' + item.textureLocation" />
  <span>{{ item.name }}</span
  ><span v-if="count"> X {{ count }}</span>
</template>

<script lang="ts" setup>
defineProps<{
  item: Object
  count?: number
}>()
</script>
