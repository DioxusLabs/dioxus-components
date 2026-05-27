import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const BASE = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:8080";
const URL = `${BASE}/component/?name=drag_and_drop_list&`;
const REMOVABLE_URL = `${BASE}/component/block/?name=drag_and_drop_list&variant=removable&`;
const LOAD_TIMEOUT = 20 * 60 * 1000;

/** Navigate to the DnD page and return the first (main) variant list. */
async function loadMainList(page: import("@playwright/test").Page) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });
  const list = page.getByRole("list", { name: "Sortable list" }).first();
  await expect(list).toBeVisible({ timeout: 30000 });
  return list;
}

/** Navigate to the DnD page and return the second (removable) variant list. */
async function loadRemovableList(page: import("@playwright/test").Page) {
  await page.goto(REMOVABLE_URL, { timeout: LOAD_TIMEOUT });
  const list = page.getByRole("list", { name: "Sortable list" }).first();
  await expect(list).toBeVisible({ timeout: 30000 });
  return list;
}

/** Helper to get list items from a dnd-list container. */
function getItems(list: import("@playwright/test").Locator) {
  return list.locator('[aria-roledescription="sortable item"]');
}

function getLiveRegion(list: import("@playwright/test").Locator) {
  return list.locator("xpath=..").locator('[role="status"][aria-live="assertive"]');
}

async function itemText(locator: import("@playwright/test").Locator) {
  return (await locator.textContent())?.replace(/\s+/g, "") ?? "";
}

async function dispatchDragLifecycle(
  page: import("@playwright/test").Page,
  options: {
    sourceIndex: number;
    targetIndex: number;
    drop?: "list" | "document";
    end?: boolean;
  },
) {
  await page.evaluate(async ({ sourceIndex, targetIndex, drop, end = true }) => {
    const list = document.querySelector('ul[aria-roledescription="sortable list"]');
    const items = list?.querySelectorAll('li[aria-roledescription="sortable item"]');
    const source = items?.[sourceIndex];
    const target = items?.[targetIndex];
    if (!list || !source || !target) {
      throw new Error("Drag-and-drop test elements were not found");
    }

    const dataTransfer = new DataTransfer();
    const dispatch = (node: EventTarget, type: string, init: DragEventInit = {}) => {
      const event = new DragEvent(type, {
        bubbles: true,
        cancelable: true,
        dataTransfer,
        ...init,
      });
      node.dispatchEvent(event);
    };

    dispatch(source, "dragstart");
    await new Promise(requestAnimationFrame);

    const targetRect = target.getBoundingClientRect();
    dispatch(target, "dragover", {
      clientX: targetRect.left + targetRect.width / 2,
      clientY: targetRect.top + targetRect.height * 0.8,
    });
    await new Promise(requestAnimationFrame);

    if (drop === "list") {
      dispatch(list, "drop");
      await new Promise(requestAnimationFrame);
    } else if (drop === "document") {
      dispatch(document, "drop");
      await new Promise(requestAnimationFrame);
    }

    if (end) {
      dispatch(source, "dragend");
      await new Promise(requestAnimationFrame);
    }
  }, options);
}

test.describe("Keyboard focus management", () => {
  test("first item is tab-reachable", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await expect(items.first()).toHaveAttribute("tabindex", "0");
    await expect(items.nth(1)).toHaveAttribute("tabindex", "-1");
  });

  test("arrow up from first wraps to last", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const lastIndex = (await items.count()) - 1;
    await items.first().click();
    await page.keyboard.press("ArrowUp");
    await expect(items.nth(lastIndex)).toBeFocused();
  });

  test("arrow down from last wraps to first", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const lastIndex = (await items.count()) - 1;
    await items.nth(lastIndex).click();
    await page.keyboard.press("ArrowDown");
    await expect(items.first()).toBeFocused();
  });

  test("roving tabindex updates on arrow navigation", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("ArrowDown");
    await expect(items.first()).toHaveAttribute("tabindex", "-1");
    await expect(items.nth(1)).toHaveAttribute("tabindex", "0");
  });
});

