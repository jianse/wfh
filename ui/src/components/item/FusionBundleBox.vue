<template>
  <ItemImage :unique-name="item.icon" />
  <span>{{ t(item.name) }}</span>
  <span v-if="count"> X {{ count * item.fusionPoints }}</span>
</template>

<script lang="ts" setup>
import type { FusionBundle } from '@/util/localDB'
import { useI18n } from 'vue-i18n'
import ItemImage from './ItemImage.vue'

const { t } = useI18n()

defineProps<{
  item: FusionBundle
  count?: number
}>()
</script>
