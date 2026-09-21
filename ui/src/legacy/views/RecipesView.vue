<script setup lang="ts">
import { computed, ref } from 'vue'
import { useElementSize } from '@vueuse/core'
import { chunk } from 'lodash'
import { useORecipes } from '@/util/localDB'
import { useTransStore } from '@/stores/translations'
import { useToast } from 'primevue/usetoast'

const scroller = ref()
const { width } = useElementSize(scroller)

const items = useORecipes()

const { wf_t } = useTransStore()
const toast = useToast()

const transItems = computed(() => {
  return items.value.map((item) => {
    const name_t = wf_t(item.uniqueName, 'name')
    const description_t = wf_t(item.uniqueName, 'description')
    const ingredients = item.ingredients.map((ingredient) => {
      const name_t = wf_t(ingredient.ItemType, 'name')

      return {
        ...ingredient,
        name_t,
      }
    })
    return {
      ...item,
      ingredients,
      name_t,
      description_t,
    }
  })
})

const filteredItems = computed(() => {
  return transItems.value.filter((item) => {
    if (ignoreCase.value) {
      const kwLower = kwFilter.value.toLowerCase()
      return (
        item.name?.toLowerCase().includes(kwLower) ||
        item.name_t?.toLowerCase().includes(kwLower) ||
        item.description?.toLowerCase().includes(kwLower) ||
        item.description_t?.toLowerCase().includes(kwLower) ||
        item.ingredients.some((ingredient) => {
          return (
            ingredient.name?.toLowerCase().includes(kwLower) ||
            ingredient.name_t?.toLowerCase().includes(kwLower)
          )
        })
      )
    } else {
      return (
        item.name?.includes(kwFilter.value) ||
        item.name_t?.includes(kwFilter.value) ||
        item.description?.includes(kwFilter.value) ||
        item.description_t?.includes(kwFilter.value) ||
        item.ingredients.some((ingredient) => {
          return (
            ingredient.name?.includes(kwFilter.value) || ingredient.name_t?.includes(kwFilter.value)
          )
        })
      )
    }
  })
})

const cardWidth = 500
const gap = 8 // gap-2 对应 8px
const itemWidth = cardWidth + gap

const itemsComputed = computed(() => {
  const lineCount = Math.max(1, Math.floor((width.value + gap) / itemWidth))
  const chunkedItems = chunk(filteredItems.value, lineCount)
  return chunkedItems
})
const kwFilter = ref('')
const ignoreCase = ref(true)

function onKwInput(e: Event) {
  if (e instanceof InputEvent && e.isComposing) {
    return
  }
  kwFilter.value = (e.target as HTMLInputElement).value
}

function copyUniqueName(uniqueName: string) {
  navigator.clipboard.writeText(uniqueName).then(() => {
    toast.add({
      severity: 'success',
      summary: '复制成功',
      detail: '已复制唯一名称到剪贴板',
      life: 3000,
    })
  })
}

function secondsToString(seconds: number) {
  let str = ''
  const hours = Math.floor(seconds / 3600)
  if (hours > 0) {
    str += `${hours} 小时 `
  }

  const minutes = Math.floor((seconds % 3600) / 60)
  if (minutes > 0) {
    str += `${minutes} 分钟 `
  }

  const s = Math.floor(seconds % 60)
  if (s > 0) {
    str += `${s} 秒 `
  }

  return str
}
</script>

