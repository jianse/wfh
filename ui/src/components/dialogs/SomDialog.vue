<script setup lang="ts">
import { ref } from 'vue'
import { useUiStore } from '@/stores/ui'
import { new_sm } from '@/api/deps'

const props = defineProps<{ onSaved?: () => void }>()
const emit = defineEmits<{ close: [] }>()
const ui = useUiStore()

interface VarRow {
  k: string
  v: string
}

const form = ref<{ id: string; name: string; description: string; vars: VarRow[] }>({
  id: '',
  name: '',
  description: '',
  vars: []
})
const err = ref('')

function addVarRow() {
  form.value.vars.push({ k: '', v: '' })
}

async function save() {
  err.value = ''
  const id = form.value.id.trim()
  const name = form.value.name.trim()
  const description = form.value.description.trim()
  const vars: Record<string, string> = {}
  form.value.vars.forEach(({ k, v }) => {
    const key = k.trim()
    if (key) vars[key] = v.trim()
  })
  if (!id) {
    err.value = '请输入 ID'
    return
  }
  if (!name) {
    err.value = '请输入名称'
    return
  }
  try {
    await new_sm({ id, name, description, vars })
    ui.toast('ok', '已创建', `修饰符 ${id} 已创建。`)
    props.onSaved?.()
    emit('close')
  } catch (e) {
    const er = e as { response?: { data?: { msg?: string } } }
    err.value = '保存失败：' + (er?.response?.data?.msg ?? String(e))
  }
}
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head"><h3 class="h3" id="som-dlg-title">新建修饰符</h3></div>
    <div class="dlg-body">
      <div class="form-grid">
        <div class="field">
          <label for="som-id">ID</label>
          <input v-model="form.id" class="input" id="som-id" type="text" placeholder="例如 SORTIE_MODIFIER" />
        </div>
        <div class="field">
          <label for="som-name">名称</label>
          <input v-model="form.name" class="input" id="som-name" type="text" placeholder="修饰符名称" />
        </div>
      </div>
      <div class="field">
        <label for="som-desc">描述</label>
        <textarea v-model="form.description" class="input" id="som-desc" rows="2" placeholder="描述"></textarea>
      </div>
      <div class="field">
        <label>参数</label>
        <div id="som-vars">
          <div v-for="(row, i) in form.vars" :key="i" class="var-row">
            <input v-model="row.k" class="input var-k" type="text" placeholder="参数名" />
            <input v-model="row.v" class="input var-v" type="text" placeholder="参数值" />
            <button class="icon-btn danger var-del" title="移除" @click="form.vars.splice(i, 1)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M4 7h16M9 7V4h6v3M7 7l1 13h8l1-13" /></svg>
            </button>
          </div>
        </div>
        <button class="btn btn-ghost mt-xs" id="som-add-var" @click="addVarRow">添加参数</button>
      </div>
      <div class="field-err" id="som-err">{{ err }}</div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="emit('close')">取消</button>
      <button class="btn btn-primary" id="som-save" @click="save">保存</button>
    </div>
  </div>
</template>
