import { expect, SEL, test } from "../fixtures/base";

test.describe("Navigation", () => {
  test("site-nav is present and fixed", async ({ homePage: page }) => {
    const nav = page.locator(SEL.nav);
    await expect(nav).toBeVisible();
    const position = await nav.evaluate((el) => window.getComputedStyle(el).position);
    expect(position).toBe("fixed");
  });

  test("logo mark is visible", async ({ homePage: page }) => {
    await expect(page.locator(SEL.navLogoMark)).toBeVisible();
  });

  test("nav acquires --scrolled modifier after scrolling", async ({ homePage: page }) => {
    const nav = page.locator(SEL.nav);
    await expect(nav).not.toHaveClass(new RegExp(SEL.navScrolled));
    await page.evaluate(() => window.scrollTo(0, 200));
    await expect(nav).toHaveClass(new RegExp(SEL.navScrolled));
  });

  test("nav links are present on desktop", async ({ desktopPage: page }) => {
    const links = page.locator(SEL.navLink);
    await expect(links.first()).toBeVisible();
    expect(await links.count()).toBeGreaterThan(0);
  });
});
