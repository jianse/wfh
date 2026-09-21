<script setup lang="ts">
import { ref } from 'vue'
import { useTransStore } from '@/stores/translations'
import { useUiStore } from '@/stores/ui'
import { save_translation, delete_translation } from '@/api/translations'
import type { RelicRewardRow } from './types'

const props = defineProps<{ relicName: string; rewards: RelicRewardRow[] }>()
const emit = defineEmits<{ close: [] }>()

const { wf_t, write_back_trans, remove_trans } = useTransStore()
const ui = useUiStore()

const draft = ref<Record<string, string>>(
  Object.fromEntries(
    props.rewards.map((x): [string, string] => [
      x.rewardName,
      wf_t(x.uniqueName, 'name') || x.name || ''
    ])
  )
)
const saving = ref<Record<string, boolean>>({})

async function saveTrans(reward: RelicRewardRow) {
  saving.value[reward.rewardName] = true
  try {
    const it = {
      unique_name: reward.uniqueName,
      prop: 'name',
      content: draft.value[reward.rewardName] || ''
    }
    await save_translation(it)
    write_back_trans(it)
    ui.toast('ok', '操作成功', '添加译文成功！')
  } catch {
    ui.toast('err', '保存失败', '无法保存译文。')
  } finally {
    saving.value[reward.rewardName] = false
  }
}

async function clearTrans(reward: RelicRewardRow) {
  saving.value[reward.rewardName] = true
  try {
    const it = { unique_name: reward.uniqueName, prop: 'name' }
    await delete_translation(it)
    remove_trans(it)
    draft.value[reward.rewardName] = ''
    ui.toast('ok', '操作成功', '清空译文成功！')
  } catch {
    ui.toast('err', '清除失败', '无法清除译文。')
  } finally {
    saving.value[reward.rewardName] = false
  }
}
</script>

<template>
  <div class="modal-panel">
    <div class="dlg-head">
      <h3 class="h3">编辑奖励译文 · {{ props.relicName }}</h3>
      <button class="icon-btn" title="关闭" @click="emit('close')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M6 6l12 12M18 6L6 18" /></svg>
      </button>
    </div>
    <div class="dlg-body">
      <div v-for="reward in props.rewards" :key="reward.rewardName" class="field">
        <label>原文：{{ reward.name }}</label>
        <input v-model="draft[reward.rewardName]" class="input" type="text" placeholder="请输入译文" />
        <div class="row-actions mt-xs">
          <button class="btn btn-primary" :disabled="saving[reward.rewardName]" @click="saveTrans(reward)">保存</button>
          <button class="btn btn-secondary" :disabled="saving[reward.rewardName]" @click="clearTrans(reward)">清除</button>
        </div>
      </div>
    </div>
    <div class="dlg-foot">
      <button class="btn btn-secondary" @click="emit('close')">关闭</button>
    </div>
  </div>
</template>
