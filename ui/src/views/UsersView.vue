<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import DataPager from '@/components/DataPager.vue'
import DataTable from '@/components/DataTable.vue'
import DataTableColumn from '@/components/DataTableColumn.vue'
import { useAuthStore } from '@/stores/auth'
import { useUiStore } from '@/stores/ui'
import { roleLabel } from '@/lib/utils'
import { get_user_list, delete_user, type User } from '@/api/user'
import UserFormDialog from '@/components/dialogs/UserFormDialog.vue'
import ResetPasswordDialog from '@/components/dialogs/ResetPasswordDialog.vue'

const PAGE_SIZE = 8

const auth = useAuthStore()
const ui = useUiStore()

const page = ref(1)
const users = ref<User[]>([])
const total = ref(0)
const loading = ref(false)

const stats = computed(() => ({
  total: total.value,
  admins: users.value.filter((u) => u.role === 'admin').length,
  maint: users.value.filter((u) => u.role === 'maintainer').length,
  normal: users.value.filter((u) => u.role === 'normal').length
}))

const rows = computed(() => users.value)

async function loadUsers() {
  loading.value = true
  try {
    const res = await get_user_list(page.value, PAGE_SIZE)
    if (res && res.data) {
      users.value = res.data.items
      total.value = res.data.total
    }
  } catch {
    ui.toast('err', '加载失败', '无法获取用户列表。')
  } finally {
    loading.value = false
  }
}

onMounted(loadUsers)

function openUserDialog(user: User | null) {
  ui.openDialog(UserFormDialog, { user, onSaved: loadUsers })
}

function openResetDialog(user: User) {
  ui.openDialog(ResetPasswordDialog, { user, onSaved: loadUsers })
}

function delUser(user: User) {
  if (auth.currentUser && user.id === auth.currentUser.id) {
    ui.toast('err', '无法删除', '不能删除当前登录的账号。')
    return
  }
  ui.askConfirm('删除用户', `确定要删除用户「${user.username}」吗？此操作不可撤销。`, async () => {
    try {
      await delete_user(user.id)
      ui.toast('ok', '已删除', `用户 ${user.username} 已删除。`)
      if (rows.value.length === 1 && page.value > 1) page.value--
      await loadUsers()
    } catch {
      ui.toast('err', '删除失败', '无法删除该用户。')
    }
  })
}

function refresh() {
  loadUsers()
  ui.toast('info', '已刷新', '用户列表已重新载入。')
}
</script>

<template>
  <section class="view active" id="view-users" data-od-id="users">
    <div class="page-head">
      <p class="eyebrow">管理后台 · 用户管理</p>
      <h1 class="h2">用户管理</h1>
      <p class="lead">仅管理员可访问：新增、编辑、重置密码与删除账号。</p>
    </div>

    <div class="stat-grid" id="users-stats">
      <div class="stat"><div class="stat-num">{{ stats.total }}</div><div class="stat-label">账号总数</div></div>
      <div class="stat"><div class="stat-num ok">{{ stats.admins }}</div><div class="stat-label">管理员</div></div>
      <div class="stat"><div class="stat-num">{{ stats.maint }}</div><div class="stat-label">维护者</div></div>
      <div class="stat"><div class="stat-num">{{ stats.normal }}</div><div class="stat-label">普通用户</div></div>
    </div>

    <div class="page-tools">
      <button class="btn btn-secondary" id="users-refresh" @click="refresh">刷新</button>
      <button class="btn btn-primary" id="users-create" @click="openUserDialog(null)">新增用户</button>
      <div class="spacer"></div>
      <span class="meta" id="users-total">共 {{ total }} 个账号</span>
    </div>

    <DataTable :rows="rows" row-key="id" :loading="loading" empty-text="暂无用户。">
      <DataTableColumn prop="id" label="ID">
        <template #default="{ row }"><span class="mono">#{{ row.id }}</span></template>
      </DataTableColumn>
      <DataTableColumn prop="username" label="用户名">
        <template #default="{ row }">
          {{ row.username
          }}<span
            v-if="auth.currentUser && auth.currentUser.id === row.id"
            class="cell-sub"
            >当前登录账号</span
          >
        </template>
      </DataTableColumn>
      <DataTableColumn prop="role" label="角色">
        <template #default="{ row }">
          <span class="role-tag" :class="`role-${row.role}`">{{ roleLabel(row.role) }}</span>
        </template>
      </DataTableColumn>
      <DataTableColumn label="操作">
        <template #default="{ row }">
          <div class="row-actions">
            <button class="icon-btn" title="编辑" @click="openUserDialog(row)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M4 20h4L19 9l-4-4L4 16v4z" /><path d="M14 6l4 4" /></svg>
            </button>
            <button class="icon-btn" title="重置密码" @click="openResetDialog(row)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><circle cx="8" cy="15" r="4" /><path d="M11 12l9-9M18 5l2 2M16 7l2 2" /></svg>
            </button>
            <button class="icon-btn danger" title="删除" @click="delUser(row)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M4 7h16M9 7V4h6v3M7 7l1 13h8l1-13" /></svg>
            </button>
          </div>
        </template>
      </DataTableColumn>
    </DataTable>
    <DataPager
      :page="page"
      :total="total"
      :size="PAGE_SIZE"
      :disabled="loading"
      @go="(p) => { page = p; loadUsers() }"
    />
  </section>
</template>