<template>
  <div class="flex flex-col gap-4 p-4">
    <!-- Toolbar Card -->
    <pv-card class="sticky top-0 z-10 shadow-md">
      <template #content>
        <div class="flex flex-row justify-between items-center">
          <div class="text-lg font-bold text-gray-800 dark:text-white">配方</div>

          <!-- Search Controls -->
          <div class="flex items-center gap-4">
            <label class="flex items-center gap-2">
              <span class="text-sm font-medium">搜索:</span>
              <pv-input-text
                placeholder="请输入关键词"
                size="small"
                :default-value="kwFilter"
                @input="onKwInput"
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

    <!-- Recipe Cards with Virtual Scroller -->
    <pv-virtual-scroller
      ref="scroller"
      :items="itemsComputed"
      :item-size="348"
      style="width: 100%; height: calc(100vh - 120px)"
    >
      <template v-slot:item="{ item: row }">
        <div class="flex flex-row gap-2 pb-2">
          <pv-card
            v-for="item in row"
            :key="item.uniqueName"
            class="w-125 h-85 hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 border border-gray-200 dark:border-gray-700"
          >
            <template #content>
              <div class="flex flex-col gap-2 h-full">
                <!-- icon name and description -->
                <div class="flex flex-row gap-3 shrink-0 h-28">
                  <div class="rounded-sm bg-gray-500 p-1 shrink-0">
                    <pv-image
                      :alt="item.name_t || item.name"
                      :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
                      imageClass="h-26 w-26 object-contain"
                      loading="lazy"
                      preview
                    />
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="text-base font-bold text-gray-800 dark:text-white truncate">
                      {{ item.name_t || item.name }}
                    </div>
                    <div
                      class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 font-mono mt-1"
                    >
                      <span class="truncate flex-1">{{ item.uniqueName }}</span>
                      <button
                        class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
                        @click="copyUniqueName(item.uniqueName)"
                        title="复制唯一名称"
                      >
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          width="14"
                          height="14"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          stroke-width="2"
                          stroke-linecap="round"
                          stroke-linejoin="round"
                        >
                          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                        </svg>
                      </button>
                    </div>
                    <div
                      class="text-xs text-gray-600 dark:text-gray-300 max-h-16 overflow-auto leading-relaxed mt-1"
                    >
                      {{ item.description_t || item.description }}
                    </div>
                  </div>
                </div>

                <!-- Ingredients -->
                <div class="border-t border-gray-200 dark:border-gray-700 pt-3 h-[145px]">
                  <div class="text-sm font-bold text-gray-700 dark:text-gray-300 mb-2">材料:</div>
                  <div class="grid grid-cols-2 gap-2">
                    <div
                      v-for="ingredient in item.ingredients"
                      :key="ingredient.ItemType"
                      class="flex items-center gap-2 p-2 bg-gray-50 dark:bg-gray-800 rounded"
                    >
                      <img
                        loading="lazy"
                        :src="
                          'https://content.warframe.com/PublicExport' + ingredient.textureLocation
                        "
                        class="h-8 w-8 object-contain bg-gray-200 dark:bg-gray-700 rounded shrink-0"
                      />
                      <div class="flex-1 min-w-0">
                        <div class="text-xs text-gray-800 dark:text-gray-200 truncate">
                          {{ ingredient.name_t || ingredient.name }}
                        </div>
                      </div>
                      <div class="text-xs font-bold text-gray-700 dark:text-gray-300 shrink-0">
                        ×{{ ingredient.ItemCount }}
                      </div>
                    </div>
                  </div>
                </div>
                <!-- time price and so on -->
                <div class="flex gap-3">
                  <!-- 构建价格 星币 -->
                  <div class="flex items-center">
                    <img
                      src="@/assets/icons/Credits.png"
                      alt="Credits"
                      class="size-4 object-contain"
                    />
                    <div>{{ item.buildPrice }}</div>
                  </div>

                  <!-- 构建时间 -->
                  <div class="flex items-center">
                    <img
                      src="@/assets/icons/TimerStarted.png"
                      alt="time"
                      class="size-4 object-contain"
                    />
                    <div>{{ secondsToString(item.buildTime) }}</div>
                  </div>

                  <!-- 跳过构建时间价格 白金 -->
                  <div class="flex items-center" v-if="item.skipBuildTimePrice !== undefined">
                    <img
                      src="@/assets/icons/PlatinumCredits.png"
                      alt="Credits"
                      class="size-4 object-contain"
                    />
                    <div>{{ item.skipBuildTimePrice }}</div>
                  </div>

                  <!-- 出售价格 杜卡德金币 -->
                  <div class="flex items-center" v-if="item.primeSellingPrice !== undefined">
                    <img
                      src="@/assets/icons/PrimeBucks.png"
                      alt="Prime"
                      class="size-4 object-contain"
                    />
                    <div>{{ item.primeSellingPrice }}</div>
                  </div>
                  <div>
                    {{ item.resultCategory }}
                  </div>
                  <!-- <div>
                    {{ item }}
                  </div> -->
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
