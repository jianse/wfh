<script setup lang="ts">
import { useOUpgrades } from '@/util/localDB'
import { useElementSize } from '@vueuse/core'
import { chunk } from 'lodash'
import { computed, ref } from 'vue'

const items = useOUpgrades()
const kwFilter = ref('')
const cItems = computed(() => {
  const r = items.value.filter(
    (item) => item.name.includes(kwFilter.value) || item.compatName?.includes(kwFilter.value),
  )
  // console.log(r)
  return r
})

const scroller = ref()
const { width } = useElementSize(scroller)
const itemWidth = 310
const itemHeight = 514
const itemsComputed = computed(() => {
  const lineCount = Math.max(1, Math.floor(width.value / itemWidth))
  const chunkedItems = chunk(cItems.value, lineCount)
  return chunkedItems
})

function onKwInput(e: Event) {
  if (e instanceof InputEvent && e.isComposing) {
    return
  }

  kwFilter.value = (e.target as HTMLInputElement).value
}
</script>

<template>
  <div class="sticky top-0">
    <pv-input-text
      :default-value="kwFilter"
      @input="onKwInput"
      placeholder="请输入关键词"
    ></pv-input-text>
  </div>
  <pv-virtual-scroller
    class="mx-2"
    ref="scroller"
    :items="itemsComputed"
    :item-size="itemHeight"
    style="width: calc(100% - 16px); height: calc(100% - 58px)"
  >
    <template v-slot:item="{ item: row }">
      <div class="flex flex-row gap-2 pb-2">
        <div class="card" v-for="item in row" :key="item.uniqueName">
          <h5>{{ item.name }}</h5>
          <pos-mod :mod="item" max-rank></pos-mod>
        </div>
      </div>
    </template>
  </pv-virtual-scroller>
  <!-- <div style="display: flex; flex-wrap: wrap; gap: 8px">
    <div class="card" v-for="item in cItems" :key="item.uniqueName">
      <h5>{{ item.name }}</h5>
      <common-mod :mod="item" max-rank></common-mod>
    </div>
  </div> -->
</template>

<style scoped>
.card {
  flex: initial;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 8px;
}

.img-bg {
  background-color: #aaa;
}

.sticky-header {
  position: sticky;
  /* top: 0; */
  height: 5rem;
  background-color: #fff;
  border-bottom: 1px solid #333;
}

.card img {
  object-fit: contain;
  /* height: 150px; */
  /* height: fit-content; */
  /* height: min-content; */
}
</style>
