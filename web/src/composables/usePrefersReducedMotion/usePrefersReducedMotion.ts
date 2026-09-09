import { onScopeDispose, ref } from "vue";

import type { PrefersReducedMotion } from "./usePrefersReducedMotion.model";

const QUERY = "(prefers-reduced-motion: reduce)";

/**
 * Tracks whether the visitor has asked for reduced motion.
 *
 * CSS handles this for anything declarative; this is for the cases where the
 * motion is driven from script, such as the hero ticker's interval.
 *
 * @returns A ref that is true while reduced motion is preferred.
 */
export const usePrefersReducedMotion = (): PrefersReducedMotion => {
  const prefersReduced = ref(false);

  if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
    return prefersReduced;
  }

  const query = window.matchMedia(QUERY);
  prefersReduced.value = query.matches;

  const update = (event: MediaQueryListEvent): void => {
    prefersReduced.value = event.matches;
  };

  query.addEventListener("change", update);
  onScopeDispose(() => query.removeEventListener("change", update));

  return prefersReduced;
};
