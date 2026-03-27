<template>
  <div class="flex flex-col m-4 gap-4">
    <!-- 配置面板 -->
    <pv-panel header="世界状态面板配置" toggleable>
      <div class="flex flex-col gap-4">
        <div class="flex gap-2 flex-wrap">
          <pv-tag
            v-for="section in allSections"
            :key="section.value"
            :value="section.value"
            :severity="wsPrefs.enabledSections.includes(section.value) ? 'success' : 'secondary'"
            class="cursor-pointer select-none px-3 py-2 transition-all hover:scale-105"
            @click="wsPrefs.toggleSection(section.value)"
          >
            {{ section.label }}
          </pv-tag>
        </div>
        <div class="flex gap-2">
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

    <!-- 主内容区域 -->
    <div
      :class="
        wsPrefs.layout === 'grid'
          ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4'
          : 'flex flex-col gap-4'
      "
    >
      <!-- 周期循环 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('cycles')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-clock"></i>
            <span>周期循环</span>
          </div>
        </template>
        <template #content>
          <div class="flex flex-row gap-2 flex-wrap">
            <pv-card class="min-w-48 grow">
              <template #title>夜灵平野</template>
              <template #content>
                <div class="text-xl font-bold mb-2">{{ cetusCycle.state }}</div>
                <p><CountDownTo :time="cetusCycle.expiry" @hit-zero="timeUp"></CountDownTo>切换</p>
              </template>
            </pv-card>
            <pv-card class="min-w-48 grow">
              <template #title>奥布山谷</template>
              <template #content>
                <div class="text-xl font-bold mb-2">{{ vallisCycle.state }}</div>
                <p><CountDownTo :time="vallisCycle.expiry" @hit-zero="timeUp"></CountDownTo>结束</p>
              </template>
            </pv-card>
            <pv-card class="min-w-48 grow">
              <template #title>魔胎之境</template>
              <template #content>
                <div class="text-xl font-bold mb-2">{{ cambionCycle.state }}</div>
                <p>
                  <CountDownTo :time="cambionCycle.expiry" @hit-zero="timeUp"></CountDownTo>结束
                </p>
              </template>
            </pv-card>
            <pv-card class="min-w-48 grow">
              <template #title>扎里曼号</template>
              <template #content>
                <div class="text-xl font-bold mb-2">{{ zarimanCycle.state }}</div>
                <p>
                  <CountDownTo :time="zarimanCycle.expiry" @hit-zero="timeUp"></CountDownTo>结束
                </p>
              </template>
            </pv-card>
            <pv-card class="min-w-48 grow">
              <template #title>地球</template>
              <template #content>
                <div class="text-xl font-bold mb-2">{{ earthCycle.state }}</div>
                <p><CountDownTo :time="earthCycle.expiry" @hit-zero="timeUp"></CountDownTo>结束</p>
              </template>
            </pv-card>
          </div>
        </template>
      </pv-card>

      <!-- 活动 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('events')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-flag"></i>
            <span>活动</span>
          </div>
        </template>
        <template #content>
          <div class="flex flex-col gap-2">
            <div
              v-for="event in rawWorldState?.Goals"
              :key="event._id"
              class="rounded-md border p-3 border-gray-600 bg-surface-50 dark:bg-surface-900"
            >
              <div class="font-bold text-lg">{{ event.Desc }}</div>
              <div class="text-sm text-muted-color">{{ event.Node || event.VictimNode }}</div>
              <CountDownTo :time="event.Expiry" />结束

              <div class="mt-2 text-sm">{{ event.ToolTip }}</div>
              <pv-progress-bar
                class="mt-2"
                v-if="event.Community"
                :value="event.Count"
                :max="event.Goal"
              />

              <pv-progress-bar
                class="mt-2"
                v-if="event.Tag === 'GhoulEmergence'"
                :value="event.HealthPct * 100"
              />
              <div v-for="(inter, i) in event.InterimRewards" :key="i" class="text-sm">
                {{ rewardAsString(inter) }}
              </div>
              <div v-if="event.Reward" class="text-sm font-medium">
                奖励：{{ rewardAsString(event.Reward) }}
              </div>
            </div>
          </div>
        </template>
      </pv-card>

      <!-- 警报 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('alerts')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-bell"></i>
            <span>警报</span>
          </div>
        </template>
        <template #content>
          <div class="flex flex-row gap-2 flex-wrap">
            <div
              class="border rounded-md p-3 border-gray-600 min-w-64"
              v-for="alert in rawWorldState?.Alerts"
              :key="alert._id"
            >
              <div class="font-bold">{{ alert.MissionInfo.location }}</div>
              <div class="text-sm text-muted-color">{{ alert.MissionInfo.missionType }}</div>
              <p class="text-sm mt-1">{{ alert.MissionInfo.descText }}</p>
              <p class="text-sm"><CountDownTo :time="alert.Expiry" />结束</p>
              <StringWithIcon :text="rewardAsString(alert.MissionInfo.missionReward)" />
            </div>
          </div>
        </template>
      </pv-card>

      <!-- 虚空裂缝 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('fissures')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-shield"></i>
            <span>虚空裂缝</span>
          </div>
        </template>
        <template #content>
          <div>
            <!-- 筛选虚空裂缝 -->
            <div class="card flex flex-wrap justify-center gap-4 mb-4">
              <div class="flex items-center gap-2">
                <pv-checkbox v-model="filterTiers" inputId="tier1" name="fissure" value="VoidT1" />
                <label for="tier1">古纪</label>
              </div>
              <div class="flex items-center gap-2">
                <pv-checkbox v-model="filterTiers" inputId="tier2" name="fissure" value="VoidT2" />
                <label for="tier2">前纪</label>
              </div>
              <div class="flex items-center gap-2">
                <pv-checkbox v-model="filterTiers" inputId="tier3" name="fissure" value="VoidT3" />
                <label for="tier3">中纪</label>
              </div>
              <div class="flex items-center gap-2">
                <pv-checkbox v-model="filterTiers" inputId="tier4" name="fissure" value="VoidT4" />
                <label for="tier4">后纪</label>
              </div>
              <div class="flex items-center gap-2">
                <pv-checkbox v-model="filterTiers" inputId="tier5" name="fissure" value="VoidT5" />
                <label for="tier5">安魂</label>
              </div>
              <div class="flex items-center gap-2">
                <pv-checkbox v-model="filterTiers" inputId="tier6" name="fissure" value="VoidT6" />
                <label for="tier6">全能</label>
              </div>
              <pv-button label="全选" size="small" @click="selectAllTiers" />
              <pv-button label="取消" size="small" @click="deselectAllTiers" />
            </div>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <h3 class="text-lg font-bold mb-2 flex items-center gap-2">
                <i class="pi pi-circle-fill text-blue-500"></i>
                普通
              </h3>
              <ul class="list-disc list-outside pl-4 space-y-1">
                <li v-for="fissure in normalFissures" :key="fissure._id" class="text-sm">
                  <span class="font-medium">{{ tierAsString(fissure.Modifier) }}</span>
                  {{ fissure.Node }} {{ fissure.MissionType }}
                  <br />
                  <CountDownTo :time="fissure.Expiry" @hit-zero="timeUp"></CountDownTo>结束
                </li>
              </ul>
            </div>
            <div>
              <h3 class="text-lg font-bold mb-2 flex items-center gap-2">
                <i class="pi pi-circle-fill text-red-500"></i>
                钢铁之路
              </h3>
              <ul class="list-disc list-outside pl-4 space-y-1">
                <li v-for="fissure in hardFissures" :key="fissure._id" class="text-sm">
                  <span class="font-medium">{{ tierAsString(fissure.Modifier) }}</span>
                  {{ fissure.Node }} {{ fissure.MissionType }}
                  <br />
                  <CountDownTo :time="fissure.Expiry" @hit-zero="timeUp"></CountDownTo>结束
                </li>
              </ul>
            </div>
            <div>
              <h3 class="text-lg font-bold mb-2 flex items-center gap-2">
                <i class="pi pi-circle-fill text-purple-500"></i>
                虚空风暴
              </h3>
              <ul class="list-disc list-outside pl-4 space-y-1">
                <li v-for="fissure in voidStorm" :key="fissure._id" class="text-sm">
                  <span class="font-medium">{{ tierAsString(fissure.ActiveMissionTier) }}</span>
                  {{ fissure.Node }}
                  <br />
                  <CountDownTo :time="fissure.Expiry" @hit-zero="timeUp"></CountDownTo>结束
                </li>
              </ul>
            </div>
          </div>
        </template>
      </pv-card>

      <!-- 钢铁之路每周 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('steelPath')">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-star"></i>
            <span>钢铁之路每周</span>
          </div>
        </template>
        <template #content>
          <div class="text-lg font-bold">
            {{ rawWorldState?.SteelPath.current_reward.name }}
          </div>
          <div class="text-sm text-muted-color">
            {{ rawWorldState?.SteelPath.current_reward.cost }} 钢铁精华
          </div>
          <p class="mt-2"><CountDownTo :time="rawWorldState?.SteelPath.Expiry" /> 轮换</p>
        </template>
      </pv-card>

      <!-- 达尔沃的特惠商品 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('dailyDeals')">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-shopping-cart"></i>
            <span>达尔沃特惠</span>
          </div>
        </template>
        <template #content>
          <ul class="space-y-2">
            <li v-for="deal in rawWorldState?.DailyDeals" :key="deal.StoreItem" class="text-sm">
              <div class="font-medium">{{ deal.StoreItemName }}</div>
              <div class="text-primary font-bold">{{ deal.SalePrice }}P</div>
              <div class="text-xs text-muted-color"><CountDownTo :time="deal.Expiry" />轮换</div>
            </li>
          </ul>
        </template>
      </pv-card>

      <!-- 执刑官猎杀 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('archonHunt')">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-bolt"></i>
            <span>执刑官猎杀</span>
          </div>
        </template>
        <template #content>
          <div v-for="hunt in rawWorldState?.LiteSorties" :key="hunt._id">
            <div class="font-bold text-lg mb-2">BOSS: {{ hunt.BossName }}</div>
            <p class="text-sm text-muted-color mb-2"><CountDownTo :time="hunt.Expiry" />轮换</p>
            <ul class="list-disc list-outside pl-4 text-sm space-y-1">
              <li v-for="(mission, index) in hunt.Missions" :key="index">
                {{ mission.node }} {{ mission.missionType }}
              </li>
            </ul>
          </div>
        </template>
      </pv-card>

      <!-- 灵煞异常 -->
      <pv-card
        v-if="
          wsPrefs.enabledSections.includes('sentientOutposts') && rawWorldState?.SentientOutpost
        "
      >
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-exclamation-triangle"></i>
            <span>灵煞异常</span>
          </div>
        </template>
        <template #content>
          <p class="text-sm text-muted-color mb-2">
            <CountDownTo :time="rawWorldState?.SentientOutpost.expiry" />轮换
          </p>
          <div class="font-medium">
            {{ rawWorldState?.SentientOutpost.node_name || rawWorldState?.SentientOutpost.node }}
          </div>
          <div class="text-sm">{{ rawWorldState?.SentientOutpost.mission_type }}</div>
        </template>
      </pv-card>

      <!-- 突击任务 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('sortie')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-flash"></i>
            <span>突击任务</span>
          </div>
        </template>
        <template #content>
          <div v-for="sortie in rawWorldState?.Sorties" :key="sortie._id">
            <p class="text-sm text-muted-color mb-4"><CountDownTo :time="sortie.Expiry" />轮换</p>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <pv-card
                class="border border-gray-600"
                v-for="(mission, index) in sortie.Variants"
                :key="index"
              >
                <template #title>{{ mission.missionType }}</template>
                <template #content>
                  <div class="flex flex-col gap-2 text-sm">
                    <div class="font-medium">{{ mission.node }}</div>
                    <div class="font-bold text-primary">{{ mission.modifierName }}</div>
                    <div class="text-muted-color">
                      <StringWithIcon :text="mission.modifierDescription || ''" />
                    </div>
                  </div>
                </template>
              </pv-card>
            </div>
          </div>
        </template>
      </pv-card>

      <!-- 虚空商人 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('voidTrader')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-store"></i>
            <span>虚空商人</span>
          </div>
        </template>
        <template #content>
          <div v-for="(voidTrader, index) in rawWorldState?.VoidTraders" :key="index">
            <div v-if="tsBetween(now, voidTrader.Activation, voidTrader.Expiry)">
              <div class="flex items-center gap-2 mb-2">
                <pv-tag value="已到来" severity="success" />
                <span class="font-medium">{{ voidTrader.Node }}</span>
              </div>
              <div class="text-sm text-muted-color mb-4">
                <CountDownTo :time="voidTrader.Expiry"></CountDownTo> 离开
              </div>
              <pv-data-table
                :value="voidTrader.Manifest"
                scrollable
                scroll-height="400px"
                striped-rows
              >
                <pv-column field="ItemName" header="物品" style="width: 50%"></pv-column>
                <pv-column field="PrimePrice" header="杜卡德金币" style="width: 25%"></pv-column>
                <pv-column field="RegularPrice" header="星币" style="width: 25%"></pv-column>
              </pv-data-table>
            </div>
            <div v-else>
              <div class="flex items-center gap-2 mb-2">
                <pv-tag value="未到来" severity="warn" />
                <span class="font-medium">{{ voidTrader.Node }}</span>
              </div>
              <div class="text-sm text-muted-color">
                <CountDownTo :time="voidTrader.Activation"></CountDownTo> 到达
              </div>
            </div>
          </div>
        </template>
      </pv-card>

      <!-- 圣装重生 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('vaultTrader')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-box"></i>
            <span>圣装重生</span>
          </div>
        </template>
        <template #content>
          <div v-for="pvTrader in rawWorldState?.PrimeVaultTraders" :key="pvTrader._id">
            <p class="text-sm text-muted-color mb-4">
              <CountDownTo :time="pvTrader.Expiry" /> 轮换
            </p>
            <pv-tabs value="rotation">
              <pv-tab-list>
                <pv-tab value="rotation">轮换</pv-tab>
                <pv-tab value="evergreen">常驻</pv-tab>
              </pv-tab-list>
              <pv-tab-panels>
                <pv-tab-panel value="rotation">
                  <pv-data-table
                    :value="pvTrader.Manifest"
                    scrollable
                    scroll-height="400px"
                    striped-rows
                  >
                    <pv-column field="ItemName" header="物品"></pv-column>
                    <pv-column
                      field="PrimePrice"
                      header="御品阿耶精华"
                      style="width: 30%"
                    ></pv-column>
                    <pv-column
                      field="RegularPrice"
                      header="阿耶精华"
                      style="width: 25%"
                    ></pv-column>
                  </pv-data-table>
                </pv-tab-panel>
                <pv-tab-panel value="evergreen">
                  <pv-data-table
                    :value="pvTrader.EvergreenManifest"
                    scrollable
                    scroll-height="400px"
                    striped-rows
                  >
                    <pv-column field="ItemName" header="物品"></pv-column>
                    <pv-column
                      field="PrimePrice"
                      header="御品阿耶精华"
                      style="width: 30%"
                    ></pv-column>
                    <pv-column
                      field="RegularPrice"
                      header="阿耶精华"
                      style="width: 25%"
                    ></pv-column>
                  </pv-data-table>
                </pv-tab-panel>
              </pv-tab-panels>
            </pv-tabs>
          </div>
        </template>
      </pv-card>

      <!-- 午夜电波 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('nightwave')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-radio"></i>
            <span>午夜电波</span>
          </div>
        </template>
        <template #content>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div
              v-for="c in rawWorldState?.SeasonInfo.ActiveChallenges"
              :key="c._id"
              class="border rounded-md p-3 border-gray-600"
            >
              <div class="flex justify-between items-center mb-1">
                <span class="font-bold">{{ c.Name }}</span>
                <pv-tag v-if="c.Daily" value="每日" size="small" severity="info" />
              </div>
              <div class="text-sm text-muted-color mb-1">
                <StringWithIcon :text="c.Desc"></StringWithIcon>
              </div>
              <div class="flex justify-between items-center text-sm">
                <span class="text-primary font-bold">{{ c.Xp }} XP</span>
                <span class="text-muted-color"><CountDownTo :time="c.Expiry" />结束</span>
              </div>
            </div>
          </div>
        </template>
      </pv-card>

      <!-- 入侵任务 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('invasions')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-swords"></i>
            <span>入侵任务</span>
          </div>
        </template>
        <template #content>
          <div
            v-for="invasion in rawWorldState?.Invasions"
            :key="invasion._id"
            class="border rounded-md p-4 mb-4 border-gray-600 last:mb-0"
          >
            <div class="font-bold text-lg mb-3">{{ invasion.Node }}</div>

            <div class="flex flex-row place-content-between items-center gap-4">
              <div class="flex flex-col flex-1 items-start">
                <div class="font-medium">{{ invasion.DefenderMissionInfo.faction }}</div>
                <div class="text-sm">{{ rewardAsString(invasion.AttackerReward) }}</div>
              </div>
              <div class="text-center flex-1 font-bold text-lg">
                {{ Math.abs(invasion.Count) }} / {{ invasion.Goal }}
              </div>
              <div class="flex flex-col flex-1 items-end">
                <div class="font-medium">{{ invasion.AttackerMissionInfo.faction }}</div>
                <div class="text-sm">{{ rewardAsString(invasion.DefenderReward) }}</div>
              </div>
            </div>

            <pv-progress-bar :value="calcInvasionProgress(invasion)" class="mt-3" />
          </div>
        </template>
      </pv-card>

      <!-- 每周科研 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('conquests')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-flask"></i>
            <span>每周科研</span>
          </div>
        </template>
        <template #content>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div v-for="conquest in rawWorldState?.Conquests" :key="conquest.Type">
              <ConquestPanel :conquest="conquest"></ConquestPanel>
            </div>
          </div>
        </template>
      </pv-card>

      <!-- 1999 日历 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('hexCalendar')" class="col-span-full">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-calendar"></i>
            <span>1999 日历</span>
          </div>
        </template>
        <template #content>
          <div v-for="c in rawWorldState?.KnownCalendarSeasons" :key="c.Season">
            <div class="flex items-center gap-2 mb-2">
              <span class="font-bold text-lg">{{ c.Season }}</span>
              <span class="text-sm text-muted-color"> <CountDownTo :time="c.Expiry" />轮换 </span>
            </div>
            <HexCalendarDays :days="c.Days" />
          </div>
        </template>
      </pv-card>

      <!-- 无尽回廊奖励 -->
      <pv-card v-if="wsPrefs.enabledSections.includes('endlessXp')">
        <template #title>
          <div class="flex items-center gap-2">
            <i class="pi pi-infinity"></i>
            <span>无尽回廊</span>
          </div>
        </template>
        <template #content>
          <div class="flex flex-col gap-3">
            <div v-for="cat in rawWorldState?.EndlessXpChoices" :key="cat.Category">
              <div class="font-bold mb-2">
                {{ cat.Category === 'EXC_NORMAL' ? '普通' : '困难' }}
              </div>
              <ul class="list-disc list-outside pl-4 text-sm space-y-1">
                <li v-for="c in cat.Choices" :key="c">{{ c }}</li>
              </ul>
            </div>
          </div>
        </template>
      </pv-card>
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
import {
  get_raw_world_state,
  type Invasion,
  type RawWorldState,
  type Reward,
} from '@/api/world_state'
import { useUserStore } from '@/stores/user'
import { storeToRefs } from 'pinia'
import { useToast } from 'primevue/usetoast'
import { computed, onActivated, onDeactivated, ref } from 'vue'
import dayjs from 'dayjs'
import isBetween from 'dayjs/plugin/isBetween'
import { useEventListener, useTimestamp } from '@vueuse/core'
import { clone, throttle } from 'lodash'
import StringWithIcon from '@/components/StringWithIcon.vue'
import { useWorldStatePrefsStore, type WorldStateSection } from '@/stores/world_state_prefs'

