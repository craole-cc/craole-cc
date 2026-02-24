import { expect, SEL, test } from "../fixtures/base";

test.describe("Footer", () => {
  test.beforeEach(async ({ homePage: page }) => {
    await page.locator(SEL.footer).scrollIntoViewIfNeeded();
  });

  test("footer is present", async ({ homePage: page }) => {
    await expect(page.locator(SEL.footer)).toBeVisible();
  });

  test("divider lines are present", async ({ homePage: page }) => {
    const dividers = page.locator(SEL.divider);
    expect(await dividers.count()).toBeGreaterThanOrEqual(1);
  });

  test("dot divider accent dot is present", async ({ homePage: page }) => {
    await expect(page.locator(SEL.dividerDot)).toBeVisible();
  });

  test("social icons are present", async ({ homePage: page }) => {
    const socials = page.locator(SEL.footerSocialLink);
    expect(await socials.count()).toBeGreaterThan(0);
    await expect(socials.first()).toBeVisible();
  });

  test("facet nav links are present", async ({ homePage: page }) => {
    const facets = page.locator(SEL.footerFacet);
    expect(await facets.count()).toBeGreaterThan(0);
  });

  test("copyright alias name is visible", async ({ homePage: page }) => {
    const alias = page.locator(SEL.footerAlias);
    await expect(alias).toBeVisible();
    const text = await alias.textContent();
    expect(text?.trim().length).toBeGreaterThan(0);
  });

  test("Rust and Leptos tech credit links are present", async ({ homePage: page }) => {
    await expect(page.locator(SEL.footerTechRust)).toBeVisible();
    await expect(page.locator(SEL.footerTechLeptos)).toBeVisible();
  });
});
