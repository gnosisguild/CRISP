import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import viteTsconfigPaths from 'vite-tsconfig-paths'
import svgr from '@svgr/rollup'

const development: boolean = !process.env.NODE_ENV || process.env.NODE_ENV === 'development'

export default defineConfig({
  base: development ? '' : '/CRISP',
  define: {
    // here is the main update
    global: 'globalThis',
  },
  plugins: [
    // here is the main update
    react({
      jsxImportSource: '@emotion/react',
      babel: {
        plugins: ['@emotion/babel-plugin'],
      },
    }),
    viteTsconfigPaths(),
    svgr(),
  ],
  server: {
    open: true,
    // this sets a default port to 3000
    port: 3000,
  },
})