dayjs.extend(isBetween)

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

const cetusCycle = computed(() => {
  if (rawWorldState.value) {
    const cetusMission = rawWorldState.value.SyndicateMissions.find(
      (m) => m.Tag == 'CetusSyndicate',
    )

    if (cetusMission) {
      const activation_time = dayjs(cetusMission.Activation)

      const overlap = (now.value - activation_time.valueOf()) % (150 * 60 * 1000)
      const state = overlap < 100 * 60 * 1000 ? '白昼' : '黑夜'
      let expiry = ''
      if (overlap < 100 * 60 * 1000) {
        expiry = dayjs(now.value).add(100, 'm').subtract(overlap, 'ms').toISOString()
      } else {
        expiry = dayjs(now.value).add(150, 'm').subtract(overlap, 'ms').toISOString()
      }
      return {
        state,
        expiry,
      }
    }
  }

  return {
    state: 'unknown',
    expiry: '',
  }
})

const lStart = new Date('November 10, 2018 08:13:48 UTC')
const loopTime = 1600000
const coldTime = loopTime - 400000

const vallisCycle = computed(() => {
  const offset = (now.value - lStart.getUTCMilliseconds()) % loopTime
  if (offset < coldTime) {
    return {
      state: '寒冷',
      expiry: now.value - offset + coldTime,
    }
  } else {
    return {
      state: '温暖',
      expiry: now.value - offset + loopTime,
    }
  }
})

