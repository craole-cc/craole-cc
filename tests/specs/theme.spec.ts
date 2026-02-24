import { expect, SEL, test } from "../fixtures/base";

test.describe("Theme switcher", () => {
  test("theme button is present in nav controls", async ({ homePage: page }) => {
    await expect(page.locator(SEL.themeBtn)).toBeVisible();
  });

  test("clicking theme button opens the dropdown", async ({ homePage: page }) => {
    await page.locator(SEL.themeBtn).click();
    await expect(page.locator(SEL.themeMenu)).toBeVisible();
  });

  test("theme options are present in dropdown", async ({ homePage: page }) => {
    await page.locator(SEL.themeBtn).click();
    const options = page.locator(SEL.themeOption);
    expect(await options.count()).toBeGreaterThanOrEqual(2);
  });

  test("selecting dark sets data-theme attribute", async ({ homePage: page }) => {
    await page.locator(SEL.themeBtn).click();
    await page.locator(SEL.themeOption, { hasText: /dark/i }).click();
    await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
  });

  test("selecting light sets data-theme attribute", async ({ homePage: page }) => {
    await page.locator(SEL.themeBtn).click();
    await page.locator(SEL.themeOption, { hasText: /light/i }).click();
    await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
  });
});
