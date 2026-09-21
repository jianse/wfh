<script setup lang="ts">
import { useOWeapons } from '@/util/localDB'
import { computed, ref } from 'vue'
import { chunk } from 'lodash'
import { useElementSize } from '@vueuse/core'

const data = useOWeapons()

const itemsFiltered = computed(() => {
  if (productCategory.value === 'Any') {
    return data.value
  }
  if (productCategory.value === 'Other') {
    return data.value.filter((item) => {
      return item.slot === undefined
    })
  }
  return data.value.filter(
    (item) => item.productCategory === productCategory.value && item.slot !== undefined,
  )
})

const items = computed(() => {
  return itemsFiltered.value.filter((item) => {
    if (ignoreCase.value) {
      const kw = kwFilter.value.toLowerCase()
      return (
        item.name.toLowerCase().includes(kw) ||
        item.description.toLowerCase().includes(kw) ||
        item.uniqueName.toLowerCase().includes(kw)
      )
    } else {
      return (
        item.name.includes(kwFilter.value) ||
        item.description.includes(kwFilter.value) ||
        item.uniqueName.includes(kwFilter.value)
      )
    }
  })
})

const productCategory = ref('Any')
const kwFilter = ref('')
const ignoreCase = ref(true)

const tabOptions = [
  { label: '全部', value: 'Any' },
  { label: '主武器', value: 'LongGuns' },
  { label: '副武器', value: 'Pistols' },
  { label: '近战武器', value: 'Melee' },
  { label: '增幅器', value: 'OperatorAmps' },
  { label: '曲翼武器', value: 'SpaceGuns' },
  { label: '曲翼近战武器', value: 'SpaceMelee' },
  { label: '守护武器', value: 'SentinelWeapons' },
  { label: '特殊武器', value: 'SpecialItems' },
  { label: '其他物品', value: 'Other' },
]

// Virtual scroller setup
const scroller = ref()
const { width } = useElementSize(scroller)
const cardWidth = 288 // w-72 = 288px
const gap = 16 // gap-4 = 16px
const itemWidth = cardWidth + gap
const itemHeight = 520 // 调整为合适的行高，考虑卡片高度和padding
const itemsComputed = computed(() => {
  const lineCount = Math.max(1, Math.floor((width.value + gap) / itemWidth))
  const chunkedItems = chunk(items.value, lineCount)
  return chunkedItems
})

function onInput(e: Event) {
  if (e instanceof InputEvent && e.isComposing) {
    return
  }
  kwFilter.value = (e.target as HTMLInputElement).value
}
</script>

<template>
  <div class="h-full flex flex-col p-4">
    <!-- Toolbar Card -->
    <pv-card class="sticky top-0 z-10 shadow-md mb-4">
      <template #content>
        <div class="flex flex-row justify-between items-center gap-4 min-w-0">
          <!-- Custom Tabs -->
          <CustomTabs v-model="productCategory" :options="tabOptions" />

          <!-- Search Controls -->
          <div class="flex items-center gap-4 shrink-0">
            <label class="flex items-center gap-2"
              ><span class="text-sm font-medium">搜索:</span>
              <pv-input-text
                placeholder="输入关键字"
                size="small"
                :default-value="kwFilter"
                @input="onInput"
                class="w-48"
              ></pv-input-text
            ></label>

            <div class="flex items-center gap-2">
              <pv-checkbox v-model="ignoreCase" inputId="ignoreCase" binary />
              <label for="ignoreCase" class="text-sm"> 忽略大小写 </label>
            </div>
          </div>
        </div>
      </template>
    </pv-card>

    <!-- Virtual Scroller with Weapon Cards -->
    <pv-virtual-scroller
      ref="scroller"
      :items="itemsComputed"
      :item-size="itemHeight"
      style="width: 100%; height: calc(100vh - 120px)"
    >
      <template v-slot:item="{ item: row }">
        <div class="flex flex-row gap-4 pb-2">
          <pv-card
            v-for="item in row"
            :key="item.uniqueName"
            class="w-72 hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 border border-gray-200 dark:border-gray-700 h-130"
          >
            <template #header>
              <div class="relative group">
                <img
                  loading="lazy"
                  class="w-72 h-72 bg-gray-300 dark:bg-transparent rounded-t-(--p-card-border-radius) object-cover"
                  :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
                  :alt="item.name"
                />
              </div>
            </template>
            <template #title>
              <div class="text-lg font-bold text-gray-800 dark:text-white truncate">
                <string-with-icon :text="item.name"></string-with-icon>
              </div>
            </template>
            <template #content>
              <div class="space-y-2">
                <div class="text-xs text-gray-500 dark:text-gray-400 font-mono">
                  {{ item.uniqueName }}
                </div>
                <p class="text-sm text-gray-600 dark:text-gray-300">
                  <span class="font-bold">归类： </span>{{ item.productCategory }}
                </p>
                <div
                  class="text-sm text-gray-600 dark:text-gray-300 max-h-24 overflow-auto leading-relaxed"
                >
                  <string-with-icon :text="item.description"></string-with-icon>
                </div>
              </div>
            </template>
          </pv-card>
        </div>
      </template>
    </pv-virtual-scroller>
  </div>
</template>

<style scoped>
/* Smooth scrolling for description text */
.max-h-24::-webkit-scrollbar {
  width: 4px;
}

.max-h-24::-webkit-scrollbar-track {
  background: transparent;
}

.max-h-24::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 2px;
}

.max-h-24::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

/* Dark mode scrollbar */
.dark .max-h-24::-webkit-scrollbar-thumb {
  background: #475569;
}

.dark .max-h-24::-webkit-scrollbar-thumb:hover {
  background: #64748b;
}
</style>
