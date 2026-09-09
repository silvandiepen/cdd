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
 * `cdd` has no tagged release yet, so the Homebrew tap does not exist. The site
 * says so rather than printing a command that fails. Flip this when the first
 * release ships and the install section changes with it.
 */
export const HOMEBREW_AVAILABLE = false;

export const HOMEBREW_COMMAND = "brew install silvandiepen/tap/cdd";

export const SHELL_INIT_COMMAND = 'eval "$(cdd init zsh)"';

export const BUILD_COMMAND = "cargo build --release";
