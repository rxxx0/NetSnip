/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        'sans': ['Inter', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'sans-serif'],
        'mono': ['JetBrains Mono', 'SF Mono', 'Monaco', 'monospace'],
      },
      colors: {
        // Use CSS variables for dynamic theming
        'neu-bg': 'var(--neu-bg)',
        'neu-surface': 'var(--neu-surface)',
        'neu-light': 'var(--neu-light)',
        'neu-dark': 'var(--neu-dark)',

        // Accent colors
        'accent': {
          'primary': 'var(--accent-primary)',
          'primary-light': 'var(--accent-primary-light)',
          'success': 'var(--accent-success)',
          'warning': 'var(--accent-warning)',
          'danger': 'var(--accent-danger)',
        },

        // Text colors
        'text': {
          'primary': 'var(--text-primary)',
          'secondary': 'var(--text-secondary)',
          'muted': 'var(--text-muted)',
        },
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