<script setup lang="ts">
import { useUiStore } from '@/stores/ui'

const ui = useUiStore()
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head">
      <h3 class="h3" id="confirm-title">{{ ui.confirm?.title || '确定删除吗？' }}</h3>
    </div>
    <div class="dlg-body">
      <p class="help-note" id="confirm-body">{{ ui.confirm?.body }}</p>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="ui.cancelConfirm()">取消</button>
      <button class="btn btn-danger" id="confirm-ok" @click="ui.resolveConfirm()">确定</button>
    </div>
  </div>
</template>
