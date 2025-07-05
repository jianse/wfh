import { get_all_translations, type TranslationId, type TranslationTerm } from '@/api/translations'
import { useAsyncState } from '@vueuse/core'
import { cloneDeep } from 'lodash'
import { defineStore } from 'pinia'
import { triggerRef } from 'vue'

class Key {
  public readonly uniqueName: string
  public readonly prop: string
  constructor(uniqueName: string, prop: string) {
    this.uniqueName = uniqueName
    this.prop = prop
  }
}

async function init() {
  const data = (await get_all_translations()).data ?? []
  const res = new Map<Key, string>()
  data.forEach((it) => {
    const key = new Key(it.unique_name, it.prop)
    res.set(key, it.content)
  })
  return res
}

export const useTransStore = defineStore('translations', () => {
  const { state } = useAsyncState(init, new Map())

  function wf_t(uniqueName: string, prop: string = 'name'): string | undefined {
    console.log('t?')
    return state.value.get(new Key(uniqueName, prop))
  }

  function write_back_trans(trans: TranslationTerm) {
    const key = new Key(trans.unique_name, trans.prop)
    state.value.set(key, trans.content)
    triggerRef(state)
  }

  function remove_trans(trans: TranslationId) {
    const key = new Key(trans.unique_name, trans.prop)
    state.value.delete(key)
    triggerRef(state)
  }

  return { state, wf_t, write_back_trans, remove_trans }
})
