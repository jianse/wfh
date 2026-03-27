<template>
  <div class="flex gap-2">
    <pv-button @click="fetch" label="获取" />
    <pv-button @click="openNewDialog" label="新建" />
    <pv-button @click="reloadDeps" label="重载依赖" :loading="reloading" />
  </div>
  <pv-data-table :value="data" scrollable scrollHeight="calc(100vh - 14rem)">
    <pv-column field="id" header="ID"></pv-column>
    <pv-column field="name" header="名称"></pv-column>
    <pv-column field="description" header="描述"></pv-column>
    <pv-column field="vars" header="参数"></pv-column>
    <pv-column header="操作" :exportable="false" style="min-width: 12rem">
      <template #body="slotProps">
        <!-- <pv-button
            icon="pi pi-pencil"
            size="small"
            variant="outlined"
            rounded
            class="mr-2"
            @click="editReplacement(slotProps.data)"
          /> -->
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

  <!-- 新增对话框 -->
  <pv-dialog
    v-model:visible="dialogVis"
    :style="{ width: '750px' }"
    header="世界状态重写详情"
    :modal="true"
  >
    <div class="flex flex-col gap-6">
      <div>
        <label for="path" class="block font-bold mb-3">ID</label>
        <pv-input-text
          id="path"
          v-model="sortieMod.id"
          required="true"
          rows="3"
          cols="20"
          fluid
          placeholder="请输入ID"
        />
      </div>
      <div>
        <label for="origin" class="block font-bold mb-3">名称</label>
        <pv-input-text
          id="origin"
          v-model="sortieMod.name"
          optionLabel="label"
          placeholder="请输入名称"
          fluid
        />
      </div>

      <div>
        <label for="replacement" class="block font-bold mb-3">描述</label>
        <pv-input-text
          id="replacement"
          v-model="sortieMod.description"
          optionLabel="label"
          placeholder="请输入描述"
          fluid
        />
      </div>
      <div>
        <label for="remark" class="block font-bold mb-3">参数</label>
        <div class="flex flex-col gap-2">
          <div><pv-button label="新参数" @click="addVar"></pv-button></div>
          <div v-for="(v, index) in vars" :key="v.key" class="flex gap-2">
            <pv-input-text
              class="flex-1"
              id="remark"
              v-model="v.key"
              required="true"
              rows="3"
              cols="20"
              placeholder="请输入参数名"
              fluid
            />
            <pv-input-text
              id="remark"
              class="flex-2"
              v-model="v.value"
              required="true"
              rows="3"
              cols="20"
              placeholder="请输入参数值"
              fluid
            />
            <pv-button
              icon="pi pi-trash"
              variant="outlined"
              rounded
              severity="danger"
              @click="vars.splice(index, 1)"
            />
            <!-- <div></div> -->
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <pv-button label="取消" icon="pi pi-times" outlined @click="dialogVis = false" />
      <pv-button label="保存" icon="pi pi-check" @click="saveSOM" />
    </template>
  </pv-dialog>

  <!-- 删除对话框 -->
  <pv-dialog
    v-model:visible="deleteDlgVis"
    :style="{ width: '450px' }"
    header="确定删除吗？"
    :modal="true"
  >
    <div class="flex items-center gap-4">
      <i class="pi pi-exclamation-triangle text-3xl!" />
      <span v-if="si"
        >您确定要删除
        <b
          ><code>{{ si.id }}</code>
        </b>
        ?</span
      >
    </div>
    <template #footer>
      <pv-button
        label="取消"
        icon="pi pi-times"
        @click="deleteDlgVis = false"
        severity="secondary"
        variant="outlined"
      />
      <pv-button label="确定" icon="pi pi-check" @click="doDelete" severity="danger" />
    </template>
  </pv-dialog>
</template>
<script setup lang="ts">
import { delete_sm, list_sm, new_sm, reload, type SortieModifier } from '@/api/deps'
import { ref } from 'vue'
import { useToast } from 'primevue/usetoast'

const data = ref<SortieModifier[]>([])
const toast = useToast()
const reloading = ref(false)

const dialogVis = ref(false)
const sortieMod = ref<SortieModifier>({
  id: '',
  name: '',
  description: '',
  vars: {},
})

const vars = ref<{ key: string; value: string }[]>([
  {
    key: '',
    value: '',
  },
])

function addVar() {
  vars.value.push({
    key: '',
    value: '',
  })
}

async function fetch() {
  data.value = await list_sm()
  toast.add({
    severity: 'success',
    summary: '获取成功',
    detail: '已成功获取数据',
    life: 3000,
  })
}

fetch()

function openNewDialog() {
  sortieMod.value = {
    id: '',
    name: '',
    description: '',
    vars: {},
  }
  vars.value = [
    {
      key: '',
      value: '',
    },
  ]
  dialogVis.value = true
}

async function saveSOM() {
  console.log(sortieMod.value)
  vars.value.forEach((v) => {
    if (!v.key) return
    sortieMod.value.vars[v.key] = v.value
  })
  await new_sm(sortieMod.value)
  fetch()
  dialogVis.value = false
  toast.add({
    severity: 'success',
    summary: '保存成功',
    detail: '已成功保存数据',
    life: 3000,
  })
}

async function reloadDeps() {
  reloading.value = true
  try {
    await reload()
    toast.add({
      severity: 'success',
      summary: '重载成功',
      detail: '已成功重载依赖',
      life: 3000,
    })
  } finally {
    reloading.value = false
  }
}

const deleteDlgVis = ref(false)
const si = ref<SortieModifier>()
function confirmDelete(item: SortieModifier) {
  si.value = item
  deleteDlgVis.value = true
}

async function doDelete() {
  if (!si.value) return
  await delete_sm(si.value.id)
  deleteDlgVis.value = false
  fetch()
  toast.add({
    severity: 'success',
    summary: '删除成功',
    detail: '已成功删除数据',
    life: 3000,
  })
}
</script>
