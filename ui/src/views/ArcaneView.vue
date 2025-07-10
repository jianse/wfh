<script setup lang="ts">
const items = useOArcane()

import ArcaneCommon from '@/assets/arcane/ArcaneCommon.png'
import ArcaneRare from '@/assets/arcane/ArcaneRare.png'
import ArcaneUncommon from '@/assets/arcane/ArcaneUncommon.png'
import ArcaneLegendary from '@/assets/arcane/ArcaneLegendary.png'
import { useOArcane } from '@/util/localDB'

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
</script>

<template>
  <!-- <Tabs value="0">
    <TabList>
      <Tab value="0">Header I</Tab>
      <Tab value="1">Header II</Tab>
        <Tab value="2">Header III</Tab>
    </TabList>
  </Tabs> -->
  <div style="display: flex; flex-wrap: wrap; gap: 8px; padding: 8px; overflow: scroll">
    <pv-card class="w-72" v-for="item in items" :key="item.uniqueName">
      <template #header>
        <div class="relative w-72">
          <pv-image loading="lazy" :src="rarityBg(item.rarity)" />
          <img
            loading="lazy"
            :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
            style="position: absolute; left: 0; top: 0; transform: scale(0.68) translate(0, -17.5%)"
          />
        </div>
        <!-- <pv-image :src="'https://content.warframe.com/PublicExport' + item.textureLocation" /> -->
      </template>
      <template #title>{{ item.name }}</template>
      <template #content>
        <h5>唯一名称：{{ item.uniqueName }}</h5>
      </template>
    </pv-card>
  </div>
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
