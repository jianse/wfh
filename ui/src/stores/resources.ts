import { defineStore } from 'pinia'
import { useObservable } from '@vueuse/rxjs'
import { liveQuery } from 'dexie'
import { db, type Manifest, type Relics, type Resources, type Warframes } from '@/util/localDB'
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

  // const relics = useObservable(
  //   liveQuery(async () => await db.Relics.toArray()) as unknown as Observable<Relics[]>,
  //   { initialValue: [] },
  // )

  // const resources = useObservable(
  //   liveQuery(async () => await db.Resources.toArray()) as unknown as Observable<Resources[]>,
  //   { initialValue: [] },
  // )
  return { manifest, warframes }
})
