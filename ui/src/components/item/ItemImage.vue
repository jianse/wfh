<template>
  <img :src="url" />
</template>
<script lang="ts" setup>
import { findImage } from '@/util/localDB'
import { ref, watch } from 'vue'

const props = defineProps<{
  uniqueName: string
}>()

const loading = ref(true)
const url = ref('')

watch(
  () => props.uniqueName,
  async (v) => {
    if (!v) return
    loading.value = true
    const item = await findImage(v)
    if (item) {
      url.value = 'https://content.warframe.com/PublicExport/' + v + '!' + item.contentHash
    } else {
      url.value = 'https://browse.wf' + v
    }

    loading.value = false
  },
  { immediate: true },
)
</script>
