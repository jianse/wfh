<template>
  <div class="p-4">
    <div class="flex justify-between items-center mb-4">
      <h1 class="text-2xl font-bold">用户管理</h1>
      <div class="flex gap-2">
        <pv-button @click="loadUsers" label="刷新" icon="pi pi-refresh" />
        <pv-button
          @click="openCreateDialog"
          label="新增用户"
          icon="pi pi-plus"
          severity="primary"
        />
      </div>
    </div>

    <!-- 统计信息 -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
      <pv-panel header="总用户数">
        <div class="text-2xl font-bold">{{ total }}</div>
      </pv-panel>
      <pv-panel header="管理员">
        <div class="text-2xl font-bold text-blue-600">{{ adminCount }}</div>
      </pv-panel>
      <pv-panel header="维护者">
        <div class="text-2xl font-bold text-purple-600">{{ maintainerCount }}</div>
      </pv-panel>
      <pv-panel header="普通用户">
        <div class="text-2xl font-bold text-green-600">{{ normalCount }}</div>
      </pv-panel>
    </div>

    <!-- 数据表格 -->
    <pv-data-table
      :value="users"
      scrollable
      scrollHeight="calc(100vh - 330px)"
      paginator
      lazy
      :rows="pageSize"
      :totalRecords="total"
      :rowsPerPageOptions="[10, 20, 50, 100]"
      paginatorTemplate="CurrentPageReport FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown JumpToPageInput"
      currentPageReportTemplate="{currentPage}/{totalPages} 共 {totalRecords} 条"
      @page="onPageChange"
      :loading="loading"
    >
      <pv-column field="id" header="ID" style="width: 80px"></pv-column>
      <pv-column field="username" header="用户名"></pv-column>
      <pv-column field="role" header="角色" style="width: 120px">
        <template #body="slotProps">
          <pv-tag
            :severity="
              slotProps.data.role === 'admin'
                ? 'primary'
                : slotProps.data.role === 'maintainer'
                  ? 'warning'
                  : 'info'
            "
            :value="
              slotProps.data.role === 'admin'
                ? '管理员'
                : slotProps.data.role === 'maintainer'
                  ? '维护者'
                  : '普通用户'
            "
          />
        </template>
      </pv-column>
      <pv-column header="操作" :exportable="false" style="min-width: 200px">
        <template #body="slotProps">
          <pv-button
            icon="pi pi-pencil"
            size="small"
            variant="outlined"
            rounded
            class="mr-2"
            @click="openEditDialog(slotProps.data)"
          />
          <pv-button
            icon="pi pi-refresh"
            size="small"
            variant="outlined"
            rounded
            class="mr-2"
            severity="warning"
            @click="openResetPasswordDialog(slotProps.data)"
          />
          <pv-button
            icon="pi pi-trash"
            size="small"
            variant="outlined"
            rounded
            severity="danger"
            @click="confirmDelete(slotProps.data)"
          />
        </template>
      </pv-column>
    </pv-data-table>

    <!-- 新增用户对话框 -->
    <pv-dialog
      v-model:visible="dialogVisible.create"
      :style="{ width: '500px' }"
      header="新增用户"
      :modal="true"
    >
      <div class="flex flex-col gap-4">
        <div>
          <label class="block font-bold mb-2">用户名</label>
          <pv-input-text v-model="formData.username" class="w-full" />
        </div>
        <div>
          <label class="block font-bold mb-2">密码</label>
          <pv-input-text v-model="formData.password" type="password" class="w-full" />
        </div>
        <div>
          <label class="block font-bold mb-2">角色</label>
          <pv-select
            v-model="formData.role"
            :options="roleOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
          />
        </div>
      </div>
      <template #footer>
        <div class="flex justify-end gap-2">
          <pv-button @click="dialogVisible.create = false" label="取消" />
          <pv-button @click="handleCreate" label="确定" severity="primary" />
        </div>
      </template>
    </pv-dialog>

    <!-- 编辑用户对话框 -->
    <pv-dialog
      v-model:visible="dialogVisible.edit"
      :style="{ width: '500px' }"
      header="编辑用户"
      :modal="true"
    >
      <div class="flex flex-col gap-4">
        <div>
          <label class="block font-bold mb-2">用户名</label>
          <pv-input-text v-model="formData.username" class="w-full" />
        </div>
        <div>
          <label class="block font-bold mb-2">角色</label>
          <pv-select
            v-model="formData.role"
            :options="roleOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
          />
        </div>
      </div>
      <template #footer>
        <div class="flex justify-end gap-2">
          <pv-button @click="dialogVisible.edit = false" label="取消" />
          <pv-button @click="handleEdit" label="确定" severity="primary" />
        </div>
      </template>
    </pv-dialog>

    <!-- 重置密码对话框 -->
    <pv-dialog
      v-model:visible="dialogVisible.resetPassword"
      :style="{ width: '500px' }"
      header="重置密码"
      :modal="true"
    >
      <div class="flex flex-col gap-4">
        <div>
          <label class="block font-bold mb-2">新密码</label>
          <pv-input-text v-model="resetPasswordForm.password" type="password" class="w-full" />
        </div>
      </div>
      <template #footer>
        <div class="flex justify-end gap-2">
          <pv-button @click="dialogVisible.resetPassword = false" label="取消" />
          <pv-button @click="handleResetPassword" label="确定" severity="primary" />
        </div>
      </template>
    </pv-dialog>

    <!-- 删除确认对话框 -->
    <pv-dialog
      v-model:visible="dialogVisible.delete"
      :style="{ width: '450px' }"
      header="确定删除吗？"
      :modal="true"
    >
      <div class="flex items-center gap-4">
        <i class="pi pi-exclamation-triangle text-3xl text-orange-500" />
        <span v-if="selectedUser"
          >您确定要删除
          <b
            ><code>{{ selectedUser.username }}</code>
          </b>
          ?</span
        >
      </div>
      <template #footer>
        <div class="flex justify-end gap-2">
          <pv-button
            label="取消"
            icon="pi pi-times"
            @click="dialogVisible.delete = false"
            severity="secondary"
            variant="outlined"
          />
          <pv-button label="确定" icon="pi pi-check" @click="handleDelete" severity="danger" />
        </div>
      </template>
    </pv-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive, computed } from 'vue'
