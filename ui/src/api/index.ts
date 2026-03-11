import { useTokenStore } from '@/stores/token'
import { useCommonDialog, useCommonToast } from '@/util/common'
import axios, { type InternalAxiosRequestConfig } from 'axios'
import dayjs from 'dayjs'
import { defineAsyncComponent } from 'vue'
import { do_refresh_token } from './token'

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
}

// TODO: add more detailed type of err
function error_handler(err: Err) {
  console.log('axios got error', err)
  // unauthorized
  if (err.status === 401) {
    const dialog = useCommonDialog()
    const comp = defineAsyncComponent(() => import('@/components/LoginOrOfflineDialog.vue'))
    dialog.open(comp, {
      props: {
        header: '提示',
        modal: true,
        dismissableMask: true,
      },
    })
  } else {
    const toast = useCommonToast()
    toast.add({
      severity: 'error',
      summary: '请求异常',
      detail: err,
      life: 3000,
    })
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

  const { token, token_valid_until } = useTokenStore()
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
      // const { refresh_token, clientId } = useTokenStore()
      const res = await do_refresh_token({ refresh_token, client_id: clientId })
      await setToken(res.data)
    } else {
      // refresh token not exist
      return Promise.reject('refresh token not exist')
    }
  }
  // }

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
