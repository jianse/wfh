<script setup lang="ts">
import { computed, ref } from 'vue'
import { useToast } from 'primevue/usetoast'
import { useUserStore } from '@/stores/user'
import { storeToRefs } from 'pinia'
import { cloneDeep } from 'lodash'
import { delete_translation, save_translation } from '@/api/translations'
import { useTransStore } from '@/stores/translations'
import { useOWarframes, type Warframes } from '@/util/localDB'

interface WarframeItemWithTrans extends Warframes {
  name_t?: string
  description_t?: string
  // image_url: string
}

const warframes = useOWarframes()

const translatedItems = computed<WarframeItemWithTrans[]>(() => {
  return warframes.value.map((it) => {
    const name_t = wf_t(it.uniqueName, 'name') || ''
    const description_t = wf_t(it.uniqueName, 'description') || ''
    return { ...it, name_t, description_t }
  })
})

const items = computed<WarframeItemWithTrans[]>(() => {
  return translatedItems.value.filter((it) => {
    if (ignoreCase.value) {
      const kwLower = kwFilter.value.toLowerCase()
      return (
        it.name.toLowerCase().includes(kwLower) ||
        it.description.toLowerCase().includes(kwLower) ||
        it.name_t?.toLowerCase().includes(kwLower) ||
        it.description_t?.toLowerCase().includes(kwLower)
      )
    } else {
      return (
        it.name.includes(kwFilter.value) ||
        it.description.includes(kwFilter.value) ||
        it.name_t?.includes(kwFilter.value) ||
        it.description_t?.includes(kwFilter.value)
      )
    }
  })
})

const toast = useToast()

const { isAdmin } = storeToRefs(useUserStore())

const actions = ref([
  {
    label: 'Add',
    icon: 'pi pi-pencil',
    command: () => {
      toast.add({ severity: 'info', summary: 'Add', detail: 'Data Added', life: 3000 })
    },
  },
  {
    label: 'Update',
    icon: 'pi pi-refresh',
    command: () => {
      toast.add({ severity: 'success', summary: 'Update', detail: 'Data Updated', life: 3000 })
    },
  },
  {
    label: 'Delete',
    icon: 'pi pi-trash',
    command: () => {
      toast.add({ severity: 'error', summary: 'Delete', detail: 'Data Deleted', life: 3000 })
    },
  },
  {
    label: 'Upload',
    icon: 'pi pi-upload',
  },
  {
    label: 'Vue Website',
    icon: 'pi pi-external-link',
    command: () => {
      window.location.href = 'https://vuejs.org/'
    },
  },
])

const editDialogVisible = ref(false)
const dialogData = ref<Warframes>({
  name: '',
  description: '',
  uniqueName: '',
  productCategory: 'SpaceSuits',
  textureLocation: '',
})

const nameTrans = ref('')
const descriptionTrans = ref('')

function openEditDialog(item: WarframeItemWithTrans) {
  dialogData.value = cloneDeep(item)
  nameTrans.value = item.name_t || ''
  descriptionTrans.value = item.description_t || ''
  editDialogVisible.value = true
}

const { wf_t, write_back_trans, remove_trans } = useTransStore()
const nameLoading = ref(false)
const descriptionLoading = ref(false)

async function saveTrans(unique_name: string, prop: string, content: string) {
  // start loading
  if (prop === 'name') {
    nameLoading.value = true
  }
  if (prop === 'description') {
    descriptionLoading.value = true
  }
  try {
    const it = {
      unique_name,
      prop,
      content,
    }
    await save_translation(it)
    write_back_trans(it)
    toast.add({ severity: 'success', summary: '操作成功', detail: '添加译文成功！', life: 3000 })
  } finally {
    // stop loading
    if (prop === 'name') {
      nameLoading.value = false
    }

    if (prop === 'description') {
      descriptionLoading.value = false
    }
  }
}

async function clearTrans(unique_name: string, prop: string) {
  // start loading
  if (prop === 'name') {
    nameLoading.value = true
  }
  if (prop === 'description') {
    descriptionLoading.value = true
  }
  try {
    const it = {
      unique_name,
      prop,
    }
    await delete_translation(it)
    remove_trans(it)
    toast.add({ severity: 'success', summary: '操作成功', detail: '清空译文成功！', life: 3000 })
    if (prop === 'name') {
      nameTrans.value = ''
    }
    if (prop === 'description') {
      descriptionTrans.value = ''
    }
  } finally {
    // stop loading
    if (prop === 'name') {
      nameLoading.value = false
    }

    if (prop === 'description') {
      descriptionLoading.value = false
    }
  }
}

