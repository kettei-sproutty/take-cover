import type { Config } from 'tailwindcss';
import colors from 'tailwindcss/colors';

export default {
  content: ['./src/**/*.svelte'],
  theme: {
    fontFamily: {
			sans: ["Exo\\ 2"],
			mono: ["JetBrains\\ Mono"],
		},
    colors: {
      primary: {
        100: 'hsla(var(--primary-100), <alpha-value>)',
        200: 'hsla(var(--primary-200), <alpha-value>)',
        300: 'hsla(var(--primary-300), <alpha-value>)',
        400: 'hsla(var(--primary-400), <alpha-value>)',
        500: 'hsla(var(--primary-500), <alpha-value>)',
        600: 'hsla(var(--primary-600), <alpha-value>)',
        700: 'hsla(var(--primary-700), <alpha-value>)',
        800: 'hsla(var(--primary-800), <alpha-value>)',
        900: 'hsla(var(--primary-900), <alpha-value>)',
      },
      transparent: colors.transparent,
    },
  },
} satisfies Config;
