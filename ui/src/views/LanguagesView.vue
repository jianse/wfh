<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import DataPager from '@/components/DataPager.vue'
import ClearableInput from '@/components/ClearableInput.vue'
import DataTable from '@/components/DataTable.vue'
import DataTableColumn from '@/components/DataTableColumn.vue'
import LangEditDialog from '@/components/dialogs/LangEditDialog.vue'
import LangUploadDialog from '@/components/dialogs/LangUploadDialog.vue'
import { useUiStore } from '@/stores/ui'
import {
  list_langs,
  search_langs,
  new_lang_item,
  update_lang_item,
  remove_lang_item,
  upload_langs,
  type LangItem
} from '@/api/langs'

const ui = useUiStore()

const page = ref(1)
const size = ref(50)
const total = ref(0)
const rows = ref<LangItem[]>([])
const loading = ref(false)
const query = ref('')

const emptyText = computed(() => (query.value.trim() ? '没有匹配的词条。' : '暂无词条。'))

async function load() {
  loading.value = true
  const q = query.value.trim()
  try {
    const res = q
      ? await search_langs(q, page.value - 1, size.value)
      : await list_langs(page.value - 1, size.value)
    if (res && res.data) {
      rows.value = res.data.items
      total.value = res.data.total
    }
  } catch {
    ui.toast('err', '加载失败', '无法获取语言词条。')
  } finally {
    loading.value = false
  }
}

let searchTimer: ReturnType<typeof setTimeout> | null = null
function onSearchInput(e?: Event) {
  // 输入法组合输入（如中文拼音）期间不触发搜索
  if (e instanceof InputEvent && e.isComposing) return
  page.value = 1
  if (searchTimer) clearTimeout(searchTimer)
  searchTimer = setTimeout(load, 300)
}

onMounted(load)
onBeforeUnmount(() => {
  if (searchTimer) clearTimeout(searchTimer)
})

function onClearInput() {
  if (searchTimer) clearTimeout(searchTimer)
  page.value = 1
  load()
}

function onSizeChange(s: number) {
  size.value = s
  page.value = 1
  load()
}

async function refresh() {
  await load()
  ui.toast('info', '已刷新', '语言词条已重新载入。')
}

function openEdit(item: LangItem) {
  ui.openDialog(LangEditDialog, {
    uniqueName: item.unique_name,
    initial: item.value,
    mode: 'edit',
    onSave: async (it: LangItem) => {
      await update_lang_item(it)
      ui.toast('ok', '已保存', `「${it.unique_name}」已更新。`)
      await load()
    }
  })
}

function openCreate() {
  ui.openDialog(LangEditDialog, {
    mode: 'create',
    onSave: async (it: LangItem) => {
      await new_lang_item(it)
      ui.toast('ok', '已创建', `「${it.unique_name}」已新增。`)
      page.value = 1
      await load()
    }
  })
}

function del(item: LangItem) {
  ui.askConfirm('删除词条', `确定要删除「${item.unique_name}」吗？`, async () => {
    try {
      await remove_lang_item(item.unique_name)
      ui.toast('ok', '已删除', `「${item.unique_name}」已删除。`)
      if (rows.value.length === 1 && page.value > 1) page.value--
      await load()
    } catch {
      ui.toast('err', '删除失败', '无法删除该词条。')
    }
  })
}

function openUpload() {
  ui.openDialog(LangUploadDialog, {
    onApply: async (data: Record<string, string>, onProgress: (pct: number) => void) => {
      const file = new File([JSON.stringify(data)], 'langs.json', { type: 'application/json' })
      await upload_langs(file, onProgress)
      ui.toast('ok', '覆写完成', `已写入 ${Object.keys(data).length} 条词条。`)
      page.value = 1
      await load()
    }
  })
}
</script>

<template>
  <section class="view active" id="view-languages" data-od-id="languages">
    <div class="page-head">
      <p class="eyebrow">管理后台 · 语言管理</p>
      <h1 class="h2">语言管理</h1>
      <p class="lead">维护游戏语言词条（当前仅支持简体中文）。支持分页浏览、单条新增/编辑/删除，或上传 JSON 整体覆写。</p>
    </div>

    <div class="page-tools">
      <ClearableInput
        v-model="query"
        id="lang-search"
        placeholder="搜索键名或译文…"
        @input="onSearchInput"
        @clear="onClearInput"
      />
      <button class="btn btn-secondary" id="lang-refresh" :disabled="loading" @click="refresh">刷新</button>
      <button class="btn btn-primary" id="lang-create" :disabled="loading" @click="openCreate">新增词条</button>
      <button class="btn btn-secondary" id="lang-upload" :disabled="loading" @click="openUpload">
        上传 JSON 覆写
      </button>
      <div class="spacer"></div>
      <span class="meta" id="lang-total">共 {{ total }} 条 · 简体中文</span>
    </div>

    <DataTable
      :rows="rows"
      row-key="unique_name"
      :loading="loading"
      :empty-text="emptyText"
      loading-text="正在从后台加载语言词条…"
    >
      <DataTableColumn prop="unique_name" label="键（key）" width="45%" cell-class="lang-key">
        <template #default="{ row }">{{ row.unique_name }}</template>
      </DataTableColumn>
      <DataTableColumn prop="value" label="译文（value）">
        <template #default="{ row }">
          <span class="lang-val">{{ row.value }}</span>
        </template>
      </DataTableColumn>
      <DataTableColumn label="操作" width="160px">
        <template #default="{ row }">
          <button class="btn btn-ghost" @click="openEdit(row)">编辑</button>
          <button class="btn btn-ghost" @click="del(row)">删除</button>
        </template>
      </DataTableColumn>
    </DataTable>

    <DataPager
      :page="page"
      :total="total"
      :size="size"
      :page-sizes="[50, 100, 200]"
      :disabled="loading"
      @size-change="onSizeChange"
      @go="
        (p) => {
          page = p
          load()
        }
      "
    />
  </section>
</template>
