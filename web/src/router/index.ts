import { createRouter, createWebHistory } from "vue-router";

import { routes } from "./routes";

export const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior(to, _from, savedPosition) {
    if (savedPosition) return savedPosition;
    if (to.hash) return { el: to.hash, behavior: "smooth", top: 96 };
    return { top: 0 };
  },
});
