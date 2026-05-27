import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=alert_dialog&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('button', { name: 'Show Alert Dialog' }).click();
  // Assert the dialog is open
  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();
  // Assert the cancel button is focused
  const cancelButton = page.getByRole('button', { name: 'Cancel' });
  await expect(cancelButton).toBeFocused();
  // Hitting tab should move to the confirm button
  await page.keyboard.press('Tab');
  // Hitting tab again should move focus back to the cancel button
  await page.keyboard.press('Tab');
  await expect(cancelButton).toBeFocused();
  // Hitting escape should close the dialog
  await page.keyboard.press('Escape');
  // Assert the dialog is closed
  await expect(dialog).toHaveCount(0);

  // Reopen the dialog
  await page.getByRole('button', { name: 'Show Alert Dialog' }).click();
  // Assert the dialog is open again
  await expect(dialog).toBeVisible();
  // Click the confirm button
  const confirmButton = page.getByRole('button', { name: 'Delete' });
  await confirmButton.click();
  // Assert the dialog is closed after confirming
  await expect(dialog).toHaveCount(0);
});
