/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        colors: {
          pageBackground: '#FBFFFB',
        },
      },
    },
    plugins: [],
    important: true,
  }