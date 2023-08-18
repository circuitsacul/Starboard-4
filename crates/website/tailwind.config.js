/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  daisyui: {
    themes: [
      {
        dark: {
          primary: "#ffe19c",
          'primary-focus' : '#a08e64',
          secondary: "#e879f9",
          accent: "#67e8f9",
          neutral: "#2a323c",
          "base-100": "#1d232a",
          info: "#3abff8",
          success: "#36d399",
          warning: "#fbbd23",
          error: "#f87272",
        },
      },
    ],
  },
  plugins: [require("daisyui")],
};
