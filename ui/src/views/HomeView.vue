<script setup lang="ts">
import uiIcons from '@/assets/icons/ui/index.ts'
import { useSyncStore, type SyncItem } from '@/stores/sync'
import { check_update_available, update_one } from '@/util/resources'
import { computed, ref } from 'vue'

interface SyncState {
  label: string
  icon: string
  category: string
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

async function syncOne(cate: string) {
  try {
    loading.value = true
    await update_one(cate)
  } finally {
    loading.value = false
  }
}

import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import 'dayjs/locale/zh-cn'
dayjs.extend(relativeTime) // 相对时间
dayjs.locale('zh-cn') // 使用本地化语言

function humanFormat(time: Date) {
  if (!time) {
    return '-'
  }
  return dayjs(time).fromNow()
}
</script>

<template>
  <div class="w-screen h-full p-4">
    <pv-card class="w-2xl">
      <!-- <template #title>同步状态</template>
      <template #subtitle><i class="pi pi-check text-green-400"></i>同步完成</template> -->
      <template #content>
        <pv-data-table :value="itemsComputed" tableStyle="min-width:100%">
          <template #header>
            <span class="text-xl font-bold">数据状态</span>
          </template>
          <pv-column header="分组">
            <template #body="{ data }">
              <img
                v-if="data.icon"
                class="invert-50 dark:invert-25 inline h-lh"
                :alt="data.label"
                :src="data.icon"
              />{{ data.label }}
            </template>
          </pv-column>
          <pv-column header="上次同步时间">
            <template #body="{ data }">
              {{ humanFormat(data.state?.last_sync_time) }}
            </template>
          </pv-column>
          <pv-column field="state.phase" header="同步状态"></pv-column>
          <pv-column header="同步进度">
            <template #body="{ data }">
              <span v-if="data.state == undefined">?/?</span>
              <span v-else>{{ data.state.completed }}/{{ data.state.total }}</span>
            </template>
          </pv-column>
          <pv-column header="操作">
            <template #body="{ data }">
              <pv-button
                class="w-auto"
                :loading="loading"
                icon="pi pi-refresh"
                size="small"
                variant="text"
                label="重新同步"
                @click="syncOne(data.category)"
              ></pv-button>
            </template>
          </pv-column>
        </pv-data-table>
      </template>
      <template #footer>
        <div class="flex gap-4 mt-1">
          <pv-button
            :loading="loading"
            class="w-full"
            size="small"
            icon="pi pi-refresh"
            @click="syncAll()"
            label="全部同步"
          ></pv-button>
          <pv-button
            :loading="loading"
            class="w-full"
            size="small"
            icon="pi pi-sync"
            severity="warn"
            @click="syncAll(true)"
            label="强制同步"
          ></pv-button>
        </div>
      </template>
    </pv-card>
  </div>
</template>
