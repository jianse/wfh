<template>
  <!-- <div>??? no mod</div> -->
  <div class="relative h-[420px] w-[256px]">
    <!-- frame top -->
    <img class="absolute" draggable="false" src="./head.png" :style="headStyle" />

    <!-- frame bg -->
    <img class="absolute" draggable="false" src="./bg.png" style="left: 0; top: -44px" />

    <!-- content bg -->

    <div class="absolute" style="top: 34px; left: 7px">
      <!-- content body -->
      <div class="flex flex-col h-[295px] w-[242px]">
        <!-- cover image -->
        <div class="shrink-30 overflow-hidden __cover">
          <img
            loading="lazy"
            draggable="false"
            v-if="props.mod?.textureLocation"
            :src="'https://content.warframe.com/PublicExport' + props.mod.textureLocation"
            style="width: 242px; height: 242px"
          />
        </div>

        <!-- title & effect -->
        <div class="w-full" :style="{ color: pColor }" style="z-index: 6">
          <!-- title -->
          <div class="text-center text-[24px]">{{ props.mod.name }}</div>
          <div class="text-center px-3" v-for="(stat, index) in effect" :key="index">
            <string-with-icon :text="stat" />
          </div>
        </div>
      </div>
    </div>

    <!-- frame bottom -->
    <img class="absolute" draggable="false" src="./bottom.png" style="left: -128px; top: 217px" />

    <!-- side light -->
    <img
      class="absolute"
      draggable="false"
      src="./side.png"
      style="top: 16px; width: 32px; height: 370px"
    />
    <img
      class="absolute"
      draggable="false"
      src="./side.png"
      style="top: 16px; right: 0; transform: scaleX(-1); width: 32px; height: 370px"
    />

    <!-- top right marker -->
    <div
      class="absolute flex flex-row place-content-center place-items-center pb-1 pr-3 __trMarker"
      v-if="props.mod.polarity !== 'AP_UNIVERSAL'"
      style="right: 0; top: 45px"
      :style="trMarkerStyle"
    >
      <div class="text-[18px] font-bold" style="line-height: 1" :style="{ color: pColor }">
        {{ drain }}
      </div>
      <div class="inline-block z-5 ms-0.5" :style="polarityIconStyle"></div>
    </div>

    <!-- bottom type marker -->
    <div class="absolute z-5" style="top: 347px; left: 23px">
      <img draggable="false" src="@/assets/mod/GoldLowerTab.png" />
      <div class="absolute top-0 left-0 right-0 bottom-0 content-center text-center">
        <span class="text-bold" :style="{ color: pColor }">{{ props.mod.compatName }}</span>
      </div>
    </div>

    <!-- max level marker -->
    <img
      class="absolute z-3"
      style="left: 0; top: 396px"
      v-if="rankMaxed"
      src="./RankCompleteLine.png"
    />

    <!-- rank stars -->
    <div class="absolute z-3" :style="rankStartsStyle">
      <img
        class="inline-block"
        draggable="false"
        v-for="i in currentRank"
        :key="i"
        src="./rankStar2.png"
      />
      <img
        class="inline-block"
        draggable="false"
        v-for="i in props.mod.fusionLimit - currentRank"
        :key="i"
        src="@/assets/mod/RankSlotEmpty.png"
      />
    </div>
    <!-- top marker -->
    <img draggable="false" class="absolute z-5" src="./top.png" style="top: 19px; left: 96px" />
  </div>
  {{ props.mod.rarity }}
  {{ props.mod.polarity }}
</template>

<script lang="ts" setup>
import polarity from '@/assets/polarity'
import type { Upgrade } from '@/util/localDB'
// import { head } from 'lodash';
import { computed, ref } from 'vue'

const props = defineProps<{ mod: Upgrade; maxRank?: boolean }>()

const headStyle = {
  left: (256 - 512) / 2 + 'px',
  top: 0,
  zIndex: 4,
}

const currentRank = computed(() => {
  if (props.maxRank) {
    return props.mod.fusionLimit
  }
  return props.mod.fusionLimit
})

const pColor = ref('#AC995E')
const effect = computed(() => {
  if (props.mod?.type === 'STANCE') {
    return props.mod.description || props.mod?.levelStats?.[currentRank.value]?.stats
  }
  return props.mod?.levelStats?.[currentRank.value]?.stats
})

const rankMaxed = computed(() => {
  return props.mod?.fusionLimit === currentRank.value
})

const drain = computed(() => {
  // if (props.mod.)
  return Math.abs(props.mod.baseDrain) + currentRank.value
})
const polarityIconStyle = computed(() => {
  const mask_url = polarity[props.mod.polarity] || polarity.AP_ATTACK
  const style = {
    height: '14px',
    width: '14px',
    backgroundColor: pColor.value,
    top: '5px',
    right: '4px',
    mask: `url(${mask_url})`,
    maskSize: 'cover',
  }
  return style
})

const rankStartsStyle = computed(() => ({
  top: '386px',

  left: 256 / 2 - (12 * props.mod.fusionLimit) / 2 + 'px',
  // zIndex: '40',
}))

import trMarkerBg from '@/assets/mod/GoldTopRightBacker.png'
const trMarkerStyle = {
  '--tr-bg-url': `url("${trMarkerBg}")`,
}
</script>
<style scoped>
img {
  max-width: none;
  user-select: none;
}

.__trMarker {
  background-position-x: -20px;
  height: 26px;
  background-repeat: no-repeat;
  background-size: 64px 26px;
  background-image: var(--tr-bg-url);
}

.__trMarker::before {
  content: '';
  position: absolute;
  background-image: var(--tr-bg-url);
  width: 15px;
  height: 26px;
  top: 0;
  left: -15px;
}
</style>
