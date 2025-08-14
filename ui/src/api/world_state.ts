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
  voidTrader: {
    activation: string
    expiry: string
    active: boolean
    inventory: {
      item: string
      credits: number
      ducats: number
    }[]
    location: string
  }
  steelPath: {
    currentReward: {
      name: string
      cost: number
    }
    expiry: string
  }
  vaultTrader: {
    expiry: string
    active: boolean
    inventory: {
      item: string
      credits: number
      ducats: number
    }[]
  }
  dailyDeals: {
    item: string
    expiry: string
    originalPrice: number
    salePrice: number
  }[]
  archonHunt: {
    expiry: string
    missions: {
      node: string
      type: string
    }[]
  }
  sentientOutposts: {
    expiry: string
    mission: {
      node: string
      type: string
    }
  }
  sortie: {
    expiry: string
    variants: {
      node: string
      missionType: string
      modifier: string
    }[]
  }
  nightwave: {
    activeChallenges: {
      expiry: string
      isDaily: boolean
      isElite: boolean
      title: string
      reputation: number
      desc: string
      id: string
    }[]
  }
  invasions: {
    id: string
    count: number
    requiredRuns: number
    node: string
    completion: number
    attackingFaction: string
    attackerReward: {
      asString: string
    }
    defendingFaction: string
    defenderReward: {
      asString: string
    }
  }[]
}

export async function get_world_state() {
  const { data } = await http.get<BizResp<WorldState>>('/world_state/current')
  return data
}
