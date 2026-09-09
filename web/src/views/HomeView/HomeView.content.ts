/** The words the final `d` of `cdd` rotates through in the hero. */
export const TICKER_WORDS = [
  "Directly",
  "Dynamically",
  "Discovery",
  "Display",
  "Down",
  "Deluxe",
  "Differently",
  "Discover",
  "Dynamic",
];

export interface Step {
  title: string;
  example: string;
}

export const STEPS: Step[] = [
  { title: "Type a path or partial name.", example: "cdd ti" },
  { title: "Watch the results update live.", example: "cdd src/co" },
  { title: "Arrow down to the right folder.", example: "cdd ~/Projects" },
  { title: "Press Enter and go there.", example: "cdd .." },
];

export interface Behaviour {
  title: string;
  detail: string;
}

export const BEHAVIOURS: Behaviour[] = [
  {
    title: "Live filtering while typing",
    detail: "Every character narrows the list, before you press Enter.",
  },
  {
    title: "Inline under the prompt",
    detail: "No alternate screen, no full-screen window to escape from.",
  },
  {
    title: "Directories only",
    detail: "Files are never in the way, because you are changing directory.",
  },
  {
    title: "Tab to keep drilling down",
    detail: "Completes the selected directory and lists its children.",
  },
  {
    title: "Normal paths still work",
    detail: "cdd .., cdd ~/Projects and cdd src/components behave like cd.",
  },
  {
    title: "Clean shell integration",
    detail: "Cancel and the prompt is exactly as you left it.",
  },
  {
    title: "Local only",
    detail: "No network, no account, no telemetry.",
  },
  {
    title: "No dependency on fzf",
    detail: "One small binary and a shell script.",
  },
];

export interface ControlKey {
  label: string;
  action: string;
}

export const CONTROLS: ControlKey[] = [
  { label: "↓", action: "Next match" },
  { label: "↑", action: "Previous match" },
  { label: "Enter", action: "Change into it" },
  { label: "Tab", action: "Complete and keep going" },
  { label: "Backspace", action: "Broaden again" },
];

export type ShellState = "current" | "planned";

export interface ShellSupport {
  name: string;
  state: ShellState;
  detail: string;
}

export const SHELL_SUPPORT: ShellSupport[] = [
  { name: "Zsh", state: "current", detail: "First-class target. Live inline integration." },
  { name: "Bash", state: "planned", detail: "Planned." },
  { name: "Fish", state: "planned", detail: "Planned." },
  { name: "Linux", state: "planned", detail: "Planned after the macOS launch." },
];
