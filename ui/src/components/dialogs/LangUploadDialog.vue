<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  locale: string
  current: Record<string, string>
  onApply?: (pairs: [string, string][]) => void
}>()
const emit = defineEmits<{ close: [] }>()

const jsonText = ref('')
const fileName = ref('')
interface LangPreview {
  add: [string, string][]
  upd: [string, string][]
  same: number
  bad: number
}
const preview = ref<LangPreview | null>(null)
const previewErr = ref('')
const pending = ref<[string, string][]>([])

function parseLangJson(raw: string): [string, string][] {
  const data: unknown = JSON.parse(raw)
  const entries: [string, string][] = []
  if (Array.isArray(data)) {
    data.forEach((o) => {
      if (!o || typeof o !== 'object') return
      const obj = o as Record<string, unknown>
      const base = String(obj.unique_name ?? obj.key ?? '')
      if (!base) return
      const content = obj.content ?? obj.value
      entries.push([obj.prop ? base + '@' + String(obj.prop) : base, content != null ? String(content) : ''])
    })
  } else if (data && typeof data === 'object') {
    const obj = data as Record<string, unknown>
    Object.keys(obj).forEach((k) => {
      const v = obj[k]
      if (v && typeof v === 'object') {
        const inner = v as Record<string, unknown>
        const content = inner.content ?? inner.value
        entries.push([k, content != null ? String(content) : ''])
      } else {
        entries.push([k, v != null ? String(v) : ''])
      }
    })
  } else {
    throw new Error('JSON 结构不受支持')
  }
  return entries
}

function runLangPreview() {
  const raw = jsonText.value.trim()
  if (!raw) {
    previewErr.value = '请先选择文件或粘贴 JSON 内容。'
    preview.value = null
    pending.value = []
    return
  }
  let entries: [string, string][]
  try {
    entries = parseLangJson(raw)
  } catch (e) {
    previewErr.value = `JSON 解析失败：${(e as Error).message}`
    preview.value = null
    pending.value = []
    return
  }
  previewErr.value = ''
  const cur = props.current
  const add: [string, string][] = []
  const upd: [string, string][] = []
  let same = 0
  let bad = 0
  const valid: [string, string][] = []
  entries.forEach((pair) => {
    const k = pair[0]
    const v = pair[1]
    if (typeof k !== 'string' || !k.trim() || typeof v !== 'string') {
      bad++
      return
    }
    valid.push([k, v])
    if (!(k in cur)) add.push([k, v])
    else if (cur[k] !== v) upd.push([k, v])
    else same++
  })
  pending.value = valid
  preview.value = { add, upd, same, bad }
}

function readLangFile(file: File | null | undefined) {
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    jsonText.value = String(reader.result || '')
    fileName.value = file.name
    runLangPreview()
  }
  reader.readAsText(file)
}

function onFilePick(e: Event) {
  readLangFile((e.target as HTMLInputElement).files?.[0])
}

function onDrop(e: DragEvent) {
  readLangFile(e.dataTransfer?.files?.[0])
}

function apply() {
  if (!pending.value.length) return
  props.onApply?.(pending.value)
  emit('close')
}
</script>

<template>
  <div class="modal-panel wide">
    <div class="dlg-head">
      <h3 class="h3">上传 JSON 覆写</h3>
      <button class="btn btn-ghost dlg-close" @click="emit('close')">关闭</button>
    </div>
    <div class="dlg-body">
      <p class="help-note">
        将覆写 <strong>{{ props.locale }}</strong> 的现有词条。支持扁平键值对
        <span class="mono">{ "key": "value" }</span>，或 <span class="mono">unique_name / prop / content</span> 对象数组。
      </p>
      <div class="lang-drop" @dragenter.prevent @dragover.prevent @dragleave.prevent @drop.prevent="onDrop">
        <input type="file" id="lang-file" accept="application/json,.json" hidden @change="onFilePick" />
        <div class="lang-drop-inner">
          <strong>拖拽 JSON 文件到此处</strong>
          <span class="meta">或</span>
          <label class="btn btn-secondary" for="lang-file">选择文件</label>
          <span class="meta">{{ fileName }}</span>
        </div>
      </div>
      <div class="field mt-sm">
        <label>或直接粘贴 JSON</label>
        <textarea v-model="jsonText" class="input mono" rows="7" placeholder='{ "ui.nav.home": "主页" }'></textarea>
      </div>
      <button class="btn btn-secondary" @click="runLangPreview">解析预览</button>
      <div class="mt-sm">
        <div v-if="previewErr" class="field-err">{{ previewErr }}</div>
        <template v-else-if="preview">
          <div class="form-grid">
            <div class="field"><label>新增</label><div class="code-block">{{ preview.add.length }}</div></div>
            <div class="field"><label>更新</label><div class="code-block">{{ preview.upd.length }}</div></div>
            <div class="field"><label>无变化</label><div class="code-block">{{ preview.same }}</div></div>
            <div class="field"><label>忽略</label><div class="code-block">{{ preview.bad }}</div></div>
          </div>
          <div v-if="preview.add.length" class="field">
            <label>新增词条</label>
            <div class="lang-diff">
              <div v-for="(p, i) in preview.add.slice(0, 6)" :key="i" class="lang-diff-row lang-diff-add"><span class="k">{{ p[0] }}</span><span class="v">{{ p[1] }}</span></div>
            </div>
          </div>
          <div v-if="preview.upd.length" class="field">
            <label>更新词条</label>
            <div class="lang-diff">
              <div v-for="(p, i) in preview.upd.slice(0, 6)" :key="i" class="lang-diff-row lang-diff-add"><span class="k">{{ p[0] }}</span><span class="v">{{ p[1] }}</span></div>
            </div>
          </div>
        </template>
      </div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="emit('close')">取消</button>
      <button class="btn btn-primary" :disabled="!preview || preview.add.length + preview.upd.length === 0" @click="apply">应用覆写</button>
    </div>
  </div>
</template>
