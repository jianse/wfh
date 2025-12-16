import { count_in_db, get_last_update_time, RESOURCES_INCLUDED } from '@/util/localDB'
import { useAsyncState } from '@vueuse/core'
import { defineStore } from 'pinia'
import { triggerRef } from 'vue'

export enum SyncPhase {
  NOT_SYNC = '尚未同步',
  IN_PROGRESS = '正在同步',
  DONE = '同步完成',
}

export interface SyncItem {
  phase: SyncPhase
  total: number
  completed: number
  last_sync_time?: Date
}

type SyncState = Record<string, SyncItem>

async function initState(): Promise<SyncState> {
  const res: SyncState = {}
  for (const cate of RESOURCES_INCLUDED) {
    const lut = await get_last_update_time(cate)
    const cid = await count_in_db(cate)
    if (lut) {
      res[cate] = {
        phase: SyncPhase.DONE,
        total: lut.total,
        completed: cid,
        last_sync_time: lut.last_update_time,
      }
    } else {
      res[cate] = {
        phase: SyncPhase.NOT_SYNC,
        total: -1,
        completed: cid,
        last_sync_time: undefined,
      }
    }
  }
  return res
}

export const useSyncStore = defineStore('sync', () => {
  const { state } = useAsyncState(initState(), {})

  function setCategoryState(category: string, syncState: SyncItem) {
    state.value[category] = syncState
    triggerRef(state)
  }

  /**
   * set sync state of an category to inprogress
   * @param category category
   */
  function setInprogress(category: string) {
    state.value[category] = {
      phase: SyncPhase.IN_PROGRESS,
      total: -1,
      completed: 0,
      last_sync_time: undefined,
    }
    triggerRef(state)
  }

  function setCategoryTotal(category: string, total: number) {
    const cat = state.value[category]
    if (cat) {
      cat.total = total
    }
    triggerRef(state)
  }

  function increaseCategoryCompleted(category: string) {
    const cat = state.value[category]
    if (cat) {
      cat.completed++
    }
    triggerRef(state)
  }

  function markCategoryDone(category: string) {
    const item = state.value[category]
    if (!item) {
      return
    }
    item.phase = SyncPhase.DONE
    item.last_sync_time = new Date()
    triggerRef(state)
  }

  function increaseCategoryCompletedBy(category: string, amount: number) {
    const cat = state.value[category]
    if (cat) {
      cat.completed += amount
    }
    triggerRef(state)
  }

  return {
    state,
    setCategoryState,
    setInprogress,
    increaseCategoryCompleted,
    increaseCategoryCompletedBy,
    markCategoryDone,
    setCategoryTotal,
  }
})
