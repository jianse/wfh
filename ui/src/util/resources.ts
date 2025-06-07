import { useSyncStore } from '@/stores/sync'
import {
  current_index,
  put,
  RESOURCES_INCLUDED,
  save_index,
  set_last_update_time,
  type StoreName,
} from './localDB'

enum categoryEnum {
  Warframes = 'Warframes',
  Relics = 'Relics',
}



/**
 * load index for all resources
 * @returns resource index
 */
export async function load_index() {
  const resp = await fetch('/data2/index.txt')
  const index = await resp.text()
  return index
}

async function load_resource(key: string): Promise<Array<Object>> {
  const resp = await fetch(`/data2/${key}`)
  const root = await resp.json()
  const category = get_category(key)
  if (category == 'Manifest') {
    return root.Manifest
  } else {
    return root[`Export${category}`]
  }
}

const debug_sleep = true
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))
let inprogress = false

export async function check_update_available(force: boolean = false) {
  console.log('start checking update')
  if (inprogress) {
    console.warn('another update process is inprogress!')
  }
  try {
    inprogress = true

    const index = await load_index()
    let local_index = (await current_index()) || ''
    if (force) {
      local_index = ''
    }
    const update = index !== local_index

    if (update) {
      console.log('update found!')
      // do update
      const upd_list = to_be_update_items(local_index, index)
      for (const item of upd_list) {
        const store = useSyncStore()
        const category = get_category(item)
        if (RESOURCES_INCLUDED.includes(category as StoreName)) {
          console.log('interested category found!', category)
          store.setInprogress(category)
          if (debug_sleep) {
            await sleep(1000)
          }
          const arr = await load_resource(item)
          if (debug_sleep) {
            await sleep(1000)
          }
          store.setCategoryTotal(category, arr.length)
          for (const it of arr) {
            const ret = await put(category as StoreName, it)
            // if (debug_sleep) {
            //   await sleep(10)
            // }
            store.increaseCategoryCompleted(category)
          }
          set_last_update_time(category, new Date(), arr.length)

          store.markCategoryDone(category)
          if (debug_sleep) {
            await sleep(1000)
          }
        }
      }
      await save_index(index)
      console.log('updating done!')
    } else {
      console.log('already up to date!')
    }
  } finally {
    inprogress = false
  }
}
// interface Category extends StoreName

function get_category(name: string): string {
  return name.replace('Export', '').replace(/(_[a-z]{2})?\.json.*/, '')
}

function to_be_update_items(old_index: string, new_index: string): string[] {
  const old_items = old_index.split(/\r?\n/g)
  const new_items = new_index.split(/\r?\n/g)
  return new_items.filter((el) => !old_items.includes(el))
}

/**
 * get a string's sha256 hash
 * @param message message to be hashed
 * @returns hex string hash value
 */
export async function sha256(message: string) {
  const msg = new TextEncoder().encode(message)
  const hashBuffer = await crypto.subtle.digest('SHA-256', msg)
  const hashArray = Array.from(new Uint8Array(hashBuffer))
  const hashHex = hashArray.map((b) => b.toString(16).padStart(2, '0')).join('')
  return hashHex
}
