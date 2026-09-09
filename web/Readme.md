# cdd website

The marketing site for `cdd`, built from [`../docs/WEBSITE.md`](../docs/WEBSITE.md) and
[`../docs/website-copy.md`](../docs/website-copy.md). Those two documents are canonical for
structure and copy; this one covers how it is built and where it deliberately differs.

## Running it

```sh
npm install
npm run dev      # local development
npm run build    # typecheck, bundle, emit sitemap.xml
npm run preview  # serve the build
npm test         # vitest
npm run lint     # eslint + prettier
```

## Structure

```
src/
  components/   folder-per-component: .vue, .model.ts, .test.ts, index.ts, Readme.md
  composables/  same scaffold
  config/       site.ts — every outbound link and the production origin
  router/       routes.ts is the single route table; the sitemap is generated from it
  styles/       @sil/ui token configuration and the page base
  views/        one folder per route
```

## Decisions worth knowing

**The production origin is assumed.** `SITE_URL` in `src/config/site.ts` is set to
`https://cdd.sil.mt`. Nothing in the docs names a domain. It is referenced from the sitemap,
`public/robots.txt` and every canonical URL, so change it in those places together if it is wrong.

**Homebrew is not advertised as working yet.** `docs/website-copy.md` gives
`Install with Homebrew` as the primary call to action, but there is no tagged release and the tap
does not exist, so that command would fail for anyone who ran it. `HOMEBREW_AVAILABLE` in
`src/config/site.ts` gates this: while it is `false` the hero says `How to install`, the metadata
row carries a `Pre-release` marker, and the install section gives `cargo build --release` with a
line explaining that the tap arrives with the first release. Set it to `true` when the tap is
published and the page reverts to the copy as written. A test asserts the page does not print a
`brew install` command while the flag is off.

**`@sil/ui` supplies the tokens, not the components.** The site uses `@sil/ui/defaults` and
`@sil/ui/styles/main` for the whole token and base layer, which is what
`~/Projects/Agents/css-conventions.md` requires. It does not import `Button` and `Kbd` from the
package: those come only through the package barrel, which pulls in its TipTap rich-text editor
and the entire `open-icon` catalogue. Measured, that was **762 kB of JavaScript across 1133
chunks** for two buttons and five keycaps; without them the site is **100 kB across 8 chunks**.
Deep imports are blocked by the package's `exports` map. So `ActionLink` and `KeyCap` are local,
and small enough to stay that way. If `@sil/ui` gains a tree-shakeable entry point, they should be
deleted in favour of the real primitives.

**The `@` alias is this project's `src`.** That only works because the `@sil/ui` Vite plugin is not
installed — it prepends aliases for `@/composables`, `@/types`, `@/utils` and friends, which
collide with this project's own folders. The plugin is only needed to resolve the package's
source-shipped components, which the point above means are not used.

## SEO

- `sitemap.xml` is emitted at build time from `src/router/routes.ts`, so a new page cannot ship
  uncrawlable. `src/router/routes.test.ts` fails if the route table and the sitemap list drift.
- `public/robots.txt` points at it.
- Every view calls `usePageMeta` once, setting its own title, description, canonical URL and
  Open Graph tags.

## Not done yet

`~/Projects/Agents/content-conventions.md` also asks a public site for a blog with at least thirty
articles at launch. That is not here — it is its own task, and several of the comparison articles
would need to state plainly that `cdd` is pre-release. Tracked on the board.
