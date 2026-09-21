export const fmt = (n: number | string) =>
  n == null ? '—' : typeof n === 'number' ? n.toLocaleString('zh-CN') : n

export const esc = (s) =>
  String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]))

export const pad2 = (n) => String(n).padStart(2, '0')

export function fmtTime(ts) {
  const d = new Date(ts)
  return `${d.getFullYear()}-${pad2(d.getMonth() + 1)}-${pad2(d.getDate())} ${pad2(d.getHours())}:${pad2(d.getMinutes())}:${pad2(d.getSeconds())}`
}

export function dbDur(sec) {
  if (sec == null) return '—'
  if (sec < 60) return `${sec} 秒`
  if (sec < 3600) return `${Math.round(sec / 60)} 分钟`
  const h = sec / 3600
  return `${Number.isInteger(h) ? h : h.toFixed(1)} 小时`
}

export const ROLE_RANK = { normal: 0, maintainer: 1, admin: 2 }
export const ROLE_LABEL = { normal: '普通用户', maintainer: '维护者', admin: '管理员' }

export const roleRank = (r) => (r != null && r in ROLE_RANK ? ROLE_RANK[r] : -1)
export const roleLabel = (r) => ROLE_LABEL[r] || r

export const WF_CAT_LABEL = { Suits: '战甲', SpaceSuits: '曲翼', MechSuits: '殁世机甲' }
export const WF_CAT_TOTAL = { all: 125, Suits: 117, SpaceSuits: 5, MechSuits: 2 }
