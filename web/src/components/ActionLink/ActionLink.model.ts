export type ActionLinkVariant = "primary" | "outline" | "ghost";

export interface ActionLinkProps {
  /** Where the action goes. In-page anchors and external URLs both work. */
  href: string;
  variant?: ActionLinkVariant;
  /** Show a trailing arrow, for the one action that leads somewhere onward. */
  arrow?: boolean;
}
