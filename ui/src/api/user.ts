import http, { type BizResp, type Page } from './index.ts'
export interface CurrentUser {
  id: number
  username: string
  role: 'normal' | 'maintainer' | 'admin'
}

export interface User {
  id: number
  username: string
  role: 'normal' | 'maintainer' | 'admin'
}

export interface CreateUserRequest {
  username: string
  password: string
  role: 'normal' | 'maintainer' | 'admin'
}

export interface UpdateUserRequest {
  username: string
  role: 'normal' | 'maintainer' | 'admin'
}

export interface UpdateUserRoleRequest {
  role: 'normal' | 'maintainer' | 'admin'
}

export interface ResetPasswordRequest {
  password: string
}

export async function get_current_user() {
  const { data } = await http.get<BizResp<CurrentUser>>('/v1/authed/user/current')
  console.log('current user info is:', data)
  return data
}

export async function get_user_list(page: number, page_size: number) {
  const { data } = await http.get<BizResp<Page<User>>>('/v1/authed/user/admin/list', {
    params: { page, page_size },
  })
  return data
}

export async function create_user(req: CreateUserRequest) {
  const { data } = await http.post<BizResp<User>>('/v1/authed/user/admin/create', req)
  return data
}

export async function update_user(id: number, req: UpdateUserRequest) {
  const { data } = await http.put<BizResp<User>>(`/v1/authed/user/admin/update/${id}`, req)
  return data
}

export async function delete_user(id: number) {
  const { data } = await http.delete<BizResp<void>>(`/v1/authed/user/admin/delete/${id}`)
  return data
}

export async function reset_password(id: number, req: ResetPasswordRequest) {
  const { data } = await http.post<BizResp<void>>(`/v1/authed/user/admin/reset-password/${id}`, req)
  return data
}

export async function update_user_role(id: number, req: UpdateUserRoleRequest) {
  const { data } = await http.put<BizResp<User>>(`/v1/authed/user/admin/update-role/${id}`, req)
  return data
}
