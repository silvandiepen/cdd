import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import ActionLink from "./ActionLink.vue";

describe("ActionLink", () => {
  it("renders as a link to the given href", () => {
    const wrapper = mount(ActionLink, {
      props: { href: "#install" },
      slots: { default: "How to install" },
    });

    expect(wrapper.element.tagName).toBe("A");
    expect(wrapper.attributes("href")).toBe("#install");
    expect(wrapper.text()).toBe("How to install");
  });

  it("is primary unless told otherwise", () => {
    const wrapper = mount(ActionLink, { props: { href: "/" } });

    expect(wrapper.classes()).toContain("action-link--primary");
  });

  it("carries the variant as a modifier", () => {
    const wrapper = mount(ActionLink, { props: { href: "/", variant: "ghost" } });

    expect(wrapper.classes()).toContain("action-link--ghost");
    expect(wrapper.classes()).not.toContain("action-link--primary");
  });

  it("shows the arrow only when asked, and hides it from screen readers", () => {
    expect(
      mount(ActionLink, { props: { href: "/" } })
        .find(".action-link__arrow")
        .exists(),
    ).toBe(false);

    const arrow = mount(ActionLink, { props: { href: "/", arrow: true } }).get(
      ".action-link__arrow",
    );
    expect(arrow.attributes("aria-hidden")).toBe("true");
  });
});
