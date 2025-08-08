import http, { type BizResp } from './index.ts'

export interface WorldState {
  cetusCycle: {
    state: 'day' | 'night'
    expiry: string
  }
  vallisCycle: {
    state: 'cold' | 'warm'
    expiry: string
  }
  cambionCycle: {
    active: 'fass' | 'vome'
    expiry: string
  }
  zarimanCycle: {
    state: 'corpus' | 'grineer'
    expiry: string
  }
  earthCycle: {
    state: 'day' | 'night'
    expiry: string
  }
  fissures: {
    id: string
    missionType: string
    tier: number
    // enemy: string
    node: string
    expiry: string
    expired: boolean
    isHard: boolean
    isStorm: boolean
    tierNum: string
  }[]
}

export async function get_world_state() {
  const { data } = await http.get<BizResp<WorldState>>('/world_state/current')
  return data
}
