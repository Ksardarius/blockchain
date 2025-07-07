// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import storybook from "eslint-plugin-storybook";

import globals from 'globals'
import eslint from '@eslint/js'
import tseslint from 'typescript-eslint'
import pluginReact from 'eslint-plugin-react'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import eslintPluginPrettierRecomended from 'eslint-plugin-prettier/recommended'

/** @type {import('eslint').Linter.Config[]} */
export default [
    {files: ['**/*.{js,mjs,cjs,ts,jsx,tsx}']},
    {languageOptions: {ecmaVersion: 2020, globals: globals.browser}},
    {
        ignores: [
            '**/node_modules/**',
            'coverage',
            '**/public',
            '**/dist',
            'pnpm-lock.yaml',
            'pnpm-workspace.yaml',
            '**/tailwind.config.js'
        ]
    },
    {
        files: ['packages/ui/**/*.{ts,tsx}'],
        settings: {
            react: {version: '18.2'}
        },
        // languageOptions: {
        //     parserOptions: {
        //         project: ['./tsconfig.node.json', './tsconfig.app.json'],
        //         tsconfigRootDir: './src/CookingApp/packages/ui'
        //     }
        // },
        plugins: {
            'react': pluginReact,
            'react-hooks': reactHooks
        },
        rules: {
            ...pluginReact.configs.recommended.rules,
            ...pluginReact.configs['jsx-runtime'].rules,
            ...reactHooks.configs.recommended.rules
        }
    },
    {
        files: ['packages/ui/**/*.{ts,tsx}'],
        settings: {
            react: {version: '18.2'}
        },
        plugins: {
            'react-refresh': reactRefresh
        },
        rules: {
            'react-refresh/only-export-components': ['warn', {allowConstantExport: true}]
        }
    },
    eslint.configs.recommended,
    ...tseslint.configs.recommended,
    ...tseslint.configs.stylistic,
    eslintPluginPrettierRecomended,
    ...storybook.configs["flat/recommended"]
];
