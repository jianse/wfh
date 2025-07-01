import { get_all_translations, type TranslationId, type TranslationTerm } from '@/api/translations'
import { useAsyncState } from '@vueuse/core'
import { cloneDeep } from 'lodash'
import { defineStore } from 'pinia'
import { triggerRef } from 'vue'

async function init() {
  return (await get_all_translations()).data ?? []
}

export const useTransStore = defineStore('translations', () => {
  const { state } = useAsyncState(init, [])

  function wf_t(uniqueName: string, prop: string = 'name'): string | undefined {
    console.log('t?')
    return state.value.find((it) => it.unique_name === uniqueName && it.prop === prop)?.content
  }

  function write_back_trans(trans: TranslationTerm) {
    const it = state.value.find(
      (it) => it.unique_name === trans.unique_name && it.prop === trans.prop,
    )
    if (it) {
      it.content = trans.content
    } else {
      state.value.push(cloneDeep(trans))
    }
    triggerRef(state)
  }

  function remove_trans(trans: TranslationId) {
    state.value = state.value.filter(
      (it) => it.unique_name !== trans.unique_name || it.prop !== trans.prop,
    )
  }

  return { state, wf_t, write_back_trans, remove_trans }
})
