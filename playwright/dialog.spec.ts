import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=dialog&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('button', { name: 'Show Dialog' }).click();
  // Assert the dialog is open
  const dialog = page.getByRole('dialog');
  await expect(dialog).toBeVisible();
  // Assert the close button is focused
  const closeButton = dialog.getByRole('button');
  await expect(closeButton).toBeFocused();
  // Hitting tab should keep focus on the close button
  await page.keyboard.press('Tab');
  await expect(closeButton).toBeFocused();
  // Hitting escape should close the dialog
  await page.keyboard.press('Escape');
  // Assert the dialog can no longer be found
  await expect(dialog).toHaveCount(0);

  // Reopen the dialog
  await page.getByRole('button', { name: 'Show Dialog' }).click();
  // Assert the dialog is open again
  await expect(dialog).toBeVisible();
  // Click the close button
  await closeButton.click();
  // Assert the dialog is closed after clicking close
  await expect(dialog).toHaveCount(0);

  // Reopen the dialog
  await page.getByRole('button', { name: 'Show Dialog' }).click();
  await expect(dialog).toBeVisible();
  // Clicking far outside the dialog content should dismiss it.
  await page.mouse.click(2, 2);
  await expect(dialog).toHaveCount(0);
});

test('modal dialog marks background content inert', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=dialog&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes

  // Is the trigger (which sits behind the dialog) inside an inert subtree?
  const triggerIsInert = () => page.evaluate(() => {
    const trigger = Array.from(document.querySelectorAll('button')).find(
      (button) => button.textContent?.trim() === 'Show Dialog'
    );
    return trigger ? trigger.closest('[inert]') !== null : null;
  });

  const trigger = page.getByRole('button', { name: 'Show Dialog' });
  await expect(trigger).toBeVisible();
  await expect.poll(triggerIsInert).toBe(false);
  await trigger.click();
  await expect(page.getByRole('dialog')).toBeVisible();

  // Background content is inert while the modal is open, and every element that was made
  // inert records the dialog that did it.
  await expect.poll(triggerIsInert).toBe(true);
  await expect(page.locator('[data-inert-by]').first()).toBeAttached();
  // The dialog itself stays interactive.
  await expect(page.getByRole('dialog')).not.toHaveAttribute('inert', '');

  await page.keyboard.press('Escape');
  await expect(page.getByRole('dialog')).toHaveCount(0);

  // Closing unwinds everything it marked and returns focus to the opener.
  await expect.poll(triggerIsInert).toBe(false);
  await expect(page.locator('[data-inert-by]')).toHaveCount(0);
  await expect(trigger).toBeFocused();
});

// The stacking rules live in the focus trap module rather than in any one component, and two
// dialogs open at once has no demo to drive. Exercise the shipped bundle directly: the dialog
// page loads it, and `createFocusTrap` is the same entry point the primitive calls.
test('stacked focus traps compose and unwind independently', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=dialog&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.waitForFunction(() => typeof (window as any).createFocusTrap === 'function');

  const result = await page.evaluate(() => {
    // <fixture>
    //   <aside>          background for both dialogs
    //   <wrapper>        background for the first, ancestor of the second
    //     <filler>       background for the second only
    //     <second>
    //   <first>
    const html = `<div id="t-aside"></div>
      <div id="t-wrapper"><div id="t-filler"></div><div id="t-second"></div></div>
      <div id="t-first"></div>`;
    const fixture = document.createElement('div');
    fixture.id = 't-fixture';
    fixture.innerHTML = html;
    document.body.append(fixture);
    const el = (id: string) => document.getElementById(id)!;
    // `inert` is inherited, so an element is unreachable if it or any ancestor carries it.
    const blocked = (id: string) => el(id).closest('[inert]') !== null;

    const createFocusTrap = (window as any).createFocusTrap;
    const first = createFocusTrap(el('t-first'), { inertBackground: 'owner-first' });
    const onlyFirst = {
      aside: blocked('t-aside'),
      wrapper: blocked('t-wrapper'),
      second: blocked('t-second'),
      first: blocked('t-first'),
    };

    const second = createFocusTrap(el('t-second'), { inertBackground: 'owner-second' });
    const bothOpen = {
      aside: blocked('t-aside'),
      marker: el('t-aside').getAttribute('data-inert-by'),
      wrapper: blocked('t-wrapper'),
      filler: blocked('t-filler'),
      second: blocked('t-second'),
      first: blocked('t-first'),
    };

    second.remove();
    const afterSecond = {
      aside: blocked('t-aside'),
      marker: el('t-aside').getAttribute('data-inert-by'),
      wrapper: blocked('t-wrapper'),
      first: blocked('t-first'),
    };

    first.remove();
    const afterBoth = document.querySelectorAll('#t-fixture [inert], #t-fixture [data-inert-by]').length;

    fixture.remove();
    return { onlyFirst, bothOpen, afterSecond, afterBoth };
  });

  // One trap: everything outside it is inert, the trap itself is not.
  expect(result.onlyFirst).toEqual({ aside: true, wrapper: true, second: true, first: false });

  // Two traps: the one installed last is on top, so nothing on its path to <body> is inert —
  // and the one underneath becomes inert itself. Shared background records both owners.
  expect(result.bothOpen).toEqual({
    aside: true,
    marker: 'owner-first owner-second',
    wrapper: false,
    filler: true,
    second: false,
    first: true,
  });

  // Closing the top one hands the page back to the one underneath, not to the application.
  expect(result.afterSecond).toEqual({
    aside: true,
    marker: 'owner-first',
    wrapper: true,
    first: false,
  });

  // Closing both leaves nothing behind.
  expect(result.afterBoth).toBe(0);
});
