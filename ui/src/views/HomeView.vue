<script setup lang="ts">
import uiIcons from '@/assets/icons/ui/index.ts'
import { useSyncStore, type SyncItem } from '@/stores/sync'
import { check_update_available } from '@/util/resources'
import { computed, ref } from 'vue'

interface SyncState {
  label: string
  icon: string
  category?: string
  state?: SyncItem
}

const items: SyncState[] = [
  { label: '战甲', icon: uiIcons.warframe, category: 'Warframes' },
  { label: '武器', icon: uiIcons.weapon, category: 'Weapons' },
  { label: '振幅晶体', icon: uiIcons.mod, category: 'Upgrades' },
  { label: '赋能', icon: uiIcons.arcane, category: 'Arcane' },
  { label: '配方', icon: uiIcons.recipe, category: 'Recipes' },
  { label: '遗物', icon: uiIcons.relic, category: 'Relics' },
  { label: '清单', icon: uiIcons.manifest, category: 'Manifest' },
]

const store = useSyncStore()

const itemsComputed = computed<SyncState[]>(() => {
  return items.map((it) => {
    if (it.category) {
      const si = store.state[it.category]
      // it.state = si
      return { ...it, state: si }
    } else {
      return it
    }
  })
})

const loading = ref(false)
async function syncAll(force: boolean = false) {
  try {
    loading.value = true
    await check_update_available(force)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="w-screen h-full p-4">
    <pv-card class="w-68">
      <template #title>同步状态</template>
      <template #subtitle><i class="pi pi-check text-green-400"></i>同步完成</template>
      <template #content>
        <ul class="flex gap-0.5 flex-col">
          <li class="flex justify-between" v-for="item in itemsComputed" :key="item.label">
            <div>
              <img
                v-if="item.icon"
                class="invert-50 dark:invert-25 inline h-lh"
                :alt="item.label"
                :src="item.icon"
              />{{ item.label }}
            </div>
            <div>
              <span v-if="item.state == undefined">?/?</span>
              <span v-else>{{ item.state.completed }}/{{ item.state.total }}</span>
              <pv-button icon="pi pi-refresh" size="small" variant="link"></pv-button>
            </div>
          </li>
        </ul>
      </template>
      <template #footer>
        <div class="flex gap-4 mt-1">
          <pv-button
            :loading="loading"
            severity="secondary"
            class="w-full"
            size="small"
            icon="pi pi-refresh"
            @click="syncAll()"
            >全部同步</pv-button
          >
          <pv-button
            :loading="loading"
            class="w-full"
            size="small"
            icon="pi pi-refresh"
            @click="syncAll(true)"
            >强制同步</pv-button
          >
        </div>
      </template>
    </pv-card>
  </div>
</template>