const kwFilter = ref('')
const ignoreCase = ref(true)
</script>

<template>
  <div class="c-toolbar w-full sticky top-0 z-10 mt-2 px-2">
    <div class="bg-stone-200 dark:bg-stone-600 py-1 px-2 rounded-md flex flex-row gap-3">
      <div>搜索：<pv-input-text placeholder="请输入关键词" size="small" v-model="kwFilter"></pv-input-text></div>
      <div class="flex items-center gap-2">
        <pv-checkbox v-model="ignoreCase" inputId="ignoreCase" binary />
        <label for="ignoreCase"> 忽略大小写 </label>
      </div>
      <div>筛选：<pv-select size="small" /></div>
    </div>
  </div>

  <div class="flex gap-2 flex-wrap p-2">
    <pv-card v-for="item in items" :key="item.uniqueName" class="w-72">
      <template #header>
        <div class="relative">
          <pv-image
            :alt="item.name_t || item.name"
            :src="'https://content.warframe.com/PublicExport' + item.textureLocation"
            imageClass="w-72 h-72"
            loading="lazy"
          />
          <pv-button
            v-if="isAdmin"
            class="absolute top-2 right-2"
            size="small"
            rounded
            variant="outlined"
            icon="pi pi-pencil"
            @click="openEditDialog(item)"
          ></pv-button>
        </div>
      </template>
      <template #title>
        <StringWithIcon :text="item.name_t || item.name"></StringWithIcon
      ></template>
      <template #content>
        <div class="max-h-24 overflow-auto">
          {{ item.description_t || item.description }}
        </div>
      </template>
    </pv-card>

    <PvSpeedDial
      pt:pcButton:root:class="size-15"
      :model="actions"
      :radius="120"
      :actionButtonProps="{
        rounded: true,
      }"
      type="quarter-circle"
      direction="up-left"
      :style="{ position: 'absolute', right: '30px', bottom: '30px' }"
    />
  </div>
  <pv-dialog
    v-model:visible="editDialogVisible"
    modal
    :header="`修改[${dialogData.name}]`"
    class="min-w-lg w-lg"
    dismissableMask
  >
    <div v-show="false" class="overflow-auto h-120 bg-stone-100 p-3 mb-3">
      <code>
        <pre class="font-[comic_mono]">{{ JSON.stringify(dialogData, null, 2) }}</pre>
      </code>
    </div>

    <pv-fieldset legend="名称">
      <div class="mb-2">原文: {{ dialogData.name }}</div>
      <div class="mb-2">
        <label>译文: </label
        ><pv-input-text
          class="w-96"
          type="text"
          size="small"
          v-model="nameTrans"
          autofocus
        ></pv-input-text>
      </div>
      <div class="flex flex-row gap-2">
        <pv-button
          label="保存"
          size="small"
          icon="pi pi-save"
          :loading="nameLoading"
          @click="saveTrans(dialogData.uniqueName, 'name', nameTrans)"
        ></pv-button>
        <pv-button
          label="清除"
          size="small"
          icon="pi pi-trash"
          severity="warn"
          :loading="nameLoading"
          @click="clearTrans(dialogData.uniqueName, 'name')"
        ></pv-button>
      </div>
    </pv-fieldset>

    <pv-fieldset legend="描述">
      <div class="mb-2">原文: {{ dialogData.description }}</div>
      <div class="mb-2 align-top">
        <label>译文: </label
        ><pv-textarea class="w-96" rows="3" size="small" v-model="descriptionTrans"></pv-textarea>
      </div>
      <div class="flex flex-row gap-2">
        <pv-button
          label="保存"
          size="small"
          icon="pi pi-save"
          :loading="descriptionLoading"
          @click="saveTrans(dialogData.uniqueName, 'description', descriptionTrans)"
        ></pv-button>
        <pv-button
          label="清除"
          size="small"
          icon="pi pi-trash"
          severity="warn"
          :loading="descriptionLoading"
          @click="clearTrans(dialogData.uniqueName, 'description')"
        ></pv-button>
      </div>
    </pv-fieldset>

    <div class="flex justify-end gap-2 mt-2">
      <pv-button type="button" label="关闭" @click="editDialogVisible = false"></pv-button>
    </div>
  </pv-dialog>
</template>