test.describe("Drag and drop lifecycle", () => {
  test("each arrow press moves one position with announcement", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const liveRegion = getLiveRegion(list);
    const itemCount = await items.count();

    // Grab item 3 (index 2)
    await items.nth(2).click();
    await page.keyboard.press("Enter");
    await expect(liveRegion).toContainText(
      `You have lifted an item in position 3 of ${itemCount}`,
    );

    // First ArrowUp immediately moves to position 2
    await page.keyboard.press("ArrowUp");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 2 of ${itemCount}`,
    );

    // Second ArrowUp moves to position 1
    await page.keyboard.press("ArrowUp");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 1 of ${itemCount}`,
    );

    // ArrowDown back to position 2
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 2 of ${itemCount}`,
    );

    // ArrowDown past original position to position 3
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 3 of ${itemCount}`,
    );

    // ArrowDown to position 4
    await page.keyboard.press("ArrowDown");
    await expect(liveRegion).toContainText(
      `You have moved the item to position 4 of ${itemCount}`,
    );

    // Drop
    await page.keyboard.press("Enter");
    await expect(liveRegion).toContainText(
      "You have dropped the item. It has moved from position 3 to position 4",
    );
  });

  test("cancelling announces and returns focus to source", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Escape");
    const liveRegion = getLiveRegion(list);
    await expect(liveRegion).toContainText(
      "Movement cancelled. The item has returned to its starting position of 1",
    );
    await expect(items.first()).toBeFocused();
  });

  test("grabbed item has aria-grabbed true", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("Enter");
    await expect(items.first()).toHaveAttribute("aria-grabbed", "true");
  });

  test("focus after successful drop lands on moved item", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.first().click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(items.nth(1)).toBeFocused();
  });

  test("space key grabs and drops items", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const itemCount = await items.count();
    await items.first().click();
    await page.keyboard.press("Space");
    const liveRegion = getLiveRegion(list);
    await expect(liveRegion).toContainText(
      `You have lifted an item in position 1 of ${itemCount}`,
    );
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Space");
    await expect(liveRegion).toContainText(
      "You have dropped the item. It has moved from position 1 to position 2",
    );
  });

  async function dropIndicatorOpacity(
    page: import("@playwright/test").Page,
  ): Promise<number> {
    return page.evaluate(() => {
      const beforeTarget = document.querySelector(
        '[data-position="before"] + li[aria-roledescription="sortable item"]',
      );
      if (beforeTarget) {
        return Number.parseFloat(
          getComputedStyle(beforeTarget, "::before").opacity,
        );
      }
      const afterTarget = document.querySelector(
        'li[aria-roledescription="sortable item"]:has(+ [data-position="after"])',
      );
      if (afterTarget) {
        return Number.parseFloat(
          getComputedStyle(afterTarget, "::after").opacity,
        );
      }
      return 0;
    });
  }

  test("mouse drag shows the drop-indicator line", async ({ page }) => {
    const list = await loadMainList(page);

    await dispatchDragLifecycle(page, {
      sourceIndex: 2,
      targetIndex: 3,
      end: false,
    });

    await expect(page.locator("[data-position]")).toHaveCount(1);
    await expect.poll(() => dropIndicatorOpacity(page)).toBeGreaterThan(0.5);
  });

  test("keyboard drag shows the drop-indicator line", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    await items.nth(2).click();
    await expect(items.nth(2)).toBeFocused();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");

    await expect.poll(() => dropIndicatorOpacity(page)).toBeGreaterThan(0.5);

    await page.keyboard.press("Escape");
  });

  test("mouse drop to the side commits without cancelling the native drop", async ({
    page,
  }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const sourceTitle = await itemText(items.nth(2));

    await dispatchDragLifecycle(page, {
      sourceIndex: 2,
      targetIndex: 3,
      drop: "document",
    });

    await expect.poll(() => itemText(items.nth(3))).toBe(sourceTitle);
  });

  test("cancelled mouse drag does not reorder the list", async ({ page }) => {
    const list = await loadMainList(page);
    const items = getItems(list);
    const sourceText = await itemText(items.nth(2));
    const targetText = await itemText(items.nth(3));

    await dispatchDragLifecycle(page, {
      sourceIndex: 2,
      targetIndex: 3,
    });

    await expect.poll(() => itemText(items.nth(2))).toBe(sourceText);
    await expect.poll(() => itemText(items.nth(3))).toBe(targetText);
  });
});

test.describe("Remove behavior", () => {
  test("focus moves to item at same index after removal", async ({
    page,
  }) => {
    const list = await loadRemovableList(page);
    const items = getItems(list);
    const initialCount = await items.count();
    const removeButtons = list.getByRole("button", { name: /Remove item/ });
    await removeButtons.nth(2).click();
    await expect(items).toHaveCount(initialCount - 1);
    await expect(items.nth(2)).toBeFocused();
  });

  test("focus moves to new last item when removing last item", async ({
    page,
  }) => {
    const list = await loadRemovableList(page);
    const items = getItems(list);
    const initialCount = await items.count();
    const removeButtons = list.getByRole("button", { name: /Remove item/ });
    await removeButtons.nth(initialCount - 1).click();
    await expect(items).toHaveCount(initialCount - 1);
    await expect(items.nth(initialCount - 2)).toBeFocused();
  });

  test("remove button tracks a keyed item after reordering", async ({
    page,
  }) => {
    const list = await loadRemovableList(page);
    const items = getItems(list);
    const initialCount = await items.count();
    const movedText = await itemText(items.nth(1));

    await items.nth(1).click();
    await page.keyboard.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");

    await expect.poll(() => itemText(items.nth(2))).toBe(movedText);
    await items.nth(2).getByRole("button", { name: /Remove item/ }).click();

    await expect(items).toHaveCount(initialCount - 1);
    await expect.poll(async () => {
      const count = await items.count();
      const texts = await Promise.all(
        Array.from({ length: count }, (_, index) => itemText(items.nth(index))),
      );
      return texts;
    }).not.toContain(movedText);
  });
});

test.describe("Axe automated scan", () => {
  test("no automatically detectable a11y issues", async ({ page }) => {
    await loadMainList(page);

    const accessibilityScanResults = await new AxeBuilder({ page })
      .include('ul[aria-roledescription="sortable list"]')
      .disableRules(["color-contrast"])
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});
