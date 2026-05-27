import { test, expect, type Locator, type Page } from '@playwright/test';

async function sliderTrackPoint(track: Locator, frac: number) {
  const box = await track.boundingBox();
  if (!box) throw new Error('slider track has no bounding box');
  return { x: box.x + box.width * frac, y: box.y + box.height / 2 };
}

async function clickSliderTrack(page: Page, track: Locator, frac: number) {
  const point = await sliderTrackPoint(track, frac);
  await page.mouse.click(point.x, point.y);
}

function sliderGroup(page: Page, name: string | RegExp) {
  return page
    .getByRole('slider', { name })
    .first()
    .locator('xpath=ancestor::*[@role="group" and @data-orientation="horizontal"][1]');
}

function sliderTrack(slider: Locator) {
  return slider.locator('div[data-orientation="horizontal"]:has([role="slider"])').first();
}

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=slider&', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  const thumb = page.getByRole('slider', { name: 'Demo Slider' });
  // The initial aria-valuenow should be 50
  await expect(thumb).toHaveAttribute('aria-valuenow', '50');
  await thumb.focus();
  // The aria-valuenow should be 60 after pressing Shift+ArrowRight
  await page.keyboard.press('Shift+ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
  await page.keyboard.press('Shift+ArrowRight');
  // The aria-valuenow should be 70 after pressing Shift+ArrowRight again
  await expect(thumb).toHaveAttribute('aria-valuenow', '70');
  // Pressing Shift+ArrowLeft should decrease the value by 10
  await page.keyboard.press('Shift+ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
  // Pressing ArrowLeft should decrease the value by 1
  await page.keyboard.press('ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '59');
  // Pressing ArrowRight should increase the value by 1
  await page.keyboard.press('ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
});

test('drag survives pointercancel (iPad system gesture)', async ({ page }) => {
  // iPad can fire `pointercancel` (without a following `pointerup`) when an OS
  // gesture interrupts a drag. Regression: the slider didn't listen for
  // `pointercancel`, so its internal "active pointer" state stayed set and
  // every subsequent tap was ignored.
  await page.goto('http://127.0.0.1:8080/component/?name=slider&', { timeout: 20 * 60 * 1000 });
  const slider = sliderGroup(page, 'Demo Slider');
  const thumb = page.getByRole('slider', { name: 'Demo Slider' });

  await expect(thumb).toHaveAttribute('aria-valuenow', '50');

  const dispatchPointerDown = async (frac: number, pointerId: number) => {
    // Re-measure each time — focusing the thumb can scroll the page, which
    // shifts the slider's viewport coordinates between taps.
    const box = await sliderTrack(slider).boundingBox();
    if (!box) throw new Error('slider track has no bounding box');
    const x = box.x + box.width * frac;
    const y = box.y + box.height / 2;

    await slider.evaluate((el, { x, y, pointerId }) => {
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

    return { x, y };
  };

  // Start a drag so the slider records an active pointer.
  const firstPointerId = 99;
  const firstDrag = await dispatchPointerDown(0.5, firstPointerId);
  await expect(thumb).toHaveAttribute('data-dragging', 'true');

  // OS gesture: pointercancel fires without a matching pointerup. Dispatch it
  // through window so the shared pointer tracker observes the cancellation.
  await page.evaluate(({ x, y, pointerId }) => {
    window.dispatchEvent(new PointerEvent('pointercancel', {
      pointerId,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      bubbles: true,
      cancelable: true,
    }));
  }, { ...firstDrag, pointerId: firstPointerId });
  await expect(thumb).toHaveAttribute('data-dragging', 'false');

  // New tap at 70% with a fresh pointer should still update the slider.
  await clickSliderTrack(page, sliderTrack(slider), 0.7);
  await expect(thumb).toHaveAttribute('aria-valuenow', '70');
  await expect(thumb).toHaveAttribute('data-dragging', 'false');
  await expect(thumb).toHaveAttribute('aria-valuenow', '70');
});

test('drag ignores pageX/clientX mismatch (iPad pinch-zoom analog)', async ({ page }) => {
  // On iPad pinch-zoomed, `e.pageX` and `e.clientX` differ by the visual
  // viewport offset. The slider's onpointerdown stored client coords in the
  // global POINTERS table while the window pointermove listener wrote pageX —
  // so the very first pointermove produced a delta equal to that offset and
  // jammed the value at 100%. Reproduce by forging pageX on synthetic events.
  await page.goto('http://127.0.0.1:8080/component/?name=slider&', { timeout: 20 * 60 * 1000 });

  const slider = sliderGroup(page, 'Demo Slider');
  const thumb = page.getByRole('slider', { name: 'Demo Slider' });
  await expect(thumb).toHaveAttribute('aria-valuenow', '50');

  const box = await sliderTrack(slider).boundingBox();
  if (!box) throw new Error('slider has no bounding box');
  const x = box.x + box.width * 0.3;
  const y = box.y + box.height / 2;
  const pageOffset = 1000; // way larger than the slider width — would clamp to 100

  // Slider's onpointerdown reads client_coordinates, so push that. Firefox does
  // not reliably map this synthetic event to the exact track coordinate, so the
  // assertion below only depends on staying below the midpoint before and after
  // the forged pointermove.
  const pointerId = 51;
  await slider.evaluate((el, { x, y, pointerId }) => {
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
  await expect(thumb).toHaveAttribute('data-dragging', 'true');
  const before = await thumb.getAttribute('aria-valuenow');
  expect(parseInt(before!, 10)).toBeLessThan(50);

  // Prime the drag loop with matching client/page coordinates so the next move
  // measures the forged pageX delta.
  await page.evaluate(({ x, y, pointerId }) => {
    window.dispatchEvent(new PointerEvent('pointermove', {
      pointerId,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      bubbles: true,
    }));
  }, { x, y, pointerId });

  // Pointermove with clientX unchanged but pageX forged so it differs.
  // Mirrors what iPad sends when the visual viewport is offset from layout.
  await page.evaluate(({ x, y, pageOffset, pointerId }) => {
    const evt = new PointerEvent('pointermove', {
      pointerId,
      pointerType: 'touch',
      isPrimary: true,
      clientX: x,
      clientY: y,
      bubbles: true,
    });
    Object.defineProperty(evt, 'pageX', { value: x + pageOffset });
    Object.defineProperty(evt, 'pageY', { value: y });
    window.dispatchEvent(evt);
  }, { x, y, pageOffset, pointerId });

  // Without the fix the value jumps to ~100. With consistent coords it stays.
  const after = await thumb.getAttribute('aria-valuenow');
  expect(parseInt(after!, 10)).toBeLessThan(50);
});

test('dynamic min/max', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=dynamic_range&', { timeout: 20 * 60 * 1000 });
  const thumb = page.getByRole('slider', { name: 'Dynamic Range Slider' });

  // Initial state: percentage mode (0-100)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '100');

  // Switch to absolute number mode
  await page.getByRole('switch', { name: 'Percentage' }).click();

  // Should now be absolute mode (0-1000)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '1000');

  // Click back to percentage mode
  await page.getByRole('switch', { name: 'Percentage' }).click();

  // Should be back to percentage mode (0-100)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '100');
});

test('range two thumbs', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=range&', { timeout: 20 * 60 * 1000 });
  const thumbs = page.getByRole('slider', { name: 'Range Slider' });
  await expect(thumbs).toHaveCount(2);
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);

  // Initial values
  await expect(t0).toHaveAttribute('aria-valuenow', '20');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');

  // Per-thumb ARIA bounds reflect the live neighbor constraint
  await expect(t0).toHaveAttribute('aria-valuemax', '80');
  await expect(t1).toHaveAttribute('aria-valuemin', '20');

  // Keyboard nudges thumb 0
  await t0.focus();
  await page.keyboard.press('ArrowRight');
  await expect(t0).toHaveAttribute('aria-valuenow', '21');

  // Spam right past thumb 1's value — thumb 0 must clamp at 80, not cross
  for (let i = 0; i < 200; i++) await page.keyboard.press('ArrowRight');
  await expect(t0).toHaveAttribute('aria-valuenow', '80');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');
  // After clamping, thumb 1's lower bound moves up to thumb 0's value
  await expect(t1).toHaveAttribute('aria-valuemin', '80');
});

test('range thumbs recover from collision', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=range&', { timeout: 20 * 60 * 1000 });
  const thumbs = page.getByRole('slider', { name: 'Range Slider' });
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);

  // Drive both thumbs to 80
  await t0.focus();
  for (let i = 0; i < 200; i++) await page.keyboard.press('ArrowRight');
  await expect(t0).toHaveAttribute('aria-valuenow', '80');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');

  // Thumb 1 must still be movable up; once it does, thumb 0's max should follow
  await t1.focus();
  await page.keyboard.press('ArrowRight');
  await expect(t1).toHaveAttribute('aria-valuenow', '81');
  await expect(t0).toHaveAttribute('aria-valuemax', '81');

  // And thumb 0 must still be movable down
  await t0.focus();
  await page.keyboard.press('ArrowLeft');
  await expect(t0).toHaveAttribute('aria-valuenow', '79');
  await expect(t1).toHaveAttribute('aria-valuemin', '79');
});

test('range track click activates closest thumb', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=range&', { timeout: 20 * 60 * 1000 });
  const thumbs = page.getByRole('slider', { name: 'Range Slider' });
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);
  const slider = sliderGroup(page, 'Range Slider');
  const track = sliderTrack(slider);

  await expect(t0).toHaveAttribute('aria-valuenow', '20');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');

  // Click near the right edge — should activate thumb 1, jumping it close to 100
  await clickSliderTrack(page, track, 0.95);
  await expect(t0).toHaveAttribute('aria-valuenow', '20');
  await expect.poll(async () => Number(await t1.getAttribute('aria-valuenow'))).toBeGreaterThan(80);

  // Click near the left edge — should activate thumb 0, jumping it close to 0
  await clickSliderTrack(page, track, 0.05);
  await expect.poll(async () => Number(await t0.getAttribute('aria-valuenow'))).toBeLessThan(20);
});

