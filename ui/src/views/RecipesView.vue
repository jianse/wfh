<script setup lang="ts">
import { computed, ref } from 'vue'
import { VirtualScroller } from 'primevue'
import Image from 'primevue/image'

import { useElementSize } from '@vueuse/core'
// import { useTemplateRef } from 'vue'
import { chunk } from 'lodash'

const scroller = ref()
const { width } = useElementSize(scroller)

type Ingredient = {
  ItemType: string
  ItemCount: number
  item: CommonItem
}

type CommonItem = {
  name: string
  uniqueName: string
  description: string
  imgUrl?: string
}

type RecipeItem = {
  uniqueName: string
  name: string
  textureLocation?: string
  resultType: string
  resultItem: CommonItem
  ingredients: Ingredient[]
}

const items = ref<RecipeItem[]>([])

async function fetchResources() {
  const res = await fetch('/data/Recipes-filled.json')
  const data = await res.json()
  console.log(data)
  items.value = data.ExportRecipes
}
fetchResources()
const itemWidth = 500

const itemsComputed = computed(() => {
  const lineCount = Math.max(1, Math.floor(width.value / itemWidth))
  console.log('lineCount', lineCount)
  const chunkedItems = chunk(items.value, lineCount)
  console.log('chunkedItems', chunkedItems)
  return chunkedItems
})
</script>

<template>
  <VirtualScroller
    ref="scroller"
    :items="itemsComputed"
    :item-size="300"
    style="width: 100%; height: calc(100vh - 58px)"
  >
    <template v-slot:item="{ item }">
      <!-- <div>{{ line.length }}</div> -->
      <div style="display: flex; flex-direction: row">
        <template v-for="innerItem in item" :key="innerItem.uniqueName">
          <div class="card-cnt">
            <div class="card">
              <h3>{{ innerItem.resultItem.name }}</h3>
              <div class="description">
                <p>{{ innerItem.resultItem.description }}</p>
              </div>
              <div class="card-content">
                <div>
                  <Image
                    :src="innerItem.resultItem.imgUrl"
                    preview
                    style="background-color: #eee; height: 195px"
                  />
                </div>
                <div class="recipe-ingredients">
                  <div
                    v-for="ingredient in innerItem.ingredients"
                    :key="ingredient.ItemType"
                    style="display: flex; gap: 4px; height: 46px"
                  >
                    <img
                      :src="ingredient.item.imgUrl"
                      style="height: 45px; background-color: #eee"
                    />
                    <div>
                      <span>{{ ingredient.item.name }}</span>
                      <p style="font-weight: bold">{{ ingredient.ItemCount }}</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </div>
    </template>
  </VirtualScroller>
</template>

<style scoped>
.card-cnt {
  width: 500px;
  height: 312px;
  padding: 8px;
}

.card {
  /* width: 500px; */
  /* height: 250px; */
  flex: initial;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 8px;
  /* margin: 8px; */
}

.card h3 {
  font-weight: bold;
  white-space: nowrap;
  overflow: hidden;
}

.card .description {
  height: 3rem;
  overflow: scroll;
}

.card-content {
  display: flex;
  flex-direction: row;
  gap: 8px;
}

.img-bg {
  background-color: #ddd;
  line-height: 0;
}

.sticky-header {
  position: sticky;
  top: 0;
  height: 5rem;
  background-color: #fff;
  border-bottom: 1px solid #333;
  /* margin-bottom: 10px; */
}

.card img {
  object-fit: contain;
  height: 150px;
  /* height: fit-content; */
  /* height: min-content; */
}

pre {
  font-family: 'ubuntu mono', monospace;
}

.recipe-ingredients {
  display: flex;
  flex-direction: column;
  gap: 4px;
  /* padding-top: 10px;
  padding-bottom: 10px; */
}
</style>
