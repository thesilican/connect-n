import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwaitPlugin from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [wasm(), topLevelAwaitPlugin()],
});
