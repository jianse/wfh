<script setup lang="ts">
import { computed, ref } from 'vue'
import RarityIconCommon from '@/assets/icons/RarityIconCommon.png'
import RarityIconUncommon from '@/assets/icons/RarityIconUncommon.png'
import RarityIconRare from '@/assets/icons/RarityIconRare.png'
import { db, type RelicReward, type Relic } from '@/util/localDB'
import { useObservable } from '@vueuse/rxjs'
import { liveQuery } from 'dexie'
import type { Observable } from 'rxjs'

interface RelicRewardEx extends RelicReward {
  name_t?: string
}

interface RelicEx extends Relic {
  image_url?: string
  relicRewards: RelicRewardEx[]
  tier: string
  vault_status: string
}
// const resStore = useResourcesStore()

const items = computed(() => {
  if (tier.value) {
    return data.value.filter((it) => it.tier === tier.value)
  } else {
    return data.value
  }
})

const relics = useObservable(
  liveQuery(async () => await db.Relics.toArray()) as unknown as Observable<Relic[]>,
  { initialValue: [] },
)

const data = computed((): RelicEx[] => {
  const res = relics.value.map((it: Relic) => {
    const vault_status = 'InVault'

    return { ...it, vault_status }
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
</script>

<template>
  <div class="h-full flex flex-col">
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
      <div class="card" v-for="item in items" :key="item.uniqueName">
        <!-- <pre v-if="idx === 0">{{ JSON.stringify(item, undefined, 2) }}</pre> -->
        <h3>{{ item.name }} {{ suffix(item.uniqueName) }}</h3>
        <!-- <p>{{ item.uniqueName.substring(item.uniqueName.lastIndexOf('/') + 1) }}</p> -->
        <!-- <div class="img-bg"> -->
        <img
          :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
          style="background-color: #eee"
        />
        <p class="vault-status" v-if="item.vault_status == 'InVault'">
          <i class="pi pi-ban" style="color: #e31"></i>入库
        </p>
        <p class="vault-status" v-if="item.vault_status == 'OutVault'">
          <i class="pi pi-check" style="color: green"></i>出库
        </p>
        <p class="vault-status" v-if="item.vault_status == 'Reborn'">
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
            />{{ reward.name }}<span v-if="reward.itemCount > 1">*{{ reward.itemCount }}</span
            ><span style="font-weight: bold"> {{ reward.primeSellingPrice }}</span>
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
