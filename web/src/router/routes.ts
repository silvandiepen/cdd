import type { RouteRecordRaw } from "vue-router";

/**
 * A crawlable route and the sitemap metadata that goes with it.
 *
 * The sitemap is generated from this list at build time, so adding a page here
 * is the only step needed to get it indexed.
 */
export interface CrawlableRoute {
  path: string;
  changefreq: "daily" | "weekly" | "monthly" | "yearly";
  priority: string;
}

export const crawlableRoutes: CrawlableRoute[] = [
  { path: "/", changefreq: "weekly", priority: "1.0" },
  { path: "/support", changefreq: "monthly", priority: "0.5" },
  { path: "/privacy", changefreq: "yearly", priority: "0.3" },
  { path: "/terms", changefreq: "yearly", priority: "0.3" },
];

export const routes: RouteRecordRaw[] = [
  { path: "/", name: "home", component: () => import("@/views/HomeView") },
  { path: "/support", name: "support", component: () => import("@/views/SupportView") },
  { path: "/privacy", name: "privacy", component: () => import("@/views/PrivacyView") },
  { path: "/terms", name: "terms", component: () => import("@/views/TermsView") },
  { path: "/:pathMatch(.*)*", name: "not-found", component: () => import("@/views/NotFoundView") },
];
