<script setup lang="ts">
import { useOWeapons } from '@/util/localDB'
import { computed, ref } from 'vue'
const data = useOWeapons()

const items = computed(() => {
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
const productCategory = ref('Any')
</script>

<template>
  <!-- <header class="sticky-header">
    <input />
     <button> </button>
    <button @click="fetchResources">刷新</button>
  </header> -->
  <pv-tabs class="sticky top-0 z-10" v-model:value="productCategory">
    <pv-tab-list>
      <pv-tab value="Any">全部</pv-tab>
      <pv-tab value="LongGuns">主武器</pv-tab>
      <pv-tab value="Pistols">副武器</pv-tab>
      <pv-tab value="Melee">近战武器</pv-tab>
      <pv-tab value="OperatorAmps">增幅器</pv-tab>
      <pv-tab value="SpaceGuns">曲翼武器</pv-tab>
      <pv-tab value="SpaceMelee">曲翼近战武器</pv-tab>
      <pv-tab value="SentinelWeapons">守护武器</pv-tab>
      <pv-tab value="SpecialItems">特殊武器</pv-tab>
      <pv-tab value="Other">其他物品</pv-tab>
    </pv-tab-list>
  </pv-tabs>
  <div class="flex gap-2 p-2 flex-wrap">
    <pv-card class="w-72" v-for="item in items" :key="item.uniqueName">
      <template #header>
        <div class="relative">
          <img
            loading="lazy"
            class="w-72 h-72 bg-gray-300 dark:bg-transparent rounded-t-(--p-card-border-radius)"
            :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
          />
        </div>
      </template>
      <template #title>
        <string-with-icon :text="item.name"></string-with-icon>
      </template>
      <template #content>
        <p><span class="font-bold">归类: </span>{{ item.productCategory }}</p>
        <div class="max-h-24 overflow-auto">
          <string-with-icon :text="item.description"></string-with-icon>
        </div>
      </template>
    </pv-card>
  </div>
</template>

<style scoped>
.card {
  width: 500px;
  height: 200px;
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
</style>
