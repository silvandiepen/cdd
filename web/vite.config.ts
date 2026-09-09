import { fileURLToPath, URL } from "node:url";
import type { Plugin } from "vite";
import { defineConfig } from "vitest/config";
import vue from "@vitejs/plugin-vue";

import { SITE_URL } from "./src/config/site";
import { crawlableRoutes } from "./src/router/routes";

/**
 * Writes `sitemap.xml` from the router's own route table, so a new page cannot
 * ship without being crawlable. `robots.txt` is static and lives in `public/`.
 */
const sitemap = (): Plugin => ({
  name: "cdd-sitemap",
  apply: "build",
  generateBundle() {
    const today = new Date().toISOString().slice(0, 10);
    const urls = crawlableRoutes
      .map(
        (route) =>
          `  <url>\n` +
          `    <loc>${SITE_URL}${route.path === "/" ? "" : route.path}</loc>\n` +
          `    <lastmod>${today}</lastmod>\n` +
          `    <changefreq>${route.changefreq}</changefreq>\n` +
          `    <priority>${route.priority}</priority>\n` +
          `  </url>`,
      )
      .join("\n");

    this.emitFile({
      type: "asset",
      fileName: "sitemap.xml",
      source:
        `<?xml version="1.0" encoding="UTF-8"?>\n` +
        `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`,
    });
  },
});

export default defineConfig({
  plugins: [vue(), sitemap()],
  resolve: {
    alias: { "@": fileURLToPath(new URL("./src", import.meta.url)) },
  },
  css: {
    preprocessorOptions: {
      scss: { loadPaths: ["node_modules"], api: "modern-compiler" },
    },
  },
  test: {
    environment: "happy-dom",
    include: ["src/**/*.test.ts"],
  },
});
