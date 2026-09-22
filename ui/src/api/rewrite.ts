import http, { type BizResp, type Page } from './index.ts'

export interface Rep {
  path: string
  origin: string
  replacement: string
  remark: string
}

export interface RepPk {
  path: string
  origin: string
}

export async function get_rewards_page(page: number, page_size: number) {
  const { data } = await http.get<BizResp<Page<Rep>>>('/v1/authed/world_state/rep', {
    params: {
      page,
      page_size,
    },
  })
  return data.data
}

export async function delete_reward(pk: RepPk) {
  await http.delete<BizResp<void>>('/v1/authed/world_state/rep', {
    params: pk,
  })
}

export async function edit_reward(rep: Rep) {
  await http.put<BizResp<void>>('/v1/authed/world_state/rep', rep)
}
