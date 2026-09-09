import { mount } from "@vue/test-utils";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import HeroTicker from "./HeroTicker.vue";

const WORDS = ["Directly", "Dynamically", "Discovery"];

const currentWord = (wrapper: ReturnType<typeof mount>): string =>
  wrapper.get(".hero-ticker__item--current").text();

describe("HeroTicker", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.stubGlobal(
      "matchMedia",
      vi.fn().mockReturnValue({
        matches: false,
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
      }),
    );
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
  });

  it("shows the stable phrase and every word", () => {
    const wrapper = mount(HeroTicker, { props: { words: WORDS } });

    expect(wrapper.get(".hero-ticker__static").text()).toBe("Change Directory");
    expect(wrapper.findAll(".hero-ticker__item")).toHaveLength(WORDS.length);
  });

  it("marks exactly one word as current, starting with the first", () => {
    const wrapper = mount(HeroTicker, { props: { words: WORDS } });

    expect(wrapper.findAll(".hero-ticker__item--current")).toHaveLength(1);
    expect(currentWord(wrapper)).toBe("Directly");
  });

  it("advances on the interval", async () => {
    const wrapper = mount(HeroTicker, { props: { words: WORDS, intervalMs: 500 } });

    vi.advanceTimersByTime(500);
    await wrapper.vm.$nextTick();

    expect(currentWord(wrapper)).toBe("Dynamically");
  });

  it("selects a word on hover", async () => {
    const wrapper = mount(HeroTicker, { props: { words: WORDS } });

    await wrapper.findAll(".hero-ticker__item")[2]?.trigger("mouseenter");

    expect(currentWord(wrapper)).toBe("Discovery");
  });
});
