import http, { type BizResp, type Page } from './index.ts'

export interface LangItem {
  unique_name: string
  value: string
}

/** 分页读取语言词条（后端 page 从 0 开始） */
export async function list_langs(page: number, page_size: number): Promise<BizResp<Page<LangItem>>> {
  const { data } = await http.get<BizResp<Page<LangItem>>>('/v1/authed/raw_world_state/deps/langs', {
    params: { page, page_size }
  })
  return data
}

/** 全文检索语言词条（按相关度排序，后端 page 从 0 开始） */
export async function search_langs(
  q: string,
  page: number,
  page_size: number
): Promise<BizResp<Page<LangItem>>> {
  const { data } = await http.get<BizResp<Page<LangItem>>>('/v1/authed/raw_world_state/deps/langs/search', {
    params: { q, page, page_size }
  })
  return data
}

/** 新增词条 */
export async function new_lang_item(item: LangItem) {
  await http.post('/v1/authed/raw_world_state/deps/langs', item)
}

/** 修改已有词条 */
export async function update_lang_item(item: LangItem) {
  await http.patch('/v1/authed/raw_world_state/deps/lang', item)
}

/** 删除词条 */
export async function remove_lang_item(unique_name: string) {
  await http.delete(`/v1/authed/raw_world_state/deps/lang/${encodeURIComponent(unique_name)}`)
}

/** 上传 JSON（扁平 { key: value }）整体覆写语言表；onProgress 回调 0~1 的进度 */
export async function upload_langs(file: File, onProgress?: (pct: number) => void) {
  const form = new FormData()
  form.append('file', file)
  await http.post('/v1/authed/raw_world_state/deps/langs/upload', form, {
    headers: { 'Content-Type': 'multipart/form-data' },
    onUploadProgress: (e) => {
      const total = e.total ?? 0
      if (total > 0) onProgress?.((e.loaded ?? 0) / total)
    }
  })
}
