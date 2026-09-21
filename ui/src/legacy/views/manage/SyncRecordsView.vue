<template>
  <div class="p-4">
    <div class="flex justify-between items-center mb-4">
      <h1 class="text-2xl font-bold">同步记录</h1>
      <div class="flex gap-2">
        <pv-button @click="fetch" label="刷新" icon="pi pi-refresh" />
        <pv-button @click="forceReload" label="强制同步" icon="pi pi-sync" :loading="syncing" />
      </div>
    </div>

    <!-- 统计信息 -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
      <pv-panel header="总记录数">
        <div class="text-2xl font-bold">{{ total }}</div>
      </pv-panel>
      <pv-panel header="成功次数">
        <div class="text-2xl font-bold text-green-600">{{ successCount }}</div>
      </pv-panel>
      <pv-panel header="失败次数">
        <div class="text-2xl font-bold text-red-600">{{ failCount }}</div>
      </pv-panel>
      <pv-panel header="平均耗时">
        <div class="text-2xl font-bold">{{ avgTime }} ms</div>
      </pv-panel>
    </div>

    <!-- 数据表格 -->
    <pv-data-table
      :value="data"
      scrollable
      scrollHeight="calc(100vh - 330px)"
      paginator
      lazy
      :rows="pageSize"
      :totalRecords="total"
      :rowsPerPageOptions="[10, 20, 50, 100]"
      paginatorTemplate="CurrentPageReport FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown JumpToPageInput"
      currentPageReportTemplate="{currentPage}/{totalPages} 共 {totalRecords} 条"
      @page="onPageChange"
      :loading="loading"
    >
      <pv-column field="id" header="ID" style="width: 80px"></pv-column>
      <pv-column field="sync_time" header="同步时间" style="width: 180px">
        <template #body="slotProps">
          {{ formatDateTime(slotProps.data.sync_time) }}
        </template>
      </pv-column>
      <pv-column field="is_success" header="状态" style="width: 100px">
        <template #body="slotProps">
          <pv-tag
            :severity="slotProps.data.is_success ? 'success' : 'danger'"
            :value="slotProps.data.is_success ? '成功' : '失败'"
          />
        </template>
      </pv-column>
      <pv-column field="time_spent_ms" header="耗时 (ms)" style="width: 100px">
        <template #body="slotProps">
          <span :class="getTimeColor(slotProps.data.time_spent_ms)">
            {{ slotProps.data.time_spent_ms }}
          </span>
        </template>
      </pv-column>
      <pv-column field="error_msg" header="错误信息" :exportable="false">
        <template #body="slotProps">
          <span v-if="slotProps.data.error_msg" class="text-red-600">
            {{ truncate(slotProps.data.error_msg, 50) }}
          </span>
          <span v-else class="text-gray-400">-</span>
        </template>
      </pv-column>
      <pv-column field="warnings" header="警告" :exportable="false">
        <template #body="slotProps">
          <span v-if="slotProps.data.warnings" class="text-yellow-600">
            {{ truncate(slotProps.data.warnings, 50) }}
          </span>
          <span v-else class="text-gray-400">-</span>
        </template>
      </pv-column>
      <pv-column field="nearest_expiry" header="最近过期时间" style="width: 180px">
        <template #body="slotProps">
          <span v-if="slotProps.data.nearest_expiry">
            {{ formatDateTime(slotProps.data.nearest_expiry) }}
          </span>
          <span v-else class="text-gray-400">-</span>
        </template>
      </pv-column>
      <pv-column header="操作" :exportable="false" style="min-width: 120px">
        <template #body="slotProps">
          <pv-button
            icon="pi pi-info-circle"
            size="small"
            variant="outlined"
            rounded
            class="mr-2"
            @click="showDetail(slotProps.data)"
          />
          <pv-button
            icon="pi pi-copy"
            size="small"
            variant="outlined"
            rounded
            @click="copyWarnings(slotProps.data)"
          />
        </template>
      </pv-column>
    </pv-data-table>

    <!-- 详情对话框 -->
    <pv-dialog
      v-model:visible="detailDialogVis"
      :style="{ width: '800px' }"
      header="同步记录详情"
      :modal="true"
    >
      <div v-if="selectedRecord" class="flex flex-col gap-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block font-bold mb-2">ID</label>
            <div>{{ selectedRecord.id }}</div>
          </div>
          <div>
            <label class="block font-bold mb-2">状态</label>
            <pv-tag
              :severity="selectedRecord.is_success ? 'success' : 'danger'"
              :value="selectedRecord.is_success ? '成功' : '失败'"
            />
          </div>
        </div>

        <div>
          <label class="block font-bold mb-2">同步时间</label>
          <div>{{ formatDateTime(selectedRecord.sync_time) }}</div>
        </div>

        <div>
          <label class="block font-bold mb-2">耗时</label>
          <div>{{ selectedRecord.time_spent_ms }} ms</div>
        </div>

        <div v-if="selectedRecord.nearest_expiry">
          <label class="block font-bold mb-2">最近过期时间</label>
          <div>{{ formatDateTime(selectedRecord.nearest_expiry) }}</div>
        </div>

        <div v-if="selectedRecord.error_msg">
          <label class="block font-bold mb-2">错误信息</label>
          <pre class="bg-red-50 p-3 rounded text-red-700 text-sm overflow-auto max-h-40">{{
            selectedRecord.error_msg
          }}</pre>
        </div>

        <div v-if="selectedRecord.warnings">
          <label class="block font-bold mb-2">警告信息</label>
          <pre class="bg-yellow-50 p-3 rounded text-yellow-700 text-sm overflow-auto max-h-40">{{
            selectedRecord.warnings
          }}</pre>
        </div>
      </div>
    </pv-dialog>
  </div>
