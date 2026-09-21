<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import DataPager from '@/components/DataPager.vue'
import DataTable from '@/components/DataTable.vue'
import DataTableColumn from '@/components/DataTableColumn.vue'
import { useUiStore } from '@/stores/ui'
import { list_sm, delete_sm, reload, type SortieModifier } from '@/api/deps'
import { get_rewards_page, delete_reward, type Rep } from '@/api/rewrite'
import SomDialog from '@/components/dialogs/SomDialog.vue'
import RepDialog from '@/components/dialogs/RepDialog.vue'

const PAGE_SIZE = 8

const ui = useUiStore()

const tab = ref('som')
const som = ref<SortieModifier[]>([])
const rep = ref<Rep[]>([])

/* ── SOM（本地分页：接口一次性返回全部）── */
const somPage = ref(1)
const somRows = computed(() =>
  som.value.slice((somPage.value - 1) * PAGE_SIZE, somPage.value * PAGE_SIZE)
)

async function fetchSom() {
  try {
    som.value = await list_sm()
    somPage.value = 1
    ui.toast('ok', '已获取', `已获取 ${som.value.length} 条修饰符。`)
  } catch {
    ui.toast('err', '获取失败', '无法获取修饰符列表。')
  }
}

function delSom(item: SortieModifier) {
  ui.askConfirm('删除修饰符', `确定要删除「${item.name}」吗？`, async () => {
    try {
      await delete_sm(item.id)
      await fetchSom()
      ui.toast('ok', '已删除', '修饰符已删除。')
    } catch {
      ui.toast('err', '删除失败', '无法删除该修饰符。')
    }
  })
}

async function reloadDeps() {
  try {
    await reload()
    ui.toast('info', '依赖已重载', 'sortie_modifiers 依赖已重新载入。')
  } catch {
    ui.toast('err', '重载失败', '无法重载依赖。')
  }
}

/* ── 旧数据重写（服务端分页；后端 page 从 0 开始）── */
const repPage = ref(1)
const repTotal = ref(0)

async function fetchReps() {
  try {
    const res = await get_rewards_page(repPage.value - 1, PAGE_SIZE)
    if (res) {
      rep.value = res.items || []
      repTotal.value = res.total || 0
    }
  } catch {
    ui.toast('err', '加载失败', '无法获取重写规则。')
  }
}

function openRepDialog(index: number) {
  const r = rep.value[index]
  if (!r) return
  ui.openDialog(RepDialog, { rep: r, onSaved: fetchReps })
}

function delRep(index: number) {
  const r = rep.value[index]
  if (!r) return
  ui.askConfirm('删除重写规则', `确定要删除「${r.path}」的规则吗？`, async () => {
    try {
      await delete_reward({ path: r.path, origin: r.origin })
      if (rep.value.length === 1 && repPage.value > 1) repPage.value--
      await fetchReps()
      ui.toast('ok', '已删除', '重写规则已删除。')
    } catch {
      ui.toast('err', '删除失败', '无法删除该规则。')
    }
  })
}

function refreshRep() {
  fetchReps()
  ui.toast('info', '已刷新', '重写规则已重新载入。')
}

onMounted(() => {
  fetchSom()
  fetchReps()
})
</script>

<template>
  <section class="view active" id="view-rewrite" data-od-id="rewrite">
    <div class="page-head">
      <p class="eyebrow">管理后台 · 重写数据</p>
      <h1 class="h2">重写数据</h1>
      <p class="lead">修正官方导出中的突击修饰符与历史世界状态字段，改动即时生效。</p>
    </div>

    <div class="tabs" id="rewrite-tabs">
      <button class="tab" :class="{ active: tab === 'som' }" @click="tab = 'som'">SOM 修饰符</button>
      <button class="tab" :class="{ active: tab === 'legacy' }" @click="tab = 'legacy'">旧数据 / 世界状态重写</button>
    </div>

    <div v-show="tab === 'som'" id="tab-som">
      <div class="page-tools">
        <button class="btn btn-secondary" id="som-fetch" @click="fetchSom">获取</button>
        <button class="btn btn-primary" id="som-new" @click="ui.openDialog(SomDialog, { onSaved: fetchSom })">新建</button>
        <button class="btn btn-secondary" id="som-reload" @click="reloadDeps">重载依赖</button>
        <div class="spacer"></div>
        <span class="meta" id="som-count">共 {{ som.length }} 条</span>
      </div>
      <DataTable :rows="somRows" row-key="id" empty-text="暂无修饰符。">
        <DataTableColumn prop="id" label="ID">
          <template #default="{ row }"><span class="mono">{{ row.id }}</span></template>
        </DataTableColumn>
        <DataTableColumn prop="name" label="名称">
          <template #default="{ row }">{{ row.name }}</template>
        </DataTableColumn>
        <DataTableColumn prop="description" label="描述" cell-class="wrap">
          <template #default="{ row }">{{ row.description }}</template>
        </DataTableColumn>
        <DataTableColumn label="参数">
          <template #default="{ row }">
            <span v-for="(v, k) in row.vars" :key="k" class="role-tag">{{ k }}={{ v }}</span>
            <template v-if="!Object.keys(row.vars).length">—</template>
          </template>
        </DataTableColumn>
        <DataTableColumn label="操作">
          <template #default="{ row }">
            <div class="row-actions">
              <button class="icon-btn danger" title="删除" @click="delSom(row)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M4 7h16M9 7V4h6v3M7 7l1 13h8l1-13" /></svg>
              </button>
            </div>
          </template>
        </DataTableColumn>
      </DataTable>
      <DataPager
        :page="somPage"
        :total="som.length"
        :size="PAGE_SIZE"
        @go="(p) => (somPage = p)"
      />
    </div>

    <div v-show="tab === 'legacy'" id="tab-legacy">
      <div class="page-tools">
        <button class="btn btn-secondary" id="rep-refresh" @click="refreshRep">刷新</button>
        <div class="spacer"></div>
        <span class="meta" id="rep-count">共 {{ repTotal }} 条规则</span>
      </div>
      <DataTable :rows="rep" :row-key="(r) => r.path + r.origin">
        <DataTableColumn prop="path" label="路径" cell-class="mono wrap">
          <template #default="{ row }">{{ row.path }}</template>
        </DataTableColumn>
        <DataTableColumn prop="origin" label="原值" cell-class="mono">
          <template #default="{ row }">{{ row.origin || '—' }}</template>
        </DataTableColumn>
        <DataTableColumn prop="replacement" label="替换值" cell-class="mono">
          <template #default="{ row }">{{ row.replacement || '—' }}</template>
        </DataTableColumn>
        <DataTableColumn prop="remark" label="备注" cell-class="wrap">
          <template #default="{ row }">{{ row.remark || '—' }}</template>
        </DataTableColumn>
        <DataTableColumn label="操作">
          <template #default="{ $index }">
            <div class="row-actions">
              <button class="icon-btn" title="编辑" @click="openRepDialog($index)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M4 20h4L19 9l-4-4L4 16v4z" /><path d="M14 6l4 4" /></svg>
              </button>
              <button class="icon-btn danger" title="删除" @click="delRep($index)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M4 7h16M9 7V4h6v3M7 7l1 13h8l1-13" /></svg>
              </button>
            </div>
          </template>
        </DataTableColumn>
      </DataTable>
      <DataPager
        :page="repPage"
        :total="repTotal"
        :size="PAGE_SIZE"
        @go="
          (p) => {
            repPage = p
            fetchReps()
          }
        "
      />
    </div>
  </section>
</template>
