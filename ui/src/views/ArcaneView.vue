<script setup lang="ts">
import ArcaneCommon from '@/assets/arcane/ArcaneCommon.png'
import ArcaneRare from '@/assets/arcane/ArcaneRare.png'
import ArcaneUncommon from '@/assets/arcane/ArcaneUncommon.png'
import ArcaneLegendary from '@/assets/arcane/ArcaneLegendary.png'
import { useOArcane, type Arcane } from '@/util/localDB'
import { storeToRefs } from 'pinia'
import { useUserStore } from '@/stores/user'
import { computed, ref } from 'vue'

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

const items = computed(() => {
  return data.value.filter((item) => {
    if (slot.value === 'Any') {
      return true
    }
    return item.slot === slot.value
  })
})
</script>

<template>
  <pv-tabs class="sticky top-0 z-10" v-model:value="slot">
    <pv-tab-list>
      <pv-tab value="Any">所有</pv-tab>
      <pv-tab value="Warframe">战甲</pv-tab>
      <pv-tab value="Operator">指挥官</pv-tab>
      <pv-tab value="Amp">增幅器</pv-tab>
      <pv-tab value="LongGun">主武器</pv-tab>
      <pv-tab value="Pistol">副武器</pv-tab>
      <pv-tab value="Melee">近战武器</pv-tab>
      <pv-tab value="Kitgun">组合枪</pv-tab>
      <pv-tab value="ZAW">天工造武</pv-tab>
      <pv-tab value="Unknown">未知</pv-tab>
    </pv-tab-list>
  </pv-tabs>
  <div style="display: flex; flex-wrap: wrap; gap: 8px; padding: 8px">
    <!-- <div class="w-full">共{{ items.length }}项 / {{ items.filter((it) => it.inGame).length }}</div> -->
    <pv-card
      class="w-72"
      :class="{
        'bg-red-600/10': !item.inGame,
        'border-1': !item.inGame,
        'border-red-300': !item.inGame,
      }"
      v-for="item in items"
      :key="item.uniqueName"
    >
      <template #header>
        <div class="relative w-72 h-48 overflow-hidden">
          <pv-image loading="lazy" :src="rarityBg(item.rarity)" class="w-72 h-72" />
          <img
            loading="lazy"
            :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
            style="position: absolute; left: 0; top: 0; transform: scale(0.68) translate(0, -17.5%)"
          />
          <pv-button
            v-if="isAdmin"
            class="absolute top-2 right-2"
            size="small"
            rounded
            variant="outlined"
            icon="pi pi-pencil"
            @click="openEditDialog(item)"
          ></pv-button>
        </div>
        <!-- <pv-image :src="'https://content.warframe.com/PublicExport' + item.textureLocation" /> -->
      </template>
      <template #title>{{ item.name }}</template>
      <template #content>
        <p><span class="font-bold">适用：</span>{{ item.slot || '不可用' }}</p>
        <h5 class="font-bold">唯一名称：</h5>
        <p class="wrap-anywhere">{{ item.uniqueName }}</p>
        <div>
          <p class="font-bold">满级效果：</p>
          <ul class="list-disc list-inside">
            <li
              v-for="(term, index) in item.levelStats[item.levelStats.length - 1].stats"
              :key="index"
            >
              <string-with-icon :text="term"></string-with-icon>
              <!-- <span><{{ term }}</span> -->
            </li>
          </ul>
        </div>
        <!-- <p :class="{ 'text-red-500': item.codexSecret, 'text-green-500': !item.codexSecret }">
          cc: {{ item.codexSecret }}
        </p> -->
        <!-- {{ item }} -->
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
</template>

<style scoped>
.card {
  width: 500px;
  height: 250px;
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
  height: 150px;
  /* height: fit-content; */
  /* height: min-content; */
}

.card h3 {
  font-weight: bold;
  white-space: nowrap;
  overflow: hidden;
}
</style>
