import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "http://127.0.0.1:8080/component/?name=accordion&";
const LOAD_TIMEOUT = 20 * 60 * 1000;

async function loadAccordion(page: Page) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT, waitUntil: 'networkidle' });
  const accordionItems = page.locator("[data-open]").filter({ has: page.getByRole("button") });
  await expect(accordionItems.first()).toHaveAttribute("data-disabled", "false", {
    timeout: 30000,
  });
  return accordionItems;
}

async function clickOpen(button: Locator, item: Locator) {
  await expect(button).toBeEnabled();
  await button.click();
  await expect(item).toHaveAttribute("data-open", "true");
}

test("test", async ({ page }) => {
  const accordionItems = await loadAccordion(page);
  const buttons = accordionItems.getByRole("button");
  const firstAccordionItem = accordionItems.first();
  await clickOpen(buttons.first(), firstAccordionItem);

  const secondAccordionItem = accordionItems.nth(1);
  await clickOpen(buttons.nth(1), secondAccordionItem);
  await expect(firstAccordionItem).toHaveAttribute("data-open", "false");
});

test("keyboard navigation skips disabled items", async ({ page }) => {
  const accordionItems = await loadAccordion(page);
  const buttons = accordionItems.getByRole("button");

  await expect(accordionItems.nth(2)).toHaveAttribute("data-disabled", "true");
  await expect(buttons.nth(2)).toBeDisabled();

  await buttons.nth(1).focus();
  await page.keyboard.press("ArrowDown");
  await expect(buttons.nth(3)).toBeFocused();

  await page.keyboard.press("ArrowUp");
  await expect(buttons.nth(1)).toBeFocused();
});
