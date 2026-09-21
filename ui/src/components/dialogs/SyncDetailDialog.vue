<script setup lang="ts">
import { fmt, fmtTime } from '@/lib/utils'
import type { SyncRecordRow } from './types'

defineProps<{ record: SyncRecordRow }>()
const emit = defineEmits<{ close: [] }>()
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head">
      <h3 class="h3">同步详情</h3>
      <button class="icon-btn" title="关闭" @click="emit('close')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M6 6l12 12M18 6L6 18" /></svg>
      </button>
    </div>
    <div class="dlg-body" id="sync-detail-body">
      <div class="form-grid">
        <div class="field"><label>记录 ID</label><div class="code-block">#{{ record.id }}</div></div>
        <div class="field"><label>同步时间</label><div class="code-block">{{ fmtTime(record.sync_time) }}</div></div>
        <div class="field"><label>状态</label><div class="code-block">{{ record.is_success ? '成功' : '失败' }}</div></div>
        <div class="field"><label>耗时</label><div class="code-block">{{ fmt(record.time_spent_ms) }} ms</div></div>
        <div class="field"><label>最近过期</label><div class="code-block">{{ fmtTime(record.nearest_expiry) }}</div></div>
        <div class="field"><label>警告数</label><div class="code-block">{{ record.warnings.length }}</div></div>
      </div>
      <div v-if="record.error_msg" class="field">
        <label>错误信息</label>
        <div class="code-block err">{{ record.error_msg }}</div>
      </div>
      <div v-if="record.warnings.length" class="field">
        <label>警告（{{ record.warnings.length }}）</label>
        <div class="code-block warn">{{ record.warnings.join('\n') }}</div>
      </div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="emit('close')">关闭</button>
    </div>
  </div>
</template>
