<template>
  <div class="flex flex-row place-content-around m-2">
    <div class="w-[1080px] flex flex-col gap-2">
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
            {{ worldState?.vallisCycle.state === 'cold' ? '寒冷' : '温暖' }}
            <p>
              <CountDownTo
                :time="worldState?.vallisCycle.expiry"
                @hit-zero="fetchWorldState"
              ></CountDownTo
              >结束
            </p>
          </template>
        </pv-card>
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>魔胎之境</template>
          <template #content>
            {{ worldState?.cambionCycle.active }}
            <p>
              <CountDownTo
                :time="worldState?.cambionCycle.expiry"
                @hit-zero="fetchWorldState"
              ></CountDownTo
              >结束
            </p>
          </template>
        </pv-card>
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>扎里曼号</template>
          <template #content>
            {{ worldState?.zarimanCycle.state }}
            <p>
              <CountDownTo
                :time="worldState?.zarimanCycle.expiry"
                @hit-zero="fetchWorldState"
              ></CountDownTo
              >结束
            </p>
          </template>
        </pv-card>
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>地球</template>
          <template #content>
            {{ worldState?.earthCycle.state === 'night' ? '夜晚' : '白天' }}
            <p>
              <CountDownTo
                :time="worldState?.earthCycle.expiry"
                @hit-zero="fetchWorldState"
              ></CountDownTo
              >结束
            </p>
          </template>
        </pv-card>
      </div>

      <pv-card class="h-fit">
        <template #title>活动</template>
        <template #content>
          <div class="flex flex-col gap-2">
            <div v-for="event in rawWorldState?.Goals" :key="event._id">
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
              <p>{{ rewardAsString(alert.MissionInfo.missionReward) }}</p>
              <!-- TODO: show other rewards except credits -->
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

        <pv-card class="h-fit">
          <template #title>灵煞异常</template>
          <template #content>
            <p><CountDownTo :time="worldState?.sentientOutposts?.expiry" />轮换</p>
            <div>
              {{ worldState?.sentientOutposts.mission.node }}
              {{ worldState?.sentientOutposts.mission.type }}
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
              <pv-data-table
                v-if="true"
                :value="voidTrader.Manifest"
                scrollable
                scroll-height="600px"
              >
                <pv-column field="ItemType" header="物品" style="width: 50%"></pv-column>
                <pv-column field="PrimePrice" header="杜卡德金币" style="width: 30%"></pv-column>
                <pv-column field="RegularPrice" header="星币" style="width: 20%"></pv-column>
              </pv-data-table>
              <div><CountDownTo :time="voidTrader.Expiry"></CountDownTo> 离开</div>
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

          <!-- TODO: add nightwave description and rewards -->
          <!-- maybe we can add nightwave level rewards also -->
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
              <div class="flex flex-col">
                <div>{{ invasion.DefenderMissionInfo.faction }}</div>
                <div>{{ rewardAsString(invasion.AttackerReward) }}</div>
              </div>
              <div class="flex flex-col items-end">
                <div class="w-fit">{{ invasion.AttackerMissionInfo.faction }}</div>
                <div class="w-fit">{{ rewardAsString(invasion.DefenderReward) }}</div>
              </div>
            </div>

            <pv-progress-bar :value="70">
              {{ Math.abs(invasion.Count) }} / {{ invasion.Goal }}</pv-progress-bar
            >
            <!-- TODO: update progress calc here -->
          </div>
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>每周科研</template>
        <template #content>
          <div class="flex flex-row gap-2">
            <div v-for="conquest in rawWorldState?.Conquests" :key="conquest.Type">
              <p>{{ conquest.Type }}</p>
              <CountDownTo :time="conquest.Expiry"></CountDownTo>轮换

              <h3 class="mt-2 text-lg font-bold">任务</h3>
              <div v-for="mission in conquest.Missions" :key="mission.faction">
                <div>{{ mission.faction }}</div>
                <div>{{ mission.missionType }}</div>
              </div>

              <h3 class="mt-2 text-lg font-bold">可选改造</h3>
              <ul class="list-disc list-outside pl-4">
                <li v-for="variable in conquest.Variables" :key="variable">
                  {{ variable }}
                </li>
              </ul>
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
  get_world_state,
  update_and_get,
  type RawWorldState,
  type Reward,
  type WorldState,
} from '@/api/world_state'
import { useUserStore } from '@/stores/user'
import { storeToRefs } from 'pinia'
import { useToast } from 'primevue/usetoast'
import { computed, ref } from 'vue'
import dayjs from 'dayjs'
import isBetween from 'dayjs/plugin/isBetween'
import { useTimestamp } from '@vueuse/core'
import { clone, throttle } from 'lodash'

dayjs.extend(isBetween)

const worldState = ref<WorldState>()
const rawWorldState = ref<RawWorldState>()
const loading = ref(false)
const toast = useToast()

const now = useTimestamp({ interval: 1000 })

const cetusCycle = computed(() => {
  if (rawWorldState.value) {
    const cetusMission = rawWorldState.value.SyndicateMissions.find(
      (m) => m.Tag == 'CetusSyndicate',
    )

    if (cetusMission) {
      const activation_time = dayjs(cetusMission.Activation)
      const now = Date.now()

      const overlap = ((now - activation_time.valueOf()) / 1000) % (150 * 60)
      console.log(overlap)
      const state = overlap < 100 * 60 ? '白昼' : '黑夜'
      let expiry = ''
      if (overlap < 100 * 60) {
        expiry = dayjs(now).add(100, 'm').subtract(overlap, 's').toISOString()
      } else {
        expiry = dayjs(now).add(150, 'm').subtract(overlap, 's').toISOString()
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

function tsBetween(t: number, a: string, b: string) {
  return dayjs(t).isBetween(dayjs(a), dayjs(b))
}

async function fetchWorldState() {
  loading.value = true
  try {
    const { data: data1 } = await get_raw_world_state()
    console.log(data1)
    rawWorldState.value = data1
    const { data } = await get_world_state()
    if (data) {
      worldState.value = data
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
  loading.value = true
  try {
    const { data } = await update_and_get()
    if (data) {
      worldState.value = data
      toast.add({
        severity: 'success',
        summary: '更新成功',
        detail: '已强制更新世界状态数据',
        life: 3000,
      })
    }
  } finally {
    loading.value = false
  }
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
</script>
