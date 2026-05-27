import { test, expect, type Page } from "@playwright/test";

const singleSelectTrigger = (page: Page) =>
    page.getByRole("button").filter({ hasText: /Select an option|Apple|Banana/ });

const multiSelectTrigger = (page: Page) =>
    page.getByRole("button").filter({ hasText: /Pepperoni|Mushroom|Onion/ });

test("test", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&", {
        timeout: 20 * 60 * 1000,
        waitUntil: 'networkidle'
    }); // Increase timeout to 20 minutes
    // Find Select a fruit...
    let selectTrigger = singleSelectTrigger(page);
    await selectTrigger.click();
    // Assert the select menu is open
    const selectMenu = page.getByRole("listbox");
    await expect(selectMenu).toHaveAttribute("data-state", "open");

    // Assert the menu is focused
    await expect(selectMenu).toBeFocused();
    await page.keyboard.press("ArrowDown");
    const firstOption = selectMenu.getByRole("option", { name: "apple" });
    await expect(firstOption).toBeFocused();

    // Assert moving down with arrow keys moves focus to the next option
    await page.keyboard.press("ArrowDown");
    const secondOption = selectMenu.getByRole("option", { name: "banana" });
    await expect(secondOption).toBeFocused();

    // Assert moving up with arrow keys moves focus back to the previous option
    await page.keyboard.press("ArrowUp");
    await expect(firstOption).toBeFocused();

    // Assert pressing Enter selects the focused option
    await page.keyboard.press("Enter");
    // Assert the select menu is closed after selection
    await expect(selectMenu).toHaveCount(0);

    // Assert the selected value is displayed in the button
    await expect(selectTrigger).toHaveText("Apple");

    // Reopen the select menu
    await selectTrigger.click();

    // Assert typeahead functionality works
    await page.keyboard.type("Ban");
    // Assert the second option is focused after typing 'Ban'
    await expect(secondOption).toBeFocused();

    // Assert pressing Escape closes the select menu
    await page.keyboard.press("Escape");
    // Assert the select menu is closed
    await expect(selectMenu).toHaveCount(0);

    // Reopen the select menu
    await selectTrigger.click();
    // Assert the select menu is open again
    await expect(selectMenu).toHaveAttribute("data-state", "open");

    // Click the second option to select it
    let bananaOption = selectMenu.getByRole("option", { name: "banana" });
    await bananaOption.click();
    // Assert the select menu is closed after clicking an option
    await expect(selectMenu).toHaveCount(0);
    // Assert the selected value is now 'banana'
    await expect(selectTrigger).toHaveText("Banana");
});

test("tabbing out of menu closes the select menu", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&");
    // Find Select a fruit...
    let selectTrigger = singleSelectTrigger(page);
    await selectTrigger.click();
    // Assert the select menu is open
    const selectMenu = page.getByRole("listbox");
    await expect(selectMenu).toHaveAttribute("data-state", "open");

    // Assert the menu is focused
    await expect(selectMenu).toBeFocused();
    await page.keyboard.press("Tab");
    // Assert the select menu is closed
    await expect(selectMenu).toHaveCount(0);
});

test("multi-select toggles options and stays open", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&variant=multi&", {
        timeout: 20 * 60 * 1000,
    });
    const selectTrigger = multiSelectTrigger(page);
    // Default values from the demo: Pepperoni and Mushroom
    await expect(selectTrigger).toContainText("Pepperoni");
    await expect(selectTrigger).toContainText("Mushroom");

    await selectTrigger.click();
    const selectMenu = page.getByRole("listbox");
    await expect(selectMenu).toHaveAttribute("data-state", "open");

    const pepperoni = selectMenu.getByRole("option", { name: "Pepperoni" });
    const onion = selectMenu.getByRole("option", { name: "Onion" });

    await expect(pepperoni).toHaveAttribute("aria-selected", "true");
    await expect(onion).toHaveAttribute("aria-selected", "false");

    // Click an unselected option — it should toggle on without closing
    await onion.click();
    await expect(selectMenu).toHaveAttribute("data-state", "open");
    await expect(onion).toHaveAttribute("aria-selected", "true");

    // Click an already-selected option — it should toggle off without closing
    await pepperoni.click();
    await expect(selectMenu).toHaveAttribute("data-state", "open");
    await expect(pepperoni).toHaveAttribute("aria-selected", "false");

    // Escape closes the menu without selecting
    await page.keyboard.press("Escape");
    await expect(selectMenu).toHaveCount(0);
    // Trigger reflects the updated multi-selection
    await expect(selectTrigger).toContainText("Mushroom");
    await expect(selectTrigger).toContainText("Onion");
    await expect(selectTrigger).not.toContainText("Pepperoni");
});

test("mobile: multi-select tapping options keeps the dropdown open", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&variant=multi&", {
        timeout: 20 * 60 * 1000,
    });
    const selectTrigger = multiSelectTrigger(page);
    await selectTrigger.tap();

    const selectMenu = page.getByRole("listbox");
    await expect(selectMenu).toHaveAttribute("data-state", "open");

    const onion = selectMenu.getByRole("option", { name: "Onion" });
    await expect(onion).toHaveAttribute("aria-selected", "false");

    // Tapping the first option on mobile should toggle it without closing the menu
    await onion.tap();
    await expect(selectMenu).toHaveAttribute("data-state", "open");
    await expect(onion).toHaveAttribute("aria-selected", "true");

    // Tapping a second option should also leave the dropdown open
    const pepperoni = selectMenu.getByRole("option", { name: "Pepperoni" });
    await pepperoni.tap();
    await expect(selectMenu).toHaveAttribute("data-state", "open");
    await expect(pepperoni).toHaveAttribute("aria-selected", "false");
});

