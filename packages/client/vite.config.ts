import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import viteTsconfigPaths from 'vite-tsconfig-paths'
import svgr from '@svgr/rollup'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import path from 'path'
import { NodeGlobalsPolyfillPlugin } from '@esbuild-plugins/node-globals-polyfill'
// const development: boolean = !process.env.NODE_ENV || process.env.NODE_ENV === 'development'

export default defineConfig({
  base: '/',
  define: {
    // here is the main update
    global: 'globalThis',
    'process.browser': 'true',
  },
  optimizeDeps: {
    exclude: ['@rollup/browser'],
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      libs: path.resolve(__dirname, './libs'),
    },
  },
  plugins: [
    // here is the main update
    wasm(),
    topLevelAwait(),
    react({
      jsxImportSource: '@emotion/react',
      babel: {
        plugins: ['@emotion/babel-plugin'],
      },
    }),
    viteTsconfigPaths(),
    svgr(),
    NodeGlobalsPolyfillPlugin({
      buffer: true,
    }),
  ],
  server: {
    open: true,
    // this sets a default port to 3000
    port: 3000,
  },
})
