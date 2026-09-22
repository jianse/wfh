import request from '.'
import type { BizResp, Page } from '.'

export interface SyncRecord {
  id: number
  sync_time: string
  is_success: boolean
  time_spent_ms: number
  error_msg: string | null
  warnings: string | null
  nearest_expiry: string | null
}

export async function list_sync_records(
  page: number,
  page_size: number,
): Promise<BizResp<Page<SyncRecord>>> {
  const response = await request.get('/v1/authed/raw_world_state/sync_records', {
    params: { page, page_size },
  })
  return response.data
}

export interface SyncStats {
  total: number
  success: number
  fail: number
  success_rate: number
  avg_time_ms: number
  min_time_ms: number | null
  max_time_ms: number | null
  last_sync_time: string | null
  last_success_time: string | null
  last_fail_time: string | null
  last_error_msg: string | null
  updated_at: string | null
}

/** 全量同步统计（服务端聚合表，O(1) 返回，非当前页数据） */
export async function get_sync_stats(): Promise<SyncStats> {
  const response = await request.get<BizResp<SyncStats>>(
    '/v1/authed/raw_world_state/sync_records/stats',
  )
  return response.data.data
}