import { useToast } from 'primevue/usetoast'
import { type PageState } from 'primevue'
import * as userApi from '@/api/user'
import type { User, CreateUserRequest, UpdateUserRequest, ResetPasswordRequest } from '@/api/user'

const toast = useToast()

// 分页数据
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)
const users = ref<User[]>([])
const loading = ref(false)

// 角色选项
const roleOptions = [
  { label: '普通用户', value: 'normal' },
  { label: '维护者', value: 'maintainer' },
  { label: '管理员', value: 'admin' },
]

// 统计数据
const adminCount = computed(() => users.value.filter((u) => u.role === 'admin').length)
const maintainerCount = computed(() => users.value.filter((u) => u.role === 'maintainer').length)
const normalCount = computed(() => users.value.filter((u) => u.role === 'normal').length)

// 对话框状态
const dialogVisible = reactive({
  create: false,
  edit: false,
  resetPassword: false,
  delete: false,
})

// 选中的用户（用于删除确认）
const selectedUser = ref<User | null>(null)

// 表单数据
const formData = reactive({
  id: 0,
  username: '',
  password: '',
  role: 'normal' as 'normal' | 'maintainer' | 'admin',
})

// 重置密码表单
const resetPasswordForm = reactive({
  password: '',
})

// 加载用户列表
const loadUsers = async () => {
  loading.value = true
  try {
    const res = await userApi.get_user_list(page.value, pageSize.value)
    if (res.code === 20000 && res.data) {
      users.value = res.data.items
      total.value = res.data.total
    } else {
      toast.add({
        severity: 'error',
        summary: '错误',
        detail: '获取用户列表失败',
        life: 3000,
      })
    }
  } catch (error) {
    toast.add({
      severity: 'error',
      summary: '错误',
      detail: '网络错误:' + error,
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

// 分页处理
const onPageChange = (event: PageState) => {
  page.value = event.page + 1
  pageSize.value = event.rows
  loadUsers()
}

// 打开新增用户对话框
const openCreateDialog = () => {
  Object.assign(formData, {
    id: 0,
    username: '',
    password: '',
    role: 'normal',
  })
  dialogVisible.create = true
}

// 打开编辑用户对话框
const openEditDialog = (row: User) => {
  Object.assign(formData, {
    id: row.id,
    username: row.username,
    role: row.role,
  })
  dialogVisible.edit = true
}

// 打开重置密码对话框
const openResetPasswordDialog = (row: User) => {
  formData.id = row.id
  resetPasswordForm.password = ''
  dialogVisible.resetPassword = true
}

// 处理新增用户
const handleCreate = async () => {
  try {
    const req: CreateUserRequest = {
      username: formData.username,
      password: formData.password,
      role: formData.role,
    }
    const res = await userApi.create_user(req)
    if (res.code === 20000) {
      toast.add({
        severity: 'success',
        summary: '成功',
        detail: '用户创建成功',
        life: 3000,
      })
      dialogVisible.create = false
      loadUsers()
    } else {
      toast.add({
        severity: 'error',
        summary: '错误',
        detail: '用户创建失败',
        life: 3000,
      })
    }
  } catch (error) {
    toast.add({
      severity: 'error',
      summary: '错误',
      detail: '网络错误:' + error,
      life: 3000,
    })
  }
}

// 处理编辑用户
const handleEdit = async () => {
  try {
    const req: UpdateUserRequest = {
      username: formData.username,
      role: formData.role,
    }
    const res = await userApi.update_user(formData.id, req)
    if (res.code === 20000) {
      toast.add({
        severity: 'success',
        summary: '成功',
        detail: '用户更新成功',
        life: 3000,
      })
      dialogVisible.edit = false
      loadUsers()
    } else {
      toast.add({
        severity: 'error',
        summary: '错误',
        detail: '用户更新失败',
        life: 3000,
      })
    }
  } catch (error) {
    toast.add({
      severity: 'error',
      summary: '错误',
      detail: '网络错误:' + error,
      life: 3000,
    })
  }
}

// 处理重置密码
const handleResetPassword = async () => {
  try {
    const req: ResetPasswordRequest = {
      password: resetPasswordForm.password,
    }
    const res = await userApi.reset_password(formData.id, req)
    if (res.code === 20000) {
      toast.add({
        severity: 'success',
        summary: '成功',
        detail: '密码重置成功',
        life: 3000,
      })
      dialogVisible.resetPassword = false
    } else {
      toast.add({
        severity: 'error',
        summary: '错误',
        detail: '密码重置失败',
        life: 3000,
      })
    }
  } catch (error) {
    toast.add({
      severity: 'error',
      summary: '错误',
      detail: '网络错误:' + error,
      life: 3000,
    })
  }
}

// 确认删除
const confirmDelete = (row: User) => {
  selectedUser.value = row
  dialogVisible.delete = true
}

// 处理删除
const handleDelete = async () => {
  if (!selectedUser.value) return
  try {
    const res = await userApi.delete_user(selectedUser.value.id)
    if (res.code === 20000) {
      toast.add({
        severity: 'success',
        summary: '成功',
        detail: '用户删除成功',
        life: 3000,
      })
      loadUsers()
      dialogVisible.delete = false
      selectedUser.value = null
    } else {
      toast.add({
        severity: 'error',
        summary: '错误',
        detail: '用户删除失败',
        life: 3000,
      })
    }
  } catch (error) {
    toast.add({
      severity: 'error',
      summary: '错误',
      detail: '网络错误' + error,
      life: 3000,
    })
  }
}

// 页面加载时获取用户列表
onMounted(() => {
  loadUsers()
})
</script>
