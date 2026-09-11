/**
 * Single source of truth for anything that points outside the site.
 *
 * `SITE_URL` is also what the sitemap is generated against, so it has to be the
 * real production origin, without a trailing slash.
 */
export const SITE_URL = "https://cdd.sil.mt";

export const SITE_NAME = "cdd";

export const SITE_TAGLINE = "A more intentional terminal.";

export const REPO_URL = "https://github.com/silvandiepen/cdd";

export const LINKS = {
  repo: REPO_URL,
  spec: `${REPO_URL}/blob/main/docs/SPEC.md`,
  docs: `${REPO_URL}#readme`,
  changelog: `${REPO_URL}/releases`,
  issues: `${REPO_URL}/issues`,
  license: `${REPO_URL}/blob/main/LICENSE`,
} as const;

export const SUPPORT_EMAIL = "me@sil.mt";

/**
 * True since v0.1.0: the release is tagged and `silvandiepen/homebrew-tap`
 * carries the formula, so the command below actually works. The hero CTA and
 * the install section both read from this.
 */
export const HOMEBREW_AVAILABLE = true;

export const HOMEBREW_COMMAND = "brew install silvandiepen/tap/cdd";

export const SHELL_INIT_COMMAND = 'eval "$(cdd init zsh)"';

export const BUILD_COMMAND = "cargo build --release";
