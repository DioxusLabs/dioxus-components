import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=tabs&");
  let activeTab = page.locator('[role="tabpanel"][data-state="active"]:not(#component-preview-frame)')
    .filter({ hasText: /^Tab \d Content$/ });
  let tab1Button = page.getByRole("tab", { name: "Tab 1" });
  let tab2Button = page.getByRole("tab", { name: "Tab 2" });
  let tab3Button = page.getByRole("tab", { name: "Tab 3" });
  // Clicking the right arrow should focus the next tab trigger
  await tab1Button.click();
  await page.keyboard.press("ArrowRight");
  await expect(tab2Button).toBeFocused();

  // Clicking enter should activate the focused tab
  await page.keyboard.press("Enter");
  await expect(activeTab).toContainText("Tab 2 Content");

  // Clicking right twice more should bring us back to the first tab
  await page.keyboard.press("ArrowRight");
  await expect(tab3Button).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(tab1Button).toBeFocused();

  // Clicking each tab should activate it
  await tab3Button.click();
  await expect(activeTab).toContainText("Tab 3 Content");
  await tab2Button.click();
  await expect(activeTab).toContainText("Tab 2 Content");
  await tab1Button.click();
  await expect(activeTab).toContainText("Tab 1 Content");
});
