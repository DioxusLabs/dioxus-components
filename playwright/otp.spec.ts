import { test, expect, type Locator, type Page } from '@playwright/test';

const previewUrl = process.env.PREVIEW_URL ?? 'http://127.0.0.1:8080';
const otpUrl = new URL('/component/?name=otp&', previewUrl).toString();
const nonAsciiOtpUrl = new URL('/component/?name=otp&variant=non_ascii&', previewUrl).toString();

async function waitForOtpLayout(page: Page) {
  const input = page.getByRole('textbox', { name: 'One-time password' });
  const frame = page.locator('#component-preview-frame').first();
  await expect
    .poll(async () => {
      const [inputBox, frameBox] = await Promise.all([
        input.boundingBox(),
        frame.boundingBox(),
      ]);

      if (!inputBox || !frameBox) {
        return false;
      }

      return (
        inputBox.width < 400 &&
        frameBox.width < 900 &&
        inputBox.x >= frameBox.x &&
        inputBox.x + inputBox.width <= frameBox.x + frameBox.width
      );
    })
    .toBe(true);
  await input.scrollIntoViewIfNeeded();
  await expect(input).toBeInViewport();
}

function otpSlot(otp: Locator, index: number) {
  return otp.locator('[data-empty]').nth(index);
}

test('otp typing, backspace, and rejection', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await expect(input).toBeVisible();
  await expect(input).toHaveCSS('opacity', '0');

  await input.focus();
  await page.keyboard.type('123456');
  await expect(page.locator('#otp-value')).toHaveText('123456');

  await page.keyboard.press('Backspace');
  await expect(page.locator('#otp-value')).toHaveText('12345');

  await page.keyboard.press('a');
  await expect(page.locator('#otp-value')).toHaveText('12345');
});

test('otp cursor does not drift past typed length', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await input.focus();
  await page.keyboard.type('12');
  // Three right-arrows from end should be no-ops — cursor is already at end.
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('ArrowRight');
  // The next typed digit goes at position 2, not somewhere past it.
  await page.keyboard.type('3');
  await expect(page.locator('#otp-value')).toHaveText('123');
});

test('otp ArrowLeft replaces the active slot', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await input.focus();
  await page.keyboard.type('12');
  await page.keyboard.press('ArrowLeft');
  await page.keyboard.type('9');
  await expect(page.locator('#otp-value')).toHaveText('19');
});

test('otp input events replace the active slot', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await input.focus();
  await page.keyboard.type('12');
  await page.keyboard.press('ArrowLeft');

  // insertText simulates paste / IME / on-screen-keyboard input without a
  // keydown event for the inserted text.
  await page.keyboard.insertText('9');
  await page.keyboard.insertText('8');
  await expect(page.locator('#otp-value')).toHaveText('198');
});

test('otp does not let the native input grow past maxlength', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await input.focus();
  await page.keyboard.type('1234567');
  await expect(page.locator('#otp-value')).toHaveText('123457');
  await expect(input).toHaveValue('123457');

  await page.keyboard.press('Home');
  await page.keyboard.type('9');
  await expect(page.locator('#otp-value')).toHaveText('923457');
  await expect(input).toHaveValue('923457');

  await page.keyboard.press('End');
  await page.keyboard.type('8');
  await expect(page.locator('#otp-value')).toHaveText('923458');
  await expect(input).toHaveValue('923458');
});

test('otp keeps visual focus visible at the end', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const otp = input.locator('xpath=..');
  await input.focus();
  await page.keyboard.type('123456');

  await expect(otp.locator('[data-active="true"]')).toHaveCount(1);
  await expect(otpSlot(otp, 5)).toHaveAttribute('data-active', 'true');

  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('End');
  await expect(otp.locator('[data-active="true"]')).toHaveCount(1);
  await expect(otpSlot(otp, 5)).toHaveAttribute('data-active', 'true');
  await expect(otpSlot(otp, 5)).toHaveCSS('border-left-width', '1px');
  await expect(otpSlot(otp, 5)).toHaveCSS('border-left-style', 'solid');
});

test('otp renders its own selection highlight', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const otp = input.locator('xpath=..');
  await input.focus();
  await page.keyboard.type('123456');

  await page.keyboard.press('ControlOrMeta+A');
  await expect(otp.locator('[data-selected="true"]')).toHaveCount(6);

  await page.keyboard.type('9');
  await expect(page.locator('#otp-value')).toHaveText('9');
  await expect(otp.locator('[data-selected="true"]')).toHaveCount(0);
});

test('otp pointer selection highlights slots', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const otp = input.locator('xpath=..');
  await input.focus();
  await page.keyboard.type('123456');
  await input.evaluate((node: HTMLInputElement) => node.blur());
  await expect(input).not.toBeFocused();
  await waitForOtpLayout(page);

  const start = await otpSlot(otp, 1).boundingBox();
  const end = await otpSlot(otp, 4).boundingBox();
  expect(start).not.toBeNull();
  expect(end).not.toBeNull();

  await page.mouse.move(start!.x + start!.width / 2 + 1, start!.y + start!.height / 2);
  await page.mouse.down();
  await page.mouse.move(end!.x + end!.width / 2 + 1, end!.y + end!.height / 2, { steps: 5 });
  await page.mouse.up();

  await expect(otp.locator('[data-selected="true"]')).toHaveCount(4);
  await expect(otpSlot(otp, 1)).toHaveAttribute('data-selection-start', 'true');
  await expect(otpSlot(otp, 4)).toHaveAttribute('data-selection-end', 'true');

  await page.keyboard.type('9');
  await expect(page.locator('#otp-value')).toHaveText('196');
});

