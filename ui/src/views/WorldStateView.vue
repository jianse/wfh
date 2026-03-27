<template>
  <div class="flex flex-col m-4 gap-4 max-w-400 mx-auto w-full">
    <!-- 配置面板 -->
    <pv-panel header="世界状态面板配置 (拖拽标签排序)" toggleable>
      <div class="flex flex-col gap-4">
        <!-- 可选模块 -->
        <div class="border rounded-md p-4">
          <h4 class="font-bold mb-3">
            <i class="pi pi-plus-circle text-info"></i>
            可选模块
          </h4>
          <div class="flex flex-wrap gap-2">
            <pv-tag
              v-for="section in allSections"
              :key="section.value"
              :value="section.label"
              :severity="wsPrefs.enabledSections.includes(section.value) ? 'success' : 'secondary'"
              class="cursor-pointer select-none px-4 py-2 transition-all hover:scale-105"
              @click="wsPrefs.toggleSection(section.value)"
            />
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex gap-2 flex-wrap">
          <pv-button label="全选" size="small" @click="wsPrefs.selectAll()" />
          <pv-button label="取消全选" size="small" @click="wsPrefs.deselectAll()" />
          <pv-button
            label="重置为默认"
            size="small"
            severity="warn"
            @click="wsPrefs.resetToDefault()"
          />
        </div>
      </div>
    </pv-panel>

    <!-- 主内容区域 - 使用组件渲染 -->
    <div
      :class="
        wsPrefs.layout === 'grid'
          ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4'
          : 'flex flex-col gap-4'
      "
    >
      <component
        :is="componentMap[section]"
        v-for="section in wsPrefs.enabledSections"
        :key="section"
        :raw-world-state="rawWorldState"
        :now="now"
        @time-up="timeUp"
      />
    </div>
  </div>

  <div class="fixed right-5 bottom-5 flex flex-row gap-2">
    <pv-button
      v-if="isAdmin"
      icon="pi pi-refresh"
      severity="warn"
      rounded
      aria-label="ForceRefresh"
      @click="forceFetch"
      :loading="loading"
    />
    <pv-button
      icon="pi pi-refresh"
      severity="success"
      rounded
      aria-label="Refresh"
      @click="onTimeUp"
      :loading="loading"
    />
  </div>
</template>

<script setup lang="ts">
import { get_raw_world_state, force_reload, type RawWorldState } from '@/api/world_state'
import { useUserStore } from '@/stores/user'
import { storeToRefs } from 'pinia'
import { useToast } from 'primevue/usetoast'
import { onActivated, onDeactivated, ref, type Component } from 'vue'
import { useEventListener, useTimestamp } from '@vueuse/core'
import { throttle } from 'lodash'
import { useWorldStatePrefsStore, type WorldStateSection } from '@/stores/world_state_prefs'

// Import world state components
import {
  CyclesPanel,
  EventsPanel,
  AlertsPanel,
  FissuresPanel,
  SteelPathPanel,
  DailyDealsPanel,
  ArchonHuntPanel,
  SentientOutpostsPanel,
  SortiePanel,
  VoidTraderPanel,
  VaultTraderPanel,
  NightwavePanel,
  InvasionsPanel,
  ConquestsPanel,
  HexCalendarPanel,
  EndlessXpPanel,
} from '@/components/world_state'

const rawWorldState = ref<RawWorldState>()
const loading = ref(false)
const toast = useToast()

const wsPrefs = useWorldStatePrefsStore()

const allSections: { value: WorldStateSection; label: string }[] = [
  { value: 'cycles', label: '周期循环' },
  { value: 'events', label: '活动' },
  { value: 'alerts', label: '警报' },
  { value: 'fissures', label: '虚空裂缝' },
  { value: 'steelPath', label: '钢铁之路' },
  { value: 'dailyDeals', label: '特惠商品' },
  { value: 'archonHunt', label: '执刑官猎杀' },
  { value: 'sentientOutposts', label: '灵煞异常' },
  { value: 'sortie', label: '突击任务' },
  { value: 'voidTrader', label: '虚空商人' },
  { value: 'vaultTrader', label: '圣装重生' },
  { value: 'nightwave', label: '午夜电波' },
  { value: 'invasions', label: '入侵任务' },
  { value: 'conquests', label: '每周科研' },
  { value: 'hexCalendar', label: '1999 日历' },
  { value: 'endlessXp', label: '无尽回廊' },
]

// Component mapping
const componentMap: Record<string, Component> = {
  cycles: CyclesPanel,
  events: EventsPanel,
  alerts: AlertsPanel,
  fissures: FissuresPanel,
  steelPath: SteelPathPanel,
  dailyDeals: DailyDealsPanel,
  archonHunt: ArchonHuntPanel,
  sentientOutposts: SentientOutpostsPanel,
  sortie: SortiePanel,
  voidTrader: VoidTraderPanel,
  vaultTrader: VaultTraderPanel,
  nightwave: NightwavePanel,
  invasions: InvasionsPanel,
  conquests: ConquestsPanel,
  hexCalendar: HexCalendarPanel,
  endlessXp: EndlessXpPanel,
}

const fetchEnable = ref(true)

onDeactivated(() => {
  fetchEnable.value = false
})

onActivated(() => {
  fetchEnable.value = true
  onTimeUp()
})

const pageVisible = ref(true)

useEventListener(document, 'visibilitychange', () => {
  pageVisible.value = !document.hidden
  if (pageVisible.value) {
    onTimeUp()
  }
})

const now = useTimestamp({ interval: 1000 })

async function fetchWorldState() {
  if (!fetchEnable.value || !pageVisible.value) {
    return
  }
  loading.value = true
  try {
    const { data } = await get_raw_world_state()
    if (data) {
      rawWorldState.value = data
      toast.add({
        severity: 'success',
        summary: '更新成功',
        detail: '已更新世界状态数据',
        life: 3000,
      })
    }
  } finally {
    loading.value = false
  }
}

async function forceFetch() {
  loading.value = true
  try {
    const { data } = await force_reload()
    if (data) {
      rawWorldState.value = data
      toast.add({
        severity: 'success',
        summary: '强制更新成功',
        detail: '已更新世界状态数据',
        life: 3000,
      })
    }
  } finally {
    loading.value = false
  }
}

const onTimeUp = throttle(fetchWorldState, 3000)

onTimeUp()
function timeUp() {
  onTimeUp()
}

const { isAdmin } = storeToRefs(useUserStore())
</script>

<style scoped>
/* Drag and drop styles */
[draggable] {
  user-select: none;
}

[draggable]:active {
  cursor: grabbing;
}
</style>
