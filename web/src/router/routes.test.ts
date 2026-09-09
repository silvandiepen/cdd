import { describe, expect, it } from "vitest";

import { crawlableRoutes, routes } from "./routes";

describe("routes", () => {
  it("lists every real page in the sitemap", () => {
    const real = routes
      .map((route) => route.path)
      .filter((path) => !path.includes(":"))
      .sort();
    const crawlable = crawlableRoutes.map((route) => route.path).sort();

    expect(crawlable).toEqual(real);
  });

  it("keeps the catch-all route last so it cannot shadow a real page", () => {
    const catchAll = routes.findIndex((route) => route.path.includes(":pathMatch"));

    expect(catchAll).toBe(routes.length - 1);
  });

  it("ships the pages every public site needs", () => {
    const paths = routes.map((route) => route.path);

    expect(paths).toContain("/support");
    expect(paths).toContain("/privacy");
    expect(paths).toContain("/terms");
  });
});
