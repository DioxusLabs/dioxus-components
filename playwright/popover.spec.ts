import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=popover&");
  const popoverButton = page.getByRole("button", { name: "Show Popover" });
  await expect(popoverButton).toBeVisible();
  await popoverButton.click();
  // pressing the first input should be focused
  const confirm = page.getByRole("button", { name: "Confirm" });
  const cancel = page.getByRole("button", { name: "Cancel" });
  await expect(confirm).toBeFocused();
  // pressing tab again should focus the cancel button
  await page.keyboard.press("Tab");
  await expect(cancel).toBeFocused();
  // pressing tab again should focus the confirm button again
  await page.keyboard.press("Tab");
  await expect(confirm).toBeFocused();
  // pressing enter should close the popover
  await page.keyboard.press("Enter");
  // the item should show deleted under component-preview-frame
  await expect(page.locator("#component-preview-frame")).toContainText(
    "Item deleted!",
  );

  // Open the popover again
  await popoverButton.click();
  // pressing escape should close the popover
  await page.keyboard.press("Escape");
});

test("popover dismisses when clicking outside", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=popover&");
  const popoverButton = page.getByRole("button", { name: "Show Popover" });
  await popoverButton.click();
  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();
  // Click far outside the popover (corner of the document) — should dismiss.
  await page.mouse.click(2, 2);
  await expect(dialog).toHaveCount(0);
});
