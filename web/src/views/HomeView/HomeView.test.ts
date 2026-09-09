import { mount } from "@vue/test-utils";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { BUILD_COMMAND, HOMEBREW_AVAILABLE, SHELL_INIT_COMMAND } from "@/config/site";

import HomeView from "./HomeView.vue";
import { BEHAVIOURS, CONTROLS, SHELL_SUPPORT, STEPS } from "./HomeView.content";

const render = () => mount(HomeView);

describe("HomeView", () => {
  beforeEach(() => {
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
    vi.unstubAllGlobals();
  });

  it("renders every section from the website spec", () => {
    const wrapper = render();
    const headings = wrapper.findAll("h1, h2").map((heading) => heading.text());

    expect(headings).toContain("cdd");
    expect(headings).toContain("Changing directories should feel lighter.");
    expect(headings).toContain("One small command, one smoother flow.");
    expect(headings).toContain("Install in a minute.");
    expect(headings).toContain("Small, focused, and open.");
  });

  it("shows the product before it explains it", () => {
    const wrapper = render();
    const terminal = wrapper.get(".home-view__hero .terminal-mock");

    expect(terminal.get(".terminal-mock__command").text()).toBe("cdd im");
    expect(terminal.findAll(".terminal-mock__result--selected")).toHaveLength(1);
  });

  it("lists the steps, behaviours, controls and shells it is given", () => {
    const wrapper = render();

    expect(wrapper.findAll(".home-view__step")).toHaveLength(STEPS.length);
    expect(wrapper.findAll(".home-view__behaviour")).toHaveLength(BEHAVIOURS.length);
    expect(wrapper.findAll(".home-view__control")).toHaveLength(CONTROLS.length);
    expect(wrapper.findAll(".home-view__shell")).toHaveLength(SHELL_SUPPORT.length);
  });

  it("sets the page metadata", () => {
    render();

    expect(document.title).toBe("cdd — Change Directory, live");
    expect(
      document.head.querySelector('meta[name="description"]')?.getAttribute("content"),
    ).toContain("interactive alternative to cd");
  });

  it("gives the install command that actually works today", () => {
    const wrapper = render();
    const commands = wrapper.findAll(".home-view__command").map((node) => node.text());

    expect(commands).toContain(SHELL_INIT_COMMAND);

    // While there is no tagged release the site must not print a brew command
    // that would fail, and must say why.
    if (!HOMEBREW_AVAILABLE) {
      expect(commands).toContain(BUILD_COMMAND);
      expect(commands.join(" ")).not.toContain("brew install");
      expect(wrapper.text()).toContain("Homebrew tap goes live with the first tagged release");
      expect(wrapper.get(".home-view__meta-item--status").text()).toBe("Pre-release");
    }
  });

  it("does not oversell the shells it does not support", () => {
    const wrapper = render();
    const current = wrapper.findAll(".home-view__shell-state--current");

    expect(current).toHaveLength(1);
    expect(wrapper.get(".home-view__shell").text()).toContain("Zsh");
  });
});
