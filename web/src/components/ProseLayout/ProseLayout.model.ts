export interface ProseLayoutProps {
  /** Page heading. */
  title: string;
  /** One line under the heading, saying what the page is for. */
  intro: string;
  /** ISO date the page was last reviewed, shown so readers know how current it is. */
  updated?: string;
}
