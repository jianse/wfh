<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, type Component } from 'vue'
import { useTimestamp } from '@vueuse/core'
import { get_raw_world_state, force_reload, type RawWorldState } from '@/api/world_state'
import { useUiStore } from '@/stores/ui'
import { useAuthStore } from '@/stores/auth'
import { useWorldStatePrefsStore, type WorldStateSection } from '@/stores/world_state_prefs'
import {
  CyclesPanel,
  EventsPanel,
  AlertsPanel,
  FissuresPanel,
  SteelPathPanel,
  DailyDealsPanel,
  ArchonHuntPanel,
  SentientOutpostsPanel,
  SortiePanel,
  VoidTraderPanel,
  VaultTraderPanel,
  NightwavePanel,
  InvasionsPanel,
  ConquestsPanel,
  HexCalendarPanel,
  EndlessXpPanel,
  SyndicateMissionsPanel
} from '@/components/world_state'

const ui = useUiStore()
const auth = useAuthStore()
const prefs = useWorldStatePrefsStore()

const ICONS: Record<string, string> = {
  cycles: '<circle cx="12" cy="12" r="8"/><path d="M12 8v4l3 2"/>',
  events: '<path d="M6 21V4M6 4h11l-2 4 2 4H6"/>',
  alerts: '<path d="M6 16V11a6 6 0 0 1 12 0v5l1.5 2h-15z"/><path d="M10 21a2 2 0 0 0 4 0"/>',
  fissures: '<path d="M12 3l7 3v6c0 4.5-3 7.5-7 9-4-1.5-7-4.5-7-9V6z"/>',
  steelPath: '<path d="M12 4l2.4 5 5.6.6-4.2 3.8 1.2 5.6L12 16.8 6.9 19l1.2-5.6L4 9.6l5.6-.6z"/>',
  dailyDeals: '<circle cx="9" cy="20" r="1.4"/><circle cx="18" cy="20" r="1.4"/><path d="M3 4h2l2.4 11h11L21 7H6"/>',
  archonHunt: '<path d="M13 3 5 13h6l-1 8 8-10h-6z"/>',
  sentientOutposts: '<path d="M12 4l9 16H3z"/><path d="M12 10v4M12 17h.01"/>',
  sortie: '<path d="M4 12h7l-1 8 9-12h-7l1-8z"/>',
  voidTrader: '<path d="M4 9h16l-1 11H5z"/><path d="M4 9l1.5-4h13L20 9"/>',
  vaultTrader: '<path d="M12 3l8 4v10l-8 4-8-4V7z"/><path d="M4 7l8 4 8-4M12 11v10"/>',
  nightwave: '<circle cx="12" cy="12" r="2"/><path d="M7.5 7.5a6 6 0 0 0 0 9M16.5 7.5a6 6 0 0 1 0 9M4.5 4.5a10 10 0 0 0 0 15M19.5 4.5a10 10 0 0 1 0 15"/>',
  invasions: '<path d="M14 4l6 6-8 8H8v-4z"/><path d="M4 20l3-3"/>',
  conquests: '<path d="M9 3h6M10 3v6l-5 9a2 2 0 0 0 1.8 3h10.4A2 2 0 0 0 19 18l-5-9V3"/>',
  hexCalendar: '<rect x="4" y="5" width="16" height="16" rx="2"/><path d="M4 9h16M8 3v4M16 3v4"/>',
  endlessXp: '<path d="M7 12c0-2 1.2-3.2 2.6-3.2 1.7 0 2.6 1.6 4.8 3.2 2.2 1.6 3.1 3.2 4.8 3.2 1.4 0 2.6-1.2 2.6-3.2s-1.2-3.2-2.6-3.2c-1.7 0-2.6 1.6-4.8 3.2-2.2 1.6-3.1 3.2-4.8 3.2C8.2 15.2 7 14 7 12z"/>',
  syndicateMissions: '<circle cx="9" cy="8" r="3"/><path d="M3 20a6 6 0 0 1 12 0"/><path d="M16 6a3 3 0 0 1 0 6M17 20a6 6 0 0 0-3-5.2"/>'
}

interface Section {
  key: WorldStateSection
  label: string
  title?: string
  span?: number
  component: Component
}

