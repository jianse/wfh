<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useUiStore } from '@/stores/ui'

const auth = useAuthStore()
const ui = useUiStore()
const router = useRouter()
const route = useRoute()

const username = ref('')
const password = ref('')
const userErr = ref('')
const passErr = ref('')
const loading = ref(false)

async function submit() {
  if (loading.value) return
  userErr.value = ''
  passErr.value = ''
  loading.value = true
  try {
    const res = await auth.authenticate(username.value, password.value)
    if (!res.ok) {
      if (res.field === 'user') userErr.value = res.msg ?? ''
      else passErr.value = res.msg ?? ''
      return
    }
    const name = auth.displayName
    username.value = ''
    password.value = ''
    ui.toast('ok', '登录成功', `欢迎回来，${name}。`)
    goAfterLogin()
  } finally {
    loading.value = false
  }
}

/** 登录后返回此前正在浏览的页面；若无记录或权限不足则回主页。 */
function goAfterLogin() {
  const redirect = route.query.redirect
  if (typeof redirect === 'string' && redirect.startsWith('/')) {
    const target = router.resolve(redirect)
    const required = target.meta.role as string | undefined
    if (!required || auth.hasRole(required)) {
      router.replace(redirect)
      return
    }
  }
  router.push({ name: 'wfh-home' })
}
</script>

<template>
  <section class="view active" id="view-login" data-od-id="login">
    <div class="login-wrap">
      <div class="login-card">
        <span class="brand-mark">仓</span>
        <h1 class="login-title">登录仓鼠助手</h1>
        <p class="help-note mt-xs">使用维护者或管理员账号登录，解锁数据管理后台。</p>
        <div class="mt-md">
          <div class="field">
            <label for="login-user">用户名</label>
            <input v-model="username" class="input" id="login-user" type="text" placeholder="请输入用户名" autocomplete="username" @keydown.enter="submit" />
            <div class="field-err" id="login-user-err">{{ userErr }}</div>
          </div>
          <div class="field">
            <label for="login-pass">密码</label>
            <input v-model="password" class="input" id="login-pass" type="password" placeholder="请输入密码" autocomplete="current-password" @keydown.enter="submit" />
            <div class="field-err" id="login-pass-err">{{ passErr }}</div>
          </div>
          <button class="btn btn-primary w-full justify-center" id="login-submit" :disabled="loading" @click="submit">{{ loading ? '登录中…' : '登录' }}</button>
        </div>
        <div class="demo-list">
          <span class="meta">请使用站点后端账号登录，登录状态会保存在本地。</span>
        </div>
      </div>
    </div>
  </section>
</template>
