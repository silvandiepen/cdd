export interface TerminalMockProps {
  /** The directory shown in the prompt, e.g. `~/Repositories`. */
  prompt: string;
  /** What has been typed so far, e.g. `cdd im`. */
  command: string;
  /** The directories cdd is offering, in the order it ranks them. */
  results: string[];
  /** Which result is selected. */
  selected?: number;
  /** Matches beyond the visible rows, shown as `+ N more`. */
  more?: number;
  /** Show a blinking caret after the command. */
  caret?: boolean;
  /** Accessible description of what the terminal is showing. */
  label?: string;
}
