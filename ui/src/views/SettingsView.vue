<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { usePasskeyStore } from '@/stores/passkeys'
import { useUiStore } from '@/stores/ui'
import { logoutAndRedirect } from '@/lib/logout'
import type { Passkey } from '@/data/admin'

const auth = useAuthStore()
const passkeys = usePasskeyStore()
const ui = useUiStore()
const router = useRouter()
const route = useRoute()

const keyName = ref('我的设备')

function goLogin() {
  router.push({ name: 'wfh-login', query: { redirect: route.fullPath } })
}

function addPasskey() {
  const label = passkeys.add(keyName.value)
  keyName.value = '我的设备'
  ui.toast('ok', '已注册', `通行密钥「${label}」已注册。`)
}

function removePasskey(pk: Passkey) {
  ui.askConfirm('移除通行密钥', `确定要移除「${pk.name}」吗？`, () => {
    passkeys.remove(pk.id)
    ui.toast('ok', '已移除', '通行密钥已移除。')
  })
}

function logout() {
  logoutAndRedirect()
}
</script>

<template>
  <section class="view active" id="view-settings" data-od-id="settings">
    <div class="page-head">
      <p class="eyebrow">系统 · 设置</p>
      <h1 class="h2">账号设置</h1>
      <p class="lead">查看当前账号信息，并管理用于免密登录的通行密钥。</p>
    </div>

    <div v-if="!auth.isLoggedIn" id="settings-guest" class="panel">
      <div class="panel-head"><span class="panel-title">尚未登录</span></div>
      <p class="panel-desc mb-sm">登录后可查看账号信息并管理通行密钥。</p>
      <button class="btn btn-primary" @click="goLogin">前往登录</button>
    </div>

    <div v-else class="panel-grid two items-start" id="settings-body">
      <div class="panel">
        <div class="panel-head"><span class="panel-title">账号信息</span></div>
        <div class="field"><label>用户名</label><input class="input" id="set-username" :value="auth.displayName" disabled /></div>
        <div class="field"><label>角色</label><input class="input" id="set-role" :value="auth.roleText" disabled /></div>
        <button class="btn btn-secondary" id="set-logout" @click="logout">退出登录</button>
      </div>
      <div class="panel">
        <div class="panel-head"><span class="panel-title">通行密钥（Passkey）</span></div>
        <p class="panel-desc">使用设备生物识别或 PIN 免密登录，无需输入密码。</p>
        <div class="field mt-sm">
          <label for="passkey-name">密钥名称</label>
          <input v-model="keyName" class="input" id="passkey-name" type="text" />
        </div>
        <button class="btn btn-secondary mb-sm" id="passkey-add" @click="addPasskey">注册新通行密钥</button>
        <div id="passkey-list">
          <p v-if="!passkeys.items.length" class="help-note">尚未注册任何通行密钥。</p>
          <div v-for="p in passkeys.items" :key="p.id" class="passkey-row">
            <span class="pk-ico"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><circle cx="8" cy="15" r="4" /><path d="M11 12l9-9M18 5l2 2M16 7l2 2" /></svg></span>
            <div class="flex-1 min-w-0">
              <div class="pk-name">{{ p.name }}</div>
              <div class="help-note">注册于 {{ p.created }}</div>
            </div>
            <button class="icon-btn danger" :title="'移除 ' + p.name" @click="removePasskey(p)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M4 7h16M9 7V4h6v3M7 7l1 13h8l1-13" /></svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
