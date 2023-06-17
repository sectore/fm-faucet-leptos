/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  plugins: [
    require("@tailwindcss/forms"),
  ],
};
