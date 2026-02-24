import { expect, Page, test as base } from "@playwright/test";

// ── Typed fixture extensions ──────────────────────────────────────────────────

type AppFixtures = {
  /** Page already navigated to "/" */
  homePage: Page;
  /** Page at "/" with viewport set to 375×812 (iPhone-ish) */
  mobilePage: Page;
  /** Page at "/" with viewport set to 1280×800 (desktop) */
  desktopPage: Page;
};

export const test = base.extend<AppFixtures>({
  homePage: async ({ page }, use) => {
    await page.goto("/");
    await use(page);
  },

  mobilePage: async ({ page }, use) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto("/");
    await use(page);
  },

  desktopPage: async ({ page }, use) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.goto("/");
    await use(page);
  },
});

export { expect };

// ── Selector constants ────────────────────────────────────────────────────────
//   Centralise selectors so a class rename only needs one edit.

export const SEL = {
  // Navigation
  nav: ".site-nav",
  navScrolled: "site-nav--scrolled",
  navLogoMark: ".site-nav__logo-mark",
  navLink: ".site-nav__link",
  navMenuBtn: ".site-nav__menu-btn",
  navLinks: ".site-nav__links",
  navLinksOpen: "site-nav__links--open",

  // Theme
  themeBtn: ".theme-btn",
  themeMenu: ".theme-dropdown__menu",
  themeOption: ".theme-dropdown__option",

  // Hero
  hero: ".hero",
  heroHeadline: ".hero__headline",
  heroCtaPrimary: ".hero__cta-primary",
  heroCtaGhost: ".hero__cta-ghost",
  heroScroll: ".hero__scroll",
  heroBadge: ".hero__badge",
  heroStats: ".hero__stats",

  // Footer
  footer: ".footer",
  divider: ".divider",
  dividerDot: ".divider__dot",
  footerSocialLink: ".footer__social-link",
  footerFacet: ".footer__facet",
  footerAlias: ".footer__alias",
  footerTechRust: ".footer__tech--rust",
  footerTechLeptos: ".footer__tech--leptos",
} as const;
