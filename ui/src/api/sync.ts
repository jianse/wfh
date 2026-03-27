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
  const response = await request.get('/raw_world_state/sync_records', {
    params: { page, page_size },
  })
  return response.data
}
