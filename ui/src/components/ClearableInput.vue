<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  modelValue: string
  placeholder?: string
  id?: string
  type?: string
}>()
const emit = defineEmits<{
  'update:modelValue': [value: string]
  input: [event: Event]
  clear: []
}>()

const value = computed({
  get: () => props.modelValue,
  set: (v) => emit('update:modelValue', v)
})

function onInput(e: Event) {
  emit('update:modelValue', (e.target as HTMLInputElement).value)
  emit('input', e)
}

function clear() {
  emit('update:modelValue', '')
  emit('clear')
}
</script>

<template>
  <div class="clearable">
    <input
      class="input"
      :id="id"
      :type="type || 'text'"
      :placeholder="placeholder"
      :value="value"
      @input="onInput"
      @compositionend="emit('input', $event)"
    />
    <button
      v-if="value"
      class="clearable-btn"
      type="button"
      aria-label="清空"
      title="清空"
      @click="clear"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
        <path d="M6 6l12 12M18 6L6 18" />
      </svg>
    </button>
  </div>
</template>
