import type { Ref } from "vue";

export interface TickerOptions {
  /** Milliseconds each item stays selected. */
  intervalMs?: number;
  /**
   * When true the ticker never advances. The hero passes the user's reduced
   * motion preference here, so the list holds still for anyone who asked for
   * that.
   */
  paused?: Ref<boolean> | boolean;
}

export interface Ticker {
  /** Index of the selected item. */
  index: Readonly<Ref<number>>;
  /** The selected item itself. */
  current: Readonly<Ref<string>>;
  /** Select an item directly, e.g. on hover or focus. */
  select: (index: number) => void;
}
