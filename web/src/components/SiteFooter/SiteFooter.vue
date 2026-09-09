<template>
  <footer :class="bemm()">
    <div :class="bemm('inner')">
      <div :class="bemm('brand')">
        <p :class="bemm('name')">cdd</p>
        <p :class="bemm('line')">Same terminal. A smoother path.</p>
      </div>

      <nav :class="bemm('nav')" aria-label="Project">
        <p :class="bemm('heading')">Project</p>
        <a :class="bemm('link')" :href="LINKS.repo">GitHub</a>
        <a :class="bemm('link')" :href="LINKS.docs">Docs</a>
        <a :class="bemm('link')" :href="LINKS.changelog">Changelog</a>
        <a :class="bemm('link')" :href="LINKS.license">License</a>
      </nav>

      <nav :class="bemm('nav')" aria-label="Legal and support">
        <p :class="bemm('heading')">Help</p>
        <RouterLink :class="bemm('link')" to="/support">Support</RouterLink>
        <RouterLink :class="bemm('link')" to="/privacy">Privacy</RouterLink>
        <RouterLink :class="bemm('link')" to="/terms">Terms</RouterLink>
      </nav>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { useBemm } from "bemm";
import { RouterLink } from "vue-router";

import { LINKS } from "@/config/site";

const bemm = useBemm("site-footer", { includeBaseClass: true });
</script>

<style lang="scss">
.site-footer {
  padding: var(--spacing) var(--space-l) var(--space-l);

  &__inner {
    display: flex;
    flex-wrap: wrap;
    gap: var(--spacing);
    width: 100%;
    max-width: var(--max-content-width);
    margin: 0 auto;
    padding-top: var(--space-l);
    border-top: var(--border-width) solid
      color-mix(in srgb, var(--color-foreground), transparent 90%);
  }

  &__brand {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    margin-right: auto;
  }

  &__name {
    font-size: var(--font-size-l);
    font-weight: var(--font-weight-semibold);
  }

  &__line {
    color: color-mix(in srgb, var(--color-foreground), transparent 45%);
  }

  &__nav {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    min-width: 8em;
  }

  &__heading {
    color: color-mix(in srgb, var(--color-foreground), transparent 55%);
    font-size: var(--font-size-xs);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  &__link {
    color: color-mix(in srgb, var(--color-foreground), transparent 25%);
    text-decoration: none;
    transition: color var(--transition-normal) ease;

    &:hover {
      color: var(--color-foreground);
    }
  }
}

@media (max-width: 720px) {
  .site-footer {
    padding: var(--spacing) var(--space) var(--space-l);
  }
}
</style>
