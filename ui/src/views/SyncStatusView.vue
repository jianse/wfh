<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import DataPager from '@/components/DataPager.vue'
import SyncModuleDialog from '@/components/dialogs/SyncModuleDialog.vue'
import type { SyncModuleRow } from '@/components/dialogs/types'
import { useUiStore } from '@/stores/ui'
import { fmt, fmtTime } from '@/lib/utils'
import { useSyncStore } from '@/stores/sync'
import {
  RESOURCES_INCLUDED,
  count_in_db,
  get_last_update_time,
  clear,
  type StoreName
} from '@/util/localDB'
import { sync_all, force_sync_one } from '@/util/resources'

const UI = useUiStore()
const syncStore = useSyncStore()

const CAT_LABEL: Record<string, string> = {
  Warframes: '战甲',
  Weapons: '武器',
  Upgrades: '振幅晶体',
  Arcane: '赋能',
  Recipes: '配方',
  Relics: '遗物',
  Manifest: '清单',
  Resources: '资源',
  Bounties: '赏金',
  Rewards: '奖励列表',
  FusionBundles: '融合包',
  Images: '图片'
}
const WFPEP = new Set(['Rewards', 'Bounties', 'FusionBundles', 'Warframes', 'Images'])

const SYNCD_SOURCE: Record<string, string> = { export: '官方导出', api: '上游接口' }
const SYNCD_STATUS: Record<string, string> = {
  synced: '已同步',
  outdated: '待更新',
  syncing: '同步中',
  failed: '同步失败'
}

interface DbStat {
  count: number
  total: number
  lastSync: number | null
}

const dbStats = ref<Record<string, DbStat>>({})
const page = ref(1)
const size = ref(10)
const q = ref('')
const filter = ref('all')
const checked = ref(fmtTime(Date.now()))
const autoSync = ref(false)
const allBusy = ref(false)


async function refreshStats() {
  const out: Record<string, DbStat> = {}
  for (const c of RESOURCES_INCLUDED) {
    const lut = await get_last_update_time(c)
    const count = await count_in_db(c)
    out[c] = {
      count,
      total: lut ? lut.total : 0,
      lastSync: lut ? new Date(lut.last_update_time).getTime() : null
    }
  }
  dbStats.value = out
}

const modules = computed<SyncModuleRow[]>(() =>
  RESOURCES_INCLUDED.map((c) => {
    const s = dbStats.value[c] || {}
    const phase = syncStore.state[c]?.phase
    let status = 'outdated'
    if (phase === '正在同步') status = 'syncing'
    else if (s.lastSync) status = 'synced'
    return {
      key: c,
      name: CAT_LABEL[c] || c,
      group: '数据库',
      source: WFPEP.has(c) ? 'api' : 'export',
      count: s.count ?? syncStore.state[c]?.completed ?? 0,
      size: 0,
      status,
      pending: 0,
      note: status === 'synced' ? '本地缓存与上游导出一致。' : '尚未同步，点击同步以拉取数据。',
      localVer: s.lastSync ? fmtTime(s.lastSync) : '—',
      upstreamVer: '—',
      lastSync: s.lastSync
    }
  })
)

const stats = computed(() => ({
  total: modules.value.length,
  ok: modules.value.filter((m) => m.status === 'synced').length,
  out: modules.value.filter((m) => m.status === 'outdated').length,
  fail: modules.value.filter((m) => m.status === 'failed').length
}))

const pendingCount = computed(() => stats.value.out + stats.value.fail)

const TABS: [string, string][] = [
  ['all', '全部'],
  ['synced', '已同步'],
  ['outdated', '待更新'],
  ['syncing', '同步中'],
  ['failed', '同步失败']
]
function tabCount(v: string) {
  return v === 'all' ? modules.value.length : modules.value.filter((m) => m.status === v).length
}

const filtered = computed(() => {
  const term = q.value.trim().toLowerCase()
  let rows = modules.value.filter((m) => filter.value === 'all' || m.status === filter.value)
  if (term) rows = rows.filter((m) => (m.name + ' ' + m.key).toLowerCase().includes(term))
  return rows
})
const pageRows = computed(() =>
  filtered.value.slice((page.value - 1) * size.value, (page.value - 1) * size.value + size.value)
)

function syncdRel(ts: number | null | undefined): string {
  if (!ts) return '从未同步'
  const d = Date.now() - ts
  if (d < 60000) return '刚刚'
  const m = Math.floor(d / 60000)
  if (m < 60) return m + ' 分钟前'
  const h = Math.floor(m / 60)
  if (h < 24) return h + ' 小时前'
  return Math.floor(h / 24) + ' 天前'
}

function badgeCls(st: string): string {
  return st === 'synced' ? 'ok' : st === 'failed' ? 'bad' : st === 'outdated' ? 'warn' : st === 'syncing' ? 'syncing' : ''
}

function updateChecked() {
  checked.value = fmtTime(Date.now())
}

function onSizeChange(s: number) {
  size.value = s
  page.value = 1
}

async function syncOneModule(key: string) {
  const m = modules.value.find((x) => x.key === key)
  if (!m || m.status === 'syncing') return
  try {
    await force_sync_one(key)
    await refreshStats()
    updateChecked()
    UI.toast('ok', '同步完成', `「${m.name}」已重新同步。`)
  } catch {
    UI.toast('err', '同步失败', `「${m.name}」同步失败。`)
  }
}

async function forceReSync(key: string) {
  await syncOneModule(key)
}

function clearModuleCache(key: string) {
  const m = modules.value.find((x) => x.key === key)
  if (!m) return
  UI.askConfirm('清空本地缓存', `确定清空「${m.name}」的本地缓存吗？清空后需要重新同步才能恢复。`, async () => {
    await clear(key as StoreName)
    await refreshStats()
    UI.toast('info', '已清空', `「${m.name}」的本地缓存已清空。`)
  })
}



