import ts from 'typescript-eslint';
import vue from 'eslint-plugin-vue';
import prettier from 'eslint-plugin-prettier';
import prettierConfig from 'eslint-config-prettier';

export default [
  {
    ignores: [
      'dist',
      'node_modules',
      '*.config.js',
      '*.config.ts',
      'src-tauri',
      'public',
      'src/components/ui',
      'src/vite-env.d.ts',
    ],
  },
  ...ts.configs.recommended,
  ...vue.configs['flat/recommended'],
  prettierConfig,
  {
    plugins: {
      prettier,
    },
    rules: {
      'prettier/prettier': 'error',
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
    },
  },
  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: vue.parser,
      parserOptions: {
        parser: ts.parser,
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
    },
  },
];