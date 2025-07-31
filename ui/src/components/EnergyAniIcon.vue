<template>
  <div ref="icon" class="inline-block __icon" style="width: 1lh; height: 1lh" :style="style"></div>
</template>
<script lang="ts" setup>
import { useElementSize } from '@vueuse/core'
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import energyIcon from '@/assets/icons/fonts/Energy_d.png'

const icon = ref<HTMLElement | null>(null)
const eSize = useElementSize(icon)

const pIndex = ref(0)
// const iSize = 32
const [c, r, t] = [4, 4, 1]

let handle: number | undefined = undefined

const style = computed(() => {
  const x = (pIndex.value % c) * eSize.width.value + 'px'
  const y = Math.floor(pIndex.value / r) * eSize.height.value + 'px'
  // console.log(x, y)
  return {
    background: `url("${energyIcon}")`,
    backgroundPosition: `${x} ${y}`,
    backgroundSize: `${eSize.width.value * c}px ${eSize.height.value * r}px`,
  }
})
function nextFrame() {
  pIndex.value = (pIndex.value + 1) % (c * r)
}

onMounted(() => {
  if (icon.value) {
    handle = window.setInterval(nextFrame, (t * 1000) / (4 * 4))
  }
})
onBeforeUnmount(() => {
  // off
  if (handle !== undefined) {
    window.clearInterval(handle)
  }
})
// on
</script>
<style>
/* .icon {
  animation: bg-pos 1s infinite;
} */
/* @keyframes bg-pos {
  0% {
    background-position: 0 0;
  }
  100% {
    background-position: -100% 0;
  }
} */
</style>
