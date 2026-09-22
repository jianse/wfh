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
  if (props.uniqueName.startsWith('http')){
    return props.uniqueName
  }
  // console.log('resolve', props.uniqueName)
  const i = await findImage(props.uniqueName)
  // console.log(i)
  if (i) {
    return 'https://content.warframe.com/PublicExport/' + i.uniqueName + '!' + i.contentHash
  }
  return ''
}

const { state: src } = useAsyncState(resolve, props.default, { immediate: true })

// await resolve()
</script>
