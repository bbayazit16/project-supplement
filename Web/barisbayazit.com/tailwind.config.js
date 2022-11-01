/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: "class",
    content: ["./pages/**/*.{js,ts,jsx,tsx}", "./components/**/*.{js,ts,jsx,tsx}"],
    safelist: [
        {
            pattern: /hljs+/,
        },
    ],
    theme: {
        hljs: {
            theme: "night-owl",
        },
    },
    plugins: [require("@tailwindcss/typography"), require("tailwind-highlightjs")],
}
