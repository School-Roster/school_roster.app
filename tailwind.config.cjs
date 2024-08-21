const config = {
  content: ['./interface/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],

  plugins: [require('flowbite/plugin')],

  darkMode: 'selector',

  theme: {
    extend: {
      colors: {
        // flowbite-svelte
        primary: {
          50: '#FFF5F2',
          100: '#FFF1EE',
          200: '#FFE4DE',
          300: '#FFD5CC',
          400: '#FFBCAD',
          500: '#FE795D',
          600: '#EF562F',
          700: '#EB4F27',
          800: '#CC4522',
          900: '#A5371B'
        },

        // Main app colors
        ss: {
          'bg-color': '#f5f5f5',
          'fg-color': '#242424',
          'gray': '#5F6C7B',
          'gray-dark': '#8C8C8C',
          'blue': '#3da9fc',
          'blue-dark': '#094067',
          'red': '#ef4565',
        }
      }
    },
    // App default fonts
    fontFamily: {
      sans: ['Inria Sans', 'sans-serif'],
      serif: ['Inria Serif', 'serif'],
    }
  }
};

module.exports = config;
