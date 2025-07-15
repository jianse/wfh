<template>
  <parsed-text />
</template>
<script lang="ts" setup>
// import { find } from 'lodash';
import { findImageUrl } from '@/assets/icons/fonts'
import { h, watch } from 'vue'

const props = defineProps<{
  text: string
  imageClass?: string
}>()

const pat = /<(.*?)>/g

let parsedText = h('span', strToVNodes(props.text))

function strToVNodes(v: string) {
  let pos = 0
  const nodes = []
  let arr
  while ((arr = pat.exec(v)) !== null) {
    if (pos < arr.index) {
      nodes.push(v.substring(pos, arr.index))
    }
    pos = pat.lastIndex
    const image_url = findImageUrl(arr[1])
    let clazz = props.imageClass || 'inline-block h-lh'
    if ( ['PRE_ATTACK','ARCHWING'].includes(arr[1])) {
      clazz += ' invert-100 dark:invert-0'
    }
    nodes.push(h('img', { src: image_url, alt: arr[1], class: clazz }))
    // console.log(`Found ${arr[0]}. Next starts at ${pat.lastIndex}.`)
  }
  if (pos < v.length) {
    nodes.push(v.substring(pos))
  }
  return nodes
}

watch(
  () => props.text,
  (v) => {
    const nodes = strToVNodes(v)
    parsedText = h('span', nodes)
  },
)
</script>
