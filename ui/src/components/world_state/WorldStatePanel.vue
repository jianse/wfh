<script setup lang="ts">
defineProps<{ title: string; icon?: string; meta?: string; span?: boolean }>()
</script>

<template>
  <section class="panel ws-panel" :class="{ 'ws-half': span }">
    <div class="panel-head">
      <span v-if="icon" class="panel-ico">
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.6"
          stroke-linecap="round"
          stroke-linejoin="round"
          v-html="icon"
        ></svg>
      </span>
      <span class="panel-title">{{ title }}</span>
      <span v-if="meta" class="panel-meta">{{ meta }}</span>
    </div>
    <slot />
  </section>
</template>
