<template>
  <article :class="bemm()">
    <header :class="bemm('header')">
      <h1 :class="bemm('title')">{{ title }}</h1>
      <p :class="bemm('intro')">{{ intro }}</p>
      <p v-if="updated" :class="bemm('updated')">Last reviewed {{ updated }}</p>
    </header>

    <div :class="bemm('body')">
      <slot />
    </div>
  </article>
</template>

<script setup lang="ts">
import { useBemm } from "bemm";

import type { ProseLayoutProps } from "./ProseLayout.model";

const bemm = useBemm("prose-layout", { includeBaseClass: true });

defineProps<ProseLayoutProps>();
</script>

<style lang="scss">
.prose-layout {
  display: flex;
  flex-direction: column;
  gap: var(--space-xl);
  width: 100%;
  max-width: var(--max-post-width);
  margin: 0 auto;
  padding: var(--spacing) var(--space-l);

  &__header {
    display: flex;
    flex-direction: column;
    gap: var(--space-s);
  }

  &__title {
    font-size: var(--font-size-xxxl);
    font-weight: var(--font-weight-semibold);
    letter-spacing: -0.02em;
    line-height: 1.1;
  }

  &__intro {
    color: color-mix(in srgb, var(--color-foreground), transparent 30%);
    font-size: var(--font-size-l);
    line-height: 1.6;
  }

  &__updated {
    color: color-mix(in srgb, var(--color-foreground), transparent 55%);
    font-size: var(--font-size-s);
  }

  &__body {
    display: flex;
    flex-direction: column;
    gap: var(--space-xl);
  }

  &__section {
    display: flex;
    flex-direction: column;
    gap: var(--space-s);
  }

  &__heading {
    font-size: var(--font-size-l);
    font-weight: var(--font-weight-semibold);
  }

  &__text {
    color: color-mix(in srgb, var(--color-foreground), transparent 20%);
    line-height: 1.7;
  }

  &__list {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding-left: var(--space);
    color: color-mix(in srgb, var(--color-foreground), transparent 20%);
    line-height: 1.7;
  }

  &__link {
    color: var(--color-primary);
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }
}

@media (max-width: 720px) {
  .prose-layout {
    padding: var(--space-xl) var(--space);
  }
}
</style>
