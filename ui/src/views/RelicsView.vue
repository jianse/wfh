<script setup lang="ts">
import { computed, ref } from 'vue'
import RarityIconCommon from '@/assets/icons/RarityIconCommon.png'
import RarityIconUncommon from '@/assets/icons/RarityIconUncommon.png'
import RarityIconRare from '@/assets/icons/RarityIconRare.png'
import { type RelicReward, type Relic, useORelics } from '@/util/localDB'
import { useUserStore } from '@/stores/user'
import { chunk, cloneDeep } from 'lodash'
import { storeToRefs } from 'pinia'
import { delete_translation, save_translation } from '@/api/translations'
import { useToast } from 'primevue'
import { useTransStore } from '@/stores/translations'
import { useElementSize } from '@vueuse/core'

interface RelicRewardEx extends RelicReward {
  name_t?: string
  loading?: boolean
}

interface RelicEx extends Relic {
  image_url?: string
  relicRewards: RelicRewardEx[]
  tier: string
  vault_status: string
}
// const resStore = useResourcesStore()
const { wf_t, write_back_trans, remove_trans } = useTransStore()

const tabedItems = computed(() => {
  if (tier.value) {
    return translatedItems.value.filter((it) => it.tier === tier.value)
  } else {
    return translatedItems.value
  }
})

const items = computed(() => {
  if (kwFilter.value) {
    return tabedItems.value.filter((it) => {
      if (ignoreCase.value) {
        const kwLower = kwFilter.value.toLowerCase()
        return (
          it.name.toLowerCase().includes(kwLower) ||
          it.relicRewards.find(
            (r) =>
              r.name?.toLowerCase().includes(kwLower) || r.name_t?.toLowerCase().includes(kwLower),
          ) !== undefined
        )
      } else {
        return (
          it.name.includes(kwFilter.value) ||
          it.relicRewards.find(
            (r) => r.name?.includes(kwFilter.value) || r.name_t?.includes(kwFilter.value),
          ) !== undefined
        )
      }
    })
  } else {
    return tabedItems.value
  }
})

const relics = useORelics()

const translatedItems = computed((): RelicEx[] => {
  const res = relics.value.map((it: Relic) => {
    const relicRewards: RelicRewardEx[] = it.relicRewards.map((reward) => {
      const name_t = wf_t(reward.uniqueName, 'name') || ''
      return { ...reward, name_t }
    })

    const vault_status = 'InVault'

    return { ...it, relicRewards, vault_status }
  })
  return res
})

function suffix(name: string): string {
  if (name.endsWith('Bronze')) {
    return ''
  }
  if (name.endsWith('Silver')) {
    return '[优良]'
  }
  if (name.endsWith('Gold')) {
    return '[无暇]'
  }
  if (name.endsWith('Platinum')) {
    return '[光辉]'
  }
  return ''
}

function rarityIcon(rarity: string): string {
  switch (rarity) {
    case 'COMMON':
      return RarityIconCommon
    case 'UNCOMMON':
      return RarityIconUncommon
    case 'RARE':
      return RarityIconRare
  }
  return ''
}

const tier = ref('')
// function vaultStatus2Num(vaultStatus: string): number {
//   switch (vaultStatus) {
//     case 'InVault':
//       return 0
//     case 'Reborn':
//       return 1
//     case 'OutVault':
//       return 2
//   }
//   return 0
// }
const { isAdmin } = storeToRefs(useUserStore())

const dialogData = ref<RelicEx>({
  uniqueName: '',
  name: '',
  relicRewards: [],
  tier: '',
  vault_status: '',
  codexSecret: false,
  description: '',
  textureLocation: '',
  prefixChar: '',
  serial: 0,
})
const editDialogVisible = ref(false)

function openEditDialog(item: RelicEx) {
  const item_ = cloneDeep(item)
  item_.relicRewards = item_.relicRewards.map((it) => {
    return {
      ...it,
      loading: false,
    }
  })

  dialogData.value = item_
  editDialogVisible.value = true
}
const toast = useToast()

async function saveTrans(reward: RelicRewardEx) {
  if (!reward.name_t) {
    return
  }
  // start loading
  reward.loading = true
  try {
    const it = {
      unique_name: reward.uniqueName,
      prop: 'name',
      content: reward.name_t,
    }
    await save_translation(it)
    write_back_trans(it)
    toast.add({ severity: 'success', summary: '操作成功', detail: '添加译文成功！', life: 3000 })
  } finally {
    // stop loading
    reward.loading = false
  }
}

async function clearTrans(reward: RelicRewardEx) {
  // start loading
  reward.loading = true
  try {
    const it = {
      unique_name: reward.uniqueName,
      prop: 'name',
    }
    await delete_translation(it)
    remove_trans(it)
    toast.add({ severity: 'success', summary: '操作成功', detail: '清空译文成功！', life: 3000 })
    reward.name_t = ''
  } finally {
    // stop loading
    reward.loading = false
  }
}

const scroller = ref()
const { width } = useElementSize(scroller)
const itemWidth = 296
const itemHeight = 512

const itemsComputed = computed(() => {
  const lineCount = Math.max(1, Math.floor(width.value / itemWidth))
  const chunkedItems = chunk(items.value, lineCount)
  return chunkedItems
})

const kwFilter = ref('')
const ignoreCase = ref(true)
</script>

