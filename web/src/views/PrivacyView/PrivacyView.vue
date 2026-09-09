<template>
  <ProseLayout
    title="Privacy"
    intro="cdd collects nothing. This page says what that means for the tool and for this website."
    :updated="UPDATED"
  >
    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">The tool</h2>
      <p class="prose-layout__text">
        cdd runs entirely on your machine. It reads the directory you are browsing in order to list
        it, and changes your shell's working directory when you pick one. That is all it does.
      </p>
      <ul class="prose-layout__list">
        <li>It makes no network requests.</li>
        <li>It has no analytics, telemetry, crash reporting or usage counters.</li>
        <li>It has no account, and there is no server to hold data on.</li>
        <li>It writes no history, cache or database of the directories you visit.</li>
      </ul>
      <p class="prose-layout__text">
        Your shell records the commands you run, as it does for any command. cdd does not add to
        that beyond the line you typed.
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">This website</h2>
      <p class="prose-layout__text">
        This site is static. It sets no cookies, runs no analytics, and embeds nothing from a third
        party. Fonts are loaded from your own system rather than from a font service.
      </p>
      <p class="prose-layout__text">
        Whoever hosts the site keeps server logs, which is normal for any web server and typically
        includes your IP address, the page requested and your browser's user agent. That is outside
        this site's control and is not used to build any profile of you.
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">Links away from here</h2>
      <p class="prose-layout__text">
        Following a link to GitHub puts you on GitHub, under their privacy policy rather than this
        one.
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">Questions</h2>
      <p class="prose-layout__text">
        Email <a class="prose-layout__link" :href="`mailto:${SUPPORT_EMAIL}`">{{ SUPPORT_EMAIL }}</a
        >, or read the source — this is the kind of claim you should be able to verify yourself.
      </p>
    </section>
  </ProseLayout>
</template>

<script setup lang="ts">
import ProseLayout from "@/components/ProseLayout";
import { SUPPORT_EMAIL } from "@/config/site";
import { usePageMeta } from "@/composables/usePageMeta";

const UPDATED = "9 September 2026";

usePageMeta({
  title: "Privacy",
  description:
    "cdd makes no network requests and has no telemetry, analytics or account. What that means for the tool and for this website.",
  path: "/privacy",
});
</script>
