<script setup lang="ts">
/* eslint-disable @typescript-eslint/no-explicit-any */
/**
 * 声明式列定义（类似 el-table-column）。
 * 组件本身不渲染内容：DataTable 会读取它的 props 与默认插槽来渲染表头/单元格。
 * 默认插槽作用域：{ row, column, $index }。
 */
defineProps<{
  /** 绑定的字段名（用于默认取值 / 作为 key） */
  prop?: string
  label?: string
  /** 列宽，例如 '38%' 或 '90px' */
  width?: string
  align?: 'left' | 'right'
  /** 附加到单元格 td 的类名（如 'wrap'） */
  cellClass?: string
}>()

defineSlots<{
  default?: (scope: { row: any; column: any; $index: number }) => any
}>()
</script>

<template>
  <!-- 占位：实际渲染由 DataTable 读取本组件的 props/插槽完成 -->
  <span v-if="false" />
</template>
