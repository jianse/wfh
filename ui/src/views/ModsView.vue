<script setup lang="ts">
import { useOUpgrades } from '@/util/localDB'
import { useElementSize } from '@vueuse/core'
import { computed, ref } from 'vue'
// @ts-expect-error: 忽略类型检查错误，因为 RecycleScroller 组件的类型定义是不完整的
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'

const items = useOUpgrades()
const kwFilter = ref('')
const ignoreCase = ref(true)
const cItems = computed(() => {
  return items.value.filter((item) => {
    if (ignoreCase.value) {
      const kwLower = kwFilter.value.toLowerCase()
      return (
        item.name.toLowerCase().includes(kwLower) ||
        item.compatName?.toLowerCase().includes(kwLower) ||
        (item.levelStats &&
          item.levelStats.some((level) => {
            return level.stats.some((stat) => {
              return stat.toLowerCase().includes(kwLower)
            })
          }))
      )
    } else {
      return (
        item.name.includes(kwFilter.value) ||
        item.compatName?.includes(kwFilter.value) ||
        (item.levelStats &&
          item.levelStats.some((level) => {
            return level.stats.some((stat) => {
              return stat.includes(kwFilter.value)
            })
          }))
      )
    }
  })
})

const scroller = ref()
const { width } = useElementSize(scroller)
const itemWidth = 300
const itemHeight = 514

const lineCount = computed(() => {
  return Math.max(1, Math.floor(width.value / itemWidth))
})

function onKwInput(e: Event) {
  if (e instanceof InputEvent && e.isComposing) {
    return
  }

  kwFilter.value = (e.target as HTMLInputElement).value
}
</script>

<template>
  <div class="flex flex-col gap-4 p-4">
    <!-- Toolbar Card -->
    <pv-card class="sticky top-0 z-10 shadow-md">
      <template #content>
        <div class="flex flex-row justify-between items-center">
          <div class="text-lg font-bold text-gray-800 dark:text-white">MOD</div>

          <!-- Search Controls -->
          <div class="flex items-center gap-4">
            <label class="flex items-center gap-2">
              <span class="text-sm font-medium">搜索:</span>
              <pv-input-text
                :default-value="kwFilter"
                @input="onKwInput"
                placeholder="请输入关键词"
                class="w-48"
                size="small"
              ></pv-input-text>
            </label>

            <div class="flex items-center gap-2">
              <pv-checkbox v-model="ignoreCase" inputId="ignoreCase" binary />
              <label for="ignoreCase" class="text-sm"> 忽略大小写 </label>
            </div>
          </div>
        </div>
      </template>
    </pv-card>
    <recycle-scroller
      ref="scroller"
      :items="cItems"
      :item-size="itemHeight"
      :grid-items="lineCount"
      :item-secondary-size="itemWidth"
      key-field="uniqueName"
      class="w-full"
      style="height: calc(100vh - 120px)"
      :buffer="itemHeight * 2"
    >
      <template #default="{ item }">
        <div class="w-75 h-128.5 p-2">
          <pv-card class="card">
            <template #header>
              <pos-mod :mod="item" max-rank class="mx-auto"></pos-mod>
            </template>
          </pv-card>
        </div>
      </template>
    </recycle-scroller>
  </div>
</template>

<style scoped>
.card {
  transition: all 0.3s ease;
}

.card:hover {
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
  transform: translateY(-3px);
  border-color: #cbd5e1;
}

/* Smooth scrolling */
.pv-virtual-scroller::-webkit-scrollbar {
  width: 8px;
}

.pv-virtual-scroller::-webkit-scrollbar-track {
  background: transparent;
}

.pv-virtual-scroller::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 4px;
}

.pv-virtual-scroller::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

/* Dark mode scrollbar */
.dark .pv-virtual-scroller::-webkit-scrollbar-thumb {
  background: #475569;
}

.dark .pv-virtual-scroller::-webkit-scrollbar-thumb:hover {
  background: #64748b;
}
</style>
