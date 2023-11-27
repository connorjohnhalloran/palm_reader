import type { Config } from 'tailwindcss';
const defaultTheme = require('tailwindcss/defaultTheme')

const config: Config = {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx,css,md,mdx,html,json,scss}',
  ],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        'sans': ['Inter', 'Poppins', ...defaultTheme.fontFamily.sans],
      },
      colors: {
        'mb-gray': '#404040',
        'mb-gray-1': '#323232',
      },
      keyframes: {
        'fade-out': { '0%, 66%': {opacity: '1'}, '100%': {opacity: '0.3'} },
      },
      animation: {
        'fade-out': 'fade-out 3s linear forwards',
      }
    },
  },
  plugins: [],
};

export default config;