const SECTIONS: Section[] = [
  { key: 'cycles', label: '周期循环', component: CyclesPanel },
  { key: 'events', label: '活动', component: EventsPanel },
  { key: 'alerts', label: '警报', component: AlertsPanel },
  { key: 'fissures', label: '虚空裂缝', component: FissuresPanel },
  { key: 'steelPath', label: '钢铁之路', title: '钢铁之路每周', span: 1, component: SteelPathPanel },
  { key: 'dailyDeals', label: '特惠商品', title: '达尔沃特惠', span: 1, component: DailyDealsPanel },
  { key: 'archonHunt', label: '执刑官猎杀', component: ArchonHuntPanel },
  { key: 'sentientOutposts', label: '灵煞异常', span: 1, component: SentientOutpostsPanel },
  { key: 'sortie', label: '突击任务', component: SortiePanel },
  { key: 'voidTrader', label: '虚空商人', component: VoidTraderPanel },
  { key: 'vaultTrader', label: '圣装重生', component: VaultTraderPanel },
  { key: 'nightwave', label: '午夜电波', component: NightwavePanel },
  { key: 'invasions', label: '入侵任务', component: InvasionsPanel },
  { key: 'conquests', label: '每周科研', component: ConquestsPanel },
  { key: 'hexCalendar', label: '1999 日历', component: HexCalendarPanel },
  { key: 'endlessXp', label: '无尽回廊', component: EndlessXpPanel },
  { key: 'syndicateMissions', label: '集团任务', component: SyndicateMissionsPanel }
]

const byKey = (k: WorldStateSection) => SECTIONS.find((s) => s.key === k)
const enabled = computed(() =>
  prefs.enabledSections.map((k) => byKey(k)).filter((s): s is Section => !!s)
)
const raw = ref<RawWorldState>()
const loading = ref(false)
const autoRefresh = ref(true)
const now = useTimestamp({ interval: 1000 })
const lastFetch = ref(Date.now())
const REFRESH_MS = 60000

async function fetchWorldState(force = false) {
  loading.value = true
  try {
    const res = force ? await force_reload() : await get_raw_world_state()
    if (res?.data) {
      raw.value = res.data
      lastFetch.value = Date.now()
    }
  } catch {
    ui.toast('err', '更新失败', '无法获取世界状态数据。')
  } finally {
    loading.value = false
  }
}

function onTimeUp() {
  fetchWorldState(false)
}

let timer: ReturnType<typeof setInterval> | null = null
function tickAuto() {
  if (autoRefresh.value && Date.now() - lastFetch.value >= REFRESH_MS) fetchWorldState(false)
  updateAnchors()
}

onMounted(() => {
  fetchWorldState(false)
  timer = setInterval(tickAuto, 1000)
  window.addEventListener('scroll', updateAnchors, { passive: true })
})
onBeforeUnmount(() => {
  if (timer) clearInterval(timer)
  window.removeEventListener('scroll', updateAnchors)
})

function toggleAuto() {
  autoRefresh.value = !autoRefresh.value
  if (autoRefresh.value) lastFetch.value = Date.now()
}

const updatedAt = computed(() => (raw.value?.Time ? new Date(raw.value.Time).toLocaleTimeString('zh-CN') : '—'))
const buildLabel = computed(() => String(raw.value?.BuildLabel || '').split('/')[0] || '—')
const nextRefresh = computed(() => {
  if (!autoRefresh.value) return '已暂停'
  const left = Math.max(0, REFRESH_MS - (now.value - lastFetch.value))
  return Math.ceil(left / 1000) + 's'
})

/* ── 拖拽排序 ── */
const dragFrom = ref<number | null>(null)
function onDrop(to: number) {
  if (dragFrom.value != null) prefs.moveSection(dragFrom.value, to)
  dragFrom.value = null
}

/* ── 锚点导航 ── */
// const anchorOpen = ref(true)
const activeKey = ref<string>('')
function scrollToPanel(key: string) {
  const el = document.getElementById('ws-panel-' + key)
  if (!el) return
  const top = window.scrollY + el.getBoundingClientRect().top - 72
  window.scrollTo({ top, behavior: 'smooth' })
  activeKey.value = key
}
function updateAnchors() {
  const keys = enabled.value.map((s) => s.key)
  let current = keys[0] || ''
  for (const k of keys) {
    const el = document.getElementById('ws-panel-' + k)
    if (!el) continue
    if (el.getBoundingClientRect().top <= 80) current = k
  }
  activeKey.value = current
}
</script>

