<script setup lang="ts">
import { computed } from 'vue'
import type { HexEvent, RawWorldState } from '@/api/world_state'
import { useWorldStatePrefsStore } from '@/stores/world_state_prefs'
import WorldStatePanel from './WorldStatePanel.vue'
import CountDown from '@/components/CountDown.vue'
import StringWithIcon from '@/components/StringWithIcon.vue'

const props = defineProps<{ rawWorldState: RawWorldState | undefined; icon?: string }>()
const emit = defineEmits<{ timeUp: [] }>()

const store = useWorldStatePrefsStore()
const TYPES: [string, string][] = [
  ['CET_CHALLENGE', '待办事项'],
  ['CET_REWARD', '大奖！'],
  ['CET_UPGRADE', '覆写'],
  ['CET_BIRTHDAY', '生日']
]

const season = computed(() => props.rawWorldState?.KnownCalendarSeasons?.[0])
const days = computed(
  () =>
    (season.value?.Days ?? []).filter((d) =>
      d.events.some((e) => store.hexCalendarEvents.includes(e.type))
    ) || []
)

const kind = (t?: string) =>
  ({ CET_BIRTHDAY: '生日', CET_CHALLENGE: '待办事项', CET_REWARD: '大奖！', CET_UPGRADE: '覆写' })[
    t || ''
  ] || '未知'

const evs = (d: { events: HexEvent[] }) =>
  d.events.filter((e) => store.hexCalendarEvents.includes(e.type))

const evName = (e: HexEvent): string => ('name' in e ? (e.name ?? '') : '')
const evTitle = (e: HexEvent): string => ('title' in e ? (e.title ?? '') : '')
const evDesc = (e: HexEvent): string => ('desc' in e ? (e.desc ?? '') : '')
</script>

<template>
  <WorldStatePanel :title="'1999 日历'" :icon="props.icon">
    <div v-if="!season" class="ws-empty">暂无 1999 日历数据</div>
    <template v-else>
      <div class="ws-filters">
        <label v-for="[v, l] in TYPES" :key="v" class="ws-chk">
          <input
            type="checkbox"
            :checked="store.hexCalendarEvents.includes(v)"
            @change="store.toggleHexCalendarEvent(v)"
          />{{ l }}
        </label>
        <button
          class="btn btn-secondary ws-mini"
          @click="store.setHexCalendarEvents(TYPES.map(([v]) => v))"
        >
          全选
        </button>
        <button class="btn btn-secondary ws-mini" @click="store.setHexCalendarEvents([])">取消</button>
      </div>
      <div class="mb-xs"><CountDown :time="season.Expiry" @hit-zero="emit('timeUp')" />轮换</div>
      <div class="ws-hex-days">
        <div v-for="d in days" :key="d.day" class="ws-hex-day">
          <div class="ws-hex-num">{{ d.day }}</div>
          <div class="ws-hex-kind">{{ kind(d.events[0]?.type) }}</div>
          <div class="ws-hex-body">
            <template v-for="(e, i) in evs(d)" :key="i">
              <div v-if="e.type === 'CET_CHALLENGE' || e.type === 'CET_BIRTHDAY'">
                <div class="ws-hex-ev-title">{{ evTitle(e) }}</div>
                <div class="ws-hex-ev-desc"><StringWithIcon :text="evDesc(e)" /></div>
              </div>
              <div v-else-if="e.type === 'CET_REWARD'" class="ws-hex-ev-title">
                <StringWithIcon :text="evName(e)" />
              </div>
              <div v-else-if="e.type === 'CET_UPGRADE'">
                <div class="ws-hex-ev-title">{{ evName(e) }}</div>
                <div class="ws-hex-ev-desc"><StringWithIcon :text="evDesc(e)" /></div>
              </div>
            </template>
          </div>
        </div>
        <div v-if="!days.length" class="ws-empty">没有符合筛选的日历事件</div>
      </div>
    </template>
  </WorldStatePanel>
</template>
