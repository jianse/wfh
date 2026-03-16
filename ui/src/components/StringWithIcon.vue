<template>
  <parsed-text />
</template>
<script lang="ts" setup>
// import { find } from 'lodash';
import { findImageUrl } from '@/assets/icons/fonts'
import { h, watch } from 'vue'
import EnergyAniIcon from './EnergyAniIcon.vue'

const props = defineProps<{
  text: string
  imageClass?: string
}>()

// const pat = /<(.*?)>/g
const pat = /<(.*?)>|\r\n/g

let parsedText = h('span', strToVNodes(props.text))

function strToVNodes(v: string) {
  if (!v) return []
  let pos = 0
  const nodes = []
  let arr
  while ((arr = pat.exec(v)) !== null) {
    if (pos < arr.index) {
      nodes.push(v.substring(pos, arr.index))
    }
    pos = pat.lastIndex
    const name = arr[1]
    // console.log(arr)
    if (!name) {
      console.log('invalid icon name', name)
      continue
    }
    if (arr[0] === '\r\n') {
      nodes.push(h('br'))
    } else if (name === 'ENERGY') {
      nodes.push(h(EnergyAniIcon))
    } else if (name === 'LOWER_IS_BETTER') {
      // skip
    } else if (name === 'LINE_SEPARATOR') {
      nodes.push(h('hr'))
    } else {
      const image_url = findImageUrl(name)
      let clazz = props.imageClass || 'inline-block h-[20px] align-text-bottom'
      if (['PRE_ATTACK', 'ARCHWING', 'SECONDARY_FIRE'].includes(name)) {
        clazz += ' invert-100 dark:invert-0'
      }
      nodes.push(h('img', { src: image_url, alt: name, class: clazz }))
    }

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
