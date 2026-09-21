<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  locale: string
  keyName: string
  initial: string
  onSave?: (value: string) => void
}>()
const emit = defineEmits<{ close: [] }>()

const value = ref(props.initial)
const err = ref('')

function save() {
  if (!value.value.trim()) {
    err.value = '译文不能为空。'
    return
  }
  props.onSave?.(value.value)
  emit('close')
}
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head"><h3 class="h3">编辑词条</h3></div>
    <div class="dlg-body">
      <div class="field"><label>语言</label><div class="code-block">{{ props.locale }}</div></div>
      <div class="field">
        <label>键（key）</label>
        <input class="input mono" type="text" :value="props.keyName" disabled />
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
