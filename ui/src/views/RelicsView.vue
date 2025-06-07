<script setup lang="ts">
import { computed, ref } from 'vue'
import { Tabs, TabList, Tab } from 'primevue'
type Item = {
  uniqueName: string
  name: string
  textureLocation?: string
  imgUrl?: string
  relicRewards: [
    {
      rewardName: string
      rarity: string
      itemCount: number
      // rewardType: string
      // rewardValue: number
      item: {
        name: string
        primeSellingPrice?: number
      }
    },
  ]
  tier: string
  vaultStatus?: string
}

const items = ref<Item[]>([])

async function fetchResources() {
  const res = await fetch('/data/Relics-filled.json')
  const data = await res.json()
  // console.log(data.ExportRelic[0])

  items.value = data.ExportRelic.filter((it: Item) => it.uniqueName.endsWith('Bronze'))
}

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

import RarityIconCommon from '@/assets/icons/RarityIconCommon.png'
import RarityIconUncommon from '@/assets/icons/RarityIconUncommon.png'
import RarityIconRare from '@/assets/icons/RarityIconRare.png'

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
function vaultStatus2Num(vaultStatus: string): number {
  switch (vaultStatus) {
    case 'InVault':
      return 0
    case 'Reborn':
      return 1
    case 'OutVault':
      return 2
  }
  return 0
}
const relics = computed<Item[]>(() => {
  const itemsCopy = items.value.map((it) => {
    const vaultStatus = getVaultStatus(it.name)
    // console.log(vaultStatus)
    return {
      ...it,
      vaultStatus,
    }
  })

  itemsCopy.sort((a, b) => {
    const vsna = vaultStatus2Num(a.vaultStatus)
    const vsnb = vaultStatus2Num(b.vaultStatus)
    // if (vsna !== vsnb){}
    return vsnb - vsna
  })

  if (tier.value) {
    return itemsCopy.filter((relic) => relic.tier == tier.value)
  } else {
    return itemsCopy
  }
})

const relicsVaultStatus = ref(new Map())

async function fetchRelicStatus() {
  const res = await fetch('/data/Relics-status.json')
  const data = await res.json()
  // const map = new Map()
  for (const it of data.ExportRelicStatus) {
    relicsVaultStatus.value.set(it.name, it.status)
  }
  // relicsVaultStatus.value = map
  console.log('status ok')
}

fetchRelicStatus()

function getVaultStatus(name: string): string {
  return relicsVaultStatus.value.get(name)
}

fetchResources()
</script>

<template>
  <div style="padding: 0 8px; height: 100%">
    <Tabs v-model:value="tier">
      <TabList>
        <Tab value="">全部</Tab>
        <Tab value="T1">古纪</Tab>
        <Tab value="T2">前纪</Tab>
        <Tab value="T3">中纪</Tab>
        <Tab value="T4">后纪</Tab>
        <Tab value="T5">安魂</Tab>
      </TabList>
    </Tabs>
    <div
      style="
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        overflow: scroll;
        padding: 8px;
        height: calc(100% - 50px);
      "
    >
      <div class="card" v-for="item in relics" :key="item.uniqueName">
        <!-- <pre v-if="idx === 0">{{ JSON.stringify(item, undefined, 2) }}</pre> -->
        <h3>{{ item.name }} {{ suffix(item.uniqueName) }}</h3>
        <!-- <p>{{ item.uniqueName.substring(item.uniqueName.lastIndexOf('/') + 1) }}</p> -->
        <!-- <div class="img-bg"> -->
        <img :src="item.imgUrl" style="background-color: #eee" />
        <p class="vault-status" v-if="item.vaultStatus == 'InVault'">
          <i class="pi pi-ban" style="color: #e31"></i>入库
        </p>
        <p class="vault-status" v-if="item.vaultStatus == 'OutVault'">
          <i class="pi pi-check" style="color: green"></i>出库
        </p>
        <p class="vault-status" v-if="item.vaultStatus == 'Reborn'">
          <i class="pi pi-refresh" style="color: #13e"></i>重生
        </p>
        <!-- </div> -->
        <div class="relic-rewards">
          <p
            v-for="reward in item.relicRewards"
            :key="reward.rewardName"
            style="overflow: hidden; white-space: nowrap"
          >
            <img
              class="inline"
              :src="rarityIcon(reward.rarity)"
              :alt="reward.rarity"
              style="height: 20px; vertical-align: text-bottom"
            />{{ reward.item.name }}<span v-if="reward.itemCount > 1">*{{ reward.itemCount }}</span
            ><span style="font-weight: bold"> {{ reward.item.primeSellingPrice }}</span>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card {
  width: 240px;
  height: 400px;
  flex: initial;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 8px;
}

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

.card h3 {
  font-weight: bold;
}
</style>
