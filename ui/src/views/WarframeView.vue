<script setup lang="ts">
import { useResourcesStore } from '@/stores/resources'
import { imageUrl } from '@/util/common'
import { computed } from 'vue'

type Item = {
  uniqueName: string
  name?: string
}

const resStore = useResourcesStore()

const items = computed(() => {
  return resStore.warframes.map((it: Item) => {
    const url = imageUrl(it.uniqueName, resStore.manifest)
    return { ...it, imgUrl: url }
  })
})

</script>

<template>
  <header class="sticky-header">
    <input />
    <!-- <button> </button> -->
    <pv-button  size="small" label="刷新"></pv-button>
  </header>
  <div style="display: flex; flex-wrap: wrap; gap: 8px">
    <div class="card" v-for="item in items" :key="item.uniqueName">
      <h5>{{ item.name }}</h5>
      <p>{{ item.uniqueName }}</p>
      <div class="img-bg">
        <img :src="item.imgUrl" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.card {
  width: 500px;
  /* height: 200px; */
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
