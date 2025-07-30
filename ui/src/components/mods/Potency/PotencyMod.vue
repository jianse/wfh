<template>
  <div class="relative" :style="contStyle">
    <!-- frame top -->
    <img class="absolute" draggable="false" src="./head.png" :style="topStyle" />

    <!-- frame bg -->
    <img
      class="absolute"
      draggable="false"
      src="./bg.png"
      style="left: 0; top: -67px; z-index: 3"
    />

    <!-- content bg -->

    <div class="absolute" :style="contentStyle">
      <!-- content body -->
      <div class="flex flex-col h-[295px]">
        <!-- cover image -->
        <div class="shrink-30 overflow-hidden __cover">
          <img
            loading="lazy"
            draggable="false"
            v-if="props.mod?.textureLocation"
            :src="cover_img"
            :style="coverStyle"
          />
        </div>

        <!-- title & effect -->
        <div class="w-full" :style="{ color: tColor, textShadow: `0.8px 0.8px 0 ${tSColor}` }">
          <!-- title -->
          <div class="text-center text-[24px]">{{ props.mod.name }}</div>
          <div class="text-center px-3" v-for="(stat, index) in effect" :key="index">
            <string-with-icon :text="stat" />
          </div>
        </div>
      </div>
    </div>

    <!-- frame bottom -->
    <img class="absolute" draggable="false" src="./bottom.png" :style="bottomStyle" />

    <!-- bottom type marker -->
    <div class="absolute z-5" :style="lowerTabStyle">
      <img draggable="false" src="@/assets/mod/GoldLowerTab.png" />
      <div class="absolute top-0 left-0 right-0 bottom-0 content-center text-center">
        <span class="text-bold" :style="{ color: pColor }">{{ props.mod.compatName }}</span>
      </div>
    </div>

    <img class="absolute z-3" src="../Antivirus/slots.png" style="left: 64px; top: 360px" />

    <!-- rank stars -->
    <div class="absolute z-3" :style="rankStartsStyle">
      <img
        class="inline-block"
        draggable="false"
        v-for="i in currentRank"
        :key="i"
        src="./star.png"
        style="object-fit: cover; height: 32px"
        :style="{ width: starGap + 'px' }"
      />
    </div>
  </div>
  <div class="relative flex flex-row gap-3 z-6">
    <pv-button :disabled="currentRank <= 0" label="降低" @click="reduceRank()"></pv-button>
    <pv-button
      :disabled="currentRank >= props.mod.fusionLimit"
      label="升高"
      @click="increaseRank()"
    ></pv-button>
  </div>
  {{ props.mod.rarity }}
  {{ props.mod.polarity }}
</template>
<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import type { Upgrade } from '@/util/localDB'

const props = defineProps<{ mod: Upgrade; maxRank?: boolean }>()

const contStyle = {
  width: '256px',
  height: '404px',
  userDrag: 'none',
}

const topStyle = {
  top: '-99px',
  left: '-128px',
  zIndex: '6',
}
const contentStyle = computed(() => ({
  left: '7px',
  top: '18px',
  width: '242px',
  height: '338px',
  zIndex: '5',
  '--rarity-color': pColor,
}))

const bottomStyle = {
  top: '230px',
  left: '-128px',
  zIndex: '3',
}

const coverStyle = {
  width: '242px',
  height: '242px',
}

const lowerTabStyle = computed(() => ({
  left: 128 - 105 + 'px',
  top: '330px',
  // top: 14 + bgSize.height - 42 + 'px',
  // zIndex: '40',
}))

const starGap = 24

const rankStartsStyle = computed(() => ({
  width: starGap * props.mod.fusionLimit + 'px',
  top: '353px',
  left: 128 - (starGap * props.mod.fusionLimit) / 2 + 'px',
}))

const pColor = '#ffffff'
const tColor = '#671135'
const tSColor = '#d2a689'

const cover_img = computed(() => {
  // console.log(props.mod)
  return 'https://content.warframe.com/PublicExport' + props.mod?.textureLocation
})
const currentRank = ref(0)
watch(
  () => props.mod,
  (v) => {
    if (props.maxRank) {
      currentRank.value = v.fusionLimit
    }
  },
  { immediate: true },
)

function increaseRank() {
  if (currentRank.value < props.mod?.fusionLimit) {
    currentRank.value++
  }
}

function reduceRank() {
  if (currentRank.value > 0) {
    currentRank.value--
  }
}

const effect = computed(() => {
  return props.mod?.levelStats?.[currentRank.value]?.stats || props.mod.description
})
</script>
<style scoped>
img {
  user-select: none;
  max-width: none;
}
/* #cc {
  mask: linear-gradient(transparent, #000 5px);
} */
.__cover {
  mask: linear-gradient(to top, transparent, #000 3px);
}

.__cover {
  position: relative;
}

.__cover::after {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  right: 0;
  /* box-shadow: inset 0 0 10px 24px #dda085,
   0 0 10px 24px #dda085; */
  content: '';
  /* need more colors */
  background: radial-gradient(transparent 75%, var(--rarity-color));
}

.__trMarker {
  background-position-x: -20px;
  height: 26px;
  background-repeat: no-repeat;
  background-size: 64px 26px;
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
