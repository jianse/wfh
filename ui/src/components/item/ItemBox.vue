<template>
  <div>
    <component
      v-if="item !== undefined"
      :is="getComponent(item.category)"
      :item="item.value"
      :count="count"
    />
  </div>
</template>
<script setup lang="ts">
import { findItem, type StoreName } from '@/util/localDB'
import { computedAsync } from '@vueuse/core'
import { useI18n } from 'vue-i18n'
import RelicBox from './RelicBox.vue'
import CommonBox from './CommonBox.vue'
import FusionBundleBox from './FusionBundleBox.vue'

const { t } = useI18n()

const props = defineProps<{
  uniqueName: string
  count?: number
}>()

const item = computedAsync(async () => {
  const item = await findItem(props.uniqueName)
  // console.log(item)
  return item
}, undefined)

function parseCredits(name: string): string {
  console.log('parseCredits', name)
  return name.split('/').pop()?.replace('Credits', '星币') || ''
}

const getComponent = (category: StoreName) => {
  switch (category) {
    case 'Relics':
      return RelicBox
    case 'FusionBundles':
      return FusionBundleBox
    default:
      return CommonBox
  }
}
</script>
