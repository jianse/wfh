<script setup lang="ts">
import {ref} from "vue"

type ManifestItem = {
  uniqueName: string
  textureLocation: string
}

const items = ref<ManifestItem[]>()

async function fetchManifest() {
  const res = await fetch('/data/Manifest.json')
  const data = await res.json()
  items.value =  data.Manifest
}

</script>

<template>
  <header class="sticky-header">
    <input />
    <!-- <button> </button> -->
    <button @click="fetchManifest">刷新</button>
  </header>
  <div style="display: flex; flex-wrap: wrap; gap: 8px">
    <div class="card" v-for="item in items" :key="item.uniqueName">
      <h5>{{ item.uniqueName }}</h5>
      <div class="img-bg">
        <img :src="`https://content.warframe.com/PublicExport${item.textureLocation}`" />
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
