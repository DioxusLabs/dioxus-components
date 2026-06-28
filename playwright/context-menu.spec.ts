import { test, expect } from '@playwright/test';

test('pointer navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('button', { name: 'right click here' }).click({
    button: 'right'
  });

  // Assert the context menu is visible
  const contextMenu = page.getByRole('menu');
  await expect(contextMenu).toHaveAttribute('data-state', 'open');
  // Click on the "Edit" menu item
  await page.getByRole('menuitem', { name: 'Edit' }).click();
  // Assert the context menu is closed after clicking
  await expect(contextMenu).toHaveCount(0);
});

test('menu lands at the tap coordinates on touch long-press', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });
  // Push the trigger down so the tap point isn't at viewport (0, 0) — any
  // misalignment will then have a non-zero direction to detect.
  await page.evaluate(() => {
    const main = document.querySelector('main') ?? document.body;
    (main as HTMLElement).style.paddingTop = '300px';
    (main as HTMLElement).style.paddingLeft = '120px';
  });

  const trigger = page.getByRole('button', { name: 'right click here' });
  const contextMenu = page.getByRole('menu');
  const box = await trigger.boundingBox();
  if (!box) throw new Error('trigger has no bounding box');
  const tapX = box.x + box.width / 2;
  const tapY = box.y + box.height / 2;
  const pointerId = 7777;

  await trigger.evaluate((el, { x, y, pointerId }) => {
    el.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      button: 0,
      buttons: 1,
      bubbles: true,
      cancelable: true,
    }));
  }, { x: tapX, y: tapY, pointerId });

  await expect(contextMenu).toHaveAttribute('data-state', 'open');
  const menuBox = await contextMenu.boundingBox();
  if (!menuBox) throw new Error('menu has no bounding box');
  // The menu's top-left should be at the tap coords (give or take a px for
  // sub-pixel rounding). If it's off by tens of pixels, a viewport coord
  // system is mismatched somewhere.
  expect(Math.abs(menuBox.x - tapX)).toBeLessThan(2);
  expect(Math.abs(menuBox.y - tapY)).toBeLessThan(2);
});


test('touch long-press opens the context menu', async ({ page }) => {
  // iOS Safari does not fire `contextmenu` on long press, so the menu must
  // open from a held touch instead. Reproduces issue #262.
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });
  const contextMenu = page.getByRole('menu');

  const box = await trigger.boundingBox();
  if (!box) throw new Error('trigger has no bounding box');
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 4242;

  await trigger.evaluate((el, { x, y, pointerId }) => {
    el.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      button: 0,
      buttons: 1,
      bubbles: true,
      cancelable: true,
    }));
  }, { x, y, pointerId });

  await expect(contextMenu).toHaveAttribute('data-state', 'open');

  // Release the touch after the menu has opened; it should stay open.
  await trigger.evaluate((el, { x, y, pointerId }) => {
    el.dispatchEvent(new PointerEvent('pointerup', {
      pointerId,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      bubbles: true,
    }));
  }, { x, y, pointerId });

  await expect(contextMenu).toHaveAttribute('data-state', 'open');
});

test('pen long-press opens the context menu', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });
  const contextMenu = page.getByRole('menu');

  const box = await trigger.boundingBox();
  if (!box) throw new Error('trigger has no bounding box');
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 4244;

  await trigger.evaluate((el, { x, y, pointerId }) => {
    el.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId,
      pointerType: 'pen',
      isPrimary: true,
      clientX: x,
      clientY: y,
      button: 0,
      buttons: 1,
      bubbles: true,
      cancelable: true,
    }));
  }, { x, y, pointerId });

  await expect(contextMenu).toHaveAttribute('data-state', 'open');
});

test('mouse pointerdown does not arm the long-press timer', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });

  const box = await trigger.boundingBox();
  if (!box) throw new Error('trigger has no bounding box');
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 4245;

  await trigger.evaluate((el, { x, y, pointerId }) => {
    el.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId,
      pointerType: 'mouse',
      isPrimary: true,
      clientX: x,
      clientY: y,
      button: 0,
      buttons: 1,
      bubbles: true,
      cancelable: true,
    }));
  }, { x, y, pointerId });

  // Hold past the long-press threshold; the menu must remain closed because
  // mouse pointers should only open via the native `contextmenu` event.
  await page.waitForTimeout(700);
  await expect(page.getByRole('menu')).toHaveCount(0);
});

