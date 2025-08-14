<script setup lang="ts">
import { computed, ref } from 'vue'
import { useElementSize } from '@vueuse/core'
import { chunk } from 'lodash'
import { useORecipes } from '@/util/localDB'
import { useTransStore } from '@/stores/translations'

const scroller = ref()
const { width } = useElementSize(scroller)

const items = useORecipes()

const { wf_t } = useTransStore()

const transItems = computed(() => {
  return items.value.map((item) => {
    const name_t = wf_t(item.uniqueName, 'name')
    const description_t = wf_t(item.uniqueName, 'description')
    const ingredients = item.ingredients.map((ingredient) => {
      const name_t = wf_t(ingredient.ItemType, 'name')

      return {
        ...ingredient,
        name_t,
      }
    })
    return {
      ...item,
      ingredients,
      name_t,
      description_t,
    }
  })
})

const filteredItems = computed(() => {
  return transItems.value.filter((item) => {
    return (
      item.name?.includes(kwFilter.value) ||
      item.name_t?.includes(kwFilter.value) ||
      item.description?.includes(kwFilter.value) ||
      item.description_t?.includes(kwFilter.value) ||
      item.ingredients.some((ingredient) => {
        return (
          ingredient.name?.includes(kwFilter.value) || ingredient.name_t?.includes(kwFilter.value)
        )
      })
    )
  })
})

const itemWidth = 500

const itemsComputed = computed(() => {
  const lineCount = Math.max(1, Math.floor(width.value / itemWidth))
  const chunkedItems = chunk(filteredItems.value, lineCount)
  return chunkedItems
})
const kwFilter = ref('')

function onKwInput(e: Event) {
  if (e instanceof InputEvent && e.isComposing) {
    return
  }
  kwFilter.value = (e.target as HTMLInputElement).value
}
</script>

<template>
  <div class="flex sticky top-0 w-full p-2" style="background-color: var(--color-background)">
    <pv-input-text
      placeholder="请输入关键词"
      :default-value="kwFilter"
      @input="onKwInput"
    ></pv-input-text>
  </div>
  <pv-virtual-scroller
    ref="scroller"
    :items="itemsComputed"
    :item-size="300"
    style="width: 100%; height: calc(100vh - 58px)"
  >
    <template v-slot:item="{ item: row }">
      <!-- <div>{{ line.length }}</div> -->
      <div style="display: flex; flex-direction: row">
        <template v-for="item in row" :key="item.uniqueName">
          <div class="card-cnt">
            <div class="card">
              <h3>{{ item.name_t || item.name }}</h3>
              <div class="description">
                <p>{{ item.description_t || item.description }}</p>
              </div>
              <div class="card-content">
                <div>
                  <pv-image
                    loading="lazy"
                    :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
                    preview
                    style="background-color: #eee; height: 195px; width: 195px"
                  />
                </div>
                <div class="recipe-ingredients">
                  <div
                    v-for="ingredient in item.ingredients"
                    :key="ingredient.ItemType"
                    style="display: flex; gap: 4px; height: 46px"
                  >
                    <img
                      loading="lazy"
                      :src="
                        'https://content.warframe.com/PublicExport' + ingredient.textureLocation
                      "
                      style="height: 45px; width: 45px; background-color: #eee"
                    />
                    <div>
                      <span>{{ ingredient.name_t || ingredient.name }}</span>
                      <!-- <span>{{ ingredient.ItemType }}</span> -->
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
  </pv-virtual-scroller>
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
