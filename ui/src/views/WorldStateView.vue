<template>
  <div class="flex flex-row place-content-around m-2">
    <div class="w-270 flex flex-col gap-2">
      <div class="flex flex-row gap-2">
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>夜灵平野</template>
          <template #content>
            {{ cetusCycle.state }}
            <p><CountDownTo :time="cetusCycle.expiry" @hit-zero="timeUp"></CountDownTo>切换</p>
          </template>
        </pv-card>
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>奥布山谷</template>
          <template #content>
            {{ vallisCycle.state }}
            <p><CountDownTo :time="vallisCycle.expiry" @hit-zero="timeUp"></CountDownTo>结束</p>
          </template>
        </pv-card>
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>魔胎之境</template>
          <template #content>
            {{ cambionCycle.state }}
            <p><CountDownTo :time="cambionCycle.expiry" @hit-zero="timeUp"></CountDownTo>结束</p>
          </template>
        </pv-card>
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>扎里曼号</template>
          <template #content>
            {{ zarimanCycle.state }}
            <p><CountDownTo :time="zarimanCycle.expiry" @hit-zero="timeUp"></CountDownTo>结束</p>
          </template>
        </pv-card>
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>地球</template>
          <template #content>
            {{ earthCycle.state }}
            <p><CountDownTo :time="earthCycle.expiry" @hit-zero="timeUp"></CountDownTo>结束</p>
          </template>
        </pv-card>
      </div>

      <pv-card class="h-fit">
        <template #title>活动</template>
        <template #content>
          <div class="flex flex-col gap-2">
            <div v-for="event in rawWorldState?.Goals" :key="event._id">
              <!-- {{ event }} -->
              {{ event.Desc }} {{ event.Node }} <CountDownTo :time="event.Expiry" />结束

              <div>{{ event.ToolTip }}</div>
              <!-- community goal has an global progress bar,
               that indicates the progress of the goal for all players. -->
              <pv-progress-bar
                class="mt-1"
                v-if="event.Community"
                :value="event.Count"
                :max="event.Goal"
              />

              <!-- ghoul emergence has a health bar -->
              <pv-progress-bar
                class="mt-1"
                v-if="event.Tag === 'GhoulEmergence'"
                :value="event.HealthPct * 100"
              />
            </div>
          </div>
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>警报</template>
        <template #content>
          <div class="flex flex-row gap-2 flex-wrap">
            <div
              class="border rounded-md p-3 border-slate-400"
              v-for="alert in rawWorldState?.Alerts"
              :key="alert._id"
            >
              {{ alert.MissionInfo.location }} {{ alert.MissionInfo.missionType }}
              <p>{{ alert.MissionInfo.descText }}</p>
              <p><CountDownTo :time="alert.Expiry" />结束</p>
              <StringWithIcon :text="rewardAsString(alert.MissionInfo.missionReward)" />
            </div>
          </div>
        </template>
      </pv-card>

      <pv-card class="min-w-sm h-fit col-span-2">
        <template #title>虚空裂缝</template>
        <template #content>
          <div class="flex flex-row gap-5 flex-wrap">
            <div>
              <h2 class="text-lg">普通</h2>
              <ul class="list-disc list-outside pl-4">
                <li v-for="fissure in normalFissures" :key="fissure._id">
                  {{ tierAsString(fissure.Modifier) }} {{ fissure.Node }} {{ fissure.MissionType }}
                  <br />
                  <CountDownTo :time="fissure.Expiry" @hit-zero="timeUp"></CountDownTo>结束
                </li>
              </ul>
            </div>
            <div>
              <h2 class="text-lg">钢铁之路</h2>
              <ul class="list-disc list-outside pl-4">
                <li v-for="fissure in hardFissures" :key="fissure._id">
                  {{ tierAsString(fissure.Modifier) }} {{ fissure.Node }} {{ fissure.MissionType }}
                  <br />
                  <CountDownTo :time="fissure.Expiry" @hit-zero="timeUp"></CountDownTo>结束
                </li>
              </ul>
            </div>
            <div>
              <h2 class="text-lg">虚空风暴</h2>
              <ul class="list-disc list-outside pl-4">
                <li v-for="fissure in voidStorm" :key="fissure._id">
                  {{ tierAsString(fissure.ActiveMissionTier) }} {{ fissure.Node }}
                  <!-- find mission type from node information -->
                  <br />
                  <CountDownTo :time="fissure.Expiry" @hit-zero="timeUp"></CountDownTo>结束
                </li>
              </ul>
            </div>
          </div>
        </template>
      </pv-card>

      <div class="flex flex-col gap-2 flex-1">
        <pv-card class="h-fit">
          <template #title>钢铁之路每周</template>
          <template #content>
            {{ rawWorldState?.SteelPath.current_reward.name }}
            {{ rawWorldState?.SteelPath.current_reward.cost }}钢铁精华

            <p><CountDownTo :time="rawWorldState?.SteelPath.Expiry" /> 轮换</p>
            <!-- TODO:move steel path reward calc from backend to here -->
          </template>
        </pv-card>

        <pv-card class="h-fit">
          <template #title>达尔沃的特惠商品</template>
          <template #content>
            <ul>
              <li v-for="deal in rawWorldState?.DailyDeals" :key="deal.StoreItem">
                {{ deal.StoreItemName }} {{ deal.SalePrice }}P
                <CountDownTo :time="deal.Expiry" />轮换
              </li>
            </ul>
          </template>
        </pv-card>

        <pv-card class="h-fit">
          <template #title>执刑官猎杀</template>
          <template #content>
            <div v-for="hunt in rawWorldState?.LiteSorties" :key="hunt._id">
              <h3 class="text-bold text-lg mb-1">BOSS: {{ hunt.BossName }}</h3>
              <p><CountDownTo :time="hunt.Expiry" />轮换</p>
              <ul>
                <li v-for="(mission, index) in hunt.Missions" :key="index">
                  {{ mission.node }} {{ mission.missionType }}
                </li>
              </ul>
            </div>
          </template>
        </pv-card>

        <pv-card class="h-fit" v-if="rawWorldState?.SentientOutpost">
          <template #title>灵煞异常</template>
          <template #content>
            <p><CountDownTo :time="rawWorldState?.SentientOutpost.expiry" />轮换</p>
            <div>
              {{ rawWorldState?.SentientOutpost.node_name || rawWorldState?.SentientOutpost.node }}
              {{ rawWorldState?.SentientOutpost.mission_type }}
            </div>
            <!-- TODO: move sentient outposts info from backend to here -->
          </template>
        </pv-card>

        <pv-card class="h-fit">
          <template #title>突击任务</template>
          <template #content>
            <div v-for="sortie in rawWorldState?.Sorties" :key="sortie._id">
              <p><CountDownTo :time="sortie.Expiry" />轮换</p>
              <ul>
                <li v-for="(mission, index) in sortie.Variants" :key="index">
                  {{ mission.node }} {{ mission.missionType }} {{ mission.modifierType }}
                </li>
              </ul>
            </div>
          </template>
        </pv-card>
      </div>

      <pv-card class="h-fit">
        <template #title>虚空商人</template>
        <template #content>
          <div v-for="(voidTrader, index) in rawWorldState?.VoidTraders" :key="index">
            <div v-if="tsBetween(now, voidTrader.Activation, voidTrader.Expiry)">
              <div>已到来</div>
              <div>{{ voidTrader.Node }}</div>
              <div><CountDownTo :time="voidTrader.Expiry"></CountDownTo> 离开</div>
              <pv-data-table
                v-if="true"
                :value="voidTrader.Manifest"
                scrollable
                scroll-height="600px"
              >
                <pv-column field="ItemName" header="物品" style="width: 50%"></pv-column>
                <pv-column field="PrimePrice" header="杜卡德金币" style="width: 30%"></pv-column>
                <pv-column field="RegularPrice" header="星币" style="width: 20%"></pv-column>
              </pv-data-table>
            </div>
            <div v-else>
              <div>未到来</div>
              <div>{{ voidTrader.Node }}</div>
              <div><CountDownTo :time="voidTrader.Activation"></CountDownTo> 到达</div>
            </div>
          </div>
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>圣装重生</template>
        <template #content>
          <div v-for="pvTrader in rawWorldState?.PrimeVaultTraders" :key="pvTrader._id">
            <p><CountDownTo :time="pvTrader.Expiry" /> 轮换</p>
            <pv-tabs value="rotation">
              <pv-tab-list>
                <pv-tab value="rotation">轮换</pv-tab>
                <pv-tab value="evergreen">常驻</pv-tab>
              </pv-tab-list>
              <pv-tab-panels>
                <pv-tab-panel value="rotation">
                  <pv-data-table :value="pvTrader.Manifest" scrollable scroll-height="600px">
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
                    scroll-height="600px"
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

      <pv-card class="h-fit flex-1">
        <template #title>午夜电波</template>
        <template #content>
          <ul class="list-disc list-outside ml-4">
            <li v-for="c in rawWorldState?.SeasonInfo.ActiveChallenges" :key="c._id">
              <p class="inline-flex place-content-between" style="width: calc(100% - 1rem)">
                <span>{{ c.Name }} <span v-if="c.Daily">每日</span></span>
                <span>{{ c.Xp }} </span>
              </p>
              <p>{{ c.Desc }}</p>
              <p><CountDownTo :time="c.Expiry" />结束</p>
            </li>
          </ul>

          <!-- TODO: maybe we can add nightwave level rewards also -->
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>入侵任务</template>
        <template #content>
          <div
            v-for="invasion in rawWorldState?.Invasions"
            :key="invasion._id"
            class="inline-block w-full"
          >
            <p>{{ invasion.Node }}</p>

            <div class="flex flex-row place-content-between">
              <div class="flex flex-col flex-1">
                <div>{{ invasion.DefenderMissionInfo.faction }}</div>
                <div>{{ rewardAsString(invasion.AttackerReward) }}</div>
              </div>
              <!-- {{ invasion.Count }} -->
              <div class="text-center flex-1">
                {{ Math.abs(invasion.Count) }} / {{ invasion.Goal }}
              </div>
              <div class="flex flex-col items-end flex-1">
                <div class="w-fit">{{ invasion.AttackerMissionInfo.faction }}</div>
                <div class="w-fit">{{ rewardAsString(invasion.DefenderReward) }}</div>
              </div>
            </div>

            <pv-progress-bar :value="calcInvasionProgress(invasion)" />

            <!-- TODO: update progress calc here -->
          </div>
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>每周科研</template>
        <template #content>
          <div class="flex flex-row gap-2">
            <div v-for="conquest in rawWorldState?.Conquests" :key="conquest.Type" class="flex-1">
              <ConquestPanel :conquest="conquest"></ConquestPanel>
            </div>
          </div>
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>1999日历</template>
        <template #content>
          <div v-for="c in rawWorldState?.KnownCalendarSeasons" :key="c.Season">
            {{ c.Season }}
            <CountDownTo :time="c.Expiry" />轮换
            <HexCalendar :season="c.Season"> </HexCalendar>
          </div>
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>无尽回廊奖励</template>
        <template #content>
          <div class="flex flex-row gap-3">
            <div v-for="cat in rawWorldState?.EndlessXpChoices" :key="cat.Category">
              {{ cat.Category }}
              <p v-for="c in cat.Choices" :key="c">{{ c }}</p>
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
      @click="fetchWorldState"
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
import { useTimestamp } from '@vueuse/core'
import { clone, throttle } from 'lodash'
import StringWithIcon from '@/components/StringWithIcon.vue'

