<script setup lang="ts">
import { useUiStore } from '@/stores/ui'
import ConfirmDialog from '@/components/ConfirmDialog.vue'

const ui = useUiStore()
</script>

<template>
  <!-- 动态弹窗栈：各功能页通过 ui.openDialog(组件, props) 推入 -->
  <div
    v-for="d in ui.dialogStack"
    :key="d.id"
    class="overlay open"
    @click.self="ui.closeDialog(d.id)"
  >
    <component :is="d.component" v-bind="d.props" @close="ui.closeDialog(d.id)" />
  </div>

  <!-- 通用确认框 -->
  <div v-if="ui.confirm" class="overlay open" @click.self="ui.cancelConfirm()">
    <ConfirmDialog />
  </div>
</template>
