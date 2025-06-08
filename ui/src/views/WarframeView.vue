<script setup lang="ts">
import { useResourcesStore } from '@/stores/resources'
import { imageUrl } from '@/util/common'
// import toast from 'primevue/toast'
import { computed, ref } from 'vue'
import { useToast } from 'primevue/usetoast'
type Item = {
  uniqueName: string
  name?: string
}

const resStore = useResourcesStore()

const items = computed(() => {
  return resStore.warframes.map((it: Item) => {
    const url = imageUrl(it.uniqueName, resStore.manifest)
    return { ...it, imgUrl: url }
  })
})

const toast = useToast()

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
</script>

<template>
  <div class="flex gap-2 flex-wrap">
    <div class="card" v-for="item in items" :key="item.uniqueName">
      <h5>{{ item.name }}</h5>
      <p>{{ item.uniqueName }}</p>
      <div class="img-bg">
        <img :src="item.imgUrl" />
      </div>
    </div>
    <PvSpeedDial
      pt:pcButton:root:class="size-15"
      :model="actions"
      :radius="120"
      type="quarter-circle"
      direction="up-left"
      :style="{ position: 'absolute', right: '30px', bottom: '30px' }"
    />
  </div>
</template>

<style scoped>
.card {
  width: 500px;
  /* height: 200px; */
  flex: initial;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 8px;
}

.img-bg {
  background-color: #aaa;
}

.sticky-header {
  position: sticky;
  /* top: 0; */
  height: 5rem;
  background-color: #fff;
  border-bottom: 1px solid #333;
}

.card img {
  object-fit: contain;
  height: 150px;
  /* height: fit-content; */
  /* height: min-content; */
}
</style>