dayjs.extend(isBetween)

// const worldState = ref<WorldState>()
const rawWorldState = ref<RawWorldState>()
const loading = ref(false)
const toast = useToast()

const fetchEnable = ref(true)

onDeactivated(() => {
  fetchEnable.value = false
})

onActivated(() => {
  fetchEnable.value = true
  onTimeUp()
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
const warmTime = 400000
const coldTime = loopTime - warmTime

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
    state: state === '白昼' ? '混乱' : '秩序',
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
    const offset = (now.value / 1000) % (8 * 60 * 60) // 8 hours
    if (offset < 4 * 60 * 60) {
      const expiry = now.value - offset * 1000 + 4 * 60 * 60 * 1000
      return {
        state: '白天',
        expiry,
      }
    } else {
      const expiry = now.value - offset * 1000 + 8 * 60 * 60 * 1000
      return {
        state: '黑夜',
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
  if (!fetchEnable.value) {
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

fetchWorldState()

function strCmp(a: string, b: string) {
  if (a == b) {
    return 0
  } else {
    return a > b ? 1 : -1
  }
}

const normalFissures = computed(() => {
  if (!rawWorldState.value) {
    return []
  }
  return rawWorldState.value.ActiveMissions.filter((f) => !f.Hard).sort((a, b) =>
    strCmp(a.Modifier, b.Modifier),
  )
})

const hardFissures = computed(() => {
  if (!rawWorldState.value) {
    return []
  }
  return rawWorldState.value.ActiveMissions.filter((f) => f.Hard).sort((a, b) =>
    strCmp(a.Modifier, b.Modifier),
  )
})

const voidStorm = computed(() => {
  if (!rawWorldState.value) {
    return []
  }
  return clone(rawWorldState.value.VoidStorms).sort((a, b) =>
    strCmp(a.ActiveMissionTier, b.ActiveMissionTier),
  )
})

async function forceFetch() {
  toast.add({
    severity: 'warn',
    summary: 'TODO',
    detail: '强制更新世界状态数据暂未实现',
    life: 3000,
  })
}

const onTimeUp = throttle(fetchWorldState, 3000, { leading: false })
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
  // return t
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
