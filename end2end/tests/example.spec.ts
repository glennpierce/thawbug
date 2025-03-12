import { test, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
  await page.goto("http://192.168.22.8:4000/");

  await expect(page).toHaveTitle("Welcome to Leptos");

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
});
