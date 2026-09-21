<script setup lang="ts">
import { fmt, fmtTime } from '@/lib/utils'
import type { SyncModuleRow } from './types'

const props = defineProps<{
  module: SyncModuleRow
  onClear?: () => void
  onForce?: () => void
  onSync?: () => void
}>()

const emit = defineEmits<{ close: [] }>()

const SYNCD_SOURCE: Record<string, string> = { export: '官方导出', api: '上游接口' }
const SYNCD_STATUS: Record<string, string> = {
  synced: '已同步',
  outdated: '待更新',
  syncing: '同步中',
  failed: '同步失败'
}

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

function run(fn?: () => void) {
  emit('close')
  fn?.()
}
</script>

<template>
  <div class="modal-panel wide">
    <div class="dlg-head">
      <h3 class="h3" id="syncd-dlg-title">{{ props.module.name }} · 同步详情</h3>
      <button class="btn btn-ghost dlg-close" @click="emit('close')">关闭</button>
    </div>
    <div class="dlg-body" id="syncd-dlg-body">
      <div class="form-grid">
        <div class="field"><label>模块</label><div class="code-block">{{ props.module.name }}（{{ props.module.key }}）</div></div>
        <div class="field"><label>分组 / 数据源</label><div class="code-block">{{ props.module.group }} · {{ SYNCD_SOURCE[props.module.source] }}</div></div>
        <div class="field"><label>同步状态</label><div class="code-block">{{ SYNCD_STATUS[props.module.status] || '未知' }}</div></div>
        <div class="field"><label>本地版本</label><div class="code-block">{{ props.module.localVer || '—' }}</div></div>
        <div class="field"><label>记录数</label><div class="code-block">{{ fmt(props.module.count) }} 项</div></div>
        <div class="field"><label>最近同步</label><div class="code-block">{{ props.module.lastSync ? `${fmtTime(props.module.lastSync)}（${syncdRel(props.module.lastSync)}）` : '从未同步' }}</div></div>
      </div>
      <div class="field"><label>说明</label><div class="code-block">{{ props.module.note || '—' }}</div></div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-danger mr-auto" id="syncd-dlg-clear" @click="run(props.onClear)">清空本地缓存</button>
      <button class="btn btn-secondary" id="syncd-dlg-force" @click="run(props.onForce)">强制重新拉取</button>
      <button class="btn btn-secondary" id="syncd-dlg-sync" :disabled="props.module.status === 'syncing'" @click="run(props.onSync)">立即同步</button>
    </div>
  </div>
</template>
