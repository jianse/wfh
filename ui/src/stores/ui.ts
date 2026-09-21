import { defineStore } from 'pinia'
import { markRaw, type Component } from 'vue'

let toastSeq = 0
const toastTimers = new Map<number, ReturnType<typeof setTimeout>>()
let dialogSeq = 0

export interface Toast {
  id: number
  kind: string
  title: string
  desc?: string
  leaving: boolean
}

export interface ConfirmState {
  title: string
  body: string
  cb: (() => void) | null
}

export interface DialogEntry {
  id: number
  component: Component
  props: Record<string, unknown>
}

export const useUiStore = defineStore('ui', {
  state: () => ({
    toasts: [] as Toast[],
    /** 弹窗栈：由功能页通过 openDialog 推入，DialogHost 统一渲染。 */
    dialogStack: [] as DialogEntry[],
    /** 通用确认框状态；由 askConfirm 设置，DialogHost 渲染 ConfirmDialog。 */
    confirm: null as ConfirmState | null,
    navOpen: false
  }),
  getters: {
    topDialog: (s): DialogEntry | null => s.dialogStack[s.dialogStack.length - 1] ?? null
  },
  actions: {
    /** 复刻原型 showToast：3.2 秒后淡出并移除。 */
    toast(kind: string, title: string, desc?: string): number {
      const id = ++toastSeq
      this.toasts.push({ id, kind, title, desc, leaving: false })
      const t = setTimeout(() => this.dismissToast(id), 3200)
      toastTimers.set(id, t)
      return id
    },
    dismissToast(id: number) {
      const el = this.toasts.find((t) => t.id === id)
      if (!el) return
      el.leaving = true
      const t = setTimeout(() => {
        this.toasts = this.toasts.filter((x) => x.id !== id)
        toastTimers.delete(id)
      }, 240)
      toastTimers.set(id, t)
    },

    openDialog(component: Component, props: Record<string, unknown> = {}): number {
      const id = ++dialogSeq
      this.dialogStack.push({ id, component: markRaw(component), props })
      return id
    },
    closeDialog(id: number) {
      this.dialogStack = this.dialogStack.filter((d) => d.id !== id)
    },
    closeTopDialog() {
      this.dialogStack.pop()
    },
    closeAllDialogs() {
      this.dialogStack = []
    },

    askConfirm(title: string, body: string, cb: () => void) {
      this.confirm = { title, body, cb }
    },
    resolveConfirm() {
      const cb = this.confirm?.cb ?? null
      this.confirm = null
      if (cb) cb()
    },
    cancelConfirm() {
      this.confirm = null
    },

    setNavOpen(open: boolean) {
      this.navOpen = open
      const root = document.querySelector('.wfh-root')
      if (root) root.classList.toggle('nav-open', open)
    }
  }
})
