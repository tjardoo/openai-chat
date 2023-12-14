const { fontFamily } = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
	theme: {
		extend: {
			fontFamily: {
				sans: ['Noto Sans', ...fontFamily.sans]
			}
		}
	},
	plugins: []
}
