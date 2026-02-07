module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  ignorePatterns: [
    "dist",
    "node_modules",
    "*.config.js",
    "*.config.ts",
    "src-tauri",
    "public",
    "src/components/ui",
  ],
  parser: "vue-eslint-parser",
  parserOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
    parser: "@typescript-eslint/parser",
  },
  plugins: ["@typescript-eslint", "prettier"],
  extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended", "prettier"],
  rules: {
    "prettier/prettier": "error",
    "@typescript-eslint/no-explicit-any": "warn",
    "@typescript-eslint/no-unused-vars": ["warn", { argsIgnorePattern: "^_" }],
  },
};
