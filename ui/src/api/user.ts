import http, { type BizResp } from './index.ts'
interface CurrentUser {
  id: number
  username: string
}

export async function get_current_user() {
  const { data } = await http.get<BizResp<CurrentUser>>('/user/current')
  console.log('current user info is:', data)
  return data
}
