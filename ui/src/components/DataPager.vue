<script setup lang="ts">
import { computed, ref, watch, type PropType } from 'vue'

const props = defineProps({
  page: { type: Number, required: true },
  total: { type: Number, required: true },
  size: { type: Number, default: 8 },
  disabled: { type: Boolean, default: false },
  /** 提供可选值时，在分页器内显示“每页 N 条”选择器 */
  pageSizes: { type: Array as PropType<number[]>, default: undefined }
})
const emit = defineEmits<{ go: [page: number]; 'size-change': [size: number] }>()

/* 当前页两侧各展示的页码数 */
const WINDOW = 2

const totalPages = computed(() => Math.max(1, Math.ceil(props.total / props.size)))

type PageItem = number | 'left-ellipsis' | 'right-ellipsis'

/* 页数较多时只展示首页、末页与当前页附近的页码，中间用省略号折叠。 */
const pageItems = computed<PageItem[]>(() => {
  const tp = totalPages.value
  const cur = Math.min(Math.max(1, props.page), tp)
  if (tp <= 7) return Array.from({ length: tp }, (_, i) => i + 1)

  const items: PageItem[] = [1]
  const left = Math.max(2, cur - WINDOW)
  const right = Math.min(tp - 1, cur + WINDOW)
  if (left > 2) items.push('left-ellipsis')
  for (let p = left; p <= right; p++) items.push(p)
  if (right < tp - 1) items.push('right-ellipsis')
  items.push(tp)
  return items
})

const jump = ref('')

function go(target: number | string) {
  if (props.disabled) return
  const tp = totalPages.value
  const n = Math.floor(Number(target)) || 1
  const clamped = Math.min(Math.max(1, n), tp)
  if (clamped === props.page) return
  emit('go', clamped)
}

function doJump() {
  if (props.disabled || !jump.value) return
  go(jump.value)
  jump.value = ''
}

function onSizeChange(e: Event) {
  const next = Number((e.target as HTMLSelectElement).value)
  jump.value = ''
  emit('size-change', next)
}

/* 页码由外部变化时清空跳转输入 */
watch(
  () => props.page,
  () => {
    jump.value = ''
  }
)
</script>

<template>
  <div class="pager" :class="{ disabled }">
    <!-- <button class="pg pg-nav" :disabled="disabled || page <= 1" title="首页" aria-label="首页" @click="go(1)">«</button> -->
    <button class="pg pg-nav" :disabled="disabled || page <= 1" @click="go(page - 1)">上一页</button>

    <template v-for="(item, i) in pageItems" :key="`${item}-${i}`">
      <span v-if="item === 'left-ellipsis' || item === 'right-ellipsis'" class="pg-ellipsis">…</span>
      <button
        v-else
        class="pg"
        :class="{ active: item === page }"
        :disabled="disabled"
        :aria-current="item === page ? 'page' : undefined"
        @click="go(item)"
      >
        {{ item }}
      </button>
    </template>

    <button class="pg pg-nav" :disabled="disabled || page >= totalPages" @click="go(page + 1)">下一页</button>
    <!-- <button class="pg pg-nav" :disabled="disabled || page >= totalPages" title="末页" aria-label="末页" @click="go(totalPages)">»</button> -->

    <select
      v-if="pageSizes && pageSizes.length"
      class="pg-select"
      :value="size"
      :disabled="disabled"
      aria-label="每页条数"
      @change="onSizeChange"
    >
      <option v-for="s in pageSizes" :key="s" :value="s">每页 {{ s }} 条</option>
    </select>

    <span class="pg-jump">
      跳至
      <input
        v-model="jump"
        class="pg-input"
        type="number"
        inputmode="numeric"
        min="1"
        :max="totalPages"
        :disabled="disabled"
        :placeholder="String(totalPages)"
        aria-label="跳转到页码"
        @keydown.enter="doJump"
      />
      页
      <button class="pg" :disabled="disabled" @click="doJump">跳转</button>
    </span>

    <span class="pg-info">第 {{ page }} / {{ totalPages }} 页 · 共 {{ total }} 条</span>
  </div>
</template>
