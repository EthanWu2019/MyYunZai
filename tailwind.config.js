/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        bg: {
          0: "#07080d",
          1: "#0d0f17",
          2: "#161924",
          3: "#1f2230",
        },
        accent: {
          DEFAULT: "#bf5af2",
          dim: "#7c3aed",
        },
        good: "#34d399",
        bad: "#f87171",
        warn: "#fbbf24",
      },
      fontFamily: {
        sans: ["'Inter'", "'Microsoft YaHei'", "system-ui", "sans-serif"],
        mono: ["'JetBrains Mono'", "'Cascadia Code'", "monospace"],
      },
    },
  },
  plugins: [],
};
