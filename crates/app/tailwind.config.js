module.exports = {
  content: ["./src/**/*.{rs,html,js}", "./index.html"],
  darkMode: "media",
  theme: {
    extend: {
      colors: {
        "main-100": "#edf2fb",
        "main-200": "#e2eafc",
        "main-300": "#d7e3fc",
        "main-400": "#ccdbfd",
        "main-500": "#c1d3fe",
        "main-600": "#b6ccfe",
        "main-700": "#abc4ff",
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
