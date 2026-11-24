import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test.describe("homepage", () => {
  test("should not have any automatically detectable accessibility issues", async ({
    page,
  }) => {
    await page.goto("http://127.0.0.1:8080/", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes

    // Wait for the page to fully load
    let heroSection = page.locator("#hero");
    await heroSection.waitFor({ state: "visible" });

    const accessibilityScanResults = await new AxeBuilder({ page })
      .disableRules("color-contrast")
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});


test.describe("details", () => {
  test("should not have any automatically detectable accessibility issues", async ({
    page,
  }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=calendar", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes

    // Wait for the page to fully load
    let componentSection = page.getByRole("heading", { name: "calendar" });
    await componentSection.waitFor({ state: "visible" });

    const accessibilityScanResults = await new AxeBuilder({ page })
      .disableRules("color-contrast")
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});

test("preview theme persists and updates the sidebar iframe", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=sidebar");
  const frame = page.frameLocator("iframe").first();
  await expect(frame.locator('[data-slot="sidebar-wrapper"]')).toBeVisible();
  const toggle = page.getByRole("button", { name: "Enable dark mode" });
  const lightToggle = page.getByRole("button", { name: "Enable light mode" });
  if (await lightToggle.isVisible()) await lightToggle.click();
  await toggle.click();

  await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
  await expect(frame.locator("html")).toHaveAttribute("data-theme", "dark");
  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
});

test("preview copy buttons copy the command and visible code", async ({ page, context, browserName }) => {
  test.skip(browserName !== "chromium", "Clipboard permission tested in Chromium");
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.goto("http://127.0.0.1:8080/component/?name=button");

  const command = page.getByRole("button", { name: "Copy install command" }).first();
  await command.click();
  await expect(command).toHaveAttribute("data-copied", "true");
  const readClipboard = () => page.evaluate(() => navigator.clipboard.readText());
  await expect.poll(readClipboard).toBe("dx components add button");

  await page.getByRole("tab", { name: "CODE", exact: true }).first().click();
  const panel = page.getByRole("tabpanel").filter({ visible: true }).first();
  const code = await panel.locator("pre:visible").innerText();
  const copyCode = panel.getByRole("button", { name: "Copy code" });
  await copyCode.click();
  await expect(copyCode).toHaveAttribute("data-copied", "true");
  await expect.poll(readClipboard).toBe(code);
});

test("preview player advances, pauses and resumes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/");
  const player = page.locator(".dx-widget-card").filter({ hasText: "Midnight City" });
  const track = player.getByRole("slider", { name: "Track progress" });
  const trackPosition = async () => Number(await track.getAttribute("aria-valuenow"));
  await expect.poll(trackPosition).toBeGreaterThan(84.5);

  const toggle = player.getByRole("button", { name: "Play or pause" });
  await toggle.click();
  const paused = await trackPosition();
  await page.waitForTimeout(700);
  expect(await trackPosition()).toBe(paused);
  await toggle.click();
  await expect.poll(trackPosition).toBeGreaterThan(paused + 0.5);
});
