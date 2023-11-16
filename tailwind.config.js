/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{js,jsx,ts,tsx,vue,css,html}", "./index.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        light: {
          ...require("daisyui/src/theming/themes")["light"],
          primary: "3EB1C8",
          secondary: "244C5A",
        },
        dark: {
          ...require("daisyui/src/theming/themes")["dark"],
          primary: "3EB1C8",
          secondary: "244C5A",
        },
      },
    ],
  },
};