test('otp backward pointer selection includes the slot under the pointer', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const otp = input.locator('xpath=..');
  await input.focus();
  await page.keyboard.type('123456');
  await waitForOtpLayout(page);

  const start = await otpSlot(otp, 4).boundingBox();
  const end = await otpSlot(otp, 1).boundingBox();
  expect(start).not.toBeNull();
  expect(end).not.toBeNull();

  await page.mouse.move(start!.x + start!.width / 2, start!.y + start!.height / 2);
  await page.mouse.down();
  await page.mouse.move(end!.x + end!.width / 2, end!.y + end!.height / 2, { steps: 5 });
  await page.mouse.up();

  await expect(otp.locator('[data-selected="true"]')).toHaveCount(4);
  await expect(otpSlot(otp, 4)).toHaveAttribute('data-selected', 'true');
  await expect(otpSlot(otp, 1)).toHaveAttribute('data-selection-start', 'true');
  await expect(otpSlot(otp, 1)).toHaveCSS('border-left-width', '1px');
  await expect(otpSlot(otp, 1)).toHaveCSS('border-left-style', 'solid');

  await page.keyboard.type('9');
  await expect(page.locator('#otp-value')).toHaveText('196');
});

test('otp copy cut and paste use the visible selection', async ({
  page,
  context,
  browserName,
}) => {
  test.skip(browserName !== 'chromium', 'Clipboard permissions are Chromium-only in Playwright.');

  await context.grantPermissions(['clipboard-read', 'clipboard-write']);
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const otp = input.locator('xpath=..');
  await input.focus();
  await page.keyboard.type('123456');

  await page.keyboard.press('ControlOrMeta+A');
  await expect(otp.locator('[data-selected="true"]')).toHaveCount(6);

  await page.keyboard.press('ControlOrMeta+C');
  await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe('123456');

  await page.evaluate(() => navigator.clipboard.writeText('98'));
  await page.keyboard.press('ControlOrMeta+V');
  await expect(page.locator('#otp-value')).toHaveText('98');

  await page.keyboard.press('ControlOrMeta+A');
  await page.keyboard.press('ControlOrMeta+X');
  await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe('98');
  await expect(page.locator('#otp-value')).toHaveText('');
});

test('otp paste fills all slots', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await input.focus();
  // insertText simulates a paste / IME / on-screen-keyboard input event.
  await page.keyboard.insertText('987654');
  await expect(page.locator('#otp-value')).toHaveText('987654');
});

test('otp accepts emoji input in the non-ascii variant', async ({ page }) => {
  await page.goto(nonAsciiOtpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'Emoji code' });
  await expect(input).toBeVisible();
  await input.focus();

  for (const emoji of ['😀', '😃', '😄', '😁']) {
    await page.keyboard.insertText(emoji);
  }

  const value = '😀😃😄😁';
  await expect(page.locator('#otp-non-ascii-value')).toHaveText(value);
  await expect(page.locator('#otp-non-ascii-complete')).toHaveText(value);
  await expect(input).toHaveValue(value);
});

test('otp on_complete fires only when the value reaches maxlength', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const complete = page.locator('#otp-complete');

  await input.focus();
  await page.keyboard.type('12345');
  await expect(page.locator('#otp-value')).toHaveText('12345');
  await expect(complete).toHaveText('');

  await page.keyboard.type('6');
  await expect(complete).toHaveText('123456');

  // Editing back below maxlength should not re-fire complete; the last value sticks.
  await page.keyboard.press('Backspace');
  await expect(page.locator('#otp-value')).toHaveText('12345');
  await expect(complete).toHaveText('123456');
});

test('otp on_complete does not re-fire when editing a full buffer', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const complete = page.locator('#otp-complete');

  await input.focus();
  await page.keyboard.type('123456');
  await expect(complete).toHaveText('123456');

  // Move into the full buffer and type. The keydown handler replaces the active
  // slot, keeping length at maxlength but changing the value. This is
  // NOT a transition to maxlength, so on_complete must not fire again.
  await page.keyboard.press('Home');
  await page.keyboard.press('9');
  await expect(page.locator('#otp-value')).toHaveText('923456');
  await expect(complete).toHaveText('123456');
});

test('otp disabled state blocks input', async ({ page }) => {
  await page.goto(otpUrl, { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await page
    .locator('#otp-toggle-disabled')
    .evaluate((node: HTMLButtonElement) => node.click());
  await expect(input).toBeDisabled();

  // Typing into a disabled input is a no-op in the browser.
  await input.focus({ timeout: 1000 }).catch(() => { /* focus may be refused while disabled */ });
  await page.keyboard.type('123');
  await expect(page.locator('#otp-value')).toHaveText('');

  // Re-enable and confirm input works again.
  await page
    .locator('#otp-toggle-disabled')
    .evaluate((node: HTMLButtonElement) => node.click());
  await expect(input).toBeEnabled();
  await input.focus();
  await page.keyboard.type('123');
  await expect(page.locator('#otp-value')).toHaveText('123');
});
