import { expect, SEL, test } from "../fixtures/base";

test.describe("Hero section", () => {
  test("hero section is present", async ({ homePage: page }) => {
    await expect(page.locator(SEL.hero)).toBeVisible();
  });

  test("hero has a visible headline", async ({ homePage: page }) => {
    const headline = page.locator(SEL.heroHeadline);
    await expect(headline).toBeVisible();
    const text = await headline.textContent();
    expect(text?.trim().length).toBeGreaterThan(0);
  });

  test("hero has at least one CTA button", async ({ homePage: page }) => {
    const ctas = page.locator(`${SEL.heroCtaPrimary}, ${SEL.heroCtaGhost}`);
    expect(await ctas.count()).toBeGreaterThan(0);
    await expect(ctas.first()).toBeVisible();
  });

  test("hero scroll hint is present", async ({ homePage: page }) => {
    await expect(page.locator(SEL.heroScroll)).toBeVisible();
  });

  test("hero badge is present", async ({ homePage: page }) => {
    await expect(page.locator(SEL.heroBadge)).toBeVisible();
  });
});
