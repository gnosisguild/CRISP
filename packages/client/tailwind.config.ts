import type { Config } from 'tailwindcss'

const config: Config = {
  content: ['./src/**/*.{js,jsx,ts,tsx,mdx}'],
  // safelist: ["show-scrollbar", "hide-scrollbar"],
  variant: {
    extend: {
      borderColor: ['disabled'],
      backgroundColor: ['disabled'],
      textColor: ['disabled'],
      boxShadow: ['disabled'],
      cursor: ['disabled'],
    },
  },
  theme: {
    extend: {
      fontFamily: {
        jakarta: ['Plus Jakarta Sans', 'sans-serif'],
      },
      colors: {
        slate: {
          200: '#E3E9F5',
        },
      },
      letterSpacing: {
        custom: '0.03em',
      },
      boxShadow: {
        button: '0 2px 0 0 #5F9715, 0 8px 16px rgba(0,0,0,0.1)',
        'button-outlined': '0 2px 0 0 #A6E05A, 0 8px 16px rgba(0,0,0,0.1)',
      },
    },
  },
  plugins: [],
}
export default config
