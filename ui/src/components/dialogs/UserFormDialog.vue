<script setup lang="ts">
import { ref } from 'vue'
import { useUiStore } from '@/stores/ui'
import { create_user, update_user, type User } from '@/api/user'

type Role = 'normal' | 'maintainer' | 'admin'

const props = defineProps<{ user: User | null; onSaved?: () => void }>()
const emit = defineEmits<{ close: [] }>()

const ui = useUiStore()
const editing = props.user
const form = ref<{ username: string; password: string; role: Role }>({
  username: editing ? editing.username : '',
  password: '',
  role: editing ? (editing.role as Role) : 'normal'
})
const formErr = ref('')

async function save() {
  formErr.value = ''
  try {
    if (editing) {
      const res = await update_user(editing.id, {
        username: form.value.username,
        role: form.value.role
      })
      if (!res || (res.code && res.code !== 20000)) {
        formErr.value = (res && res.msg) || '保存失败'
        return
      }
      ui.toast('ok', '已保存', `用户 ${form.value.username} 的信息已更新。`)
    } else {
      if (!form.value.username.trim()) {
        formErr.value = '请输入用户名'
        return
      }
      if (!form.value.password) {
        formErr.value = '请输入密码'
        return
      }
      const res = await create_user({
        username: form.value.username,
        password: form.value.password,
        role: form.value.role
      })
      if (!res || (res.code && res.code !== 20000)) {
        formErr.value = (res && res.msg) || '创建失败'
        return
      }
      ui.toast('ok', '已创建', `用户 ${form.value.username} 已创建。`)
    }
    props.onSaved?.()
    emit('close')
  } catch {
    formErr.value = '网络错误，请稍后重试'
  }
}
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head">
      <h3 class="h3" id="user-dlg-title">{{ editing ? '编辑用户' : '新增用户' }}</h3>
    </div>
    <div class="dlg-body">
      <div class="field">
        <label for="user-form-name">用户名</label>
        <input v-model="form.username" class="input" id="user-form-name" type="text" placeholder="输入用户名" />
      </div>
      <div v-if="!editing" class="field" id="user-pass-field">
        <label for="user-form-pass">密码</label>
        <input v-model="form.password" class="input" id="user-form-pass" type="password" placeholder="设置初始密码" />
      </div>
      <div class="field">
        <label for="user-form-role">角色</label>
        <select v-model="form.role" class="input" id="user-form-role">
          <option value="normal">普通用户</option>
          <option value="maintainer">维护者</option>
          <option value="admin">管理员</option>
        </select>
      </div>
      <div class="field-err" id="user-form-err">{{ formErr }}</div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="emit('close')">取消</button>
      <button class="btn btn-primary" id="user-form-save" @click="save">保存</button>
    </div>
  </div>
</template>
