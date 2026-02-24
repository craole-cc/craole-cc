import { expect, SEL, test } from "../fixtures/base";

test.describe("Accessibility", () => {
  test("page has a main landmark", async ({ homePage: page }) => {
    await expect(page.locator("main")).toBeVisible();
  });

  test("page has a nav landmark", async ({ homePage: page }) => {
    await expect(page.locator("nav, [role='navigation']")).toBeVisible();
  });

  test("page has a footer landmark", async ({ homePage: page }) => {
    await expect(page.locator("footer")).toBeVisible();
  });

  test("images have alt attributes", async ({ homePage: page }) => {
    const images = page.locator("img");
    const count = await images.count();
    for (let i = 0; i < count; i++) {
      const alt = await images.nth(i).getAttribute("alt");
      expect(alt).not.toBeNull();
    }
  });

  test("theme button has an accessible label", async ({ homePage: page }) => {
    const btn = page.locator(SEL.themeBtn);
    const label = (await btn.getAttribute("aria-label")) || (await btn.textContent());
    expect(label?.trim().length).toBeGreaterThan(0);
  });

  test("focus is visible on interactive elements", async ({ homePage: page }) => {
    await page.keyboard.press("Tab");
    const focused = page.locator(":focus");
    await expect(focused).toBeVisible();
  });
});
