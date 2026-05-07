import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        background: "var(--background)",
        foreground: "var(--foreground)",
        // YieldVeil brand palette
        brand: {
          50: "#eef2ff",
          100: "#dce4ff",
          200: "#b9c9ff",
          300: "#8ba6ff",
          400: "#5c7cff",
          500: "#3b52fc",
          600: "#2a3aef",
          700: "#1f2bd4",
          800: "#1a24ab",
          900: "#1b2286",
          950: "#111350",
        },
        accent: {
          DEFAULT: "#00f0b5",
          50: "#edfff8",
          100: "#d5fff0",
          200: "#aeffdf",
          300: "#70ffc8",
          400: "#2bfda9",
          500: "#00f0b5",
          600: "#00c47a",
          700: "#009a63",
          800: "#067850",
          900: "#076243",
          950: "#003824",
        },
        surface: {
          DEFAULT: "#0f1117",
          50: "#f6f6f8",
          100: "#ecedf1",
          200: "#d5d7e0",
          300: "#b0b4c5",
          400: "#858ba5",
          500: "#666d8a",
          600: "#525772",
          700: "#43475d",
          800: "#3a3d4f",
          900: "#1a1c2e",
          950: "#0f1117",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "monospace"],
      },
      borderRadius: {
        "2xl": "1rem",
        "3xl": "1.5rem",
      },
      boxShadow: {
        glow: "0 0 20px rgba(0, 240, 181, 0.15)",
        "glow-brand": "0 0 20px rgba(59, 82, 252, 0.2)",
        glass: "0 8px 32px rgba(0, 0, 0, 0.3)",
      },
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "hero-gradient":
          "linear-gradient(135deg, #0f1117 0%, #1a1c2e 40%, #111350 100%)",
      },
      animation: {
        "fade-in": "fadeIn 0.5s ease-out both",
        "slide-up": "slideUp 0.6s ease-out both",
        "pulse-glow": "pulseGlow 2s ease-in-out infinite",
        float: "float 6s ease-in-out infinite",
        shimmer: "shimmer 1.6s linear infinite",
        "blob-drift": "blobDrift 14s ease-in-out infinite",
      },
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        slideUp: {
          "0%": { opacity: "0", transform: "translateY(20px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
        pulseGlow: {
          "0%, 100%": { boxShadow: "0 0 20px rgba(0, 240, 181, 0.1)" },
          "50%": { boxShadow: "0 0 40px rgba(0, 240, 181, 0.3)" },
        },
        float: {
          "0%, 100%": { transform: "translateY(0)" },
          "50%": { transform: "translateY(-10px)" },
        },
        shimmer: {
          "0%": { backgroundPosition: "-200% 0" },
          "100%": { backgroundPosition: "200% 0" },
        },
        blobDrift: {
          "0%, 100%": { transform: "translate(-50%, -50%) scale(1)" },
          "33%": { transform: "translate(-48%, -52%) scale(1.05)" },
          "66%": { transform: "translate(-52%, -48%) scale(0.95)" },
        },
      },
    },
  },
  plugins: [],
};
export default config;
