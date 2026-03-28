<script setup lang="ts">
import { computed, ref } from 'vue'
import { useElementSize } from '@vueuse/core'
import { chunk } from 'lodash'
import { useOResources } from '@/util/localDB'
import { useTransStore } from '@/stores/translations'
import CustomTabs from '@/components/CustomTabs.vue'

const resources = useOResources()

const { wf_t } = useTransStore()

const translatedItems = computed(() => {
  return resources.value.map((item) => {
    const name_t = wf_t(item.uniqueName, 'name') || ''
    const description_t = wf_t(item.uniqueName, 'description') || ''
    return { ...item, name_t, description_t }
  })
})

const items = computed(() => {
  return translatedItems.value.filter((it) => {
    if (ignoreCase.value) {
      const kwLower = kwFilter.value.toLowerCase()
      return (
        it.name.toLowerCase().includes(kwLower) ||
        it.name_t?.toLowerCase().includes(kwLower) ||
        it.description?.toLowerCase().includes(kwLower) ||
        it.description_t?.toLowerCase().includes(kwLower)
      )
    } else {
      return (
        it.name.includes(kwFilter.value) ||
        it.name_t?.includes(kwFilter.value) ||
        it.description?.includes(kwFilter.value) ||
        it.description_t?.includes(kwFilter.value)
      )
    }
  })
})

const kwFilter = ref('')
const ignoreCase = ref(true)

const cat = ref('Any')

const tabOptions = [
  { label: '所有', value: 'Any' },
  { label: '资源', value: 'Resources' },
  { label: '材料', value: 'Materials' },
]

function onInput(e: Event) {
  if (e instanceof InputEvent && e.isComposing) {
    return
  }
  kwFilter.value = (e.target as HTMLInputElement).value
}

// Virtual scroller setup
const scroller = ref()
const { width } = useElementSize(scroller)
const itemWidth = 500
const itemHeight = 220

const itemsComputed = computed(() => {
  const lineCount = Math.max(1, Math.floor(width.value / itemWidth))
  const chunkedItems = chunk(items.value, lineCount)
  return chunkedItems
})
</script>

<template>
  <div class="flex flex-col gap-4 p-4">
    <!-- Toolbar Card -->
    <pv-card class="sticky top-0 z-10 shadow-md">
      <template #content>
        <div class="flex flex-row justify-between items-center">
          <!-- Custom Tabs -->
          <CustomTabs v-model="cat" :options="tabOptions" />

          <!-- Search Controls -->
          <div class="flex items-center gap-4">
            <label class="flex items-center gap-2">
              <span class="text-sm font-medium">搜索:</span>
              <pv-input-text
                placeholder="输入关键字"
                size="small"
                :default-value="kwFilter"
                @input="onInput"
                class="w-48"
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

    <!-- Resource Cards with Virtual Scroller -->
    <pv-virtual-scroller
      ref="scroller"
      :items="itemsComputed"
      :item-size="itemHeight"
      style="width: 100%; height: calc(100vh - 120px)"
    >
      <template v-slot:item="{ item: row }">
        <div class="flex flex-row gap-2 pb-2">
          <pv-card
            v-for="item in row"
            :key="item.uniqueName"
            class="w-125 hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 border border-gray-200 dark:border-gray-700"
          >
            <template #content>
              <div class="flex flex-row gap-4">
                <div class="img-bg shrink-0">
                  <pv-image
                    :alt="item.name_t || item.name"
                    :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
                    imageClass="h-[150px] w-auto object-contain"
                    loading="lazy"
                  />
                </div>
                <div class="flex-1 min-w-0">
                  <div class="text-lg font-bold text-gray-800 dark:text-white truncate">
                    {{ item.name_t || item.name }}
                  </div>
                  <div class="text-xs text-gray-500 dark:text-gray-400 font-mono mt-1">
                    {{ item.uniqueName }}
                  </div>
                  <div
                    class="text-sm text-gray-600 dark:text-gray-300 max-h-24 overflow-auto leading-relaxed mt-2"
                  >
                    {{ item.description_t || item.description }}
                  </div>
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
.img-bg {
  background-color: #aaa;
  border-radius: 4px;
  padding: 4px;
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

/* Description scrollbar */
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

/* Dark mode description scrollbar */
.dark .max-h-24::-webkit-scrollbar-thumb {
  background: #475569;
}

.dark .max-h-24::-webkit-scrollbar-thumb:hover {
  background: #64748b;
}
</style>
