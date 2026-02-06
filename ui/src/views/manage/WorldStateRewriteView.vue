<template>
  <pv-tabs value="deps_sm">
    <pv-tab-list>
      <pv-tab value="deps_sm">SOM</pv-tab>
      <pv-tab value="old">旧数据</pv-tab>
    </pv-tab-list>
    <pv-tab-panels>
      <pv-tab-panel value="old">
        <w-s-v1-view />
      </pv-tab-panel>
      <pv-tab-panel value="deps_sm">
        <deps-s-m-view />
      </pv-tab-panel>
    </pv-tab-panels>
  </pv-tabs>
</template>
<script setup lang="ts">
import DepsSMView from './DepsSMView.vue'
import WSV1View from './WSV1View.vue'
</script>
