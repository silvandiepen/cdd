<template>
  <div :class="bemm()">
    <section :class="bemm('hero')" aria-labelledby="hero-title">
      <div :class="bemm('hero-copy')">
        <h1 id="hero-title" :class="bemm('wordmark')">cdd</h1>

        <HeroTicker :words="TICKER_WORDS" />

        <div :class="bemm('lede')">
          <p :class="bemm('lede-line')">Type, filter, arrow down, enter.</p>
          <p :class="bemm('lede-line')">Navigate directories without breaking your flow.</p>
        </div>

        <div :class="bemm('actions')">
          <ActionLink variant="primary" href="#install" arrow>
            {{ HOMEBREW_AVAILABLE ? "Install with Homebrew" : "How to install" }}
          </ActionLink>
          <ActionLink variant="outline" :href="LINKS.spec">Read the spec</ActionLink>
        </div>

        <ul :class="bemm('meta')">
          <li v-if="!HOMEBREW_AVAILABLE" :class="bemm('meta-item', 'status')">Pre-release</li>
          <li :class="bemm('meta-item')">Open source</li>
          <li :class="bemm('meta-item')">macOS first</li>
          <li :class="bemm('meta-item')">Zsh support</li>
          <li :class="bemm('meta-item')">No telemetry</li>
        </ul>
      </div>

      <div :class="bemm('hero-visual')">
        <TerminalMock
          prompt="~/Repositories"
          command="cdd im"
          :results="['imagekid', 'image-tools', 'simple-image']"
          :selected="0"
          caret
        />
      </div>
    </section>

    <section :class="bemm('section')" aria-labelledby="why-title">
      <div :class="bemm('section-head')">
        <p :class="bemm('eyebrow')">Why it exists</p>
        <h2 id="why-title" :class="bemm('title')">Changing directories should feel lighter.</h2>
      </div>

      <div :class="bemm('prose')">
        <p :class="bemm('paragraph')">
          Normal shell navigation often splits into two steps: look, then move.
        </p>
        <p :class="bemm('paragraph')">You run ls, scan the result, then type cd.</p>
        <p :class="bemm('paragraph')">
          cdd brings those steps together. While you type, it shows matching directories right below
          the prompt, so you can see, filter, and move in one flow.
        </p>
      </div>
    </section>

    <section :class="bemm('section')" aria-labelledby="how-title">
      <div :class="bemm('section-head')">
        <p :class="bemm('eyebrow')">How it works</p>
        <h2 id="how-title" :class="bemm('title')">One small command, one smoother flow.</h2>
      </div>

      <div :class="bemm('how')">
        <ol :class="bemm('steps')">
          <li v-for="(step, position) in STEPS" :key="step.title" :class="bemm('step')">
            <span :class="bemm('step-number')">{{ position + 1 }}</span>
            <span :class="bemm('step-title')">{{ step.title }}</span>
            <code :class="bemm('step-example')">{{ step.example }}</code>
          </li>
        </ol>

        <TerminalMock
          prompt="~/app"
          command="cdd src/co"
          :results="['components', 'composables', 'config']"
          :selected="0"
          label="Terminal showing cdd src/co listing components, composables and config."
        />
      </div>

      <ul :class="bemm('controls')">
        <li v-for="control in CONTROLS" :key="control.label" :class="bemm('control')">
          <KeyCap :label="control.label" />
          <span :class="bemm('control-action')">{{ control.action }}</span>
        </li>
      </ul>
    </section>

    <section :class="bemm('section')" aria-labelledby="behaviours-title">
      <div :class="bemm('section-head')">
        <p :class="bemm('eyebrow')">Key behaviours</p>
        <h2 id="behaviours-title" :class="bemm('title')">
          What it does, and what it leaves alone.
        </h2>
      </div>

      <ul :class="bemm('behaviours')">
        <li v-for="behaviour in BEHAVIOURS" :key="behaviour.title" :class="bemm('behaviour')">
          <p :class="bemm('behaviour-title')">{{ behaviour.title }}</p>
          <p :class="bemm('behaviour-detail')">{{ behaviour.detail }}</p>
        </li>
      </ul>
    </section>

    <section id="install" :class="bemm('section')" aria-labelledby="install-title">
      <div :class="bemm('section-head')">
        <p :class="bemm('eyebrow')">Installation</p>
        <h2 id="install-title" :class="bemm('title')">Install in a minute.</h2>
        <p :class="bemm('paragraph')">Install the binary, then enable the Zsh integration.</p>
      </div>

      <div :class="bemm('install')">
        <div :class="bemm('install-step')">
          <p :class="bemm('install-label')">
            {{ HOMEBREW_AVAILABLE ? "1. Install the binary" : "1. Build the binary" }}
          </p>
          <template v-if="HOMEBREW_AVAILABLE">
            <code :class="bemm('command')">{{ HOMEBREW_COMMAND }}</code>
          </template>
          <template v-else>
            <p :class="bemm('install-note')">
              The Homebrew tap goes live with the first tagged release. Until then, build it from a
              clone of the repository.
            </p>
            <code :class="bemm('command')">{{ BUILD_COMMAND }}</code>
          </template>
        </div>

        <div :class="bemm('install-step')">
          <p :class="bemm('install-label')">2. Enable the shell integration</p>
          <code :class="bemm('command')">{{ SHELL_INIT_COMMAND }}</code>
          <p :class="bemm('install-note')">
            Add that line to <code :class="bemm('inline-code')">~/.zshrc</code> to keep it in future
            sessions. Zsh 5.8 or newer, which is what macOS ships.
          </p>
        </div>
      </div>
    </section>

    <section :class="bemm('section')" aria-labelledby="shells-title">
      <div :class="bemm('section-head')">
        <p :class="bemm('eyebrow')">Shell support</p>
        <h2 id="shells-title" :class="bemm('title')">Zsh on macOS first.</h2>
      </div>

      <ul :class="bemm('shells')">
        <li v-for="shell in SHELL_SUPPORT" :key="shell.name" :class="bemm('shell')">
          <span :class="bemm('shell-name')">{{ shell.name }}</span>
          <span :class="bemm('shell-state', shell.state)">{{ shell.detail }}</span>
        </li>
      </ul>
    </section>

    <section :class="bemm('section')" aria-labelledby="open-title">
      <div :class="bemm('section-head')">
        <p :class="bemm('eyebrow')">Open source</p>
        <h2 id="open-title" :class="bemm('title')">Small, focused, and open.</h2>
      </div>

      <div :class="bemm('prose')">
        <p :class="bemm('paragraph')">
          cdd is a local-first shell utility. No backend. No account. No telemetry.
        </p>
        <p :class="bemm('paragraph')">
          Read the spec, inspect the code, or follow the build in public on GitHub.
        </p>
      </div>

      <div :class="bemm('actions')">
        <ActionLink variant="primary" :href="LINKS.repo">View on GitHub</ActionLink>
        <ActionLink variant="ghost" :href="LINKS.spec">Read the spec</ActionLink>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useBemm } from "bemm";

