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
  events: {
    id: string
    description: string
    node: string
    tooltip: string
    expiry: string
    health: number
  }[]
  alerts: {
    id: string
    mission: {
      node: string
      description: string
      type: string
      reward: {
        asString: string
      }
    }
    expiry: string
  }[]
}

export async function get_world_state() {
  const { data } = await http.get<BizResp<WorldState>>('/world_state/current')
  return data
}

export async function update_and_get() {
  const { data } = await http.get<BizResp<WorldState>>('/world_state/update')
  return data
}

export async function get_raw_world_state() {
  const { data } = await http.get<BizResp<RawWorldState>>('/raw_world_state/current')
  return data
}

export interface RawWorldState {
  Goals: {
    _id: string
    Desc: string
    Node: string
    ToolTip: string
    Expiry: string
    Community?: boolean
    Count?: number
    Goal?: number
  }[]
  Alerts: {
    _id: string
    Expiry: string
    MissionInfo: {
      location: string
      missionType: string
      missionReward: Reward
      descText: string
    }
    Tag: string
  }[]
  VoidTraders: {
    Node: string
    Activation: string
    Expiry: string
    Manifest: {
      ItemType: string
      PrimePrice: number
      RegularPrice: number
    }[]
  }[]
  SyndicateMissions: {
    _id: string
    Activation: string
    Expiry: string
    Nodes: string[]
    Tag: string
  }[]
  ActiveMissions: {
    _id: string
    Hard: boolean
    Expiry: string
    Node: string
    MissionType: string
    Modifier: string
  }[]
  VoidStorms: {
    _id: string
    Node: string
    Activation: string
    Expiry: string
    ActiveMissionTier: string
  }[]
  DailyDeals: {
    Discount: number
    StoreItem: string
    StoreItemName: string
    AmountTotal: number
    OriginalPrice: number
    SalePrice: number
    Activation: string
    Expiry: string
  }[]
  LiteSorties: {
    _id: string
    Activation: string
    Expiry: string
    Reward: string
    Seed: number
    Boss: string
    BossName: string
    Missions: {
      missionType: string
      node: string
    }[]
  }[]
  Sorties: {
    _id: string
    Activation: string
    Expiry: string
    Reward: string
    Seed: number
    Boss: string
    ExtraDrops: string[]
    Variants: {
      missionType: string
      modifierType: string
      node: string
      tileset: string
    }[]
  }[]
  PrimeVaultTraders: {
    _id: string
    Activation: string
    Expiry: string
    InitialStartDate: string
    Node: string
    Manifest: PVItem[]
    EvergreenManifest: PVItem[]
  }[]
  SeasonInfo: {
    ActiveChallenges: {
      _id: string
      Activation: string
      Expiry: string
      Daily?: boolean
      Challenge: string
    }[]
  }
  Invasions: {
    _id: string
    Activation: string
    Faction: string
    DefenderFaction: string
    Node: string
    Count: number
    Goal: number
    LocTag: string
    Completed: boolean
    ChainID: string
    AttackerReward: Reward
    AttackerMissionInfo: {
      seed: number
      faction: string
    }
    DefenderReward: Reward
    DefenderMissionInfo: {
      seed: number
      faction: string
    }
  }[]
  Conquests: {
    Activation: string
    Expiry: string
    Type: string
    Missions: {
      faction: string
      missionType: string
      difficulties: {
        type: string
        deviation: string
        risks: string[]
      }[]
    }[]
    Variables: string[]
    RandomSeed: number
  }[]
  KnownCalendarSeasons: {
    Expiry: string
    Season: string
  }[]
  EndlessXpChoices: {
    Category: 'EXC_NORMAL' | 'EXC_HARD'
    Choices: string[]
  }[]
}
export interface Reward {
  credits?: number
  xp?: number
  items?: string[]
  itemNames?: string[]
  countedItems?: {
    ItemType: string
    ItemName: string
    ItemCount: number
    ItemCountSum: number
  }[]
  randomizedItems?: string
}

interface PVItem {
  ItemType: string
  PrimePrice?: number
  RegularPrice?: number
}
