<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useCatalogStore } from '@/stores/catalog'
import { wfhName } from '@/router/wfh'
import { WF_CAT_LABEL } from '@/lib/utils'
import { useTransStore } from '@/stores/translations'
import {
  useOWarframes,
  useOWeapons,
  useOUpgrades,
  useOArcane,
  useORelics,
  useOResources,
  useORecipes
} from '@/util/localDB'

const emit = defineEmits<{ close: [] }>()

const catalog = useCatalogStore()
const router = useRouter()
const { wf_t } = useTransStore()

const warframes = useOWarframes()
const weapons = useOWeapons()
const upgrades = useOUpgrades()
const arcanes = useOArcane()
const relics = useORelics()
const resources = useOResources()
const recipes = useORecipes()

interface SearchRow {
  cat: string
  name: string
  view: string
  wfCat?: string
}

const INDEX = computed<SearchRow[]>(() => {
  const rows: SearchRow[] = []
  for (const w of warframes.value)
    rows.push({
      cat: WF_CAT_LABEL[w.productCategory] || '战甲',
      name: wf_t(w.uniqueName, 'name') || w.name,
      view: 'warframes',
      wfCat: w.productCategory
    })
  for (const w of weapons.value)
    rows.push({ cat: '武器', name: wf_t(w.uniqueName, 'name') || w.name, view: 'weapons' })
  for (const m of upgrades.value)
    rows.push({ cat: '振幅晶体', name: wf_t(m.uniqueName, 'name') || m.name, view: 'mods' })
  for (const a of arcanes.value)
    rows.push({ cat: '赋能', name: wf_t(a.uniqueName, 'name') || a.name, view: 'arcane' })
  for (const r of relics.value)
    rows.push({ cat: '遗物', name: wf_t(r.uniqueName, 'name') || r.name, view: 'relics' })
  for (const r of resources.value)
    rows.push({ cat: '资源', name: wf_t(r.uniqueName, 'name') || r.name, view: 'resources' })
  for (const r of recipes.value)
    rows.push({ cat: '配方', name: wf_t(r.uniqueName, 'name') || r.name, view: 'recipes' })
  return rows
})

const inputEl = ref<HTMLInputElement | null>(null)
const query = ref('')

const groups = computed(() => {
  const q = query.value.trim().toLowerCase()
  if (!q) return [] as { cat: string; items: SearchRow[] }[]
  const matched = INDEX.value.filter((r) => r.name.toLowerCase().includes(q)).slice(0, 200)
  const map = new Map<string, SearchRow[]>()
  for (const r of matched) {
    if (!map.has(r.cat)) map.set(r.cat, [])
    map.get(r.cat)!.push(r)
  }
  return [...map.entries()].map(([cat, items]) => ({ cat, items }))
})

onMounted(async () => {
  await nextTick()
  inputEl.value?.focus()
})

function pick(item: SearchRow) {
  if (item.view === 'warframes' && item.wfCat) {
    catalog.setWarframeCat(item.wfCat)
  }
  emit('close')
  router.push({ name: wfhName(item.view) })
}
</script>

<template>
  <div class="modal-panel">
    <div class="modal-search">
      <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="11" cy="11" r="7" /><path d="M20 20l-3.5-3.5" /></svg>
      <input ref="inputEl" id="search-input" v-model="query" type="text" placeholder="搜索…" autocomplete="off" />
      <span class="kbd">Esc</span>
    </div>
    <div class="modal-results" id="search-results">
      <div v-if="!groups.length" class="result-empty">没有匹配结果</div>
      <template v-for="g in groups" :key="g.cat">
        <div class="grp-label">{{ g.cat }}</div>
        <div
          v-for="item in g.items"
          :key="item.cat + item.name + item.view"
          class="result-item"
          @click="pick(item)"
        >
          <span class="ri-tag">{{ item.cat }}</span>
          <span class="ri-name">{{ item.name }}</span>
        </div>
      </template>
    </div>
  </div>
</template>
