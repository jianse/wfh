<template>
  <div class="flex flex-row place-content-around m-2">
    <div class="w-[1080px] flex flex-col gap-2">
      <div class="flex flex-row gap-2">
        <pv-card class="min-w-4xs grow h-fit">
          <template #title>夜灵平野</template>
          <template #content>
            {{ worldState?.cetusCycle.state === 'night' ? '夜晚' : '白天' }}
            <p>
              <CountDownTo
                :time="worldState?.cetusCycle.expiry"
                @hit-zero="cetusCycleChange"
              ></CountDownTo
              >结束
            </p>
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
            <div v-for="event in worldState?.events" :key="event.id">
              {{ event.description }} {{ event.node }} <CountDownTo :time="event.expiry" />结束

              <div>{{ event.tooltip }}</div>
              <pv-progress-bar
                class="mt-1"
                v-if="event.health !== undefined"
                :value="event.health"
                :max="100"
              />
            </div>
          </div>

          <!--  <code class="font-mono">
            <pre>{{ worldState?.events }}</pre>
          </code> -->
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>警报</template>
        <template #content>
          <div class="flex flex-row gap-2 flex-wrap">
            <div v-for="alert in worldState?.alerts" :key="alert.id">
              {{ alert.mission.node }} {{ alert.mission.type }}
              <p>{{ alert.mission.description }}</p>
              <p><CountDownTo :time="alert.expiry" />结束</p>
              <p>{{ alert.mission.reward.asString }}</p>

              <!-- <div>{{ event.tooltip }}</div> -->
            </div>
          </div>

          <!-- <code class="font-mono">
          <pre>{{ worldState?.alerts[0] }}</pre>
        </code> -->
        </template>
      </pv-card>

      <pv-card class="min-w-sm h-fit col-span-2">
        <template #title>虚空裂缝</template>
        <template #content>
          <div class="flex flex-row gap-5 flex-wrap">
            <div>
              <h2 class="text-lg">普通</h2>
              <ul>
                <li
                  class="list-disc list-inside"
                  v-for="fissure in normalFissures"
                  :key="fissure.id"
                >
                  {{ fissure.tier }} {{ fissure.node }} {{ fissure.missionType }}
                  {{ fissure.isHard ? '钢铁之路' : '' }} {{ fissure.isStorm ? '虚空暴风' : '' }}
                  <br />
                  <CountDownTo :time="fissure.expiry"></CountDownTo>结束
                  {{ fissure.expired ? '已结束' : '' }}

                  <!-- {{ fissure }} -->
                </li>
              </ul>
            </div>
            <div>
              <h2 class="text-lg">钢铁之路</h2>
              <ul>
                <li class="list-disc list-inside" v-for="fissure in hardFissures" :key="fissure.id">
                  {{ fissure.tier }} {{ fissure.node }} {{ fissure.missionType }}
                  <!-- {{ fissure.isHard ? '钢铁之路' : '' }} {{ fissure.isStorm ? '虚空暴风' : '' }} -->
                  <br />
                  <CountDownTo :time="fissure.expiry"></CountDownTo>结束
                  {{ fissure.expired ? '已结束' : '' }}

                  <!-- {{ fissure }} -->
                </li>
              </ul>
            </div>
            <div>
              <h2 class="text-lg">虚空风暴</h2>
              <ul>
                <li class="list-disc list-inside" v-for="fissure in voidStorm" :key="fissure.id">
                  {{ fissure.tier }} {{ fissure.node }} {{ fissure.missionType }}
                  <!-- {{ fissure.isHard ? '钢铁之路' : '' }} {{ fissure.isStorm ? '虚空暴风' : '' }} -->
                  <br />
                  <CountDownTo :time="fissure.expiry"></CountDownTo>结束
                  {{ fissure.expired ? '已结束' : '' }}

                  <!-- {{ fissure }} -->
                </li>
              </ul>
            </div>
          </div>

          <!-- <div v-for="fissure in worldState?.fissures" :key="fissure.id">

          {{ fissure }}
        </div> -->
        </template>
      </pv-card>

      <div class="flex flex-col gap-2 flex-1">
        <pv-card class="h-fit">
          <template #title>钢铁之路每周</template>
          <template #content>
            {{ worldState?.steelPath.currentReward.name }}
            {{ worldState?.steelPath.currentReward.cost }}钢铁精华

            <p><CountDownTo :time="worldState?.steelPath.expiry" /> 轮换</p>
            <!-- {{ worldState?.steelPath }} -->
          </template>
        </pv-card>

        <pv-card class="h-fit">
          <template #title>达尔沃的特惠商品</template>
          <template #content>
            <ul>
              <li v-for="deal in worldState?.dailyDeals" :key="deal.item">
                {{ deal.item }} {{ deal.salePrice }}P <CountDownTo :time="deal.expiry" />轮换
              </li>
            </ul>
            <!-- {{ worldState?.dailyDeals }} -->
          </template>
        </pv-card>

        <pv-card class="h-fit">
          <template #title>执刑官猎杀</template>
          <template #content>
            <p><CountDownTo :time="worldState?.archonHunt?.expiry" />轮换</p>
            <ul>
              <li v-for="(mission, index) in worldState?.archonHunt.missions" :key="index">
                {{ mission.node }} {{ mission.type }}
              </li>
            </ul>
            <!-- {{ worldState?.archonHunt }} -->
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
          </template>
        </pv-card>
        <pv-card class="h-fit">
          <template #title>突击任务</template>
          <template #content>
            <p><CountDownTo :time="worldState?.sortie?.expiry" />轮换</p>
            <ul>
              <li v-for="(mission, index) in worldState?.sortie.variants" :key="index">
                {{ mission.node }} {{ mission.missionType }} {{ mission.modifier }}
              </li>
            </ul>
            <!-- {{ worldState?.archonHunt }} -->
          </template>
        </pv-card>
      </div>

      <pv-card class="h-fit">
        <template #title>虚空商人</template>
        <template #content>
          <div>{{ worldState?.voidTrader.active ? '已到来' : '未到来' }}</div>
          <div>{{ worldState?.voidTrader.location }}</div>
          <div v-if="worldState?.voidTrader.active">
            <CountDownTo :time="worldState?.voidTrader.expiry"></CountDownTo> 离开
          </div>
          <div v-else>
            <CountDownTo :time="worldState?.voidTrader.activation"></CountDownTo> 到达
          </div>
          <pv-data-table
            v-if="worldState?.voidTrader.active"
            :value="worldState?.voidTrader.inventory"
            scrollable
            scroll-height="250px"
          >
            <pv-column field="item" header="物品" style="width: 50%"></pv-column>
            <pv-column field="ducats" header="杜卡德金币" style="width: 30%"></pv-column>
            <pv-column field="credits" header="星币" style="width: 20%"></pv-column>
          </pv-data-table>
          <!-- {{ worldState?.voidTrader }} -->
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>圣装重生</template>
        <template #content>
          <p><CountDownTo :time="worldState?.vaultTrader.expiry" /> 轮换</p>
          <!-- {{ worldState?.vaultTrader}} -->
          <pv-data-table
            v-if="worldState?.vaultTrader.active"
            :value="worldState?.vaultTrader.inventory"
          >
            <pv-column field="item" header="物品"></pv-column>
            <pv-column field="ducats" header="御品阿耶精华" style="width: 30%"></pv-column>
            <pv-column field="credits" header="阿耶精华" style="width: 25%"></pv-column>
          </pv-data-table>
        </template>
      </pv-card>

      <pv-card class="h-fit flex-1">
        <template #title>午夜电波</template>
        <template #content>
          <ul class="list-disc list-inside">
            <li v-for="c in worldState?.nightwave.activeChallenges" :key="c.id">
              <p class="inline-flex place-content-between" style="width: calc(100% - 1rem)">
                <span
                  >{{ c.title }} {{ c.isDaily ? '每日' : '' }} {{ c.isElite ? '精英' : '' }}</span
                >
                <span>{{ c.reputation }}</span>
              </p>

              <p><CountDownTo :time="c.expiry" />结束</p>

              <p>{{ c.desc }}</p>
            </li>
          </ul>

          <!-- {{ worldState?.nightwave }} -->
        </template>
      </pv-card>

      <pv-card class="h-fit">
        <template #title>入侵任务</template>
        <template #content>
          <div
            v-for="invasion in worldState?.invasions"
            :key="invasion.id"
            class="inline-block w-full"
          >
            <p>{{ invasion.node }}</p>

            <div class="flex flex-row place-content-between">
              <div class="flex flex-col">
                <div>{{ invasion.attackingFaction }}</div>
                <div>{{ invasion.attackerReward.asString }}</div>
              </div>
              <div class="flex flex-col items-end">
                <div class="w-fit">{{ invasion.defendingFaction }}</div>
                <div class="w-fit">{{ invasion.defenderReward.asString }}</div>
              </div>
            </div>

            <!-- {{ invasion.defendingReward.item }} {{ invasion.defendingFaction }} -->
            <pv-progress-bar :value="Math.abs(invasion.completion)">
              {{ Math.abs(invasion.count) }} / {{ invasion.requiredRuns }}</pv-progress-bar
            >
          </div>

          <!-- <code>
          <pre>{{ JSON.stringify(worldState?.invasions[1],undefined,2) }}</pre>
        </code> -->
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
import { get_world_state, update_and_get, type WorldState } from '@/api/world_state'
import { useUserStore } from '@/stores/user'
import { storeToRefs } from 'pinia'
import { useToast } from 'primevue/usetoast'
import { computed, ref } from 'vue'
import dayjs from 'dayjs'