test('touch tap outside closes the open menu', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });
  const contextMenu = page.getByRole('menu');

  await trigger.click({ button: 'right' });
  await expect(contextMenu).toHaveAttribute('data-state', 'open');

  // Tap near the bottom-right of the viewport, well outside the menu.
  const viewport = page.viewportSize();
  if (!viewport) throw new Error('no viewport');
  const farX = viewport.width - 10;
  const farY = viewport.height - 10;
  await page.evaluate(({ x, y }) => {
    const target = document.elementFromPoint(x, y);
    if (!target) throw new Error('no element at outside point');
    target.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId: 5050,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      button: 0,
      buttons: 1,
      bubbles: true,
      cancelable: true,
    }));
  }, { x: farX, y: farY });

  await expect(contextMenu).toHaveCount(0);
});

test('pointerdown at the trigger location does not dismiss an open menu', async ({ page }) => {
  // Regression for the long-press dismiss bug: on iOS Safari a fresh
  // pointerdown could land at the original touch coordinates right after the
  // menu opened (either from a topology-change re-dispatch under the active
  // touch, or from compat-mouse promotion). The dismiss listener must treat
  // the trigger as "inside" the menu's root and ignore it.
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });
  const contextMenu = page.getByRole('menu');

  await trigger.click({ button: 'right' });
  await expect(contextMenu).toHaveAttribute('data-state', 'open');

  await trigger.evaluate((el) => {
    const triggerRect = el.getBoundingClientRect();
    if (triggerRect.width === 0 || triggerRect.height === 0) {
      throw new Error('trigger has no bounding box');
    }

    const x = triggerRect.left + triggerRect.width / 2;
    const y = triggerRect.top + triggerRect.height / 2;

    if (
      x < triggerRect.left ||
      x > triggerRect.right ||
      y < triggerRect.top ||
      y > triggerRect.bottom
    ) {
      throw new Error('point is outside trigger bounds');
    }

    const root = el.parentElement;
    if (!root) throw new Error('trigger has no root');
    const rootRect = root.getBoundingClientRect();
    if (x < rootRect.left || x > rootRect.right || y < rootRect.top || y > rootRect.bottom) {
      throw new Error('point is outside context menu root bounds');
    }

    el.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId: 6060,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      button: 0,
      buttons: 1,
      bubbles: true,
      cancelable: true,
    }));
  });

  await expect(contextMenu).toHaveAttribute('data-state', 'open');
});

test('touch released before long-press threshold does not open the menu', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });

  const box = await trigger.boundingBox();
  if (!box) throw new Error('trigger has no bounding box');
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 4343;

  await trigger.evaluate((el, { x, y, pointerId }) => {
    el.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      button: 0,
      buttons: 1,
      bubbles: true,
      cancelable: true,
    }));
  }, { x, y, pointerId });

  // Quick tap — release well before the long-press threshold.
  await page.waitForTimeout(50);
  await trigger.evaluate((el, { x, y, pointerId }) => {
    el.dispatchEvent(new PointerEvent('pointerup', {
      pointerId,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      bubbles: true,
    }));
  }, { x, y, pointerId });

  // Wait past the long-press threshold; menu must remain closed.
  await page.waitForTimeout(700);
  await expect(page.getByRole('menu')).toHaveCount(0);
});

test('keyboard navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('button', { name: 'right click here' }).click({
    button: 'right'
  });

  // Assert the context menu is visible
  const contextMenu = page.getByRole('menu');
  await expect(contextMenu).toHaveAttribute('data-state', 'open');
  // Hit escape to close the context menu
  await page.keyboard.press('Escape');
  // Assert the context menu is closed after pressing escape
  await expect(contextMenu).toHaveCount(0);

  // Reopen the context menu
  await page.getByRole('button', { name: 'right click here' }).click({
    button: 'right'
  });
  await page.keyboard.press('ArrowDown');
  // Assert the "Edit" menu item is focused
  await expect(page.getByRole('menuitem', { name: 'Edit' })).toBeFocused();
  await expect(page.getByRole('menuitem', { name: 'Undo' })).toHaveAttribute('data-disabled', 'true');
  // Move down to the "Duplicate" menu item
  await page.keyboard.press('ArrowDown');
  // Assert the "Duplicate" menu item is focused
  await expect(page.getByRole('menuitem', { name: 'Duplicate' })).toBeFocused();
  // Hit Enter to select the "Duplicate" menu item
  await page.keyboard.press('Enter');
  // Assert the context menu is closed after selection
  await expect(contextMenu).toHaveCount(0);
  // Assert the selected item is displayed
  await expect(page.getByText('Selected: Duplicate')).toBeVisible();
});
