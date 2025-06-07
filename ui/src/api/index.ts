import { useTokenStore } from '@/stores/token'
import axios from 'axios'
import { useToast } from 'primevue'

const instance = axios.create({
  baseURL: import.meta.env.VITE_BASEURL,
})

// TODO: add more detailed type of err
const error_handler = function (err: object) {
  const toast = useToast()
  toast.add({
    severity: 'error',
    summary: '请求异常',
    detail: err,
    life: 3000,
  })
  console.log('axios got error', err)
}
instance.interceptors.request.use((config) => {
  const { token } = useTokenStore()
  config.headers.Authorization = token
  return config
})
instance.interceptors.response.use(undefined, error_handler)

export default instance

export interface BizResp<T> {
  code: number
  msg: string
  data?: T
}

export function is_success<T>(resp: BizResp<T>): boolean {
  return resp.code >= 20000 && resp.code < 30000
}