<template>
  <section class="view active" id="view-world-state" data-od-id="world-state">
    <div class="page-head">
      <p class="eyebrow">世界状态 · 事件追踪</p>
      <h1 class="h2">事件追踪</h1>
      <p class="lead">始源星系实时动态：17 个可自定义的世界状态面板，支持显示配置、筛选与逐秒倒计时。</p>
    </div>

    <div class="ws-statusbar">
      <span class="ws-live"><span class="status-dot"></span>实时同步</span>
      <span class="ws-sb-meta">数据时间 <span class="num">{{ updatedAt }}</span></span>
      <span class="ws-sb-meta">下次自动刷新 <span class="num">{{ nextRefresh }}</span></span>
      <span class="ws-sb-meta">版本 <span class="num">{{ buildLabel }}</span></span>
      <button class="ws-auto" :class="{ on: autoRefresh }" :aria-pressed="autoRefresh" @click="toggleAuto">
        自动刷新：{{ autoRefresh ? '开' : '关' }}
      </button>
    </div>

    <!-- 配置面板 -->
    <div class="panel ws-config" :class="{ collapsed: !prefs.panelExpanded }">
      <button class="ws-config-head" @click="prefs.togglePanel()">
        <span class="ws-caret">▾</span>
        <span class="panel-title">世界状态面板配置</span>
        <span class="meta">已启用 {{ enabled.length }} / {{ SECTIONS.length }}</span>
      </button>
      <div v-show="prefs.panelExpanded" class="ws-config-body">
        <div class="ws-sec-group">
          <h4 class="ws-sec-title">可选模块</h4>
          <div class="ws-tags">
            <button
              v-for="s in SECTIONS"
              :key="s.key"
              class="ws-tag"
              :class="{ on: prefs.enabledSections.includes(s.key) }"
              :aria-pressed="prefs.enabledSections.includes(s.key)"
              @click="prefs.toggleSection(s.key)"
            >
              {{ s.label }}
            </button>
          </div>
        </div>
        <div v-if="enabled.length" class="ws-sec-group">
          <h4 class="ws-sec-title">显示顺序 · 拖拽标签调整</h4>
          <div class="ws-order">
            <div
              v-for="(s, i) in enabled"
              :key="s.key"
              class="ws-order-chip"
              draggable="true"
              @dragstart="dragFrom = i"
              @dragover.prevent
              @drop="onDrop(i)"
            >
              <span class="oc-grip">⋮⋮</span>
              <span class="oc-name">{{ s.label }}</span>
              <button class="oc-btn" :disabled="i === 0" @click="prefs.moveSection(i, i - 1)">↑</button>
              <button
                class="oc-btn"
                :disabled="i === enabled.length - 1"
                @click="prefs.moveSection(i, i + 1)"
              >
                ↓
              </button>
            </div>
          </div>
        </div>
        <div class="ws-config-foot">
          <div class="ws-actions">
            <button class="btn btn-secondary ws-mini" @click="prefs.selectAll()">全选</button>
            <button class="btn btn-secondary ws-mini" @click="prefs.deselectAll()">取消全选</button>
            <button class="btn btn-secondary ws-mini" @click="prefs.resetToDefault()">重置为默认</button>
          </div>
          <div class="ws-seg">
            <button :class="{ on: prefs.layout === 'grid' }" @click="prefs.setLayout('grid')">网格</button>
            <button :class="{ on: prefs.layout === 'list' }" @click="prefs.setLayout('list')">列表</button>
          </div>
        </div>
      </div>
    </div>

    <div class="ws-panels" :class="prefs.layout">
      <component
        :is="s.component"
        v-for="s in enabled"
        :id="'ws-panel-' + s.key"
        :key="s.key"
        :raw-world-state="raw"
        :icon="ICONS[s.key]"
        :span="s.span === 1"
        @time-up="onTimeUp"
      />
    </div>

    <div class="ws-fabs">
      <button class="btn btn-primary ws-fab" title="刷新世界状态" aria-label="刷新世界状态" @click="fetchWorldState(false)">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><path d="M20 11a8 8 0 1 0-1.6 4.8" /><path d="M20 4v5h-5" /></svg>
      </button>
      <button
        v-if="auth.canAdmin"
        class="btn ws-fab warn"
        title="强制同步上游数据"
        aria-label="强制同步"
        @click="fetchWorldState(true)"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round"><path d="M13 3 5 13h6l-1 8 8-10h-6z" /></svg>
      </button>
    </div>

    <nav class="ws-anchor" :class="{ collapsed: !prefs.anchorExpanded }" aria-label="面板快速跳转">
      <button class="ws-anchor-toggle" type="button" :aria-expanded="prefs.anchorExpanded" @click="prefs.toggleAnchor()">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="9" /><path d="m15.5 8.5-2.1 4.9-4.9 2.1 2.1-4.9z" /></svg>
        <span>快速跳转</span>
      </button>
      <div v-show="prefs.anchorExpanded" class="ws-anchor-list" role="list">
        <button
          v-for="(s, i) in enabled"
          :key="s.key"
          type="button"
          role="listitem"
          class="ws-anchor-item"
          :class="{ on: activeKey === s.key }"
          @click="scrollToPanel(s.key)"
        >
          <span class="an-dot" aria-hidden="true"></span>
          <span class="an-name">{{ s.title || s.label }}</span>
          <span class="an-idx" aria-hidden="true">{{ String(i + 1).padStart(2, '0') }}</span>
        </button>
      </div>
    </nav>
  </section>
</template>
