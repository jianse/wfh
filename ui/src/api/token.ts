import http, { type BizResp, type CustomRequestConfig } from './index.ts'

interface UserCred {
  username: string
  password: string
  client_id: string
}

interface TokenRes {
  token: string
  token_valid_until: string
  refresh_token: string
  refresh_valid_until: string
}

export async function get_token(user: UserCred): Promise<BizResp<TokenRes>> {
  const { data } = await http.post<BizResp<TokenRes>>('/token', user, {
    headers: { 'Content-Type': 'application/json' },
    skipAuth: true,
  } as CustomRequestConfig)
  return data
}

export async function get_client_id() {
  const { data } = await http.get<BizResp<{ nonce: string }>>('/nonce', {
    skipAuth: true,
  } as CustomRequestConfig)
  return data
}

interface RefreshCred {
  refresh_token: string
  client_id: string
}

export async function do_refresh_token(cred: RefreshCred): Promise<BizResp<TokenRes>> {
  const { data } = await http.post<BizResp<TokenRes>>('/refresh', cred, {
    skipAuth: true,
  } as CustomRequestConfig)
  return data
}
