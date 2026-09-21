import pluginVue from 'eslint-plugin-vue'
import {
  defineConfigWithVueTs,
  vueTsConfigs,
  configureVueProject,
} from '@vue/eslint-config-typescript'
import oxlint from 'eslint-plugin-oxlint'
import skipFormatting from '@vue/eslint-config-prettier/skip-formatting'

// 仓鼠助手现已全面 TS 化，.vue 仅允许 ts/tsx 脚本块。
configureVueProject({ scriptLangs: ['ts', 'tsx'] })
// More info at https://github.com/vuejs/eslint-config-typescript/#advanced-setup

export default defineConfigWithVueTs(
  {
    name: 'app/files-to-lint',
    files: ['**/*.{ts,mts,tsx,vue}'],
  },

  {
    name: 'app/files-to-ignore',
    ignores: ['**/dist/**', '**/dist-ssr/**', '**/coverage/**', 'cache/**', '**/legacy/**'],
  },

  pluginVue.configs['flat/essential'],
  vueTsConfigs.recommended,

  {
    // 原型自动抽取的世界状态渲染引擎，暂以 @ts-nocheck 跳过逐行类型校验。
    name: 'app/worldstate-port',
    files: ['src/lib/worldstate.ts'],
    rules: {
      '@typescript-eslint/ban-ts-comment': 'off',
    },
  },

  oxlint.configs['flat/recommended'],
  skipFormatting,
)
