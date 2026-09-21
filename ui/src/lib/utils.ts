export const fmt = (n: number | string | null | undefined): string =>
  n == null ? '—' : typeof n === 'number' ? n.toLocaleString('zh-CN') : String(n)

const ESC_MAP: Record<string, string> = {
  '&': '&amp;',
  '<': '&lt;',
  '>': '&gt;',
  '"': '&quot;',
}

export const esc = (s: unknown): string => String(s).replace(/[&<>"]/g, (c) => ESC_MAP[c] ?? c)

export const pad2 = (n: number): string => String(n).padStart(2, '0')

export function fmtTime(ts: number | string | Date | null | undefined): string {
  if (ts == null) return '—'
  const d = new Date(ts)
  if (Number.isNaN(d.getTime())) return '—'
  return `${d.getFullYear()}-${pad2(d.getMonth() + 1)}-${pad2(d.getDate())} ${pad2(d.getHours())}:${pad2(d.getMinutes())}:${pad2(d.getSeconds())}`
}

export function dbDur(sec: number | null | undefined): string {
  if (sec == null) return '—'
  if (sec < 60) return `${sec} 秒`
  if (sec < 3600) return `${Math.round(sec / 60)} 分钟`
  const h = sec / 3600
  return `${Number.isInteger(h) ? h : h.toFixed(1)} 小时`
}

export const ROLE_RANK: Record<string, number> = { normal: 0, maintainer: 1, admin: 2 }
export const ROLE_LABEL: Record<string, string> = {
  normal: '普通用户',
  maintainer: '维护者',
  admin: '管理员',
}

export const roleRank = (r?: string | null): number =>
  r != null && r in ROLE_RANK ? (ROLE_RANK[r] as number) : -1

export const roleLabel = (r?: string | null): string => (r != null ? (ROLE_LABEL[r] ?? r) : '')

export const WF_CAT_LABEL: Record<string, string> = {
  Suits: '战甲',
  SpaceSuits: '曲翼',
  MechSuits: '殁世机甲',
}
export const WF_CAT_TOTAL: Record<string, number> = {
  all: 125,
  Suits: 117,
  SpaceSuits: 5,
  MechSuits: 2,
}
