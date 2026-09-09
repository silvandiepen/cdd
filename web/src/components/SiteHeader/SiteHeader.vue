<template>
  <header :class="bemm()">
    <div :class="bemm('inner')">
      <RouterLink :class="bemm('brand')" to="/">cdd</RouterLink>

      <nav :class="bemm('nav')" aria-label="Primary">
        <a :class="bemm('link')" :href="LINKS.docs">Docs</a>
        <a :class="bemm('link')" :href="LINKS.repo">GitHub</a>
        <a :class="bemm('link')" :href="LINKS.changelog">Changelog</a>
      </nav>

      <p :class="bemm('tagline')">{{ SITE_TAGLINE }}</p>
    </div>
  </header>
</template>

<script setup lang="ts">
import { useBemm } from "bemm";
import { RouterLink } from "vue-router";

import { LINKS, SITE_TAGLINE } from "@/config/site";

const bemm = useBemm("site-header", { includeBaseClass: true });
</script>

<style lang="scss">
.site-header {
  padding: var(--space-l) var(--space-l) 0;

  &__inner {
    display: flex;
    align-items: baseline;
    gap: var(--space-l);
    width: 100%;
    max-width: var(--max-content-width);
    margin: 0 auto;
  }

  &__brand {
    color: var(--color-foreground);
    font-size: var(--font-size-l);
    font-weight: var(--font-weight-semibold);
    letter-spacing: -0.01em;
    text-decoration: none;
  }

  &__nav {
    display: flex;
    gap: var(--space);
  }

  &__link {
    color: color-mix(in srgb, var(--color-foreground), transparent 30%);
    text-decoration: none;
    transition: color var(--transition-normal) ease;

    &:hover {
      color: var(--color-foreground);
    }
  }

  &__tagline {
    margin-left: auto;
    color: color-mix(in srgb, var(--color-foreground), transparent 55%);
    font-size: var(--font-size-s);
  }
}

@media (max-width: 720px) {
  .site-header {
    padding: var(--space) var(--space) 0;

    &__tagline {
      display: none;
    }
  }
}
</style>
