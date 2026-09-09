import { effectScope, nextTick, ref } from "vue";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { useTicker } from "./useTicker";

const WORDS = ["Directly", "Dynamically", "Discovery"];

describe("useTicker", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("starts on the first item", () => {
    const { index, current } = useTicker(WORDS);

    expect(index.value).toBe(0);
    expect(current.value).toBe("Directly");
  });

  it("advances on the interval and wraps at the end", () => {
    const { current } = useTicker(WORDS, { intervalMs: 1000 });

    vi.advanceTimersByTime(1000);
    expect(current.value).toBe("Dynamically");

    vi.advanceTimersByTime(2000);
    expect(current.value).toBe("Directly");
  });

  it("holds still while paused", async () => {
    const paused = ref(true);
    const { current } = useTicker(WORDS, { intervalMs: 1000, paused });

    vi.advanceTimersByTime(5000);
    expect(current.value).toBe("Directly");

    paused.value = false;
    await nextTick();

    vi.advanceTimersByTime(1000);
    expect(current.value).toBe("Dynamically");
  });

  it("can be pointed at an item directly, wrapping out-of-range values", () => {
    const { select, current } = useTicker(WORDS);

    select(2);
    expect(current.value).toBe("Discovery");

    select(-1);
    expect(current.value).toBe("Discovery");

    select(4);
    expect(current.value).toBe("Dynamically");
  });

  it("stops ticking once its scope is disposed", () => {
    const scope = effectScope();
    const ticker = scope.run(() => useTicker(WORDS, { intervalMs: 1000 }));
    scope.stop();

    vi.advanceTimersByTime(5000);
    expect(ticker?.current.value).toBe("Directly");
  });

  it("refuses an empty list rather than rendering nothing", () => {
    expect(() => useTicker([])).toThrow(/at least one item/);
  });
});
