<script setup lang="ts">
import { computed } from 'vue'
import { useCatalogStore } from '@/stores/catalog'
import { useUiStore } from '@/stores/ui'
import { WF_CAT_LABEL, WF_CAT_TOTAL } from '@/lib/utils'
import { useOWarframes } from '@/util/localDB'
import WarframeDetailModal from '@/components/WarframeDetailModal.vue'
import { useI18n } from 'vue-i18n'

const WARFRAMES = useOWarframes()

const catalog = useCatalogStore()
const ui = useUiStore()
const { t } = useI18n()

const TABS = [
  { cat: 'all', label: '全部', count: '125' },
  { cat: 'Suits', label: '战甲', count: '117' },
  { cat: 'SpaceSuits', label: '曲翼', count: '5' },
  { cat: 'MechSuits', label: '殁世机甲', count: '2' },
]

const SORTS = [
  { value: 'default', label: '默认排序' },
  { value: 'hp', label: '生命值' },
  { value: 'shield', label: '护盾' },
  { value: 'armor', label: '护甲' },
  { value: 'power', label: '能量' },
]

const SORT_KEY: Record<string, 'health' | 'shield' | 'armor' | 'power'> = {
  hp: 'health',
  shield: 'shield',
  armor: 'armor',
  power: 'power',
}

const list = computed(() => {
  const q = catalog.wfQuery.trim().toLowerCase()
  let out = WARFRAMES.value.filter(
    (w) => catalog.wfCat === 'all' || w.productCategory === catalog.wfCat,
  )
  if (q) out = out.filter((w) => t(w.name).toLowerCase().includes(q))
  const key = SORT_KEY[catalog.wfSort]
  if (key) out = [...out].sort((a, b) => (b[key] ?? 0) - (a[key] ?? 0))
  return out
})

const totalCount = computed(() => WF_CAT_TOTAL[catalog.wfCat] ?? WARFRAMES.value.length)
</script>

<template>
  <section class="view active" id="view-warframes" data-od-id="warframes">
    <div class="page-head">
      <p class="eyebrow">数据库 · 战甲</p>
      <h1 class="h2">战甲</h1>
      <p class="lead">战甲、曲翼与殁世机甲的生命、护盾、护甲与能量四维属性，来自官方导出数据。</p>
    </div>

    <div class="tabs" id="wf-tabs" role="tablist" aria-label="战甲分类">
      <button
        v-for="t in TABS"
        :key="t.cat"
        class="tab"
        :class="{ active: catalog.wfCat === t.cat }"
        role="tab"
        :aria-selected="catalog.wfCat === t.cat ? 'true' : 'false'"
        @click="catalog.setWarframeCat(t.cat)"
      >
        {{ t.label }}<span class="tab-count">{{ t.count }}</span>
      </button>
    </div>

    <div class="page-tools">
      <input
        v-model="catalog.wfQuery"
        class="input"
        id="wf-search"
        type="text"
        placeholder="搜索战甲、曲翼或殁世机甲…"
      />
      <select v-model="catalog.wfSort" class="input" id="wf-sort">
        <option v-for="s in SORTS" :key="s.value" :value="s.value">{{ s.label }}</option>
      </select>
      <div class="spacer"></div>
      <span class="meta" id="wf-count">{{ list.length }} / {{ totalCount }}</span>
    </div>

    <div class="wf-grid" id="wf-grid">
      <article
        v-for="w in list"
        :key="w.name"
        class="wf-card"
        :data-wf="w.name"
        @click="ui.openDialog(WarframeDetailModal, { warframe: w })"
      >
        <wf-pep-img
          class="wf-portrait"
          :alt="t(w.name)"
          width="512"
          height="512"
          loading="lazy"
          :unique-name="w.icon"
        />
        <!-- <img class="wf-portrait" :src="w.img" :alt="w.name" width="512" height="512" loading="lazy" /> -->
        <div class="wf-body">
          <div class="wf-name">
            <span class="h3"><string-with-icon :text="t(w.name)" /></span>
            <span class="wf-cat">{{ WF_CAT_LABEL[w.productCategory] || '' }}</span>
          </div>
          <p class="wf-desc"><string-with-icon :text="t(w.description)" /></p>
          <div class="wf-stats">
            <div class="wstat">
              <span class="wstat-val num">{{ w.health }}</span
              ><span class="wstat-label">生命</span>
            </div>
            <div class="wstat">
              <span class="wstat-val num">{{ w.shield }}</span
              ><span class="wstat-label">护盾</span>
            </div>
            <div class="wstat">
              <span class="wstat-val num">{{ w.armor }}</span
              ><span class="wstat-label">护甲</span>
            </div>
            <div class="wstat">
              <span class="wstat-val num">{{ w.power }}</span
              ><span class="wstat-label">能量</span>
            </div>
          </div>
        </div>
      </article>
    </div>
  </section>
</template>
