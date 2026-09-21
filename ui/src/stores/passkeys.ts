import { defineStore } from 'pinia'
import { PASSKEYS as SEED_PASSKEYS, type Passkey } from '@/data/admin'

export const usePasskeyStore = defineStore('passkeys', {
  state: () => ({
    items: SEED_PASSKEYS.map((p) => ({ ...p })) as Passkey[],
  }),
  getters: {
    nextId: (s) => s.items.reduce((m, p) => Math.max(m, p.id), 0) + 1,
  },
  actions: {
    add(name?: string): string {
      const label = (name || '').trim() || '未命名设备'
      this.items.push({
        id: this.nextId,
        name: label,
        created: new Date().toISOString().slice(0, 10),
      })
      return label
    },
    remove(id: number) {
      const i = this.items.findIndex((p) => p.id === id)
      if (i > -1) this.items.splice(i, 1)
    },
  },
})
