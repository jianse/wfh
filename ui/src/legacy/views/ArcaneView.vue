<script setup lang="ts">
import ArcaneCommon from '@/assets/arcane/ArcaneCommon.png'
import ArcaneRare from '@/assets/arcane/ArcaneRare.png'
import ArcaneUncommon from '@/assets/arcane/ArcaneUncommon.png'
import ArcaneLegendary from '@/assets/arcane/ArcaneLegendary.png'
import { useOArcane, type Arcane } from '@/util/localDB'
import { storeToRefs } from 'pinia'
import { useUserStore } from '@/stores/user'
import { computed, ref } from 'vue'
import CustomTabs from '@/components/CustomTabs.vue'

function rarityBg(rarity: string | undefined): string {
  switch (rarity) {
    case 'UNCOMMON':
      return ArcaneUncommon
    case 'RARE':
      return ArcaneRare
    case 'LEGENDARY':
      return ArcaneLegendary
    default:
      return ArcaneCommon
  }
}

const { isAdmin } = storeToRefs(useUserStore())
const editDialogVisible = ref(false)
const dialogData = ref<Arcane>({
  name: '',
  uniqueName: '',
  textureLocation: '',
  codexSecret: false,
  levelStats: [],
  inGame: false,
  slot: '',
})

function openEditDialog(arcane: Arcane) {
  dialogData.value = arcane
  editDialogVisible.value = true
}

const slotOptions = [
  'Warframe',
  'Operator',
  'Amp',
  'LongGun',
  'Pistol',
  'Melee',
  'Kitgun',
  'ZAW',
  'Unknown',
]

function saveData() {
  const blob = new Blob([JSON.stringify(items.value, ['uniqueName', 'inGame', 'slot'], 2)], {
    type: 'application/json',
  })
  const saveLink = document.createElement('a')
  saveLink.href = URL.createObjectURL(blob)
  saveLink.download = 'arcane-ext.json'
  saveLink.click()
}

const slot = ref('Any')
const data = useOArcane()

const tabOptions = [
  { label: '所有', value: 'Any' },
  { label: '战甲', value: 'Warframe' },
  { label: '指挥官', value: 'Operator' },
  { label: '增幅器', value: 'Amp' },
  { label: '主武器', value: 'LongGun' },
  { label: '副武器', value: 'Pistol' },
  { label: '近战武器', value: 'Melee' },
  { label: '组合枪', value: 'Kitgun' },
  { label: '天工造武', value: 'ZAW' },
  { label: '未知', value: 'Unknown' },
]

const items = computed(() => {
  return data.value
    .filter((item) => {
      if (slot.value === 'Any') {
        return true
      }
      return item.slot === slot.value
    })
    .filter((item) => {
      if (!kwFilter.value) {
        return true
      }
      if (ignoreCase.value) {
        const kwLower = kwFilter.value.toLowerCase()
        return (
          item.name.toLowerCase().includes(kwLower) ||
          item.levelStats[item.levelStats.length - 1].stats.some((term) => {
            return term.toLowerCase().includes(kwLower)
          })
        )
      } else {
        return (
          item.name.includes(kwFilter.value) ||
          item.levelStats[item.levelStats.length - 1].stats.some((term) => {
            return term.includes(kwFilter.value)
          })
        )
      }
    })
})
const kwFilter = ref('')
const ignoreCase = ref(true)

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
          <!-- Tabs -->
          <CustomTabs v-model="slot" :options="tabOptions" />

          <!-- Search Controls -->
          <div class="flex items-center gap-4">
            <label class="flex items-center gap-2">
              <span class="text-sm font-medium">搜索:</span>
              <pv-input-text
                placeholder="输入关键字"
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

    <!-- Arcane Cards -->
    <div class="flex gap-4 flex-wrap">
      <!-- <div class="w-full">共{{ items.length }}项 / {{ items.filter((it) => it.inGame).length }}</div> -->
      <pv-card
        v-for="item in items"
        :key="item.uniqueName"
        class="w-72 hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 border border-gray-200 dark:border-gray-700"
        :class="{
          'bg-red-600/10 border-red-300': !item.inGame,
        }"
      >
        <template #header>
          <div class="relative group h-48">
            <pv-image
              loading="lazy"
              :src="rarityBg(item.rarity)"
              class="w-72 h-72 overflow-hidden"
            />
            <img
              loading="lazy"
              :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
              style="
                position: absolute;
                left: 0;
                top: 0;
                transform: scale(0.68) translate(0, -17.5%);
              "
            />
            <div
              class="absolute inset-0 bg-linear-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"
            ></div>
            <pv-button
              v-if="isAdmin"
              class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity duration-300"
              size="small"
              rounded
              variant="filled"
              icon="pi pi-pencil"
              @click="openEditDialog(item)"
            ></pv-button>
          </div>
        </template>
        <template #title>
          <div class="text-lg font-bold text-gray-800 dark:text-white truncate">
            <StringWithIcon :text="item.name"></StringWithIcon>
          </div>
        </template>
        <template #content>
          <div class="space-y-2">
            <div class="text-sm">
              <span class="font-bold">适用：</span>{{ item.slot || '不可用' }}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400 font-mono">
              {{ item.uniqueName }}
            </div>
            <div>
              <p class="font-bold text-sm">满级效果：</p>
              <ul class="list-disc list-inside text-sm text-gray-600 dark:text-gray-300">
                <li
                  v-for="(term, index) in item.levelStats[item.levelStats.length - 1].stats"
                  :key="index"
                >
                  <StringWithIcon :text="term"></StringWithIcon>
                </li>
              </ul>
            </div>
          </div>
        </template>
      </pv-card>
      <pv-button
        v-if="isAdmin"
        icon="pi pi-download"
        class="absolute right-3 bottom-2"
        @click="saveData"
      ></pv-button>
    </div>
    <pv-dialog
      v-model:visible="editDialogVisible"
      modal
      :header="`修改[${dialogData.name}]`"
      class="min-w-lg w-lg"
      dismissableMask
    >
      <div v-show="false" class="overflow-auto h-120 bg-stone-100 p-3 mb-3">
        <code>
          <pre class="font-[comic_mono]">{{ JSON.stringify(dialogData, null, 2) }}</pre>
        </code>
      </div>

      <div class="flex items-center gap-2 mb-3">
        <pv-checkbox inputId="cheese" v-model="dialogData.inGame" binary />
        <label for="cheese"> 游戏中 </label>
      </div>

      <pv-select
        v-model="dialogData.slot"
        :options="slotOptions"
        placeholder="选择装备位"
        class="w-full md:w-56"
        size="small"
      />

      <div class="flex justify-end gap-2 mt-2">
        <pv-button type="button" label="关闭" @click="editDialogVisible = false"></pv-button>
      </div>
    </pv-dialog>
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
