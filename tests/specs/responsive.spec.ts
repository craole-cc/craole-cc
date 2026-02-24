import { expect, SEL, test } from "../fixtures/base";

test.describe("Responsive layout", () => {
  test("hamburger menu is visible on mobile", async ({ mobilePage: page }) => {
    await expect(page.locator(SEL.navMenuBtn)).toBeVisible();
  });

  test("hamburger menu is hidden on desktop", async ({ desktopPage: page }) => {
    await expect(page.locator(SEL.navMenuBtn)).toBeHidden();
  });

  test("mobile drawer opens on hamburger click", async ({ mobilePage: page }) => {
    await page.locator(SEL.navMenuBtn).click();
    await expect(page.locator(SEL.navLinks)).toHaveClass(
      new RegExp(SEL.navLinksOpen),
    );
  });

  test("hero stats panel is hidden on mobile", async ({ mobilePage: page }) => {
    await expect(page.locator(SEL.heroStats)).toBeHidden();
  });

  test("hero stats panel is visible on desktop", async ({ desktopPage: page }) => {
    await expect(page.locator(SEL.heroStats)).toBeVisible();
  });
});