import ActionLink from "@/components/ActionLink";
import HeroTicker from "@/components/HeroTicker";
import KeyCap from "@/components/KeyCap";
import TerminalMock from "@/components/TerminalMock";
import {
  BUILD_COMMAND,
  HOMEBREW_AVAILABLE,
  HOMEBREW_COMMAND,
  LINKS,
  SHELL_INIT_COMMAND,
} from "@/config/site";
import { usePageMeta } from "@/composables/usePageMeta";

import { BEHAVIOURS, CONTROLS, SHELL_SUPPORT, STEPS, TICKER_WORDS } from "./HomeView.content";

const bemm = useBemm("home-view", { includeBaseClass: true });

usePageMeta({
  title: "cdd — Change Directory, live",
  description:
    "cdd is an interactive alternative to cd. It shows matching directories below your prompt while you type, so you can filter, arrow down and move in one flow.",
  path: "/",
});
</script>

<style lang="scss">
.home-view {
  display: flex;
  flex-direction: column;
  gap: var(--spacing);
  width: 100%;
  max-width: var(--max-content-width);
  margin: 0 auto;
  padding: var(--spacing) var(--space-l);

  &__hero {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    align-items: center;
    gap: var(--spacing);
  }

  &__hero-copy {
    display: flex;
    flex-direction: column;
    gap: var(--space-l);
  }

  &__wordmark {
    font-size: var(--font-size-xxxxl);
    font-weight: var(--font-weight-bold);
    letter-spacing: -0.03em;
    line-height: 0.9;
  }

  &__lede {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  &__lede-line {
    color: color-mix(in srgb, var(--color-foreground), transparent 25%);
    font-size: var(--font-size-l);
    line-height: 1.5;
  }

  &__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-s);
  }

  &__meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space);
    list-style: none;
  }

  &__meta-item {
    color: color-mix(in srgb, var(--color-foreground), transparent 55%);
    font-size: var(--font-size-s);

    &--status {
      color: var(--color-primary);
    }
  }

  &__section {
    display: flex;
    flex-direction: column;
    gap: var(--space-xl);
    scroll-margin-top: var(--space-xl);
  }

  &__section-head {
    display: flex;
    flex-direction: column;
    gap: var(--space-s);
    max-width: 40ch;
  }

  &__eyebrow {
    color: color-mix(in srgb, var(--color-foreground), transparent 55%);
    font-size: var(--font-size-xs);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  &__title {
    font-size: var(--font-size-xxxl);
    font-weight: var(--font-weight-semibold);
    letter-spacing: -0.02em;
    line-height: 1.15;
  }

  &__prose {
    display: flex;
    flex-direction: column;
    gap: var(--space);
    max-width: 60ch;
  }

  &__paragraph {
    color: color-mix(in srgb, var(--color-foreground), transparent 25%);
    font-size: var(--font-size-l);
    line-height: 1.7;
  }

  &__how {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    align-items: center;
    gap: var(--spacing);
  }

  &__steps {
    display: flex;
    flex-direction: column;
    gap: var(--space-l);
    list-style: none;
  }

  &__step {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: var(--space-xs) var(--space);
    align-items: baseline;
  }

  &__step-number {
    color: color-mix(in srgb, var(--color-foreground), transparent 65%);
    font-family: var(--font-family-mono);
    font-size: var(--font-size-s);
  }

  &__step-title {
    font-size: var(--font-size-l);
  }

  &__step-example {
    grid-column: 2;
    color: var(--color-primary);
    font-family: var(--font-family-mono);
    font-size: var(--font-size-s);
  }

  &__controls {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-l);
    list-style: none;
  }

  &__control {
    display: flex;
    align-items: center;
    gap: var(--space-s);
  }

  &__control-action {
    color: color-mix(in srgb, var(--color-foreground), transparent 40%);
    font-size: var(--font-size-s);
  }

  &__behaviours {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(15em, 1fr));
    gap: var(--space-xl) var(--spacing);
    list-style: none;
  }

  &__behaviour {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  &__behaviour-title {
    font-weight: var(--font-weight-semibold);
  }

  &__behaviour-detail {
    color: color-mix(in srgb, var(--color-foreground), transparent 40%);
    line-height: 1.6;
  }

  &__install {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(18em, 1fr));
    gap: var(--spacing);
  }

  &__install-step {
    display: flex;
    flex-direction: column;
    gap: var(--space-s);
  }

  &__install-label {
    color: color-mix(in srgb, var(--color-foreground), transparent 55%);
    font-size: var(--font-size-xs);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  &__install-note {
    color: color-mix(in srgb, var(--color-foreground), transparent 40%);
    font-size: var(--font-size-s);
    line-height: 1.6;
  }

  &__command {
    padding: var(--space-s) var(--space);
    overflow-x: auto;
    border-radius: var(--border-radius-s);
    background-color: color-mix(in srgb, var(--color-foreground), transparent 94%);
    font-family: var(--font-family-mono);
    font-size: var(--font-size-s);
  }

  &__inline-code {
    font-family: var(--font-family-mono);
    font-size: 0.9em;
  }

  &__shells {
    display: flex;
    flex-direction: column;
    gap: var(--space);
    max-width: 40em;
    list-style: none;
  }

  &__shell {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-s) var(--space-l);
    align-items: baseline;
    padding-bottom: var(--space);
    border-bottom: var(--border-width) solid
      color-mix(in srgb, var(--color-foreground), transparent 92%);
  }

  &__shell-name {
    min-width: 5em;
    font-weight: var(--font-weight-semibold);
  }

  &__shell-state {
    color: color-mix(in srgb, var(--color-foreground), transparent 45%);

    &--current {
      color: var(--color-primary);
    }
  }
}

@media (max-width: 900px) {
  .home-view {
    padding: var(--space-xl) var(--space);

    &__hero,
    &__how {
      grid-template-columns: minmax(0, 1fr);
    }
  }
}
</style>
