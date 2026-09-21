<script setup lang="ts">
import { computed } from 'vue'
import { useOWeapons } from '@/util/localDB'
import { useTransStore } from '@/stores/translations'
import { useCatalogStore } from '@/stores/catalog'
import { fmt } from '@/lib/utils'

const catalog = useCatalogStore()
const weapons = useOWeapons()
const { wf_t } = useTransStore()

const IMG = 'https://content.warframe.com/PublicExport'

const CAT_LABELS: Record<string, string> = {
  LongGuns: '主武器',
  Pistols: '副武器',
  Melee: '近战',
  OperatorAmps: '增幅器',
  SpaceGuns: '曲翼武器',
  SpaceMelee: '曲翼近战',
  SentinelWeapons: '守护武器',
  SpecialItems: '特殊武器'
}

const CATS = [
  { cat: 'all', label: '全部' },
  { cat: 'LongGuns', label: '主武器' },
  { cat: 'Pistols', label: '副武器' },
  { cat: 'Melee', label: '近战' },
  { cat: 'OperatorAmps', label: '增幅器' },
  { cat: 'SpaceGuns', label: '曲翼武器' },
  { cat: 'SpaceMelee', label: '曲翼近战' },
  { cat: 'SentinelWeapons', label: '守护武器' },
  { cat: 'SpecialItems', label: '特殊武器' }
]

const SORTS = [
  { value: 'default', label: '默认排序' },
  { value: 'dmg', label: '总伤害' },
  { value: 'mr', label: '段位需求' },
  { value: 'crit', label: '暴击几率' }
]
const SORT_KEY: Record<string, 'dmg' | 'mr' | 'crit'> = {
  dmg: 'dmg',
  mr: 'mr',
  crit: 'crit'
}

const mapped = computed(() =>
  weapons.value.map((w) => ({
    uniqueName: w.uniqueName,
    name: wf_t(w.uniqueName, 'name') || w.name,
    img: IMG + w.textureLocation,
    cat: CAT_LABELS[w.productCategory] || w.productCategory,
    category: w.productCategory,
    desc: wf_t(w.uniqueName, 'description') || w.description,
    mr: w.masteryReq,
    dmg: w.totalDamage,
    crit: w.criticalChance,
    status: w.procChance
  }))
)

const filtered = computed(() => {
  const q = catalog.wpQuery.trim().toLowerCase()
  let out = mapped.value.filter((w) => catalog.wpCat === 'all' || w.category === catalog.wpCat)
  if (q) out = out.filter((w) => (w.name + w.desc + w.uniqueName).toLowerCase().includes(q))
  const key = SORT_KEY[catalog.wpSort]
  if (key) out = [...out].sort((a, b) => (b[key] || 0) - (a[key] || 0))

  return out
})

const list = filtered

const TABS = computed(() =>
  CATS.map((c) => ({
    ...c,
    count: c.cat === 'all' ? mapped.value.length : mapped.value.filter((w) => w.category === c.cat).length
  }))
)

const totalCount = computed(() => mapped.value.length)
</script>

<template>
  <section class="view active" id="view-weapons" data-od-id="weapons">
    <div class="page-head">
      <p class="eyebrow">数据库 · 武器</p>
      <h1 class="h2">武器</h1>
      <p class="lead">主武器、副武器、近战、增幅器、曲翼与守护武器，来自本地缓存数据。</p>
    </div>

    <div class="tabs" id="wp-tabs" role="tablist" aria-label="武器分类">
      <button
        v-for="t in TABS"
        :key="t.cat"
        class="tab"
        :class="{ active: catalog.wpCat === t.cat }"
        role="tab"
        :aria-selected="catalog.wpCat === t.cat ? 'true' : 'false'"
        @click="catalog.wpCat = t.cat"
      >
        {{ t.label }}<span class="tab-count">{{ t.count }}</span>
      </button>
    </div>

    <div class="page-tools">
      <input v-model="catalog.wpQuery" class="input" id="weapons-search" type="text" placeholder="搜索武器…" />
      <select v-model="catalog.wpSort" class="input" id="weapons-sort">
        <option v-for="s in SORTS" :key="s.value" :value="s.value">{{ s.label }}</option>
      </select>
      <div class="spacer"></div>
      <span class="meta" id="weapons-count">{{ list.length }} / {{ totalCount }}</span>
    </div>

    <div class="wp-grid" id="weapons-grid">
      <article v-for="(w, i) in list" :key="w.uniqueName" class="wp-card" :data-od-id="`weapon-card-${i + 1}`">
        <img class="wp-portrait" :src="w.img" :alt="w.name" width="512" height="512" loading="lazy" />
        <div class="wp-body">
          <div class="wp-name">
            <span class="h3"><string-with-icon :text="w.name" /></span>
            <span class="wp-tag">{{ w.cat }}</span>
          </div>
          <p class="wp-desc"><string-with-icon :text="w.desc" /></p>
          <div class="wp-mastery">段位需求 <b>{{ w.mr }}</b></div>
          <div class="wp-stats">
            <div class="wp-stat"><span class="wp-val num">{{ fmt(w.dmg) }}</span><span class="wp-label">总伤害</span></div>
            <div class="wp-stat"><span class="wp-val num">{{ fmt(w.crit) }}%</span><span class="wp-label">暴击</span></div>
            <div class="wp-stat"><span class="wp-val num">{{ fmt(w.status) }}%</span><span class="wp-label">触发</span></div>
          </div>
        </div>
      </article>
      <p v-if="!list.length" class="db-empty">没有匹配的武器，或本地数据尚未同步。</p>
    </div>
  </section>
</template>
