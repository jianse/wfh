export interface RelicRewardRow {
  rewardName: string
  uniqueName: string
  name?: string
}

export interface SyncModuleRow {
  key: string
  name: string
  group: string
  source: 'export' | 'api'
  count: number
  size: number
  status: string
  pending: number
  note: string
  localVer: string
  upstreamVer: string
  lastSync: number | null
}

export interface SyncRecordRow {
  id: number
  sync_time: number | null
  is_success: boolean
  time_spent_ms: number
  error_msg: string
  warnings: string[]
  nearest_expiry: number | null
}
