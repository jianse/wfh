<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  onApply?: (
    data: Record<string, string>,
    onProgress: (pct: number) => void
  ) => Promise<void> | void
}>()
const emit = defineEmits<{ close: [] }>()

const jsonText = ref('')
const fileName = ref('')
const err = ref('')
const count = ref(0)
const applying = ref(false)
const progress = ref(0)

const percent = () => Math.round(progress.value * 100)

function parse(raw: string): Record<string, string> {
  const data: unknown = JSON.parse(raw)
  if (!data || typeof data !== 'object' || Array.isArray(data)) {
    throw new Error('需要扁平对象 { "key": "value" }')
  }
  const out: Record<string, string> = {}
  for (const [k, v] of Object.entries(data as Record<string, unknown>)) {
    if (typeof v === 'string') out[k] = v
  }
  return out
}

function preview() {
  err.value = ''
  count.value = 0
  const raw = jsonText.value.trim()
  if (!raw) {
    err.value = '请先选择文件或粘贴 JSON 内容。'
    return
  }
  try {
    count.value = Object.keys(parse(raw)).length
  } catch (e) {
    err.value = 'JSON 解析失败：' + (e as Error).message
  }
}

function readFile(file?: File | null) {
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    jsonText.value = String(reader.result || '')
    fileName.value = file.name
    preview()
  }
  reader.readAsText(file)
}

function onFilePick(e: Event) {
  readFile((e.target as HTMLInputElement).files?.[0])
}
function onDrop(e: DragEvent) {
  readFile(e.dataTransfer?.files?.[0])
}

async function apply() {
  if (applying.value) return
  err.value = ''
  const raw = jsonText.value.trim()
  if (!raw) {
    err.value = '请先选择文件或粘贴 JSON 内容。'
    return
  }
  let obj: Record<string, string>
  try {
    obj = parse(raw)
  } catch (e) {
    err.value = 'JSON 解析失败：' + (e as Error).message
    return
  }
  applying.value = true
  progress.value = 0
  try {
    await props.onApply?.(obj, (p) => {
      progress.value = p
    })
    emit('close')
  } catch (e) {
    err.value = '上传失败：' + ((e as Error).message || String(e))
  } finally {
    applying.value = false
    progress.value = 0
  }
}
</script>

<template>
  <div class="modal-panel wide">
    <div class="dlg-head">
      <h3 class="h3">上传 JSON 覆写</h3>
      <button class="btn btn-ghost dlg-close" :disabled="applying" @click="emit('close')">关闭</button>
    </div>
    <div class="dlg-body">
      <p class="help-note">
        将<strong>整体替换</strong>简体中文语言表。文件需为扁平键值对
        <span class="mono">{ "unique_name": "译文" }</span>。
      </p>
      <div
        class="lang-drop"
        :class="{ over: applying }"
        @dragenter.prevent
        @dragover.prevent
        @dragleave.prevent
        @drop.prevent="onDrop"
      >
        <input type="file" id="lang-file" accept="application/json,.json" hidden :disabled="applying" @change="onFilePick" />
        <div class="lang-drop-inner">
          <strong>拖拽 JSON 文件到此处</strong>
          <span class="meta">或</span>
          <label class="btn btn-secondary" for="lang-file">选择文件</label>
          <span class="meta">{{ fileName }}</span>
        </div>
      </div>
      <div class="field mt-sm">
        <label>或直接粘贴 JSON</label>
        <textarea
          v-model="jsonText"
          class="input mono"
          rows="7"
          :disabled="applying"
          placeholder='{ "/Lotus/Language/...": "译文" }'
        ></textarea>
      </div>
      <button class="btn btn-secondary" :disabled="applying" @click="preview">解析预览</button>

      <div class="mt-sm">
        <div v-if="err" class="field-err">{{ err }}</div>
        <div v-else-if="count && !applying" class="help-note">共解析到 {{ count }} 条词条。</div>
      </div>

      <div v-if="applying" class="field mt-sm">
        <label>上传进度</label>
        <div class="progress"><i :style="{ width: percent() + '%' }"></i></div>
        <div class="progress-label">{{ percent() }}%</div>
      </div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" :disabled="applying" @click="emit('close')">取消</button>
      <button class="btn btn-primary" :disabled="applying || !count" @click="apply">
        <span
          v-if="applying"
          class="inline-block size-4 animate-spin rounded-full border-2 border-current border-t-transparent"
          aria-hidden="true"
        ></span>
        {{ applying ? '上传中…' : '应用覆写' }}
      </button>
    </div>
  </div>
</template>
