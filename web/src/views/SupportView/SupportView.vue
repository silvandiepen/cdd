<template>
  <ProseLayout
    title="Support"
    intro="cdd is a small open-source utility maintained by one person. Here is how to get help with it."
    :updated="UPDATED"
  >
    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">Something is broken</h2>
      <p class="prose-layout__text">
        Open an issue on GitHub. Include your macOS version, the output of
        <code>zsh --version</code> and <code>cdd --version</code>, and what you typed when it went
        wrong. Bug reports about the shell integration are much easier to act on with the exact
        sequence of keys.
      </p>
      <p class="prose-layout__text">
        <a class="prose-layout__link" :href="LINKS.issues">Open an issue</a>
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">A question, or an idea</h2>
      <p class="prose-layout__text">
        Issues are the best place for those too — they stay visible to everyone else running into
        the same thing. If you would rather not open one, email
        <a class="prose-layout__link" :href="`mailto:${SUPPORT_EMAIL}`">{{ SUPPORT_EMAIL }}</a
        >.
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">What to expect</h2>
      <p class="prose-layout__text">
        cdd is not a commercial product and has no support contract or response-time commitment. It
        is worked on when there is time for it. The spec and the source are public, so you can
        always read exactly what it does.
      </p>
      <ul class="prose-layout__list">
        <li><a class="prose-layout__link" :href="LINKS.spec">The specification</a></li>
        <li><a class="prose-layout__link" :href="LINKS.repo">The source code</a></li>
        <li><a class="prose-layout__link" :href="LINKS.changelog">Releases</a></li>
      </ul>
    </section>
  </ProseLayout>
</template>

<script setup lang="ts">
import ProseLayout from "@/components/ProseLayout";
import { LINKS, SUPPORT_EMAIL } from "@/config/site";
import { usePageMeta } from "@/composables/usePageMeta";

const UPDATED = "9 September 2026";

usePageMeta({
  title: "Support",
  description:
    "How to report a bug, ask a question or get help with cdd, and what to expect from a small open-source project.",
  path: "/support",
});
</script>