const cambionCycle = computed(() => {
  const { state, expiry } = earthCycle.value

  return {
    state: state === '白昼' ? '秩序' : '混乱',
    expiry,
  }
})

const corpusTimeMillis = 1655182800000
const fullCycle = 18000000
const stateMaximum = 9000000
const zarimanCycle = computed(() => {
  const offset = (now.value - corpusTimeMillis) % fullCycle
  if (offset < stateMaximum) {
    return {
      state: '克隆尼',
      expiry: now.value - offset + stateMaximum,
    }
  } else {
    return {
      state: '科普斯',
      expiry: now.value - offset + fullCycle,
    }
  }
})

const earthCycle = computed(() => {
  if (rawWorldState.value) {
    const offset = (now.value / 1000) % (8 * 60 * 60)
    if (offset < 4 * 60 * 60) {
      const expiry = now.value - offset * 1000 + 4 * 60 * 60 * 1000
      return {
        state: '黑夜',
        expiry,
      }
    } else {
      const expiry = now.value - offset * 1000 + 8 * 60 * 60 * 1000
      return {
        state: '白天',
        expiry,
      }
    }
  }
  return {
    state: 'unknown',
    expiry: '',
  }
})

function tsBetween(t: number, a: string, b: string) {
  return dayjs(t).isBetween(dayjs(a), dayjs(b))
}

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

