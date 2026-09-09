<template>
  <a :class="bemm('', variant)" :href="href">
    <span :class="bemm('label')"><slot /></span>
    <svg
      v-if="arrow"
      :class="bemm('arrow')"
      viewBox="0 0 16 16"
      aria-hidden="true"
      focusable="false"
    >
      <path
        d="M2.5 8h10M9 4.5 12.5 8 9 11.5"
        fill="none"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
  </a>
</template>

<script setup lang="ts">
import { useBemm } from "bemm";

import type { ActionLinkProps } from "./ActionLink.model";

const bemm = useBemm("action-link", { includeBaseClass: true });

withDefaults(defineProps<ActionLinkProps>(), { variant: "primary", arrow: false });
</script>

<style lang="scss">
.action-link {
  display: inline-flex;
  align-items: center;
  gap: var(--space-s);
  padding: var(--space-s) var(--space-l);
  border: var(--border-width) solid transparent;
  border-radius: var(--border-radius-xl);
  font-size: var(--font-size);
  font-weight: var(--font-weight-semibold);
  text-decoration: none;
  transition:
    background-color var(--transition-normal) ease,
    border-color var(--transition-normal) ease,
    color var(--transition-normal) ease,
    transform var(--transition-fast) ease;

  &:hover {
    transform: translateY(-1px);
  }

  &:focus-visible {
    outline: none;
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--color-focus), transparent 70%);
  }

  &--primary {
    background-color: var(--color-foreground);
    color: var(--color-background);
  }

  &--outline {
    border-color: color-mix(in srgb, var(--color-foreground), transparent 82%);
    color: var(--color-foreground);

    &:hover {
      background-color: color-mix(in srgb, var(--color-foreground), transparent 94%);
    }
  }

  &--ghost {
    padding-inline: var(--space-s);
    color: color-mix(in srgb, var(--color-foreground), transparent 25%);

    &:hover {
      color: var(--color-foreground);
    }
  }

  &__arrow {
    width: 1em;
    height: 1em;
  }
}

@media (prefers-reduced-motion: reduce) {
  .action-link:hover {
    transform: none;
  }
}
</style>
