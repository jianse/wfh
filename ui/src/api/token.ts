import http, { type BizResp } from './index.ts'

interface UserCred {
  username: string
  password: string
}

interface TokenRes {
  token: string
}

export async function get_token(user: UserCred): Promise<BizResp<TokenRes>> {
  const { data } = await http.post<BizResp<TokenRes>>('/token', user, {
    headers: { 'Content-Type': 'application/json' },
  })
  return data
}
