<template>
  <div class="p-3 flex flex-col">
    <pv-data-table
      class="shrink"
      :value="list"
      :totalRecords="pager.total"
      :rowsPerPageOptions="[10, 20, 50, 100]"
      tableStyle="min-width: 50rem"
      :loading="tableLoading"
      :data-key="(it) => it.path + it.origin"
      striped-rows
      scrollable
      scrollHeight="calc(100vh - 14rem)"
    >
      <template #header>
        <div class="flex flex-wrap items-center justify-between gap-2">
          <span class="text-xl font-bold">世界状态重写</span>
          <pv-button icon="pi pi-refresh" rounded raised @click="fetchWorldStateRewrite" />
        </div>
      </template>
      <template #empty>
        <div class="flex place-content-around">No records found.</div>
      </template>
      <pv-column field="path" header="路径" style="width: 25%"></pv-column>
      <pv-column field="origin" header="原值" style="width: 25%"></pv-column>
      <pv-column field="replacement" header="替换值" style="width: 25%"></pv-column>
      <pv-column field="remark" header="备注" style="width: 25%"></pv-column>
      <pv-column :exportable="false" style="min-width: 12rem">
        <template #body="slotProps">
          <pv-button
            icon="pi pi-pencil"
            size="small"
            variant="outlined"
            rounded
            class="mr-2"
            @click="editReplacement(slotProps.data)"
          />
          <pv-button
            icon="pi pi-trash"
            size="small"
            variant="outlined"
            rounded
            severity="danger"
            @click="confirmDeleteReplacement(slotProps.data)"
          />
        </template>
      </pv-column>
    </pv-data-table>
    <pv-paginator
      class="mt-2"
      template="CurrentPageReport FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown JumpToPageInput"
      :rows="pager.page_size"
      :totalRecords="pager.total"
      :rowsPerPageOptions="[10, 20, 50, 100]"
      @page="onPage"
      currentPageReportTemplate="{currentPage}/{totalPages} 共 {totalRecords} 条"
    ></pv-paginator>

    <pv-dialog
      v-model:visible="replacementDialog"
      :style="{ width: '450px' }"
      header="世界状态重写详情"
      :modal="true"
    >
      <div class="flex flex-col gap-6">
        <div>
          <label for="path" class="block font-bold mb-3">路径</label>
          <pv-input-text
            id="path"
            v-model="replacement.path"
            required="true"
            rows="3"
            cols="20"
            fluid
            placeholder="请输入路径"
            :disabled="isEdit"
          />
        </div>
        <div>
          <label for="origin" class="block font-bold mb-3">原值</label>
          <pv-input-text
            id="origin"
            v-model="replacement.origin"
            optionLabel="label"
            placeholder="请输入原值"
            fluid
            :disabled="isEdit"
          />
        </div>

        <div>
          <label for="replacement" class="block font-bold mb-3">替换值</label>
          <pv-input-text
            id="replacement"
            v-model="replacement.replacement"
            optionLabel="label"
            placeholder="请输入替换值"
            fluid
          />
        </div>
        <div>
          <label for="remark" class="block font-bold mb-3">备注</label>
          <pv-textarea
            id="remark"
            v-model="replacement.remark"
            required="true"
            rows="3"
            cols="20"
            placeholder="请输入备注"
            fluid
          />
        </div>
      </div>

      <template #footer>
        <pv-button label="取消" icon="pi pi-times" outlined @click="replacementDialog = false" />
        <pv-button label="保存" icon="pi pi-check" @click="saveReplacement" />
      </template>
    </pv-dialog>

    <pv-dialog
      v-model:visible="deleteProductDialog"
      :style="{ width: '450px' }"
      header="确定删除吗？"
      :modal="true"
    >
      <div class="flex items-center gap-4">
        <i class="pi pi-exclamation-triangle text-3xl!" />
        <span v-if="replacement"
          >您确定要删除
          <b
            ><code>{{ replacement.path }}</code> "{{ replacement.origin }}"</b
          >?</span
        >
      </div>
      <template #footer>
        <pv-button
          label="取消"
          icon="pi pi-times"
          @click="deleteProductDialog = false"
          severity="secondary"
          variant="outlined"
        />
        <pv-button label="确定" icon="pi pi-check" @click="deleteReplacement" severity="danger" />
      </template>
    </pv-dialog>
  </div>
</template>
<script setup lang="ts">
import { delete_reward, edit_reward, get_rewards_page, type Rep } from '@/api/rewrite'
import { cloneDeep } from 'lodash'
import { useToast, type PageState } from 'primevue'
import { ref } from 'vue'

const list = ref<Rep[]>([])
const pager = ref({
  page: 0,
  page_size: 10,
  total: 0,
})

const tableLoading = ref(false)
const toast = useToast()

const replacementDialog = ref(false)
const isEdit = ref(false)

const replacement = ref<Rep>({
  path: '',
  origin: '',
  replacement: '',
  remark: '',
})

async function fetchWorldStateRewrite() {
  try {
    tableLoading.value = true
    const res = await get_rewards_page(pager.value.page, pager.value.page_size)
    if (res) {
      pager.value.total = res.total
      list.value = res.items
    }
  } finally {
    tableLoading.value = false
  }
}

fetchWorldStateRewrite()

async function onPage(ev: PageState) {
  pager.value.page = ev.page
  pager.value.page_size = ev.rows
  fetchWorldStateRewrite()
}

function editReplacement(rep: Rep) {
  isEdit.value = true
  replacement.value = cloneDeep(rep)
  replacementDialog.value = true
}
// const replacement = ref<Rep | null>(null)
const deleteProductDialog = ref(false)
function confirmDeleteReplacement(rep: Rep) {
  replacement.value = rep
  deleteProductDialog.value = true
}

async function deleteReplacement() {
  await delete_reward({ path: replacement.value.path, origin: replacement.value.origin })
  toast.add({ severity: 'success', summary: '操作成功', detail: '删除成功！', life: 3000 })
  deleteProductDialog.value = false
  fetchWorldStateRewrite()
}

async function saveReplacement() {
  await edit_reward(replacement.value)
  toast.add({ severity: 'success', summary: '操作成功', detail: '修改成功！', life: 3000 })
  replacementDialog.value = false
  fetchWorldStateRewrite()
}
</script>