function strCmp(a: string, b: string) {
  if (a == b) {
    return 0
  } else {
    return a > b ? 1 : -1
  }
}

const filterTiers = ref(['VoidT1', 'VoidT2', 'VoidT3', 'VoidT4', 'VoidT5', 'VoidT6'])

function selectAllTiers() {
  filterTiers.value = ['VoidT1', 'VoidT2', 'VoidT3', 'VoidT4', 'VoidT5', 'VoidT6']
}

function deselectAllTiers() {
  filterTiers.value = []
}

const normalFissures = computed(() => {
  if (!rawWorldState.value) {
    return []
  }
  return rawWorldState.value.ActiveMissions.filter((f) => !f.Hard)
    .filter((f) => {
      return filterTiers.value.includes(f.Modifier)
    })
    .sort((a, b) => strCmp(a.Modifier, b.Modifier))
})

const hardFissures = computed(() => {
  if (!rawWorldState.value) {
    return []
  }
  return rawWorldState.value.ActiveMissions.filter((f) => f.Hard)
    .filter((f) => {
      return filterTiers.value.includes(f.Modifier)
    })
    .sort((a, b) => strCmp(a.Modifier, b.Modifier))
})

const voidStorm = computed(() => {
  if (!rawWorldState.value) {
    return []
  }
  return clone(rawWorldState.value.VoidStorms)
    .filter((f) => {
      return filterTiers.value.includes(f.ActiveMissionTier)
    })
    .sort((a, b) => strCmp(a.ActiveMissionTier, b.ActiveMissionTier))
})

