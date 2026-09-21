<template>
  <img :src="src"/>
</template>
<script lang="ts" setup>
import { findImage } from '@/util/localDB'
import { useAsyncState } from '@vueuse/core'

const props = defineProps<{
  uniqueName: string
  // resolve: Function;
  default?: string
}>()

async function resolve() {
  const i = await findImage(props.uniqueName)
  if (i) {
    return 'https://content.warframe.com/PublicExport/' + i.uniqueName + '!' + i.contentHash
  }
  return ''
}

const { state: src } = useAsyncState(resolve, props.default, { immediate: true })

// await resolve()
</script>
