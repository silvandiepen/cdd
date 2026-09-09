import { effectScope } from "vue";
import { afterEach, describe, expect, it, vi } from "vitest";

import { usePrefersReducedMotion } from "./usePrefersReducedMotion";

interface FakeQuery {
  matches: boolean;
  addEventListener: ReturnType<typeof vi.fn>;
  removeEventListener: ReturnType<typeof vi.fn>;
}

const stubMatchMedia = (matches: boolean): FakeQuery => {
  const query: FakeQuery = {
    matches,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
  };
  vi.stubGlobal("matchMedia", vi.fn().mockReturnValue(query));
  return query;
};

describe("usePrefersReducedMotion", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("reports the current preference", () => {
    stubMatchMedia(true);
    expect(usePrefersReducedMotion().value).toBe(true);
  });

  it("follows the preference when it changes", () => {
    const query = stubMatchMedia(false);
    const prefersReduced = usePrefersReducedMotion();

    const [, listener] = query.addEventListener.mock.calls[0] as [string, (e: unknown) => void];
    listener({ matches: true });

    expect(prefersReduced.value).toBe(true);
  });

  it("stops listening once its scope is disposed", () => {
    const query = stubMatchMedia(false);
    const scope = effectScope();
    scope.run(() => usePrefersReducedMotion());
    scope.stop();

    expect(query.removeEventListener).toHaveBeenCalledOnce();
  });

  it("assumes motion is fine where matchMedia is unavailable", () => {
    vi.stubGlobal("matchMedia", undefined);
    expect(usePrefersReducedMotion().value).toBe(false);
  });
});
