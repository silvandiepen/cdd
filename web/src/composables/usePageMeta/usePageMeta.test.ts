import { beforeEach, describe, expect, it } from "vitest";

import { SITE_URL } from "@/config/site";

import { usePageMeta } from "./usePageMeta";

const content = (selector: string): string | null =>
  document.head.querySelector(selector)?.getAttribute("content") ?? null;

describe("usePageMeta", () => {
  beforeEach(() => {
    document.head.innerHTML = "";
  });

  it("sets the title, description and canonical url for a page", () => {
    usePageMeta({ title: "Support", description: "How to get help.", path: "/support" });

    expect(document.title).toBe("Support — cdd");
    expect(content('meta[name="description"]')).toBe("How to get help.");
    expect(document.head.querySelector('link[rel="canonical"]')?.getAttribute("href")).toBe(
      `${SITE_URL}/support`,
    );
  });

  it("leaves the home page title unsuffixed and its url without a trailing slash", () => {
    usePageMeta({ title: "cdd — Change Directory, live", description: "A shell tool.", path: "/" });

    expect(document.title).toBe("cdd — Change Directory, live");
    expect(content('meta[property="og:url"]')).toBe(SITE_URL);
  });

  it("sets the open graph pair", () => {
    usePageMeta({ title: "Privacy", description: "No telemetry.", path: "/privacy" });

    expect(content('meta[property="og:title"]')).toBe("Privacy — cdd");
    expect(content('meta[property="og:description"]')).toBe("No telemetry.");
    expect(content('meta[property="og:site_name"]')).toBe("cdd");
    expect(content('meta[property="og:type"]')).toBe("website");
  });

  it("updates the existing tags instead of appending duplicates", () => {
    usePageMeta({ title: "Support", description: "First.", path: "/support" });
    usePageMeta({ title: "Terms", description: "Second.", path: "/terms" });

    expect(document.head.querySelectorAll('meta[name="description"]')).toHaveLength(1);
    expect(document.head.querySelectorAll('link[rel="canonical"]')).toHaveLength(1);
    expect(content('meta[name="description"]')).toBe("Second.");
  });
});
