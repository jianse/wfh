<template>
  <div class="w-full h-full flex items-center flex-row justify-center">
    <div class="flex justify-center flex-col gap-4 w-96">
      <p class="text-2xl font-bold">登录</p>
      <div class="flex flex-col gap-1">
        <PvInputText
          name="username"
          v-model="userCred.username"
          type="text"
          placeholder="Username"
          @change="validate"
        />
        <PvMessage
          v-for="error in errors.username?.errors"
          :key="error"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ error }}</PvMessage
        >
      </div>
      <div class="flex flex-col gap-1">
        <PvPassword
          v-model="userCred.password"
          pt:pcInputText:root:class="w-full"
          name="password"
          type="text"
          @change="validate"
          placeholder="Password"
        />
        <PvMessage
          v-for="error in errors.password?.errors"
          :key="error"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ error }}</PvMessage
        >
      </div>
      <PvButton severity="secondary" label="登录" @click="onFormSubmit" :loading="loading" />
    </div>
    <PvToast />
  </div>
</template>
<script lang="ts" setup>
import { ref } from 'vue'
// import { zodResolver } from '@primevue/forms/resolvers/zod'
import { useToast } from 'primevue/usetoast'
// import { Form, type FormSubmitEvent } from '@primevue/forms'
import { z } from 'zod/v4'
import { get_token } from '@/api/token'
import { is_success } from '@/api'
import { useTokenStore } from '@/stores/token'
import { useRouter } from 'vue-router'

const router = useRouter()

const toast = useToast()
const userCred = ref({
  username: '',
  password: '',
})
interface ErrorItem {
  errors: string[]
}

const errors = ref<Record<string, ErrorItem>>({})

const schema = z.object({
  username: z.string().min(1, { message: '请输入用户名' }),
  password: z.string().min(1, { message: '请输入密码' }),
})

function validate(): boolean {
  const result = schema.safeParse(userCred.value)
  if (!result.success) {
    const tree = z.treeifyError(result.error)
    errors.value = tree.properties || {}
  } else {
    errors.value = {}
  }

  return result.success
}
const loading = ref(false)
const { setToken } = useTokenStore()

const onFormSubmit = async () => {
  loading.value = true
  const valid = validate()
  if (valid) {
    const res = await get_token(userCred.value)
    if (is_success(res)) {
      const token = res.data?.token || ''
      await setToken(token)
      setTimeout(() => {
        router.back()
      }, 1000)
    } else {
      toast.add({
        severity: 'info',
        summary: 'Info',
        detail: '用户名或密码错误',
        life: 3000,
      })
    }
  }
  loading.value = false
}
</script>
