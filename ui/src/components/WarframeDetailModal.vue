<script setup lang="ts">
import { WF_CAT_LABEL } from '@/lib/utils'
import { useI18n } from 'vue-i18n'
import type { Warframe } from '@/util/localDB'

defineProps<{ warframe: Warframe }>()
const emit = defineEmits<{ close: [] }>()

const { t } = useI18n()
</script>

<template>
  <div class="modal-panel">
    <div class="wf-detail" id="wf-detail">
      <item-image :unique-name="warframe.icon" :alt="warframe.name" width="512" height="512" />
      <div>
        <div class="row-between">
          <h3 class="h3">
            <span class="wf-cat wf-cat-inline">{{ WF_CAT_LABEL[warframe.productCategory] || '' }}</span
            ><string-with-icon :text="t(warframe.name)" />
          </h3>
          <button class="btn btn-ghost modal-close" id="wf-close" @click="emit('close')">关闭</button>
        </div>
        <p class="wf-desc wf-desc-full"><string-with-icon :text="t(warframe.description)" /></p>
        <div class="d-stats">
          <div class="d-stat">
            <div class="d-val num">{{ warframe.health }}</div>
            <div class="d-label">生命</div>
          </div>
          <div class="d-stat">
            <div class="d-val num">{{ warframe.shield }}</div>
            <div class="d-label">护盾</div>
          </div>
          <div class="d-stat">
            <div class="d-val num">{{ warframe.armor }}</div>
            <div class="d-label">护甲</div>
          </div>
          <div class="d-stat">
            <div class="d-val num">{{ warframe.power }}</div>
            <div class="d-label">能量</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
