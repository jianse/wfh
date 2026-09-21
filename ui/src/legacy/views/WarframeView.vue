<script setup lang="ts">
import { computed, ref } from 'vue'
import { useToast } from 'primevue/usetoast'
import CustomTabs from '@/components/CustomTabs.vue'

import { useI18n } from 'vue-i18n'
const { t } = useI18n()

import { useOWarframes, type Warframe } from '@/util/localDB'

const warframes = useOWarframes()

const items = computed<Warframe[]>(() => {
  return warframes.value
    .filter((it) => {
      if (ignoreCase.value) {
        const kwLower = kwFilter.value.toLowerCase()
        return (
          t(it.name).toLowerCase().includes(kwLower) ||
          t(it.description).toLowerCase().includes(kwLower)
          // it.name_t?.toLowerCase().includes(kwLower) ||
          // it.description_t?.toLowerCase().includes(kwLower)
        )
      } else {
        return (
          t(it.name).includes(kwFilter.value) || t(it.description).includes(kwFilter.value)
          // it.name_t?.includes(kwFilter.value) ||
          // it.description_t?.includes(kwFilter.value)
        )
      }
    })
    .filter((it) => {
      if (cat.value === 'Any') {
        return true
      } else {
        return it.productCategory === cat.value
      }
    })
})

const toast = useToast()

const actions = ref([
  {
    label: 'Add',
    icon: 'pi pi-pencil',
    command: () => {
      toast.add({ severity: 'info', summary: 'Add', detail: 'Data Added', life: 3000 })
    },
  },
  {
    label: 'Update',
    icon: 'pi pi-refresh',
    command: () => {
      toast.add({ severity: 'success', summary: 'Update', detail: 'Data Updated', life: 3000 })
    },
  },
  {
    label: 'Delete',
    icon: 'pi pi-trash',
    command: () => {
      toast.add({ severity: 'error', summary: 'Delete', detail: 'Data Deleted', life: 3000 })
    },
  },
  {
    label: 'Upload',
    icon: 'pi pi-upload',
  },
  {
    label: 'Vue Website',
    icon: 'pi pi-external-link',
    command: () => {
      window.location.href = 'https://vuejs.org/'
    },
  },
])

const kwFilter = ref('')
const ignoreCase = ref(true)

const cat = ref('Any')

const tabOptions = [
  { label: '所有', value: 'Any' },
  { label: '战甲', value: 'Suits' },
  { label: '曲翼', value: 'SpaceSuits' },
  { label: '殁世机甲', value: 'MechSuits' },
]

function onInput(e: Event) {
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
          <!-- Custom Tabs -->
          <CustomTabs v-model="cat" :options="tabOptions" />

          <!-- Search Controls -->
          <div class="flex items-center gap-4">
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

    <!-- Warframe Cards Grid -->
    <div class="flex gap-4 flex-wrap">
      <pv-card
        v-for="item in items"
        :key="item.uniqueName"
        class="w-72 hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 border border-gray-200 dark:border-gray-700"
      >
        <template #header>
          <div class="relative group">
            <pv-image
              :alt="t(item.name) || item.name"
              :src="'https://browse.wf' + item.icon"
              imageClass="w-72 h-72 object-cover"
              loading="lazy"
            />
            <div
              class="absolute inset-0 bg-linear-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"
            ></div>
          </div>
        </template>
        <template #title>
          <div class="text-lg font-bold text-gray-800 dark:text-white truncate">
            <StringWithIcon :text="t(item.name) || item.name"></StringWithIcon>
          </div>
        </template>
        <template #content>
          <div class="space-y-2">
            <div class="text-xs text-gray-500 dark:text-gray-400 font-mono wrap-break-word">
              {{ item.uniqueName }}
            </div>
            <div
              class="text-sm text-gray-600 dark:text-gray-300 max-h-24 overflow-auto leading-relaxed"
            >
              {{ t(item.description) || item.description }}
            </div>
          </div>
        </template>
      </pv-card>

      <PvSpeedDial
        pt:pcButton:root:class="size-15"
        :model="actions"
        :radius="120"
        :actionButtonProps="{
          rounded: true,
        }"
        type="quarter-circle"
        direction="up-left"
        :style="{ position: 'absolute', right: '30px', bottom: '30px' }"
      />
    </div>
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
