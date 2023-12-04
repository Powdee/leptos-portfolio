/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      screens: {
        "2xl": "2400px",
      },
      backgroundColor: {
        "gray-1": "#F8F9FA",
        "gray-2": "#E9ECEF",
        "gray-3": "#DEE2E6",
        "gray-4": "#CED4DA",
        "gray-5": "#ADB5BD",
        "gray-6": "#6C757D",
        "gray-7": "#495057",
        "gray-8": "#343A40",
        "gray-9": "#212529",
      },
      colors: {
        "gray-1": "#F8F9FA",
        "gray-2": "#E9ECEF",
        "gray-3": "#DEE2E6",
        "gray-4": "#CED4DA",
        "gray-5": "#ADB5BD",
        "gray-6": "#6C757D",
        "gray-7": "#495057",
        "gray-8": "#343A40",
        "gray-9": "#212529",
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
        "10xl": "104rem",
      },
      minHeight: {
        card_1_row: "22rem",
        card_1_row_mobile: "18rem",

        card_2_row: "39rem",
        card_2_row_mobile: "18rem",
      },
    },
  },
  plugins: [],
};
