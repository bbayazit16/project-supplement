module.exports = {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    fontFamily: {
      sans: ['Dongle', 'sans-serif']
    },
    extend: {
      rotate: {
        360: '360deg'
      },
      colors: {
        lapis: {
          50: '#f8faf9',
          100: '#ecf1f5',
          200: '#d4dfe9',
          300: '#aabece',
          400: '#7996aa',
          500: '#5c7388',
          600: '#4a596a',
          700: '#1a263b',
          800: '#1a2a45',
          900: '#111824'
        }
      }
    }
  },
  plugins: [require('tailwind-scrollbar-hide')]
};