<template>
  <div class="h-full flex flex-col">
    <div
      class="c-toolbar w-full sticky top-0 z-10 flex flex-row place-content-between mb-2"
      style="background-color: var(--color-background)"
    >
      <pv-tabs v-model:value="tier">
        <pv-tab-list>
          <pv-tab value="">全部</pv-tab>
          <pv-tab value="T1">古纪</pv-tab>
          <pv-tab value="T2">前纪</pv-tab>
          <pv-tab value="T3">中纪</pv-tab>
          <pv-tab value="T4">后纪</pv-tab>
          <pv-tab value="T5">安魂</pv-tab>
        </pv-tab-list>
      </pv-tabs>
      <div class="flex flex-col place-content-around">
        <div class="py-1 px-2 flex flex-row gap-3 place-content-around">
          <label
            >搜索：<pv-input-text
              placeholder="输入关键字"
              size="small"
              v-model="kwFilter"
            ></pv-input-text
          ></label>

          <div class="flex items-center gap-2">
            <pv-checkbox v-model="ignoreCase" inputId="ignoreCase" binary />
            <label for="ignoreCase"> 忽略大小写 </label>
          </div>
          <label>筛选：<pv-select size="small" /></label>
        </div>
      </div>
    </div>

    <pv-virtual-scroller
      class="mx-2"
      ref="scroller"
      :items="itemsComputed"
      :item-size="itemHeight"
      auto-size
    >
      <template v-slot:item="{ item: row }">
        <div class="flex flex-row gap-2 pb-2">
          <pv-card class="w-72" v-for="item in row" :key="item.uniqueName">
            <template #header>
              <div class="relative">
                <img
                  class="w-72 h-72"
                  loading="lazy"
                  :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
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
            </template>
            <template #title> {{ item.name }} {{ suffix(item.uniqueName) }} </template>
            <template #content>
              <div class="relic-rewards">
                <div
                  v-for="reward in item.relicRewards"
                  :key="reward.rewardName"
                  class="flex flex-row place-content-between"
                >
                  <div class="overflow-hidden text-nowrap">
                    <img
                      class="inline"
                      :src="rarityIcon(reward.rarity)"
                      :alt="reward.rarity"
                      style="height: 20px; vertical-align: text-bottom"
                    /><span v-if="reward.itemCount > 1">{{ reward.itemCount }} &times; </span
                    >{{ reward.name_t || reward.name }}
                  </div>
                  <span style="font-weight: bold"> {{ reward.primeSellingPrice }}</span>
                </div>
              </div>
            </template>
          </pv-card>
        </div>
      </template>
    </pv-virtual-scroller>
    <!-- <div class="flex gap-2 overflow-y-scroll flex-wrap p-2">
      <pv-card class="w-72" v-for="item in items" :key="item.uniqueName">
        <template #header>
          <div class="relative">
            <img
              class="w-72 h-72"
              loading="lazy"
              :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
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
        </template>
        <template #title> {{ item.name }} {{ suffix(item.uniqueName) }} </template>
        <template #content>
          <div class="relic-rewards">
            <div
              v-for="reward in item.relicRewards"
              :key="reward.rewardName"
              class="flex flex-row place-content-between"
            >
              <div class="overflow-hidden text-nowrap">
                <img
                  class="inline"
                  :src="rarityIcon(reward.rarity)"
                  :alt="reward.rarity"
                  style="height: 20px; vertical-align: text-bottom"
                /><span v-if="reward.itemCount > 1">{{ reward.itemCount }} &times; </span
                >{{ reward.name_t || reward.name }}
              </div>
              <span style="font-weight: bold"> {{ reward.primeSellingPrice }}</span>
            </div>
          </div>
        </template>
      </pv-card>
    </div> -->
  </div>
  <pv-dialog
    :header="`修改[${dialogData.name}]`"
    v-model:visible="editDialogVisible"
    modal
    dismissable-mask
    class="min-w-lg w-lg"
  >
    <div v-show="false" class="overflow-auto h-120 bg-stone-100 p-3 mb-3">
      <code>
        <pre class="font-[comic_mono]">{{ JSON.stringify(dialogData, null, 2) }}</pre>
      </code>
    </div>
    <div class="mb-3" v-for="reward in dialogData.relicRewards" :key="reward.rewardName">
      <div class="mb-2">原文：{{ reward.name }}</div>
      <div class="mb-2">
        <label
          >译文：
          <pv-input-text
            label="名称"
            placeholder="请输入名称"
            size="small"
            v-model="reward.name_t"
          ></pv-input-text
        ></label>
      </div>
      <div class="flex flex-row gap-2">
        <pv-button
          label="保存"
          size="small"
          icon="pi pi-save"
          :loading="reward.loading"
          @click="saveTrans(reward)"
        ></pv-button>
        <pv-button
          label="清除"
          size="small"
          icon="pi pi-trash"
          severity="warn"
          @click="clearTrans(reward)"
        ></pv-button>
      </div>
    </div>
    <div class="flex justify-end gap-2 mt-2">
      <pv-button type="button" label="关闭" @click="editDialogVisible = false"></pv-button>
    </div>
  </pv-dialog>
</template>

<style scoped>
.vault-status {
  font-weight: bold;
  font-size: 1rem;
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
</style>
