import { computed, onScopeDispose, ref, toValue, watchEffect } from "vue";

import type { Ticker, TickerOptions } from "./useTicker.model";

const DEFAULT_INTERVAL_MS = 2200;

/**
 * Steps through a list of words, one at a time, wrapping at the end.
 *
 * Used for the hero's rotating final word. Motion is decorative here, so a
 * paused ticker still renders a full list with the first item selected rather
 * than collapsing to nothing.
 *
 * @param items - The words to cycle through. Must not be empty.
 * @param options - Interval, and whether the ticker is held still.
 * @returns The selected index, the selected item, and a manual selector.
 */
export const useTicker = (items: string[], options: TickerOptions = {}): Ticker => {
  if (items.length === 0) {
    throw new Error("useTicker needs at least one item");
  }

  const { intervalMs = DEFAULT_INTERVAL_MS, paused = false } = options;

  const index = ref(0);
  const current = computed(() => items[index.value] as string);

  const select = (next: number): void => {
    index.value = ((next % items.length) + items.length) % items.length;
  };

  let timer: ReturnType<typeof setInterval> | undefined;

  const stop = (): void => {
    if (timer !== undefined) {
      clearInterval(timer);
      timer = undefined;
    }
  };

  watchEffect(() => {
    stop();
    if (toValue(paused) || items.length < 2) return;
    timer = setInterval(() => select(index.value + 1), intervalMs);
  });

  onScopeDispose(stop);

  return { index, current, select };
};
