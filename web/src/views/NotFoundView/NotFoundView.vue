<template>
  <ProseLayout
    title="Not found"
    intro="That page does not exist. It may have moved or never have been here."
  >
    <section class="prose-layout__section">
      <p class="prose-layout__text">
        <RouterLink class="prose-layout__link" to="/">Back to the start</RouterLink>
      </p>
    </section>
  </ProseLayout>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";

import ProseLayout from "@/components/ProseLayout";
import { usePageMeta } from "@/composables/usePageMeta";

usePageMeta({
  title: "Not found",
  description: "That page does not exist.",
  path: "/404",
});
</script>
