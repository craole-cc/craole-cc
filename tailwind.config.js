/** @type {import('tailwindcss').Config} */
export const content = [
  "./src/**/*.rs",
  "./index.html",
];
export const theme = {
  extend: {
    colors: {
      'bg-primary': '#0f172a',
      'bg-secondary': '#1e293b',
      'text-primary': '#f1f5f9',
      'text-secondary': '#94a3b8',
    }
  },
};
export const plugins = [];
