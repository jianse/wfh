import { useTokenStore } from '@/stores/token'
import { useUiStore } from '@/stores/ui'
import axios, { type InternalAxiosRequestConfig } from 'axios'
import dayjs from 'dayjs'
import { do_refresh_token } from './token'

// 全局锁，用于防止多个请求同时触发 refresh token
let refreshPromise: Promise<void> | null = null

const instance = axios.create({
  baseURL: import.meta.env.VITE_BASEURL,
  headers: {
    post: {
      'Content-Type': 'application/json',
    },
    put: {
      'Content-Type': 'application/json',
    },
  },
  timeout: 5000,
})

interface Err {
  status: number
  config?: CustomRequestConfig
  response?: { data?: { msg?: string } }
}

// TODO: add more detailed type of err
function error_handler(err: Err) {
  console.log('axios got error', err)
  const ui = useUiStore()
  const msg = err.response?.data?.msg

  // 登录等免鉴权请求（例如密码错误返回 401）交由调用方自行提示
  if (err.config?.skipAuth && err.status === 401) {
    return Promise.reject(err)
  }

  switch (err.status) {
    case 401:
      ui.toast('info', '需要登录', '登录状态已失效，请重新登录。')
      break
    case 403:
      ui.toast('err', '没有权限', msg || '当前账号无权执行该操作。')
      break
    case 404:
      ui.toast('err', '未找到', msg || '请求的资源不存在。')
      break
    case 502:
      ui.toast('err', '上游异常', msg || '上游服务响应异常，请稍后重试。')
      break
    default:
      if (err.status >= 500) {
        ui.toast('err', '服务器错误', msg || '服务器处理请求时发生错误，请稍后重试。')
      } else {
        ui.toast('err', '请求异常', msg || String(err))
      }
  }
  return Promise.reject(err)
}

export interface CustomRequestConfig extends InternalAxiosRequestConfig {
  skipAuth?: boolean
}

instance.interceptors.request.use(async (config: CustomRequestConfig) => {
  if (config.skipAuth) {
    return config
  }

  const { token_valid_until } = useTokenStore()
  const expire = dayjs(token_valid_until)
  if (dayjs().isAfter(expire)) {
    // token expired

    // check if refresh token expired
    const { refresh_token, refresh_valid_until, clientId, setToken } = useTokenStore()
    const refresh_expire = dayjs(refresh_valid_until)
    if (dayjs().isAfter(refresh_expire)) {
      // refresh token expired
      const { clearToken } = useTokenStore()
      clearToken()
      return Promise.reject('refresh token expired')
    } else if (refresh_token && clientId) {
      // 使用全局锁确保只有一个请求触发 refresh token
      if (!refreshPromise) {
        refreshPromise = (async () => {
          try {
            const res = await do_refresh_token({ refresh_token, client_id: clientId })
            await setToken(res.data)
          } finally {
            refreshPromise = null
          }
        })()
      }
      await refreshPromise
    } else {
      // refresh token not exist
      return Promise.reject('refresh token not exist')
    }
  }

  // add token
  const { token } = useTokenStore()
  config.headers.Authorization = token
  return config
})

instance.interceptors.response.use(undefined, error_handler)

export default instance

export interface BizResp<T> {
  code: number
  msg: string
  data: T
}

export interface Page<T> {
  page: number
  page_size: number
  total: number
  items: T[]
}

export function is_success<T>(resp: BizResp<T>): boolean {
  return resp.code >= 20000 && resp.code < 30000
}
