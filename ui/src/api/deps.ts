import http, { type BizResp } from './index.ts'

export interface SortieModifier {
  id: string
  name: string
  description: string
  vars: Record<string, string>
}

export async function list_sm(): Promise<SortieModifier[]> {
  const { data } = await http.get<BizResp<SortieModifier[]>>(
    '/raw_world_state/deps/sortie_modifiers',
  )
  return data.data || []
}

export async function new_sm(new_item: SortieModifier) {
  const { data } = await http.post<BizResp<SortieModifier>>(
    '/raw_world_state/deps/sortie_modifiers',
    new_item,
  )
  console.log('new sortie modifier resp: ', data)
}

export async function reload() {
  const { data } = await http.post<BizResp<void>>('/raw_world_state/deps/reload', null, {
    timeout: 20000,
  })
  console.log('reload sortie modifiers resp: ', data)
}

export async function delete_sm(id: string) {
  const { data } = await http.delete<BizResp<void>>(`/raw_world_state/deps/sortie_modifiers/${id}`)
  console.log('delete sortie modifier resp: ', data)
}