</template>

<script setup lang="ts">
import { list_sync_records, type SyncRecord } from '@/api/sync'
import { ref, computed, onMounted } from 'vue'
import dayjs from 'dayjs'
import { useCommonToast } from '@/util/common'
import { force_reload } from '@/api/world_state'
import { type PageState } from 'primevue'

const data = ref<SyncRecord[]>([])
const loading = ref(false)
const syncing = ref(false)
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(20)

const detailDialogVis = ref(false)
const selectedRecord = ref<SyncRecord | null>(null)

// 统计数据
const successCount = computed(() => data.value.filter((r) => r.is_success).length)
const failCount = computed(() => data.value.filter((r) => !r.is_success).length)
const avgTime = computed(() => {
  if (data.value.length === 0) return 0
  const sum = data.value.reduce((acc, r) => acc + r.time_spent_ms, 0)
  return Math.round(sum / data.value.length)
})

async function fetch() {
  loading.value = true
  try {
    const res = await list_sync_records(currentPage.value, pageSize.value)
    data.value = res.data.items
    total.value = res.data.total
  } catch (e) {
    console.error('Failed to fetch sync records', e)
  } finally {
    loading.value = false
  }
}

function onPageChange(event: PageState) {
  currentPage.value = event.page + 1
  pageSize.value = event.rows
  fetch()
}

async function forceReload() {
  syncing.value = true
  try {
    await force_reload()
    const toast = useCommonToast()
    toast.add({
      severity: 'success',
      summary: '同步成功',
      detail: '世界状态已成功同步',
      life: 3000,
    })
    await fetch()
  } catch (e) {
    console.error('Failed to force reload', e)
  } finally {
    syncing.value = false
  }
}

function showDetail(record: SyncRecord) {
  selectedRecord.value = record
  detailDialogVis.value = true
}

function copyWarnings(record: SyncRecord) {
  const content = record.warnings || record.error_msg || ''
  if (!content) {
    const toast = useCommonToast()
    toast.add({
      severity: 'info',
      summary: '无内容',
      detail: '没有可复制的内容',
      life: 2000,
    })
    return
  }
  navigator.clipboard.writeText(content)
  const toast = useCommonToast()
  toast.add({
    severity: 'success',
    summary: '复制成功',
    detail: '已复制到剪贴板',
    life: 2000,
  })
}

function formatDateTime(dt: string) {
  return dayjs(dt).format('YYYY-MM-DD HH:mm:ss')
}

function truncate(str: string, len: number) {
  if (str.length <= len) return str
  return str.substring(0, len) + '...'
}

function getTimeColor(ms: number) {
  if (ms < 1000) return 'text-green-600'
  if (ms < 3000) return 'text-yellow-600'
  return 'text-red-600'
}

onMounted(() => {
  fetch()
})
</script>
