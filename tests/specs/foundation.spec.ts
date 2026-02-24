import { expect, test } from "../fixtures/base";

test.describe("Page foundation", () => {
  test("has correct title", async ({ homePage: page }) => {
    await expect(page).toHaveTitle("Craole-CC");
  });

  test("returns 200 on homepage", async ({ page }) => {
    const response = await page.goto("/");
    expect(response?.status()).toBe(200);
  });

  test("has no console errors on load", async ({ page }) => {
    const errors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") errors.push(msg.text());
    });
    await page.goto("/");
    expect(errors).toHaveLength(0);
  });
});
