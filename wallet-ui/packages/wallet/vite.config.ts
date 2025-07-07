import {defineConfig} from 'vite'
import {resolve} from 'path'
import dts from 'vite-plugin-dts'
import wasm from 'vite-plugin-wasm'

// https://vite.dev/config/
export default defineConfig({
    build: {
        copyPublicDir: false,
        lib: {
            entry: resolve(__dirname, './lib/index.ts'),
            name: 'WasmLibrary',
            fileName: 'index',
            formats: ['es'],
        },
        minify: false,
        target: 'esnext',
        outDir: 'dist'
    },
    plugins: [wasm(), dts({tsconfigPath: './tsconfig.app.json'})],
    css: {
        postcss: {
            plugins: []
        }
    }
})
