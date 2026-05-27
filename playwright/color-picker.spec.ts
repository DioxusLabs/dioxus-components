import { test, expect, type Page } from '@playwright/test';

const PAGE_URL = 'http://127.0.0.1:8080/component/?name=color_picker&';
const PAGE_TIMEOUT = 20 * 60 * 1000;

async function openPicker(page: Page) {
  await page.goto(PAGE_URL, { timeout: PAGE_TIMEOUT });
  await page.waitForLoadState('networkidle');

  // The color picker button's aria-label contains the current hex.
  const button = page.getByRole('button', { name: /Color picker/i }).first();
  await expect(button).toBeVisible();
  await button.click();
  await expect(button).toHaveAttribute('aria-expanded', 'true');
  const popover = page.getByRole('dialog');
  await expect(popover).toBeVisible();
  return { button, popover };
}

test('opens popover and shows initial color', async ({ page }) => {
  const { button } = await openPicker(page);

  // Initial color is rgb(155, 128, 255) → #9B80FF
  await expect(button).toHaveAttribute('aria-label', /Color picker #9B80FF/i);

  // The hex field inside the popover mirrors the same hex.
  const hexField = page.locator('#color_field');
  await expect(hexField).toHaveValue('#9B80FF');

  // Both the trigger swatch and the popover swatch render the same color.
  const swatches = page.getByRole('img', { name: /Selected color #9B80FF/i });
  await expect(swatches.first()).toHaveAttribute(
    'style',
    /--swatch-color:\s*#9B80FF/i,
  );
});

test('typing a new hex updates the color picker', async ({ page }) => {
  const { button } = await openPicker(page);
  const hexField = page.locator('#color_field');

  // Replace value with a fresh, well-known color.
  await hexField.click();
  await hexField.press('ControlOrMeta+A');
  await hexField.pressSequentially('#FF0000');

  await expect(hexField).toHaveValue('#FF0000');
  // The trigger button's aria-label echoes the new color.
  await expect(button).toHaveAttribute('aria-label', /Color picker #FF0000/i);

  // The hue slider snaps to red (0°). Read it from the thumb that lives in
  // the open dialog, not the (non-existent) one outside it.
  const hueThumb = page.getByRole('slider', { name: 'Hue' });
  // Hue can be reported as 0 or 360 depending on which way palette wraps —
  // both are red.
  await expect.poll(async () => {
    const v = await hueThumb.getAttribute('aria-valuenow');
    return Number(v) % 360;
  }).toBe(0);
});

test('hex field strips invalid characters and caps at 7 chars', async ({ page }) => {
  await openPicker(page);
  const hexField = page.locator('#color_field');

  await hexField.click();
  await hexField.press('ControlOrMeta+A');
  // "ZZZ" should be dropped, only hex digits keep, '#' auto-prepends, and the
  // total is truncated to 7 chars (#RRGGBB).
  await hexField.pressSequentially('ZZZff00aabbccdd');

  const value = await hexField.inputValue();
  expect(value.length).toBeLessThanOrEqual(7);
  expect(value).toMatch(/^#[0-9A-F]{0,6}$/);
});

test('hue slider keyboard navigation updates color', async ({ page }) => {
  await openPicker(page);
  // The hue slider lives inside the popover. Use the thumb directly.
  const hueThumb = page.getByRole('slider', { name: 'Hue' });
  await hueThumb.focus();

  const before = Number(await hueThumb.getAttribute('aria-valuenow'));
  await page.keyboard.press('ArrowRight');
  const after = Number(await hueThumb.getAttribute('aria-valuenow'));

  // ArrowRight should move hue by one step; default slider step is 1.
  expect(after).toBeGreaterThan(before);
  expect(after - before).toBeLessThan(5);

  // Shift+ArrowRight applies the 10× multiplier.
  await page.keyboard.press('Shift+ArrowRight');
  const shifted = Number(await hueThumb.getAttribute('aria-valuenow'));
  expect(shifted - after).toBeGreaterThan(5);
});

test('color area thumb keyboard navigation updates saturation/value', async ({ page }) => {
  await openPicker(page);
  const areaThumb = page.getByLabel('Color area');
  await expect(areaThumb).toBeVisible();
  await areaThumb.focus();

  // Read the visually-hidden range inputs that mirror saturation (x) and value (y).
  const sInput = page.locator('input[aria-label="Saturation"]');
  const vInput = page.locator('input[aria-label="Value"]');

  const sBefore = Number(await sInput.inputValue());
  const vBefore = Number(await vInput.inputValue());

  // ArrowRight increases saturation, and focus hands off to the saturation
  // input so AT can announce the channel.
  await page.keyboard.press('ArrowRight');
  await expect(sInput).toBeFocused();

  // ArrowDown decreases value, and focus hands off to the value input.
  await page.keyboard.press('ArrowDown');
  await expect(vInput).toBeFocused();

  const sAfter = Number(await sInput.inputValue());
  const vAfter = Number(await vInput.inputValue());

  expect(sAfter).toBeGreaterThan(sBefore);
  expect(vAfter).toBeLessThan(vBefore);
});

test('escape closes the color picker popover', async ({ page }) => {
  const { popover } = await openPicker(page);
  await page.keyboard.press('Escape');
  await expect(popover).toHaveCount(0);
});

test('clicking outside closes the color picker popover', async ({ page }) => {
  const { popover } = await openPicker(page);
  // Click far outside the popover.
  await page.mouse.click(2, 2);
  await expect(popover).toHaveCount(0);
});

test('clicking the color area updates saturation and value', async ({ page }) => {
  await openPicker(page);

  const area = page.locator('[role="group"]').filter({ has: page.getByLabel('Color area') }).last();
  const sInput = page.locator('input[aria-label="Saturation"]');
  const vInput = page.locator('input[aria-label="Value"]');

  // Initial color is rgb(155, 128, 255) → saturation ~50, value 100. Click
  // near the bottom-right; saturation should jump high, value should drop.
  const box = await area.boundingBox();
  if (!box) throw new Error('color area has no bounding box');
  await page.mouse.click(box.x + box.width * 0.8, box.y + box.height * 0.8);

  await expect.poll(async () => Number(await sInput.inputValue())).toBeGreaterThan(70);
  await expect.poll(async () => Number(await vInput.inputValue())).toBeLessThan(30);
});

test('dragging the color area updates saturation and value', async ({ page }) => {
  await openPicker(page);

  const area = page.locator('[role="group"]').filter({ has: page.getByLabel('Color area') }).last();
  const sInput = page.locator('input[aria-label="Saturation"]');
  const vInput = page.locator('input[aria-label="Value"]');

  const sBefore = Number(await sInput.inputValue());
  const vBefore = Number(await vInput.inputValue());

  // Drag from the top-left toward the bottom-right of the area. Top-left is
  // (saturation=0, value=100), bottom-right is (saturation=100, value=0). After
  // the drag saturation should go up and value should go down.
  const box = await area.boundingBox();
  if (!box) throw new Error('color area has no bounding box');
  const start = { x: box.x + box.width * 0.2, y: box.y + box.height * 0.2 };
  const end = { x: box.x + box.width * 0.8, y: box.y + box.height * 0.8 };

  await page.mouse.move(start.x, start.y);
  await page.mouse.down();
  await page.mouse.move(end.x, end.y, { steps: 10 });
  await page.mouse.up();

  await expect.poll(async () => Number(await sInput.inputValue())).toBeGreaterThan(sBefore);
  await expect.poll(async () => Number(await vInput.inputValue())).toBeLessThan(vBefore);
});
