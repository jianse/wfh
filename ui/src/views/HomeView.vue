<script setup lang="ts">
import { onMounted } from 'vue'
import { wfhName } from '@/router/wfh'
import { sync_all } from '@/util/resources'

onMounted(async () => {
  try {
    await sync_all(false)
  } catch {
    /* 离线或上游不可用时保持本地缓存 */
  }
})

const QNAV = [
  {
    view: 'warframes',
    name: '战甲',
    count: '125 条记录',
    icon: 'M12 3a4 4 0 0 0-4 4v2H6a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h1v4h10v-4h1a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2h-2V7a4 4 0 0 0-4-4z'
  },
  { view: 'weapons', name: '武器', count: '952 条记录', icon: 'M4 20L16 8l-4-4L4 12zM16 8l4 4' },
  { view: 'mods', name: '振幅晶体', count: '1,803 条记录', icon: 'M3 3h18v18H3zM3 9h18M9 9v12' },
  { view: 'arcane', name: '赋能', count: '168 条记录', icon: 'M12 8l2 4-2 4-2-4z' },
  { view: 'relics', name: '遗物', count: '5,397 条记录', icon: 'M12 3l8 5-8 5-8-5z' },
  { view: 'resources', name: '资源', count: '3,465 条记录', icon: 'M12 3l7 4v5c0 5-3.5 8-7 9-3.5-1-7-4-7-9V7z' },
  { view: 'recipes', name: '配方', count: '1,861 条记录', icon: 'M4 7h16M7 7v12h10V7M9 7V4h6v3' }
]

const metrics = [
  { num: '125', unit: '条', label: '战甲记录' },
  { num: '952', unit: '条', label: '武器记录' },
  { num: '1,803', unit: null, label: '振幅晶体记录' },
  { num: '168', unit: '条', label: '赋能记录' }
]

const panels = [
  { title: '虚空商人', desc: '巴洛·基·提尔即将抵达中继站，带来稀有 Prime 部件与限定赋能。', foot: '中继站轮换' },
  { title: '突击任务', desc: '三连任务链，完成全部获得稀有赋能与传说核心奖励。', foot: '每日 00:00 刷新' },
  { title: '周期循环', desc: '地球夜灵平原昼夜循环，影响稀有种与鱼类的刷新。', foot: '昼夜交替' }
]
</script>

<template>
  <section class="view active" id="view-home" data-od-id="home">
    <div class="page-head">
      <p class="eyebrow">仓鼠助手 · Warframe 数据百科</p>
      <h1 class="h1" style="max-width: 20ch">查遍始源星系，一处搞定。</h1>
      <p class="lead">聚合战甲、武器、振幅晶体、赋能、遗物与实时世界状态，数据全部来自官方导出，自动同步。</p>
    </div>

    <div class="metric-row">
      <div v-for="m in metrics" :key="m.label" class="metric">
        <div class="metric-num">{{ m.num }}<span v-if="m.unit" class="unit">{{ m.unit }}</span></div>
        <div class="metric-label">{{ m.label }}</div>
      </div>
    </div>

    <div class="row-between mb-sm">
      <h2 class="h2">数据库</h2>
      <span class="meta">7 个分类</span>
    </div>
    <div class="qnav-grid" id="qnav-grid">
      <RouterLink v-for="q in QNAV" :key="q.view" class="qnav" :to="{ name: wfhName(q.view) }">
        <span class="q-ico"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path :d="q.icon" /></svg></span>
        <span class="q-txt"><span class="q-name">{{ q.name }}</span><span class="q-count">{{ q.count }}</span></span>
        <span class="q-arrow">→</span>
      </RouterLink>
    </div>

    <div class="row-between mt-lg mb-sm">
      <h2 class="h2">世界状态速览</h2>
      <RouterLink class="btn btn-ghost btn-arrow" :to="{ name: 'wfh-world-state' }">查看全部</RouterLink>
    </div>
    <div class="panel-grid">
      <div v-for="p in panels" :key="p.title" class="panel">
        <div class="panel-head"><span class="status-dot"></span><span class="panel-title">{{ p.title }}</span></div>
        <p class="panel-desc">{{ p.desc }}</p>
        <div class="panel-foot"><span class="live">活跃</span> · {{ p.foot }}</div>
      </div>
    </div>
  </section>
</template>
