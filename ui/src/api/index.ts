import { useTokenStore } from '@/stores/token'
import { useCommonDialog, useCommonToast } from '@/util/common'
import axios from 'axios'
import { defineAsyncComponent } from 'vue'

const instance = axios.create({
  baseURL: import.meta.env.VITE_BASEURL,
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