async function syncAllPending() {
  allBusy.value = true
  try {
    await sync_all(true)
    await refreshStats()
    updateChecked()
    UI.toast('ok', '全部同步完成', '已从上游重新拉取本地缓存。')
  } catch {
    UI.toast('err', '同步失败', '同步过程中出现错误。')
  } finally {
    allBusy.value = false
  }
}

function refresh() {
  refreshStats()
  updateChecked()
  UI.toast('info', '已刷新', '本地缓存状态已重新检测。')
}

function toggleAuto() {
  autoSync.value = !autoSync.value
  UI.toast('info', autoSync.value ? '已开启自动同步' : '已暂停自动同步', autoSync.value ? '页面加载时会自动检查上游数据。' : '需手动点击「全部同步」才会拉取更新。')
}

onMounted(() => {
  refreshStats()
})

onBeforeUnmount(() => {})
</script>

<template>
  <section class="view active" id="view-sync-status" data-od-id="sync-status">
    <div class="page-head">
      <p class="eyebrow">系统 · 数据同步</p>
      <h1 class="h2">本地数据同步状态</h1>
      <p class="lead">对照上游版本检查各模块的本地缓存，随时按模块触发同步、强制重新拉取或清空缓存，无需登录。</p>
    </div>

    <div class="ws-statusbar">
      <span class="ws-live"><span class="status-dot"></span>本地缓存已就绪</span>
      <span class="ws-sb-meta">上游数据源 <span class="num">Public Export · warframestat</span></span>
      <span class="ws-sb-meta">最后检查 <span class="num" id="syncd-checked">{{ checked }}</span></span>
      <span class="ws-sb-meta">待更新模块 <span class="num" id="syncd-pending">{{ pendingCount }}</span></span>
      <button class="ws-auto" :class="{ on: autoSync }" id="syncd-auto" :aria-pressed="autoSync" type="button" @click="toggleAuto">自动同步：{{ autoSync ? '开' : '关' }}</button>
    </div>

    <div class="stat-grid" id="syncd-stats">
      <div class="stat"><div class="stat-num">{{ stats.total }}</div><div class="stat-label">缓存模块</div></div>
      <div class="stat"><div class="stat-num ok">{{ stats.ok }}</div><div class="stat-label">已同步</div></div>
      <div class="stat"><div class="stat-num">{{ stats.out }}</div><div class="stat-label">待更新</div></div>
      <div class="stat"><div class="stat-num bad">{{ stats.fail }}</div><div class="stat-label">同步失败</div></div>
    </div>

    <div class="tabs" id="syncd-tabs" role="tablist" aria-label="按同步状态筛选" data-od-id="sync-status-tabs">
      <button
        v-for="[v, label] in TABS"
        :key="v"
        class="tab"
        :class="{ active: v === filter }"
        role="tab"
        :aria-selected="v === filter"
        @click="filter = v; page = 1"
      >
        {{ label }}<span class="tab-count">{{ tabCount(v) }}</span>
      </button>
    </div>

    <div class="page-tools">
      <input v-model="q" class="input" id="syncd-search" type="text" placeholder="搜索模块名称或键名…" @input="page = 1" />
      <button class="btn btn-secondary" id="syncd-refresh" @click="refresh">刷新状态</button>
      <button class="btn btn-primary" id="syncd-all" data-od-id="sync-all" :disabled="allBusy" @click="syncAllPending">全部同步</button>
      <div class="spacer"></div>
      <span class="meta" id="syncd-total">共 {{ filtered.length }} 个模块</span>
    </div>

    <div class="sync-list" id="syncd-list" data-od-id="sync-module-list">
      <div v-if="!pageRows.length" class="sync-empty">没有符合当前筛选条件的模块。</div>
      <div v-for="m in pageRows" :key="m.key" class="sync-item" :data-status="m.status" :data-od-id="`sync-item-${m.key}`">
        <div class="sync-main">
          <div class="sync-name">{{ m.name }}<span class="sync-key">{{ m.key }}</span></div>
          <div class="sync-sub">{{ SYNCD_SOURCE[m.source] }} · {{ fmt(m.count) }} 项</div>
        </div>
        <div class="sync-vers">
          <div><span class="lbl">本地版本</span><span class="num">{{ m.localVer || '—' }}</span></div>
          <div><span class="lbl">上游版本</span><span class="num" :class="{ ahead: m.status !== 'synced' }">{{ m.upstreamVer || '—' }}</span></div>
        </div>
        <div class="sync-when"><span>{{ fmtTime(m.lastSync) }}</span><span>{{ syncdRel(m.lastSync) }}</span></div>
        <div class="sync-state"><span class="badge" :class="badgeCls(m.status)">{{ SYNCD_STATUS[m.status] || '未知' }}</span></div>
        <div class="sync-acts">
          <button class="btn btn-secondary" :disabled="m.status === 'syncing'" @click="syncOneModule(m.key)">{{ m.status === 'syncing' ? '同步中…' : '同步' }}</button>
          <button
            class="btn btn-ghost"
            @click="
              UI.openDialog(SyncModuleDialog, {
                module: m,
                onClear: () => clearModuleCache(m.key),
                onForce: () => forceReSync(m.key),
                onSync: () => syncOneModule(m.key)
              })
            "
          >
            详情
          </button>
        </div>
      </div>
    </div>
    <DataPager
      :page="page"
      :total="filtered.length"
      :size="size"
      :page-sizes="[10, 20, 50]"
      :disabled="allBusy"
      @size-change="onSizeChange"
      @go="(p) => (page = p)"
    />
  </section>
</template>
