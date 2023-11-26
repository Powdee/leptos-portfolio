/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        white: "#FFF",
        gray: "#A8A8A8",
        black: "#181414",
      },
      fontSize: {
        "10xl": "9.5rem",
      },
      lineHeight: {
        tighter: 1.15,
        relaxish: 1.8,
      },
      maxWidth: {
        "8xl": "88rem",
      },
    },
  },
  plugins: [],
};
