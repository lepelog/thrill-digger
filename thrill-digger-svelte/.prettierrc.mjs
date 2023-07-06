/** @type {import("prettier").Options} */
const config = {
  tabWidth: 2,
  semi: true,
  singleQuote: true,
  htmlWhitespaceSensitivity: "ignore",
  plugins: ["prettier-plugin-svelte"],
};

export default config;
