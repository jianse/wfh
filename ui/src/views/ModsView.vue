<script setup lang="ts">
import { computed, ref } from 'vue'
import { useElementSize } from '@vueuse/core'
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { useOUpgrades } from '@/util/localDB'
import { useCatalogStore } from '@/stores/catalog'
import PosMod from '@/components/mods/PosMod.vue'

const catalog = useCatalogStore()
const items = useOUpgrades()

const list = computed(() => {
  const q = catalog.modQuery.trim().toLowerCase()
  if (!q) return items.value
  return items.value.filter(
    (item) =>
      (item.name || '').toLowerCase().includes(q) ||
      (item.compatName || '').toLowerCase().includes(q) ||
      (item.levelStats || []).some((lv) => (lv.stats || []).some((s) => s.toLowerCase().includes(q)))
  )
})

const scroller = ref()
const { width } = useElementSize(scroller)
const itemWidth = 300
const itemHeight = 514

const lineCount = computed(() => Math.max(1, Math.floor(width.value / itemWidth)))
</script>

<template>
  <section class="view active" id="view-mods" data-od-id="mods">
    <div class="page-head">
      <p class="eyebrow">数据库 · 振幅晶体</p>
      <h1 class="h2">振幅晶体</h1>
      <p class="lead">战甲与武器强化卡片，来自本地缓存数据。</p>
    </div>

    <div class="page-tools">
      <input v-model="catalog.modQuery" class="input" id="mods-search" type="text" placeholder="搜索振幅晶体…" />
      <div class="spacer"></div>
      <span class="meta" id="mods-count">{{ list.length }} / {{ items.length }}</span>
    </div>

    <div v-if="!list.length" class="db-empty">没有匹配的振幅晶体，或本地数据尚未同步。</div>

    <recycle-scroller
      v-else
      ref="scroller"
      class="mods-scroller"
      :items="list"
      :item-size="itemHeight"
      :grid-items="lineCount"
      :item-secondary-size="itemWidth"
      key-field="uniqueName"
      :buffer="itemHeight * 2"
    >
      <template #default="{ item }">
        <div class="mod-cell">
          <div class="mod-card bg-surface border border-border rounded-lg overflow-hidden">
            <pos-mod :mod="item" max-rank />
          </div>
        </div>
      </template>
    </recycle-scroller>
  </section>
</template>

<style scoped>
.mods-scroller {
  width: 100%;
  height: calc(100vh - 280px);
  min-height: 420px;
}
/*
 * 让 mod 卡片之间保持与战甲页一致的间距（gap-md = 20px）：
 * RecycleScroller 的单元格固定 300×514，这里用内边距把卡片从单元格里内缩，
 * 相邻卡片之间即为 10+10=20px（横向）与 20px（纵向）。
 */
.mod-cell {
  box-sizing: border-box;
  width: 300px;
  height: 514px;
  padding: 0 20px 20px 0;
}
.mod-card {
  display: flex;
  height: 100%;
  justify-content: center;
  align-items: flex-start;
}
</style>
