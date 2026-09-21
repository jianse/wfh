<script setup lang="ts">
import { ref } from 'vue'
import { useUiStore } from '@/stores/ui'
import { edit_reward, type Rep } from '@/api/rewrite'

const props = defineProps<{ rep: Rep; onSaved?: () => void }>()
const emit = defineEmits<{ close: [] }>()
const ui = useUiStore()

const form = ref<Rep>({
  path: props.rep.path,
  origin: props.rep.origin,
  replacement: props.rep.replacement,
  remark: props.rep.remark
})
const err = ref('')

async function save() {
  err.value = ''
  const path = form.value.path.trim()
  const replacement = form.value.replacement.trim()
  if (!path) {
    err.value = '请输入路径'
    return
  }
  if (!replacement) {
    err.value = '请输入替换值'
    return
  }
  try {
    await edit_reward({
      path,
      origin: form.value.origin.trim(),
      replacement,
      remark: form.value.remark.trim()
    })
    ui.toast('ok', '已保存', '重写规则已更新。')
    props.onSaved?.()
    emit('close')
  } catch {
    err.value = '保存失败，仅支持修改已有规则'
  }
}
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head"><h3 class="h3" id="rep-dlg-title">编辑重写规则</h3></div>
    <div class="dlg-body">
      <div class="field">
        <label for="rep-path">路径</label>
        <input v-model="form.path" class="input mono" id="rep-path" type="text" placeholder="例如 WorldState.Sortie" disabled />
      </div>
      <div class="form-grid">
        <div class="field">
          <label for="rep-origin">原值</label>
          <input v-model="form.origin" class="input mono" id="rep-origin" type="text" placeholder="（可留空）" disabled />
        </div>
        <div class="field">
          <label for="rep-replacement">替换值</label>
          <input v-model="form.replacement" class="input mono" id="rep-replacement" type="text" placeholder="替换后的值" />
        </div>
      </div>
      <div class="field">
        <label for="rep-remark">备注</label>
        <input v-model="form.remark" class="input" id="rep-remark" type="text" placeholder="备注原因" />
      </div>
      <div class="field-err" id="rep-err">{{ err }}</div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="emit('close')">取消</button>
      <button class="btn btn-primary" id="rep-save" @click="save">保存</button>
    </div>
  </div>
</template>
