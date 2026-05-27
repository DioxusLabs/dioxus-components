import { test, expect } from '@playwright/test';

test('sheet basic interactions', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=sheet&', { timeout: 20 * 60 * 1000 });

  // Open sheet from Right button
  await page.getByRole('button', { name: 'Right' }).click();

  // Assert the sheet is open
  const sheet = page.locator('[data-slot="sheet-root"]');
  await expect(sheet).toHaveAttribute('data-state', 'open');

  // Assert the first input is focused (focus trap)
  const nameInput = page.locator('#sheet-demo-name');
  await expect(nameInput).toBeFocused();

  // Tab through focusable elements and verify focus cycles
  // Tab: name input -> username input -> Save button -> Cancel button -> close button -> name input
  await page.keyboard.press('Tab');
  const usernameInput = page.locator('#sheet-demo-username');
  await expect(usernameInput).toBeFocused();

  await page.keyboard.press('Tab');
  const saveButton = page.getByRole('button', { name: 'Save changes' });
  await expect(saveButton).toBeFocused();

  await page.keyboard.press('Tab');
  const cancelButton = page.getByRole('button', { name: 'Cancel' });
  await expect(cancelButton).toBeFocused();

  await page.keyboard.press('Tab');
  const closeButton = sheet.getByRole('button').last();
  await expect(closeButton).toBeFocused();

  // Tab again should cycle back to first input
  await page.keyboard.press('Tab');
  await expect(nameInput).toBeFocused();

  // Hitting escape should close the sheet
  await page.keyboard.press('Escape');
  await expect(sheet).toHaveCount(0);

  // Reopen the sheet
  await page.getByRole('button', { name: 'Right' }).click();
  await expect(sheet).toHaveAttribute('data-state', 'open');

  // Click the close button
  await closeButton.click();
  await expect(sheet).toHaveCount(0);
});

test('sheet opens from different sides', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=sheet&', { timeout: 20 * 60 * 1000 });

  const sheet = page.locator('[data-slot="sheet-root"]');
  const sheetContent = page.locator('[data-slot="sheet-content"]');

  // Test Top
  await page.getByRole('button', { name: 'Top' }).click();
  await expect(sheet).toHaveAttribute('data-state', 'open');
  await expect(sheetContent).toHaveAttribute('data-side', 'top');
  await page.keyboard.press('Escape');
  await expect(sheet).toHaveCount(0);

  // Test Bottom
  await page.getByRole('button', { name: 'Bottom' }).click();
  await expect(sheet).toHaveAttribute('data-state', 'open');
  await expect(sheetContent).toHaveAttribute('data-side', 'bottom');
  await page.keyboard.press('Escape');
  await expect(sheet).toHaveCount(0);

  // Test Left
  await page.getByRole('button', { name: 'Left' }).click();
  await expect(sheet).toHaveAttribute('data-state', 'open');
  await expect(sheetContent).toHaveAttribute('data-side', 'left');
  await page.keyboard.press('Escape');
  await expect(sheet).toHaveCount(0);
});
