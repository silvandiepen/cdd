<template>
  <figure :class="bemm()" role="img" :aria-label="describedAs">
    <div :class="bemm('chrome')" aria-hidden="true">
      <span :class="bemm('dot', 'close')" />
      <span :class="bemm('dot', 'minimise')" />
      <span :class="bemm('dot', 'zoom')" />
      <span :class="bemm('shell')">zsh</span>
    </div>

    <div :class="bemm('body')" aria-hidden="true">
      <p :class="bemm('line')">
        <span :class="bemm('path')">{{ prompt }}</span>
        <span :class="bemm('sigil')">%</span>
        <span :class="bemm('command')">{{ command }}</span>
        <span v-if="caret" :class="bemm('caret')" />
      </p>

      <ul :class="bemm('results')">
        <li
          v-for="(result, position) in results"
          :key="result"
          :class="bemm('result', ['', position === selected ? 'selected' : ''])"
        >
          <span :class="bemm('marker')">{{ position === selected ? "▸" : "" }}</span>
          <span :class="bemm('name')">{{ result }}</span>
        </li>
        <li v-if="more > 0" :class="bemm('result', 'more')">
          <span :class="bemm('marker')" />
          <span :class="bemm('name')">+ {{ more }} more</span>
        </li>
      </ul>
    </div>
  </figure>
</template>

<script setup lang="ts">
import { useBemm } from "bemm";
import { computed } from "vue";

import type { TerminalMockProps } from "./TerminalMock.model";

const bemm = useBemm("terminal-mock", { includeBaseClass: true });

const props = withDefaults(defineProps<TerminalMockProps>(), {
  selected: 0,
  more: 0,
  caret: false,
  label: "",
});

/**
 * The mockup is decorative markup, so the whole thing is exposed to assistive
 * technology as one description rather than as a list a screen reader would
 * have to walk through character by character.
 */
const describedAs = computed(
  () =>
    props.label ||
    `Terminal showing ${props.command} in ${props.prompt}, ` +
      `with ${props.results.join(", ")} listed below the prompt.`,
);
</script>

<style lang="scss">
.terminal-mock {
  --terminal-surface: var(--color-dark);
  --terminal-chrome: color-mix(in srgb, var(--color-dark), var(--color-light) 6%);
  --terminal-text: color-mix(in srgb, var(--color-light), transparent 8%);
  --terminal-muted: color-mix(in srgb, var(--color-light), transparent 55%);

  display: flex;
  flex-direction: column;
  margin: 0;
  overflow: hidden;
  border-radius: var(--border-radius);
  background-color: var(--terminal-surface);
  box-shadow: 0 24px 60px color-mix(in srgb, var(--color-dark), transparent 88%);

  &__chrome {
    display: flex;
    align-items: center;
    gap: var(--space-s);
    padding: var(--space-s) var(--space);
    background-color: var(--terminal-chrome);
  }

  &__dot {
    width: 0.75em;
    height: 0.75em;
    border-radius: var(--border-radius-round);

    &--close {
      background-color: var(--color-red);
    }

    &--minimise {
      background-color: var(--color-gold);
    }

    &--zoom {
      background-color: var(--color-green);
    }
  }

  &__shell {
    margin-left: auto;
    color: var(--terminal-muted);
    font-size: var(--font-size-s);
  }

  &__body {
    display: flex;
    flex-direction: column;
    gap: var(--space);
    padding: var(--space-l) var(--space);
    font-family: var(--font-family-mono);
    font-size: var(--font-size-s);
    line-height: 1.6;
  }

  &__line {
    display: flex;
    align-items: center;
    gap: 0.5ch;
  }

  &__path {
    color: var(--color-secondary);
  }

  &__sigil,
  &__command {
    color: var(--terminal-text);
  }

  &__caret {
    width: 0.55em;
    height: 1.1em;
    background-color: var(--terminal-text);
  }

  &__results {
    display: flex;
    flex-direction: column;
    list-style: none;
  }

  &__result {
    display: flex;
    gap: 0.5ch;
    padding: var(--space-xs) var(--space-s);
    border-radius: var(--border-radius-s);
    color: var(--terminal-text);

    // Selection is a tinted fill and the marker cdd itself prints. No rail.
    &--selected {
      background-color: color-mix(in srgb, var(--color-secondary), transparent 78%);
    }

    &--more {
      color: var(--terminal-muted);
    }
  }

  &__marker {
    width: 1ch;
    color: var(--color-secondary);
  }
}

@media (prefers-reduced-motion: no-preference) {
  .terminal-mock__caret {
    animation: terminal-mock-caret 1.1s steps(1, end) infinite;
  }
}

@keyframes terminal-mock-caret {
  0%,
  50% {
    opacity: 1;
  }
  51%,
  100% {
    opacity: 0;
  }
}
</style>
