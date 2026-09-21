/** 仓鼠助手路由约定（视图短名 → 路由名 / 面包屑）。 */

export const WFH_BASE = '/wfh'

export const VIEW_TITLES: Record<string, [string, string]> = {
  home: ['主页', '概览'],
  warframes: ['数据库', '战甲'],
  weapons: ['数据库', '武器'],
  mods: ['数据库', '振幅晶体'],
  arcane: ['数据库', '赋能'],
  relics: ['数据库', '遗物'],
  resources: ['数据库', '资源'],
  recipes: ['数据库', '配方'],
  'world-state': ['世界状态', '事件追踪'],
  about: ['系统', '关于'],
  login: ['系统', '登录'],
  settings: ['系统', '设置'],
  'sync-status': ['系统', '数据同步'],
  'sync-records': ['管理后台', '同步记录'],
  rewrite: ['管理后台', '重写数据'],
  users: ['管理后台', '用户管理'],
  languages: ['管理后台', '语言管理'],
}

/** 视图短名 → 主应用路由表中的全局唯一名称。 */
export function wfhName(name: string): string {
  return 'wfh-' + name
}

/** 路由名称 → 面包屑；同时兼容带 / 不带 wfh- 前缀。 */
export function titleFor(name: unknown): [string, string] {
  const key = String(name || '').replace(/^wfh-/, '')
  return VIEW_TITLES[key] || ['', key]
}

export function crumbFor(name: unknown): [string, string] {
  return titleFor(name)
}
