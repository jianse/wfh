<script setup lang="ts" generic="T extends object">
import { computed, Fragment, h, isVNode, useSlots, type VNode } from 'vue'
import DataTableColumn from './DataTableColumn.vue'

const props = withDefaults(
  defineProps<{
    rows: T[]
    rowKey?: keyof T | ((row: T, index: number) => string | number)
    loading?: boolean
    emptyText?: string
    loadingText?: string
    minHeight?: string
  }>(),
  {
    loading: false,
    emptyText: '暂无数据。',
    loadingText: '正在加载…',
    minHeight: '120px'
  }
)

const slots = useSlots()

interface ResolvedColumn {
  key: string
  label: string
  width?: string
  align: 'left' | 'right'
  class?: string
  render?: (scope: { row: T; column: ResolvedColumn; $index: number }) => unknown
}

/** 从默认插槽里的 <DataTableColumn> 读取列定义。 */
const columns = computed<ResolvedColumn[]>(() => {
  const children = slots.default?.() ?? []
  const out: ResolvedColumn[] = []
  for (const vnode of children) {
    if (!isVNode(vnode) || vnode.type !== DataTableColumn) continue
    const p = (vnode.props ?? {}) as Record<string, unknown>
    const colSlots = vnode.children as Record<string, ((scope: unknown) => unknown) | number> | null
    const slotFn =
      colSlots && typeof colSlots.default === 'function'
        ? (colSlots.default as (scope: { row: T; column: ResolvedColumn; $index: number }) => unknown)
        : undefined
    const prop = (p.prop ?? p.propName) as string | undefined
    const label = p.label as string | undefined
    out.push({
      key: prop ?? label ?? `col-${out.length}`,
      label: label ?? '',
      width: p.width as string | undefined,
      align: (p.align as 'left' | 'right' | undefined) ?? 'left',
      class: (p.cellClass ?? p['cell-class'] ?? p.class) as string | undefined,
      render: slotFn
    })
  }
  return out
})

function valueOf(row: T, column: ResolvedColumn): unknown {
  return (row as Record<string, unknown>)[column.key]
}

function keyOf(row: T, index: number): string | number {
  const rk = props.rowKey
  if (typeof rk === 'function') return rk(row, index)
  if (rk != null) return row[rk] as unknown as string | number
  return index
}

/** 渲染单元格：有默认插槽则调用，否则回退显示字段值。 */
function RenderCell(p: {
  render?: ResolvedColumn['render']
  row: T
  index: number
  column: ResolvedColumn
  value: unknown
}) {
  const out = p.render
    ? p.render({ row: p.row, column: p.column, $index: p.index })
    : p.value == null
      ? ''
      : String(p.value)
  return Array.isArray(out) ? h(Fragment, null, out) : (out as VNode | string)
}
</script>

<template>
  <!-- 外层不滚动：遮罩固定覆盖可视区域，横向滚动发生在内层 .table-wrap -->
  <div class="relative" :style="{ minHeight }">
    <div class="table-wrap">
      <table class="ds-table">
        <thead>
          <tr>
            <th
              v-for="col in columns"
              :key="col.key"
              :class="col.align === 'right' ? 'num-col' : undefined"
              :style="col.width ? { width: col.width } : undefined"
            >
              {{ col.label }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, index) in rows" :key="keyOf(row, index)">
            <td
              v-for="col in columns"
              :key="col.key"
              :class="[col.align === 'right' ? 'num-col' : undefined, col.class]"
            >
              <RenderCell
                :render="col.render"
                :row="row"
                :index="index"
                :column="col"
                :value="valueOf(row, col)"
              />
            </td>
          </tr>
          <tr v-if="!rows.length && !loading">
            <td :colspan="columns.length" class="cell-sub">{{ emptyText }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div
      v-if="loading"
      class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-2 bg-[color-mix(in_oklch,var(--surface)_72%,transparent)] backdrop-blur-[1px]"
      role="status"
      aria-live="polite"
    >
      <span
        class="inline-block size-6 animate-spin rounded-full border-2 border-border border-t-accent"
        aria-hidden="true"
      ></span>
      <slot name="loading"><span class="meta">{{ loadingText }}</span></slot>
    </div>
  </div>
</template>
