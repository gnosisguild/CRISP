import type { Config } from 'tailwindcss'

const config: Config = {
  content: ['./src/**/*.{js,jsx,ts,tsx,mdx}'],
  // safelist: ["show-scrollbar", "hide-scrollbar"],
  theme: {
    extend: {
      fontFamily: {
        jakarta: ['Plus Jakarta Sans', 'sans-serif'],
      },
      fontSize: {
        h1: '40px',
        h2: '32px',
        h3: '24px',
        // 'h4': '24px',
        // 'h5': '20px',
        // 'h6': '16px',
      },
      colors: {
        mist: {
          900: '#E3E9F5',
        },
        'twilight-blue': { 900: '#4B5665', 500: '#4B566580', 200: '#4B566533', 100: '#4B56651A' },
        green: {
          light: '#A6E05A',
          dark: { 900: '#5F9715', 800: '#5F9715CC' },
        },
        zinc: {
          900: '#35393E',
          500: '#35393E80',
        },
        gray: {
          900: '#8c9199',
        },
        white: {
          900: '#FFFFFF',
          600: '#FFFFFF99',
          500: '#FFFFFF80',
        },
      },
      lineHeight: {
        custom: '1.4', //
      },
      letterSpacing: {
        custom: '0.03em',
      },
      boxShadow: {
        modal: '0 12px 32px 0 rgba(75, 86, 101, 0.11)',
        'custom-1': '0 8px 16px 0 rgba(75, 86, 101, 0.1)',
        'custom-2': '0 2px 0 0 #5F9715',
      },
    },
  },
  plugins: [],
}
export default config
