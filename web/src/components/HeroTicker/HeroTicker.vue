<template>
  <div :class="bemm()">
    <p :class="bemm('static')">Change Directory</p>

    <ul :class="bemm('list')" aria-label="What the last d stands for">
      <li
        v-for="(word, position) in words"
        :key="word"
        :class="bemm('item', ['', position === index ? 'current' : ''])"
        @mouseenter="select(position)"
      >
        {{ word }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { useBemm } from "bemm";

import { usePrefersReducedMotion } from "@/composables/usePrefersReducedMotion";
import { useTicker } from "@/composables/useTicker";

import type { HeroTickerProps } from "./HeroTicker.model";

const bemm = useBemm("hero-ticker", { includeBaseClass: true });

const props = withDefaults(defineProps<HeroTickerProps>(), { intervalMs: 2200 });

const paused = usePrefersReducedMotion();
const { index, select } = useTicker(props.words, {
  intervalMs: props.intervalMs,
  paused,
});
</script>

<style lang="scss">
.hero-ticker {
  display: flex;
  align-items: center;
  gap: var(--space-l);

  &__static {
    font-size: var(--font-size-xxl);
    font-weight: var(--font-weight-normal);
    letter-spacing: -0.02em;
    line-height: 1.1;
  }

  &__list {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding-left: var(--space-l);
    border-left: var(--border-width) solid
      color-mix(in srgb, var(--color-foreground), transparent 88%);
    list-style: none;
  }

  &__item {
    padding: var(--space-xs) var(--space-s);
    border-radius: var(--border-radius-s);
    color: color-mix(in srgb, var(--color-foreground), transparent 55%);
    font-size: var(--font-size-l);
    cursor: default;
    transition:
      background-color var(--transition-normal) ease,
      color var(--transition-normal) ease;

    &--current {
      background-color: color-mix(in srgb, var(--color-primary), transparent 88%);
      color: var(--color-foreground);
    }
  }
}

@media (max-width: 720px) {
  .hero-ticker {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space);

    &__list {
      flex-direction: row;
      flex-wrap: wrap;
      padding-left: 0;
      border-left: none;
    }
  }
}
</style>
