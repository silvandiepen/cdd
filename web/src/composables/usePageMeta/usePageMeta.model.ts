export interface PageMeta {
  /** Page title, without the site name — the composable appends it. */
  title: string;
  /** Meta description. One sentence, specific to this page. */
  description: string;
  /** Path relative to the site root, e.g. `/support`. */
  path: string;
}
