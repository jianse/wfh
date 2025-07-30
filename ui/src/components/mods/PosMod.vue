<template>
  <div style="width: 256px; height: 450px">
    <component :is="comp" v-bind="props" />
  </div>
</template>
<script lang="ts" setup>
import type { Upgrade } from '@/util/localDB'
import { computed, defineAsyncComponent } from 'vue'

// import CommonMod from './CommonMod.vue'
import { getModClass } from '@/util/mod'
// import ArchonMod from './Archon/ArchonMod.vue'
import EmptyMod from './EmptyMod.vue'

const props = defineProps<{ mod: Upgrade; maxRank?: boolean }>()

const comp = computed(() => {
  const mc = getModClass(props.mod)
  switch (mc) {
    case 'Omega Unknown':
    case 'Omega Opened':
      return defineAsyncComponent(() => import('./Omega/OmegaMod.vue'))
    case 'Amalgam':
      return defineAsyncComponent(() => import('./Amalgam/AmalgamMod.vue'))
    case 'Galvanized':
      return defineAsyncComponent(() => import('./Galvanized/GalvanizedMod.vue'))
    // case 'Railjack COMMON':
    // case 'Railjack UNCOMMON':
    // case 'Railjack RARE':
    case 'Peculiar':
      return defineAsyncComponent(() => import('./Peculiar/PeculiarMod.vue'))
    case 'Tome':
      return defineAsyncComponent(() => import('./Tome/TomeMod.vue'))
    case 'Immortal':
      return defineAsyncComponent(() => import('./Immortal/ImmortalMod.vue'))
    case 'Antivirus':
      return defineAsyncComponent(() => import('./Antivirus/AntivirusMod.vue'))
    case 'Potency':
      return defineAsyncComponent(() => import('./Potency/PotencyMod.vue'))
    case 'COMMON':
    case 'UNCOMMON':
    case 'RARE':
    case 'LEGENDARY':
      return defineAsyncComponent(() => import('./CommonMod.vue'))
    case 'Archon':
      return defineAsyncComponent(() => import('./Archon/ArchonMod.vue'))
  }

  return EmptyMod
})
</script>