test("multi-select keyboard toggles and exposes aria-multiselectable", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&variant=multi&", {
        timeout: 20 * 60 * 1000,
    });
    const selectTrigger = multiSelectTrigger(page);
    await selectTrigger.click();

    const selectMenu = page.getByRole("listbox");
    await expect(selectMenu).toHaveAttribute("data-state", "open");
    // Listbox advertises multi-select mode for assistive tech
    await expect(selectMenu).toHaveAttribute("aria-multiselectable", "true");

    // Arrow down to focus the first option (Pepperoni — already selected by default)
    await page.keyboard.press("ArrowDown");
    const pepperoni = selectMenu.getByRole("option", { name: "Pepperoni" });
    await expect(pepperoni).toBeFocused();

    // Space toggles the focused option off without closing
    await page.keyboard.press(" ");
    await expect(selectMenu).toHaveAttribute("data-state", "open");
    await expect(pepperoni).toHaveAttribute("aria-selected", "false");

    // Arrow down to Onion and toggle on with Enter — menu still open in multi-mode
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("ArrowDown");
    const onion = selectMenu.getByRole("option", { name: "Onion" });
    await expect(onion).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(selectMenu).toHaveAttribute("data-state", "open");
    await expect(onion).toHaveAttribute("aria-selected", "true");

    await page.keyboard.press("Escape");
    await expect(selectMenu).toHaveCount(0);
    await expect(selectTrigger).toContainText("Mushroom");
    await expect(selectTrigger).toContainText("Onion");
    await expect(selectTrigger).not.toContainText("Pepperoni");
});

test("tabbing out of item closes the select menu", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&");
    // Find Select a fruit...
    let selectTrigger = singleSelectTrigger(page);
    await selectTrigger.click();
    // Assert the select menu is open
    const selectMenu = page.getByRole("listbox");
    await expect(selectMenu).toHaveAttribute("data-state", "open");

    // Assert the menu is focused
    await expect(selectMenu).toBeFocused();

    // Navigate to the first option
    await page.keyboard.press("ArrowDown");
    const firstOption = selectMenu.getByRole("option", { name: "apple" });
    await expect(firstOption).toBeFocused();
    await page.keyboard.press("Tab");
    // Assert the select menu is closed
    await expect(selectMenu).toHaveCount(0);
});

test("options selected", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&");
    // Find Select a fruit...
    let selectTrigger = singleSelectTrigger(page);
    await selectTrigger.click();
    // Assert the select menu is open
    const selectMenu = page.getByRole("listbox");
    await expect(selectMenu).toHaveAttribute("data-state", "open");

    // Assert no items have aria-selected
    const options = selectMenu.getByRole("option");
    let optionCount = await options.count();
    for (let i = 0; i < optionCount; i++) {
        await expect(options.nth(i)).not.toHaveAttribute("aria-selected", "true");
    }

    // Select the first option
    await page.keyboard.press("ArrowDown");
    const firstOption = selectMenu.getByRole("option", { name: "apple" });
    await expect(firstOption).toBeFocused();
    await page.keyboard.press("Enter");
    // Assert the select menu is closed after selection
    await expect(selectMenu).toHaveCount(0);
    // Open the select menu again
    await selectTrigger.click();
    // Assert the first option is now selected
    await expect(firstOption).toHaveAttribute("aria-selected", "true");
});

test("down arrow selects first element", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&");
    // Find Select a fruit...
    let selectTrigger = singleSelectTrigger(page);
    const selectMenu = page.getByRole("listbox");
    await selectTrigger.focus();

    // Select the first option
    await page.keyboard.press("ArrowDown");
    const firstOption = selectMenu.getByRole("option", { name: "apple" });
    await expect(firstOption).toBeFocused();
});

test("up arrow selects last element", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&");
    // Find Select a fruit...
    let selectTrigger = singleSelectTrigger(page);
    const selectMenu = page.getByRole("listbox");
    await selectTrigger.focus();

    // Select the first option
    await page.keyboard.press("ArrowUp");
    const firstOption = selectMenu.getByRole("option", { name: "other" });
    await expect(firstOption).toBeFocused();
});

test("keyboard navigation skips disabled options", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&");
    const selectTrigger = singleSelectTrigger(page);
    await selectTrigger.click();

    const selectMenu = page.getByRole("listbox");
    const orange = selectMenu.getByRole("option").filter({ hasText: "Orange" }).first();
    const orangeade = selectMenu.getByRole("option").filter({ hasText: "Orangeade" });
    await expect(orange).toHaveAttribute("aria-disabled", "true");

    await page.keyboard.press("ArrowDown");
    await expect(selectMenu.getByRole("option", { name: "apple" })).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(selectMenu.getByRole("option", { name: "banana" })).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(orangeade).toBeFocused();
    await expect(orange).not.toBeFocused();
});

test("typeahead skips disabled options", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/component/?name=select&");
    const selectTrigger = singleSelectTrigger(page);
    await selectTrigger.click();

    const selectMenu = page.getByRole("listbox");
    const orange = selectMenu.getByRole("option").filter({ hasText: "Orange" }).first();
    const orangeade = selectMenu.getByRole("option").filter({ hasText: "Orangeade" });
    await expect(orange).toHaveAttribute("aria-disabled", "true");

    await page.keyboard.type("Ora");
    await expect(orangeade).toBeFocused();
    await expect(orange).not.toBeFocused();
});
