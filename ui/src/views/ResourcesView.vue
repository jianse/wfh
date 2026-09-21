<script setup lang="ts">
import { computed } from 'vue'
import { useOResources } from '@/util/localDB'
import { useTransStore } from '@/stores/translations'
import { useCatalogStore } from '@/stores/catalog'

const catalog = useCatalogStore()
const resources = useOResources()
const { wf_t } = useTransStore()

const IMG = 'https://content.warframe.com/PublicExport'

const mapped = computed(() =>
  resources.value.map((r) => ({
    uniqueName: r.uniqueName,
    name: wf_t(r.uniqueName, 'name') || r.name,
    img: IMG + r.textureLocation,
    desc: wf_t(r.uniqueName, 'description') || r.description
  }))
)

const list = computed(() => {
  const q = catalog.resourceQuery.trim().toLowerCase()
  if (!q) return mapped.value
  return mapped.value.filter((r) => (r.name + r.desc + r.uniqueName).toLowerCase().includes(q))
})

const totalCount = computed(() => mapped.value.length)
</script>

<template>
  <section class="view active" id="view-resources" data-od-id="resources">
    <div class="page-head">
      <p class="eyebrow">数据库 · 资源</p>
      <h1 class="h2">资源</h1>
      <p class="lead">锻造所需材料与获取信息，来自本地缓存数据。</p>
    </div>

    <div class="page-tools">
      <input v-model="catalog.resourceQuery" class="input" id="resources-search" type="text" placeholder="搜索资源…" />
      <div class="spacer"></div>
      <span class="meta" id="resources-count">{{ list.length }} / {{ totalCount }}</span>
    </div>

    <div class="db-grid resources" id="resources-grid">
      <article v-for="r in list" :key="r.uniqueName" class="db-card row">
        <div class="db-media"><img :src="r.img" :alt="r.name" loading="lazy" /></div>
        <div class="db-body">
          <div class="db-name"><span class="h3"><string-with-icon :text="r.name" /></span></div>
          <p class="db-desc"><string-with-icon :text="r.desc" /></p>
        </div>
      </article>
      <p v-if="!list.length" class="db-empty">没有匹配的资源，或本地数据尚未同步。</p>
    </div>
  </section>
</template>
