<script setup lang="ts">
import { computed, ref } from 'vue'
import ArcaneCommon from '@/assets/arcane/ArcaneCommon.png'
import ArcaneRare from '@/assets/arcane/ArcaneRare.png'
import ArcaneUncommon from '@/assets/arcane/ArcaneUncommon.png'
import ArcaneLegendary from '@/assets/arcane/ArcaneLegendary.png'
import { useOArcane } from '@/util/localDB'
import { useTransStore } from '@/stores/translations'
import { useCatalogStore } from '@/stores/catalog'

const catalog = useCatalogStore()
const arcanes = useOArcane()
const { wf_t } = useTransStore()

function rarityBg(rarity?: string): string {
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

const IMG = 'https://content.warframe.com/PublicExport'
const RARITY_ZH: Record<string, string> = {
  COMMON: '普通',
  UNCOMMON: '罕见',
  RARE: '稀有',
  LEGENDARY: '传说',
}

const slot = ref('Any')
const TABS = [
  { cat: 'Any', label: '全部' },
  { cat: 'Warframe', label: '战甲' },
  { cat: 'Operator', label: '指挥官' },
  { cat: 'Amp', label: '增幅器' },
  { cat: 'LongGun', label: '主武器' },
  { cat: 'Pistol', label: '副武器' },
  { cat: 'Melee', label: '近战' },
  { cat: 'Kitgun', label: '组合枪' },
  { cat: 'ZAW', label: '天工造武' },
  { cat: 'Unknown', label: '未知' },
]

const mapped = computed(() =>
  arcanes.value.map((a) => {
    const stats = a.levelStats?.[a.levelStats.length - 1]?.stats || []
    return {
      uniqueName: a.uniqueName,
      name: wf_t(a.uniqueName, 'name') || a.name,
      img: IMG + a.textureLocation,
      tag: a.slot || '不可用',
      effect: stats.join('  '),
      rarity: (a.rarity || 'COMMON').toLowerCase(),
      rarityZh: RARITY_ZH[a.rarity ?? ''] || '普通',
      rarityBg: rarityBg(a.rarity),
      maxRank: Math.max(0, (a.levelStats?.length || 1) - 1),
      slot: a.slot,
      inGame: a.inGame,
    }
  }),
)

const list = computed(() => {
  const q = catalog.arcaneQuery.trim().toLowerCase()
  return mapped.value.filter((a) => {
    if (slot.value !== 'Any' && a.slot !== slot.value) return false
    if (q && !(a.name + a.effect + a.uniqueName).toLowerCase().includes(q)) return false
    return true
  })
})

const totalCount = computed(() => mapped.value.length)
</script>

<template>
  <section class="view active" id="view-arcane" data-od-id="arcane">
    <div class="page-head">
      <p class="eyebrow">数据库 · 赋能</p>
      <h1 class="h2">赋能</h1>
      <p class="lead">战甲与增幅器赋能，来自本地缓存数据。</p>
    </div>

    <div class="tabs" id="arcane-tabs" role="tablist" aria-label="赋能适用">
      <button
        v-for="t in TABS"
        :key="t.cat"
        class="tab"
        :class="{ active: slot === t.cat }"
        role="tab"
        :aria-selected="slot === t.cat ? 'true' : 'false'"
        @click="slot = t.cat"
      >
        {{ t.label }}
      </button>
    </div>

    <div class="page-tools">
      <input
        v-model="catalog.arcaneQuery"
        class="input"
        id="arcane-search"
        type="text"
        placeholder="搜索赋能…"
      />
      <div class="spacer"></div>
      <span class="meta" id="arcane-count">{{ list.length }} / {{ totalCount }}</span>
    </div>

    <div class="db-grid arcane" id="arcane-grid">
      <article v-for="a in list" :key="a.uniqueName" class="db-card arcane" :data-rarity="a.rarity">
        <div class="db-media icon arcane-media">
          <div class="arcane-stage">
            <img class="arcane-bg" :src="a.rarityBg" alt="" aria-hidden="true" loading="lazy" />
            <img class="arcane-fg" :src="a.img" :alt="a.name" loading="lazy" />
          </div>
        </div>
        <div class="db-body">
          <div class="db-name">
            <span class="h3"><string-with-icon :text="a.name" /></span>
            <span class="db-tag">{{ a.tag }}</span>
          </div>
          <p class="db-effect"><string-with-icon :text="a.effect" /></p>
          <div class="db-meta">
            <span class="db-rarity">{{ a.rarityZh }}</span>
            <span
              >满级 <b>R{{ a.maxRank }}</b></span
            >
          </div>
        </div>
      </article>
      <p v-if="!list.length" class="db-empty">没有匹配的赋能，或本地数据尚未同步。</p>
    </div>
  </section>
</template>

<style scoped>
/*
 * 复刻旧版 ArcaneView 的底图 / 图标关系：
 * 旧版底图（512·0.5625）显示 288px，图标（512）经 scale(0.68) translate(0,-17.5%)
 * 后落在 348.16px，左上角相对底图偏移 (81.92, 20.992)。
 * 这里固定 --arcane-size 为底图边长，其余按该比例的百分比表达，缩放时关系不变。
 */
.db-media.icon.arcane-media {
  --arcane-size: 180px;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  /* height: calc(var(--arcane-size) * 1.281778); */
  padding: 0;
}
.arcane-stage {
  position: relative;
  width: var(--arcane-size);
  height: calc(var(--arcane-size) * 0.8);
}
.arcane-stage .arcane-bg {
  /* position: absolute; */
  /* inset: 0; */
  width:var(--arcane-size);
  height: var(--arcane-size);
  /* object-fit: contain; */
  overflow: hidden;
}
.arcane-stage .arcane-fg {
  position: absolute;
  width: var(--arcane-size);
  height: var(--arcane-size);
  left: 0;
  top: 0;
  transform: scale(0.68) translate(0, -17.5%);
}
</style>
