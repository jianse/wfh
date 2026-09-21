<script setup lang="ts">
defineProps<{
  modelValue: string
  options: { label: string; value: string }[]
}>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<template>
  <div class="overflow-x-auto scrollbar-thin flex-1 min-w-0">
    <div class="flex border-b border-surface-200 dark:border-surface-700 w-max">
      <button
        v-for="option in options"
        :key="option.value"
        :class="[
          'inline-flex items-center px-4 py-2.5 text-sm font-medium transition-colors duration-200 select-none cursor-pointer border-b-2 whitespace-nowrap focus:outline-none focus-visible:ring-2 focus-visible:ring-primary-500 focus-visible:ring-offset-2',
          modelValue === option.value
            ? 'border-primary-500 text-primary-600 dark:text-primary-400 bg-primary-50/50 dark:bg-primary-500/10'
            : 'border-transparent text-surface-600 dark:text-surface-400 hover:text-surface-800 dark:hover:text-surface-200 hover:bg-surface-50 dark:hover:bg-surface-800',
        ]"
        @click="$emit('update:modelValue', option.value)"
      >
        <span class="relative">{{ option.label }}</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.scrollbar-thin {
  scrollbar-width: thin;
  scrollbar-color: transparent transparent;
}

.scrollbar-thin::-webkit-scrollbar {
  height: 4px;
}

.scrollbar-thin::-webkit-scrollbar-track {
  background: transparent;
}

.scrollbar-thin::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 2px;
}

.scrollbar-thin::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

.dark .scrollbar-thin::-webkit-scrollbar-thumb {
  background: #475569;
}

.dark .scrollbar-thin::-webkit-scrollbar-thumb:hover {
  background: #64748b;
}
</style>
