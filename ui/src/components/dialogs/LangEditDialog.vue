<script setup lang="ts">
import { ref } from 'vue'
import type { LangItem } from '@/api/langs'

const props = withDefaults(
  defineProps<{
    uniqueName?: string
    initial?: string
    mode?: 'edit' | 'create'
    onSave?: (item: LangItem) => void
  }>(),
  { uniqueName: '', initial: '', mode: 'edit' }
)
const emit = defineEmits<{ close: [] }>()

const name = ref(props.uniqueName)
const value = ref(props.initial)
const err = ref('')
const isCreate = props.mode === 'create'

function save() {
  err.value = ''
  if (!name.value.trim()) {
    err.value = '请输入键（key）'
    return
  }
  if (!value.value.trim()) {
    err.value = '译文不能为空。'
    return
  }
  props.onSave?.({ unique_name: name.value.trim(), value: value.value })
  emit('close')
}
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head"><h3 class="h3">{{ isCreate ? '新增词条' : '编辑词条' }}</h3></div>
    <div class="dlg-body">
      <div class="field"><label>语言</label><div class="code-block">简体中文</div></div>
      <div class="field">
        <label>键（key）</label>
        <input
          v-model="name"
          class="input mono"
          type="text"
          :disabled="!isCreate"
          placeholder="例如 /Lotus/Language/..."
        />
      </div>
      <div class="field">
        <label>译文（value）</label>
        <textarea v-model="value" class="input" rows="4" placeholder="请输入该键对应的译文"></textarea>
      </div>
      <div class="field-err">{{ err }}</div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="emit('close')">取消</button>
      <button class="btn btn-primary" @click="save">保存</button>
    </div>
  </div>
</template>
