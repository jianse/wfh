import { defineStore } from 'pinia'
import { useObservable } from '@vueuse/rxjs'
import { liveQuery } from 'dexie'
import { db, type Commons, type Manifest, type Warframes } from '@/util/localDB'
import type { Observable } from 'rxjs'

export const useResourcesStore = defineStore('resources', () => {
  const manifest = useObservable(
    liveQuery(() => db.Manifest.toArray()) as unknown as Observable<Manifest[]>,
    { initialValue: [] },
  )
  const warframes = useObservable(
    liveQuery(() => db.Warframes.orderBy('name').toArray()) as unknown as Observable<Warframes[]>,
    { initialValue: [] },
  )
  return { manifest, warframes }
})
