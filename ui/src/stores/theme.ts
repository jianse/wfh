import { defineStore } from 'pinia'

const KEY = 'wfh-theme'

type Theme = 'dark' | 'light'

function readInitial(): Theme {
  let t: string | null = null
  try {
    t = localStorage.getItem(KEY)
  } catch {
    /* localStorage 不可用时回退到媒体查询 */
  }
  if (t === 'dark' || t === 'light') return t
  return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'dark'
    : 'light'
}

export const useThemeStore = defineStore('theme', {
  state: () => ({ theme: readInitial() }),
  getters: {
    isDark: (s) => s.theme === 'dark',
  },
  actions: {
    apply(t: Theme) {
      this.theme = t
      document.documentElement.setAttribute('data-theme', t)
      try {
        localStorage.setItem(KEY, t)
      } catch {
        /* 忽略写入失败 */
      }
    },
    toggle() {
      this.apply(this.theme === 'dark' ? 'light' : 'dark')
    },
    init() {
      document.documentElement.setAttribute('data-theme', this.theme)
    },
  },
})
