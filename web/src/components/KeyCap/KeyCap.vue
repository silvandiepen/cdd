<template>
  <kbd :class="bemm()">{{ label }}</kbd>
</template>

<script setup lang="ts">
import { useBemm } from "bemm";

import type { KeyCapProps } from "./KeyCap.model";

const bemm = useBemm("key-cap", { includeBaseClass: true });

defineProps<KeyCapProps>();
</script>

<style lang="scss">
.key-cap {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 2.25em;
  padding: var(--space-xs) var(--space-s);
  border: var(--border-width) solid color-mix(in srgb, var(--color-foreground), transparent 85%);
  border-radius: var(--border-radius-s);
  background-color: color-mix(in srgb, var(--color-foreground), transparent 96%);
  font-family: var(--font-family-mono);
  font-size: var(--font-size-s);
  line-height: 1;
}
</style>
