<script setup lang="ts">
import { ref } from 'vue'
import { useUiStore } from '@/stores/ui'
import { reset_password, type User } from '@/api/user'

const props = defineProps<{ user: User; onSaved?: () => void }>()
const emit = defineEmits<{ close: [] }>()

const ui = useUiStore()
const password = ref('')
const err = ref('')

async function save() {
  err.value = ''
  if (!password.value) {
    err.value = '请输入新密码'
    return
  }
  try {
    const res = await reset_password(props.user.id, { password: password.value })
    if (res && res.code && res.code !== 20000) {
      err.value = res.msg || '重置失败'
      return
    }
    ui.toast('ok', '密码已重置', `用户 ${props.user.username} 的密码已更新。`)
    props.onSaved?.()
    emit('close')
  } catch {
    err.value = '网络错误，请稍后重试'
  }
}
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head"><h3 class="h3">重置密码</h3></div>
    <div class="dlg-body">
      <p class="help-note" id="reset-target">为「{{ props.user.username }}」设置新的登录密码。</p>
      <div class="field">
        <label for="reset-pass">新密码</label>
        <input v-model="password" class="input" id="reset-pass" type="password" placeholder="输入新密码" />
      </div>
      <div class="field-err" id="reset-err">{{ err }}</div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="emit('close')">取消</button>
      <button class="btn btn-primary" id="reset-save" @click="save">保存</button>
    </div>
  </div>
</template>
