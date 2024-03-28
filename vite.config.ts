import VueI18nPlugin from "@intlify/unplugin-vue-i18n/vite";
import { fileURLToPath, URL } from "node:url";
import svgLoader from "vite-svg-loader";
import tailwind from "tailwindcss";
import autoprefixer from "autoprefixer";
import Vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";
import VueRouter from "unplugin-vue-router/vite";
import { VueRouterAutoImports } from "unplugin-vue-router";
import AutoImport from "unplugin-auto-import/vite";

// https://vitejs.dev/config/
export default defineConfig({
  css: {
    postcss: {
      plugins: [tailwind(), autoprefixer()],
    },
  },
  base: "./",
  plugins: [
    VueRouter({
      routesFolder: [
        {
          src: "src/pages",
          path: "",
        },
      ],
    }),
    Vue(),
    svgLoader(),
    VueI18nPlugin({
      include: "./src/locales/**",
    }),
    AutoImport({
      dirs: ["src/composables", "src/schemas", "src/utils"],
      eslintrc: {
        enabled: true,
      },
      imports: [
        // presets
        "vue",
        "vue-i18n",
        "@vueuse/core",
        VueRouterAutoImports,
        {
          "unplugin-vue-router/data-loaders/basic": ["defineBasicLoader"],
        },
      ],
    }),
  ],
  server: {
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});
