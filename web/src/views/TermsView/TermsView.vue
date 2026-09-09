<template>
  <ProseLayout
    title="Terms"
    intro="cdd is free, open-source software under the MIT licence. These are the plain-language terms for using it and this site."
    :updated="UPDATED"
  >
    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">The licence</h2>
      <p class="prose-layout__text">
        cdd is released under the MIT licence. You can use it, copy it, change it and redistribute
        it, including commercially, as long as the copyright notice and the licence text travel with
        it. The
        <a class="prose-layout__link" :href="LINKS.license">licence</a> is the authoritative
        document; this page only summarises it.
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">No warranty</h2>
      <p class="prose-layout__text">
        cdd is provided as is, without warranty of any kind. It changes your shell's working
        directory and reads your filesystem to do so; you run it at your own risk. Nobody is liable
        for any claim or damages arising from its use. This is the MIT licence's own disclaimer, and
        it means what it says.
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">Pre-release status</h2>
      <p class="prose-layout__text">
        cdd has not had a tagged release yet. Behaviour, commands and the shell integration can
        still change without notice, and there is no upgrade path promised between builds.
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">This website</h2>
      <p class="prose-layout__text">
        The content here describes cdd as it currently is. It carries no guarantee of accuracy or
        availability, and it can change at any time. Where this site and the
        <a class="prose-layout__link" :href="LINKS.spec">specification</a> disagree, the
        specification is correct.
      </p>
    </section>

    <section class="prose-layout__section">
      <h2 class="prose-layout__heading">Contact</h2>
      <p class="prose-layout__text">
        Questions about these terms:
        <a class="prose-layout__link" :href="`mailto:${SUPPORT_EMAIL}`">{{ SUPPORT_EMAIL }}</a
        >.
      </p>
    </section>
  </ProseLayout>
</template>

<script setup lang="ts">
import ProseLayout from "@/components/ProseLayout";
import { LINKS, SUPPORT_EMAIL } from "@/config/site";
import { usePageMeta } from "@/composables/usePageMeta";

const UPDATED = "9 September 2026";

usePageMeta({
  title: "Terms",
  description:
    "cdd is MIT-licensed, provided without warranty, and still pre-release. The plain-language terms for using it and this site.",
  path: "/terms",
});
</script>
