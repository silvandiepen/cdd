import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import TerminalMock from "./TerminalMock.vue";

const props = {
  prompt: "~/Repositories",
  command: "cdd im",
  results: ["imagekid", "image-tools", "simple-image"],
};

describe("TerminalMock", () => {
  it("shows the prompt, the command and every result", () => {
    const wrapper = mount(TerminalMock, { props });

    expect(wrapper.get(".terminal-mock__path").text()).toBe("~/Repositories");
    expect(wrapper.get(".terminal-mock__command").text()).toBe("cdd im");
    expect(wrapper.findAll(".terminal-mock__result")).toHaveLength(3);
  });

  it("marks the selected result and only that one", () => {
    const wrapper = mount(TerminalMock, { props: { ...props, selected: 1 } });
    const selected = wrapper.findAll(".terminal-mock__result--selected");

    expect(selected).toHaveLength(1);
    expect(selected[0]?.text()).toContain("image-tools");
    expect(selected[0]?.get(".terminal-mock__marker").text()).toBe("▸");
  });

  it("counts matches that did not fit", () => {
    const wrapper = mount(TerminalMock, { props: { ...props, more: 12 } });

    expect(wrapper.get(".terminal-mock__result--more").text()).toContain("+ 12 more");
  });

  it("omits the overflow row when everything fits", () => {
    const wrapper = mount(TerminalMock, { props });

    expect(wrapper.find(".terminal-mock__result--more").exists()).toBe(false);
  });

  it("describes itself to assistive technology as a single image", () => {
    const wrapper = mount(TerminalMock, { props });

    expect(wrapper.attributes("role")).toBe("img");
    expect(wrapper.attributes("aria-label")).toContain("cdd im");
    expect(wrapper.attributes("aria-label")).toContain("imagekid");
  });

  it("prefers an explicit label when one is given", () => {
    const wrapper = mount(TerminalMock, { props: { ...props, label: "Filtering by im." } });

    expect(wrapper.attributes("aria-label")).toBe("Filtering by im.");
  });
});