const worldState = ref<WorldState>()
const loading = ref(false)
const toast = useToast()

async function fetchWorldState() {
  loading.value = true
  try {
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
  if (!worldState.value) {
    return []
  }
  return worldState.value.fissures
    .filter((f) => !f.isHard && !f.isStorm)
    .sort((a, b) => strCmp(a.tierNum, b.tierNum))
})

const hardFissures = computed(() => {
  if (!worldState.value) {
    return []
  }
  return worldState.value.fissures
    .filter((f) => f.isHard)
    .sort((a, b) => strCmp(a.tierNum, b.tierNum))
})

const voidStorm = computed(() => {
  if (!worldState.value) {
    return []
  }
  return worldState.value.fissures
    .filter((f) => f.isStorm)
    .sort((a, b) => strCmp(a.tierNum, b.tierNum))
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

function cetusCycleChange() {
  if (!worldState.value) {
    return
  }
  const now_state = worldState.value.cetusCycle.state
  if (now_state == 'day') {
    worldState.value.cetusCycle.state = 'night'
    worldState.value.cetusCycle.expiry = dayjs(worldState.value.cetusCycle.expiry)
      .add(50, 'minute')
      .toISOString()
  } else {
    worldState.value.cetusCycle.state = 'day'
    worldState.value.cetusCycle.expiry = dayjs(worldState.value.cetusCycle.expiry)
      .add(100, 'minute')
      .toISOString()
  }
}

const { isAdmin } = storeToRefs(useUserStore())
</script>
