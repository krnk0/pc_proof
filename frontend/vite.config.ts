import { defineConfig } from 'vite'
import { resolve } from 'path'
import solid from 'vite-plugin-solid'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [wasm(), topLevelAwait(), solid()],
  resolve: {
    alias: {
      'pc_proof_wasm': resolve(__dirname, './pkg/pc_proof_wasm.js'),
    },
  },
})