async function forceFetch() {
  toast.add({
    severity: 'warn',
    summary: 'TODO',
    detail: '强制更新世界状态数据暂未实现',
    life: 3000,
  })
}

const onTimeUp = throttle(fetchWorldState, 3000)

onTimeUp()
function timeUp() {
  onTimeUp()
}

function tierAsString(t: string): string {
  switch (t) {
    case 'VoidT1':
      return '古纪'
    case 'VoidT2':
      return '前纪'
    case 'VoidT3':
      return '中纪'
    case 'VoidT4':
      return '后纪'
    case 'VoidT5':
      return '安魂'
    case 'VoidT6':
      return '全能'
    default:
      return t
  }
}

function rewardAsString(reward: Reward): string {
  if (!reward) {
    return ''
  }
  const l = []
  if (reward.credits) {
    l.push(`${reward.credits} 星币`)
  }
  if (reward.xp) {
    l.push(`${reward.xp} 经验值`)
  }
  if (reward.itemNames) {
    l.push(...reward.itemNames.map((i) => `${i}`))
  }
  if (reward.countedItems) {
    l.push(...reward.countedItems.map((i) => `${i.ItemCountSum} X ${i.ItemName}`))
  }

  return l.join(' + ')
}

const { isAdmin } = storeToRefs(useUserStore())

function calcInvasionProgress(invasion: Invasion): number {
  const vsInfestation = invasion.AttackerMissionInfo.faction === '感染者'
  const pct = (1 + invasion.Count / invasion.Goal) * (vsInfestation ? 100 : 50)

  return Math.min(100, Math.max(0, Math.round(pct)))
}
</script>
