<script setup lang="ts">
import { ref } from 'vue'

type Item = {
  uniqueName: string
  name: string
  textureLocation?: string
  imgUrl?: string
}

type ManifestItem = {
  uniqueName: string
  textureLocation: string
}

async function fetchManifest(): Promise<ManifestItem[]> {
  const res = await fetch('/data/Manifest.json')
  const data = await res.json()
  return data.Manifest
}

const mfItems = await fetchManifest()

const items = ref<Item[]>([])

async function fetchResources() {
  const res = await fetch('/data/Upgrades.json')
  const data = await res.json()
  console.log(data)

  items.value = data.ExportUpgrades.filter((it:Item)=>it.name.includes("执刑官")).map((it: Item) => {
    const url = imageUrl(it.uniqueName)
    return { ...it, imgUrl: url }
  })
  console.log(items.value)

  // items.value =
}

function imageUrl(uniqueName: string): string {
  // console.log(uniqueName);

  const textureLocation = mfItems.find((it) => it.uniqueName === uniqueName)?.textureLocation

  return textureLocation ? `https://content.warframe.com/PublicExport${textureLocation}` : ''
}

fetchResources()
</script>

<template>
  <header class="sticky-header">
    <input />
    <!-- <button> </button> -->
    <button @click="fetchResources">刷新</button>
  </header>
  <div style="display: flex; flex-wrap: wrap; gap: 8px">
    <div class="card" v-for="item in items" :key="item.uniqueName">
      <h5>{{ item.name }}</h5>
      <div class="img-bg">
        <img :src="item.imgUrl" />
      </div>
    </div>
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
