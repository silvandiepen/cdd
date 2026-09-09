import { SITE_NAME, SITE_URL } from "@/config/site";

import type { PageMeta } from "./usePageMeta.model";

/**
 * Sets the document title, description, canonical link and Open Graph tags for
 * one page.
 *
 * A single-page app serves one `index.html`, so without this every route would
 * be indexed as a copy of the same page. Every view calls it once.
 *
 * @param meta - Title, description and path for the current page.
 */
export const usePageMeta = (meta: PageMeta): void => {
  const url = `${SITE_URL}${meta.path === "/" ? "" : meta.path}`;
  const title = meta.path === "/" ? meta.title : `${meta.title} — ${SITE_NAME}`;

  document.title = title;

  setMetaTag("name", "description", meta.description);
  setMetaTag("property", "og:title", title);
  setMetaTag("property", "og:description", meta.description);
  setMetaTag("property", "og:url", url);
  setMetaTag("property", "og:type", "website");
  setMetaTag("property", "og:site_name", SITE_NAME);
  setCanonical(url);
};

const setMetaTag = (keyAttribute: "name" | "property", key: string, content: string): void => {
  const selector = `meta[${keyAttribute}="${key}"]`;
  let tag = document.head.querySelector<HTMLMetaElement>(selector);

  if (!tag) {
    tag = document.createElement("meta");
    tag.setAttribute(keyAttribute, key);
    document.head.appendChild(tag);
  }

  tag.setAttribute("content", content);
};

const setCanonical = (url: string): void => {
  let link = document.head.querySelector<HTMLLinkElement>('link[rel="canonical"]');

  if (!link) {
    link = document.createElement("link");
    link.setAttribute("rel", "canonical");
    document.head.appendChild(link);
  }

  link.setAttribute("href", url);
};
