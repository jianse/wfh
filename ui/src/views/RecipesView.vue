<script setup lang="ts">
import { computed } from 'vue'
import { useORecipes } from '@/util/localDB'
import { useTransStore } from '@/stores/translations'
import { useCatalogStore } from '@/stores/catalog'
import { dbDur, fmt } from '@/lib/utils'

const catalog = useCatalogStore()
const recipes = useORecipes()
const { wf_t } = useTransStore()

const IMG = 'https://content.warframe.com/PublicExport'

const mapped = computed(() =>
  recipes.value.map((r) => ({
    uniqueName: r.uniqueName,
    name: wf_t(r.uniqueName, 'name') || r.name,
    img: IMG + r.textureLocation,
    tag: (r.resultType || '').split('/').pop() || '',
    ing: (r.ingredients || []).map((i) => [
      wf_t(i.ItemType, 'name') || i.name,
      i.ItemCount
    ]),
    time: r.buildTime,
    price: r.buildPrice
  }))
)

const list = computed(() => {
  const q = catalog.recipeQuery.trim().toLowerCase()
  if (!q) return mapped.value
  return mapped.value.filter(
    (r) => (r.name + r.uniqueName + r.ing.map((i) => i[0]).join(' ')).toLowerCase().includes(q)
  )
})

const totalCount = computed(() => mapped.value.length)
</script>

<template>
  <section class="view active" id="view-recipes" data-od-id="recipes">
    <div class="page-head">
      <p class="eyebrow">数据库 · 配方</p>
      <h1 class="h2">配方</h1>
      <p class="lead">装备与消耗品蓝图，来自本地缓存数据。</p>
    </div>

    <div class="page-tools">
      <input v-model="catalog.recipeQuery" class="input" id="recipes-search" type="text" placeholder="搜索配方…" />
      <div class="spacer"></div>
      <span class="meta" id="recipes-count">{{ list.length }} / {{ totalCount }}</span>
    </div>

    <div class="db-grid recipes" id="recipes-grid">
      <article v-for="r in list" :key="r.uniqueName" class="db-card row">
        <div class="db-media"><img :src="r.img" :alt="r.name" loading="lazy" /></div>
        <div class="db-body">
          <div class="db-name">
            <span class="h3">{{ r.name }}</span>
            <span class="db-tag">{{ r.tag }}</span>
          </div>
          <div v-if="r.ing.length" class="db-chips">
            <span v-for="[n, c] in r.ing" :key="n" class="db-chip">{{ n }} <b>×{{ c }}</b></span>
          </div>
          <p v-else class="db-desc">商店购买或活动获取，无需额外材料。</p>
          <div class="db-meta">
            <span>建造 <b>{{ dbDur(r.time) }}</b></span>
            <span>花费 <b>{{ fmt(r.price) }}</b> 信用点</span>
          </div>
        </div>
      </article>
      <p v-if="!list.length" class="db-empty">没有匹配的配方，或本地数据尚未同步。</p>
    </div>
  </section>
</template>
