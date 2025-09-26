/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        'neu-bg': '#E0E5EC',
        'neu-surface': '#E0E5EC',
        'neu-primary': '#6C5CE7',
        'neu-secondary': '#00B894',
        'neu-danger': '#FF6B6B',
        'neu-warning': '#FDCB6E',
        'neu-text': '#2D3436',
        'neu-text-secondary': '#636E72',

        // Dark mode colors
        'dark-bg': '#1E1E2E',
        'dark-surface': '#2A2A3E',
        'dark-primary': '#7C6FE8',
        'dark-secondary': '#00C9A7',
        'dark-text': '#F5F5F5',
        'dark-text-secondary': '#B2B2C8',
      },
      boxShadow: {
        'neu-light': '6px 6px 12px rgba(163, 177, 198, 0.6), -6px -6px 12px rgba(255, 255, 255, 0.5)',
        'neu-dark': '6px 6px 12px rgba(0, 0, 0, 0.3), -6px -6px 12px rgba(50, 50, 70, 0.3)',
        'neu-inset': 'inset 5px 5px 10px rgba(163, 177, 198, 0.6), inset -5px -5px 10px rgba(255, 255, 255, 0.5)',
        'neu-inset-dark': 'inset 5px 5px 10px rgba(0, 0, 0, 0.3), inset -5px -5px 10px rgba(50, 50, 70, 0.3)',
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'fade-in': 'fadeIn 0.3s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}