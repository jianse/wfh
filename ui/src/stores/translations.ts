import { get_all_translations, type TranslationId, type TranslationTerm } from '@/api/translations'
import { useAsyncState } from '@vueuse/core'
import { defineStore } from 'pinia'
import { triggerRef } from 'vue'

async function init() {
  const data = (await get_all_translations()).data ?? []
  const res = new Map<string, string>()
  data.forEach((it) => {
    const key = it.unique_name + '@' + it.prop
    res.set(key, it.content)
  })
  return res
}

export const useTransStore = defineStore('translations', () => {
  const { state } = useAsyncState(init, new Map())

  function wf_t(uniqueName: string, prop: string = 'name'): string | undefined {
    // console.log('t?')
    return state.value.get(uniqueName + '@' + prop)
  }

  function write_back_trans(trans: TranslationTerm) {
    const key = trans.unique_name + '@' + trans.prop
    state.value.set(key, trans.content)
    triggerRef(state)
  }

  function remove_trans(trans: TranslationId) {
    const key = trans.unique_name + '@' + trans.prop
    state.value.delete(key)
    triggerRef(state)
  }

  return { state, wf_t, write_back_trans, remove_trans }
})
