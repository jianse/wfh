<script setup lang="ts">
import { onMounted, ref } from 'vue'
import DataPager from '@/components/DataPager.vue'
import DataTable from '@/components/DataTable.vue'
import DataTableColumn from '@/components/DataTableColumn.vue'
import { useUiStore } from '@/stores/ui'
import { fmt, fmtTime } from '@/lib/utils'
import { get_sync_stats, list_sync_records, type SyncRecord } from '@/api/sync'
import { force_reload } from '@/api/world_state'
import SyncDetailDialog from '@/components/dialogs/SyncDetailDialog.vue'
import type { SyncRecordRow } from '@/components/dialogs/types'

const ui = useUiStore()

const records = ref<SyncRecordRow[]>([])
const total = ref(0)
const loading = ref(false)
const page = ref(1)
const size = ref(10)

function mapRecord(r: SyncRecord): SyncRecordRow {
  let warnings = []
  if (r.warnings) {
    try {
      const parsed = JSON.parse(r.warnings)
      warnings = Array.isArray(parsed) ? parsed : [String(parsed)]
    } catch {
      warnings = [String(r.warnings)]
    }
  }
  return {
    id: r.id,
    sync_time: r.sync_time ? new Date(r.sync_time).getTime() : null,
    is_success: r.is_success,
    time_spent_ms: r.time_spent_ms,
    error_msg: r.error_msg || '',
    warnings,
    nearest_expiry: r.nearest_expiry ? new Date(r.nearest_expiry).getTime() : null,
  }
}

async function fetchRecords() {
  loading.value = true
  void loadStats()
  try {
    const res = await list_sync_records(page.value, size.value)
    if (res && res.data) {
      records.value = res.data.items.map(mapRecord)
      total.value = res.data.total
    }
  } catch {
    ui.toast('err', '加载失败', '无法获取同步记录。')
  } finally {
    loading.value = false
  }
}

onMounted(fetchRecords)

const stats = ref({ total: 0, ok: 0, fail: 0, avg: 0 })

async function loadStats() {
  try {
    const s = await get_sync_stats()
    stats.value = {
      total: s.total,
      ok: s.success,
      fail: s.fail,
      avg: Math.round(s.avg_time_ms),
    }
  } catch {
    // keep the previously loaded statistics on failure
  }
}

// const rows = computed(() => {
//   const totalPages = Math.max(1, Math.ceil(records.value.length / PAGE_SIZE))
//   const current = Math.min(Math.max(1, page.value), totalPages)
//   return records.value.slice((current - 1) * PAGE_SIZE, current * PAGE_SIZE)
// })

async function forceSync() {
  try {
    await force_reload()
    ui.toast('ok', '同步完成', '世界状态已成功同步。')
    await fetchRecords()
  } catch {
    ui.toast('err', '同步失败', '无法强制同步世界状态。')
  }
}

function onSizeChange(s: number) {
  size.value = s
  page.value = 1
  fetchRecords()
}

function refresh() {
  fetchRecords()
  ui.toast('info', '已刷新', '同步记录已重新载入。')
}
</script>

<template>
  <section class="view active" id="view-sync-records" data-od-id="sync-records">
    <div class="page-head">
      <p class="eyebrow">管理后台 · 同步记录</p>
      <h1 class="h2">同步记录</h1>
      <p class="lead">原始世界状态的每次同步都会留痕，可查看耗时、错误与警告。</p>
    </div>

    <div class="stat-grid" id="sync-stats">
      <div class="stat">
        <div class="stat-num">{{ stats.total }}</div>
        <div class="stat-label">同步总次数</div>
      </div>
      <div class="stat">
        <div class="stat-num ok">{{ stats.ok }}</div>
        <div class="stat-label">同步成功</div>
      </div>
      <div class="stat">
        <div class="stat-num bad">{{ stats.fail }}</div>
        <div class="stat-label">同步失败</div>
      </div>
      <div class="stat">
        <div class="stat-num">{{ stats.avg }}<span class="stat-unit">ms</span></div>
        <div class="stat-label">平均耗时</div>
      </div>
    </div>

    <div class="page-tools">
      <button class="btn btn-secondary" id="sync-refresh" :disabled="loading" @click="refresh">
        刷新
      </button>
      <button class="btn btn-primary" id="sync-force" :disabled="loading" @click="forceSync">
        强制同步
      </button>
      <div class="spacer"></div>
      <span class="meta" id="sync-total">共 {{ total }} 条记录</span>
    </div>

    <DataTable
      :rows="records"
      row-key="id"
      :loading="loading"
      empty-text="暂无同步记录。"
      loading-text="正在加载同步记录…"
    >
      <DataTableColumn prop="id" label="ID">
        <template #default="{ row }"><span class="mono">#{{ row.id }}</span></template>
      </DataTableColumn>
      <DataTableColumn prop="sync_time" label="同步时间">
        <template #default="{ row }"><span class="mono">{{ fmtTime(row.sync_time) }}</span></template>
      </DataTableColumn>
      <DataTableColumn prop="status" label="状态">
        <template #default="{ row }">
          <span class="badge" :class="row.is_success ? 'ok' : 'bad'">{{
            row.is_success ? '成功' : '失败'
          }}</span>
        </template>
      </DataTableColumn>
      <DataTableColumn prop="time_spent_ms" label="耗时 (ms)" align="right">
        <template #default="{ row }">{{ fmt(row.time_spent_ms) }}</template>
      </DataTableColumn>
      <DataTableColumn prop="nearest_expiry" label="最近过期">
        <template #default="{ row }"><span class="mono">{{ fmtTime(row.nearest_expiry) }}</span></template>
      </DataTableColumn>
      <DataTableColumn prop="warnings" label="错误 / 警告" cell-class="wrap">
        <template #default="{ row }">
          <span v-if="row.error_msg" class="cell-err">{{ row.error_msg }}</span>
          <span v-else-if="row.warnings.length" class="cell-sub"
            >{{ row.warnings.length }} 条警告，点击详情查看</span
          >
          <template v-else>—</template>
        </template>
      </DataTableColumn>
      <DataTableColumn label="操作">
        <template #default="{ row }">
          <button class="btn btn-ghost" @click="ui.openDialog(SyncDetailDialog, { record: row })">
            详情
          </button>
        </template>
      </DataTableColumn>
    </DataTable>
    <DataPager
      :page="page"
      :total="total"
      :size="size"
      :page-sizes="[10, 20, 50]"
      :disabled="loading"
      @size-change="onSizeChange"
      @go="
        (p) => {
          page = p
          fetchRecords()
        }
      "
    />
  </section>
</template>