test('range collided thumbs split by click direction', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=range&', { timeout: 20 * 60 * 1000 });
  const thumbs = page.getByRole('slider', { name: 'Range Slider' });
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);
  const slider = sliderGroup(page, 'Range Slider');
  const track = sliderTrack(slider);

  // Collide both thumbs at 80
  await t0.focus();
  for (let i = 0; i < 200; i++) await page.keyboard.press('ArrowRight');
  await expect(t0).toHaveAttribute('aria-valuenow', '80');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');

  // Clicking to the RIGHT of the collision must activate thumb 1 (not thumb 0,
  // which would otherwise win the distance tie and leave thumb 1 stranded).
  await clickSliderTrack(page, track, 0.95);
  await expect(t0).toHaveAttribute('aria-valuenow', '80');
  await expect.poll(async () => Number(await t1.getAttribute('aria-valuenow'))).toBeGreaterThan(80);
});

test('range collided thumbs drag left from just below collision', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=range&', { timeout: 20 * 60 * 1000 });
  const thumbs = page.getByRole('slider', { name: 'Range Slider' });
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);
  const slider = sliderGroup(page, 'Range Slider');
  const track = sliderTrack(slider);

  // Collide both thumbs at 80
  await t0.focus();
  for (let i = 0; i < 200; i++) await page.keyboard.press('ArrowRight');
  await expect(t0).toHaveAttribute('aria-valuenow', '80');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');

  // 79.6 snaps to 80. Thumb selection must still see the raw 79.6 position,
  // otherwise thumb 1 wins the tie and leftward dragging is clamped at 80.
  const start = await sliderTrackPoint(track, 0.796);
  const end = await sliderTrackPoint(track, 0.7);
  await page.mouse.move(start.x, start.y);
  await page.mouse.down();
  await page.mouse.move(end.x, end.y, { steps: 5 });
  await page.mouse.up();

  await expect(t0).toHaveAttribute('aria-valuenow', '70');
  await expect(t1).toHaveAttribute('aria-valuenow', '80');
});
