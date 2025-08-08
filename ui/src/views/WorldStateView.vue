<template>
  <div class="flex flex-row gap-2 m-2 flex-wrap">
    <pv-card class="w-3xs h-fit">
      <template #title>夜灵平野</template>
      <template #content>
        {{ worldState?.cetusCycle.state === 'night' ? '夜晚' : '白天' }}
        <p>
          <CountDownTo
            :time="worldState?.cetusCycle.expiry"
            @hit-zero="fetchWorldState"
          ></CountDownTo>
        </p>
      </template>
    </pv-card>
    <pv-card class="w-3xs h-fit">
      <template #title>奥布山谷</template>
      <template #content>
        {{ worldState?.vallisCycle.state === 'cold' ? '寒冷' : '温暖' }}
        <p>
          <CountDownTo
            :time="worldState?.vallisCycle.expiry"
            @hit-zero="fetchWorldState"
          ></CountDownTo>
        </p>
      </template>
    </pv-card>
    <pv-card class="w-3xs h-fit">
      <template #title>魔胎之境</template>
      <template #content>
        {{ worldState?.cambionCycle.active }}
        <p>
          <CountDownTo
            :time="worldState?.cambionCycle.expiry"
            @hit-zero="fetchWorldState"
          ></CountDownTo>
        </p>
      </template>
    </pv-card>
    <pv-card class="w-3xs h-fit">
      <template #title>扎里曼号</template>
      <template #content>
        {{ worldState?.zarimanCycle.state }}
        <p>
          <CountDownTo
            :time="worldState?.zarimanCycle.expiry"
            @hit-zero="fetchWorldState"
          ></CountDownTo>
        </p>
      </template>
    </pv-card>
    <pv-card class="w-3xs h-fit">
      <template #title>地球</template>
      <template #content>
        {{ worldState?.earthCycle.state === 'night' ? '夜晚' : '白天' }}
        <p>
          <CountDownTo
            :time="worldState?.earthCycle.expiry"
            @hit-zero="fetchWorldState"
          ></CountDownTo>
        </p>
      </template>
    </pv-card>

    <!-- 虚空裂缝 -->
    <pv-card class="w-md">
      <template #title>虚空裂缝</template>
      <template #content>
        <!-- {{ worldState?.cetusCycle.state === 'night' ? '夜晚' : '白天' }} -->
        <h2>普通</h2>
        <ul>
          <li class="list-disc list-inside" v-for="fissure in normalFissures" :key="fissure.id">
            {{ fissure.tier }} {{ fissure.node }} {{ fissure.missionType }}
            {{ fissure.isHard ? '钢铁之路' : '' }} {{ fissure.isStorm ? '虚空暴风' : '' }}
            <br />
            <CountDownTo :time="fissure.expiry"></CountDownTo>结束
            {{ fissure.expired ? '已结束' : '' }}

            <!-- {{ fissure }} -->
          </li>
        </ul>
        <h2 class="pt-3">钢铁之路</h2>
        <ul>
          <li class="list-disc list-inside" v-for="fissure in hardFissures" :key="fissure.id">
            {{ fissure.tier }} {{ fissure.node }} {{ fissure.missionType }}
            {{ fissure.isHard ? '钢铁之路' : '' }} {{ fissure.isStorm ? '虚空暴风' : '' }}
            <br />
            <CountDownTo :time="fissure.expiry"></CountDownTo>结束
            {{ fissure.expired ? '已结束' : '' }}

            <!-- {{ fissure }} -->
          </li>
        </ul>

        <h2 class="pt-3">虚空风暴</h2>
        <ul>
          <li class="list-disc list-inside" v-for="fissure in voidStorm" :key="fissure.id">
            {{ fissure.tier }} {{ fissure.node }} {{ fissure.missionType }}
            {{ fissure.isHard ? '钢铁之路' : '' }} {{ fissure.isStorm ? '虚空暴风' : '' }}
            <br />
            <CountDownTo :time="fissure.expiry"></CountDownTo>结束
            {{ fissure.expired ? '已结束' : '' }}

            <!-- {{ fissure }} -->
          </li>
        </ul>
        <!-- <div v-for="fissure in worldState?.fissures" :key="fissure.id">

          {{ fissure }}
        </div> -->
      </template>
    </pv-card>
  </div>
</template>
<script setup lang="ts">
import { get_world_state, type WorldState } from '@/api/world_state'
import { computed, ref } from 'vue'

const worldState = ref<WorldState>()

async function fetchWorldState() {
  const { data } = await get_world_state()
  if (data) {
    worldState.value = data
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
</script>
