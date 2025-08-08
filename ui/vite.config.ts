import { fileURLToPath, URL } from 'node:url'

import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import Components from 'unplugin-vue-components/vite'
import { PrimeVueResolver } from '@primevue/auto-import-resolver'
// import UnoCSS from 'unocss/vite'
import tailwindcss from '@tailwindcss/vite'
import { visualizer } from "rollup-plugin-visualizer";
import modImageSuit from './build/code-gen-plugin.js'

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')

  return {
    plugins: [
      vue(),
      vueDevTools(),
      Components({
        resolvers: [
          PrimeVueResolver({
            components: {
              prefix: "Pv"
            }

          })
        ]
      }),
      tailwindcss(),
      visualizer(),
      modImageSuit()
      // codegen()
    ],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url))
      },
    },
    server: {
      proxy: {
        '/api': env.API_SERVER,
      },
      watch: {
        ignored: ['build/**']
      }
    },
    // experimental: {
    //   enableNativePlugin: true
    // }
  }
})
