import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import KeyCap from "./KeyCap.vue";

describe("KeyCap", () => {
  it("renders the key as a kbd element", () => {
    const wrapper = mount(KeyCap, { props: { label: "Enter" } });

    expect(wrapper.element.tagName).toBe("KBD");
    expect(wrapper.text()).toBe("Enter");
  });
});
