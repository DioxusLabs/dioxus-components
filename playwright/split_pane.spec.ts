import { expect, test, type Page } from "@playwright/test";

const BASE_URL = "http://127.0.0.1:8080";

async function gotoSplitPane(page: Page, variant: string) {
  await page.goto(`${BASE_URL}/component/block/?name=split_pane&variant=${variant}&`, {
    timeout: 20 * 60 * 1000,
    waitUntil: "load",
  });
}

function splitPaneDivider(page: Page, index = 0) {
  return page.locator('[role="separator"]').nth(index);
}

function paneByIndex(page: Page, index: number) {
  return page.locator(`[data-pane-index="${index}"]`);
}

async function readLeftPaneSizeStatus(page: Page) {
  const status = page.locator("text=Left pane:");
  await expect(status).toBeVisible();
  const text = (await status.textContent()) ?? "";
  const match = text.match(/Left pane: (\d+)px/);
  if (!match) throw new Error(`unexpected status text: ${text}`);
  return Number(match[1]);
}

async function readDividerValue(divider: ReturnType<typeof splitPaneDivider>) {
  const value = await divider.getAttribute("aria-valuenow");
  if (!value) throw new Error("split pane divider is missing aria-valuenow");
  return Number(value);
}

test("split pane divider exposes separator semantics and focus", async ({ page }) => {
  await gotoSplitPane(page, "main");

  const divider = splitPaneDivider(page);

  await expect(page.locator('[role="group"][data-orientation="horizontal"]').first()).toHaveAttribute(
    "data-resizable",
    "true",
  );
  await expect(divider).toHaveAttribute("role", "separator");
  await expect(divider).toHaveAttribute("tabindex", "0");
  await expect(divider).toHaveAttribute("aria-orientation", "vertical");

  await divider.focus();
  await expect(divider).toBeFocused();
});

test("horizontal keyboard resize changes the committed pane size", async ({ page }) => {
  await gotoSplitPane(page, "main");

  const divider = splitPaneDivider(page);
  const initialSize = await readDividerValue(divider);

  await divider.focus();
  await page.keyboard.press("ArrowRight");
  const afterRight = await readDividerValue(divider);
  expect(afterRight).toBeGreaterThan(initialSize);

  await page.keyboard.press("ArrowLeft");
  const afterLeft = await readDividerValue(divider);
  expect(afterLeft).toBeLessThan(afterRight);

  const statusSize = await readLeftPaneSizeStatus(page);
  expect(statusSize).toBe(Math.round(afterLeft));
});

test("controlled example commits divider resize updates back into the slider and label", async ({ page }) => {
  await gotoSplitPane(page, "controlled");

  const divider = splitPaneDivider(page);
  const slider = page.getByRole("slider", { name: "Controlled pane size" });
  const label = page.getByText(/Sidebar \d+%/);

  await expect(slider).toHaveAttribute("aria-valuenow", "40");
  await expect(label).toHaveText("Sidebar 40%");

  await divider.focus();
  await expect(divider).toBeFocused();
  await page.keyboard.press("ArrowRight");

  await expect(slider).not.toHaveAttribute("aria-valuenow", "40");
  await expect(label).not.toHaveText("Sidebar 40%");
  await expect.poll(async () => Number((await slider.getAttribute("aria-valuenow")) ?? "0")).toBeGreaterThan(40);
});

test("multi-pane layout keeps both dividers interactive", async ({ page }) => {
  await gotoSplitPane(page, "multi_pane");

  const dividers = page.getByRole("separator");
  await expect(dividers).toHaveCount(2);

  const firstDivider = dividers.first();
  const secondDivider = dividers.nth(1);
  const inspectorPane = paneByIndex(page, 2);

  await firstDivider.focus();
  await expect(firstDivider).toBeFocused();
  await page.keyboard.press("ArrowRight");

  await expect(secondDivider).toBeVisible();
  await expect(inspectorPane).toBeVisible();
});
