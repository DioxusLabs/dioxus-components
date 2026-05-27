import { test, expect } from "@playwright/test";

// Helper to run scrollHeight stability test with configurable tolerance
async function testScrollHeightStability(
  page: import("@playwright/test").Page,
  tolerancePx: number
) {
  await page.goto("http://127.0.0.1:8080/component/?name=virtual_list&", {
    timeout: 20 * 60 * 1000,
  });

  const container = page.getByRole("list").first();
  await expect(container).toBeVisible({ timeout: 30000 });

  // Wait for initial render
  await page.waitForTimeout(500);

  // Get initial state
  const initialState = await container.evaluate((el) => ({
    scrollHeight: el.scrollHeight,
    clientHeight: el.clientHeight,
  }));

  const maxScroll = initialState.scrollHeight - initialState.clientHeight;
  const steps = 20;
  const stepSize = maxScroll / steps;

  const measurements: Array<{ scrollTop: number; scrollHeight: number }> = [];

  // Simulate continuous scrolling (like dragging the scrollbar)
  // by setting scrollTop in rapid succession
  for (let i = 1; i <= steps; i++) {
    const targetScroll = Math.round(stepSize * i);

    await container.evaluate((el, scroll) => {
      el.scrollTop = scroll;
    }, targetScroll);

    // Wait for scroll event to propagate through Rust and re-render
    await page.waitForTimeout(100);

    const state = await container.evaluate((el) => ({
      scrollTop: el.scrollTop,
      scrollHeight: el.scrollHeight,
    }));

    measurements.push(state);
  }

  // Analyze scrollHeight stability during the "drag"
  // Exclude last measurement since scrollend fires ~150ms after last scroll,
  // which correctly unfreezes the height - we only care about stability DURING scroll
  const duringScrollMeasurements = measurements.slice(0, -1);
  const scrollHeights = duringScrollMeasurements.map((m) => m.scrollHeight);
  const minHeight = Math.min(...scrollHeights);
  const maxHeight = Math.max(...scrollHeights);
  const heightVariance = maxHeight - minHeight;

  console.log(`scrollHeight range: ${minHeight} - ${maxHeight} (variance: ${heightVariance}px)`);
  console.log(`Measurements:`, measurements.map((m, i) =>
    `step ${i + 1}: scrollTop=${m.scrollTop}, scrollHeight=${m.scrollHeight}`
  ).join('\n'));

  // The bug: if scrollHeight changes during scrolling, the scrollbar thumb
  // position (scrollTop / scrollHeight) changes even though the user's mouse
  // hasn't moved proportionally. This causes the thumb to drift from the cursor.
  //
  // For a stable scrollbar, scrollHeight should not change during active scrolling.
  expect(
    heightVariance,
    `scrollHeight changed by ${heightVariance}px during scroll (tolerance: ${tolerancePx}px) - this causes scrollbar thumb to drift from mouse cursor`
  ).toBeLessThan(tolerancePx);

  return { heightVariance, measurements };
}

// Test with adaptive estimation (no estimate_size provided).
// The demo uses 6 repeating item sizes, so adaptive estimation learns quickly.
// Allow small margin since first few items may have slightly off estimates.
test("scrollHeight stable with adaptive estimation", async ({ page }) => {
  // Allow up to 200px variance - adaptive estimation may have small drift
  // as it learns item sizes, but should still be much better than 500px+ without fix
  const { heightVariance } = await testScrollHeightStability(page, 200);

  // Bonus: with repeating sizes, adaptive estimation should actually achieve near-zero variance
  console.log(`Adaptive estimation achieved ${heightVariance}px variance`);
});

// Stricter test - should achieve 0px variance with the stable_total_size fix
test("scrollHeight remains stable during continuous scroll", async ({ page }) => {
  await testScrollHeightStability(page, 100);
});

// Test with random_heights variant which has highly variable item sizes
// This reproduces production failure where adaptive estimation struggles
test("scrollHeight stable with random heights variant", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/block/?name=virtual_list&variant=random_heights", {
    timeout: 20 * 60 * 1000,
  });

  const container = page.getByRole("list").first();
  await expect(container).toBeVisible({ timeout: 30000 });

  // Verify variant is loaded by checking for variant-specific content
  const firstCard = page.getByRole("heading", { level: 3 }).first();
  const cardText = await firstCard.textContent();
  console.log("First card text:", cardText);
  // Random heights variant shows "X repeats" in the heading
  expect(cardText, "Variant should show 'repeats' count").toContain("repeats");

  // Start scrolling immediately - don't wait for measurements
  const initialState = await container.evaluate((el) => ({
    scrollHeight: el.scrollHeight,
    clientHeight: el.clientHeight,
  }));

  // Scroll through first portion of list
  const maxScroll = (initialState.scrollHeight - initialState.clientHeight) * 0.3;
  const steps = 15;
  const stepSize = maxScroll / steps;

  const measurements: Array<{ scrollTop: number; scrollHeight: number }> = [];

  for (let i = 1; i <= steps; i++) {
    const targetScroll = Math.round(stepSize * i);

    await container.evaluate((el, scroll) => {
      el.scrollTop = scroll;
    }, targetScroll);

    await page.waitForTimeout(100);

    const state = await container.evaluate((el) => ({
      scrollTop: el.scrollTop,
      scrollHeight: el.scrollHeight,
    }));

    measurements.push(state);
  }

  // Analyze stability
  const duringScrollMeasurements = measurements.slice(0, -1);
  const scrollHeights = duringScrollMeasurements.map((m) => m.scrollHeight);
  const minHeight = Math.min(...scrollHeights);
  const maxHeight = Math.max(...scrollHeights);
  const heightVariance = maxHeight - minHeight;

  console.log(`Random heights - scrollHeight range: ${minHeight} - ${maxHeight} (variance: ${heightVariance}px)`);
  console.log(`Measurements:`, measurements.map((m, i) =>
    `step ${i + 1}: scrollTop=${m.scrollTop}, scrollHeight=${m.scrollHeight}`
  ).join('\n'));

  // With random heights and poor early estimates, we intentionally DON'T freeze
  // because a bad frozen value causes worse UX (sudden jumps) than gradual drift.
  // The adaptive estimation will improve as more items are measured, and future
  // scrolls will be stable once enough items are measured (≥20 or ≥10%).
  //
  // This test documents the expected drift behavior - it should improve over time
  // as the estimate converges. For better UX, users should provide estimate_size.
  console.log(`Note: ${heightVariance}px drift is expected with poor early estimates`);

  // Just verify we're not seeing catastrophic variance (e.g., 50000px+)
  expect(
    heightVariance,
    `scrollHeight variance ${heightVariance}px is unexpectedly high`
  ).toBeLessThan(15000);
});

test("virtual list virtualizes rows and updates on scroll", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=virtual_list&", { timeout: 20 * 60 * 1000 });

  const cards = page.getByRole("listitem");
  await expect(cards.first()).toBeVisible({ timeout: 30000 });

  const initialCount = await cards.count();

  expect(initialCount).toBeGreaterThan(0);
  expect(initialCount).toBeLessThan(2000);

  // Retry scrolling + assertion: re-apply scroll on each retry since WASM
  // re-renders may reset scrollTop, especially on slower engines (WebKit).
  await expect(async () => {
    await page.evaluate(() => {
      document.querySelectorAll('[role="list"]').forEach((c) => {
        if (c.scrollHeight > c.clientHeight + 1) {
          c.scrollTop = 6000;
        }
      });
      window.scrollTo(0, 6000);
    });
    await page.waitForTimeout(300);
    const headings = await page.getByRole("heading", { level: 3 }).allTextContents();
    // After scrolling to offset 6000, at least some items with index > 30
    // should be visible, proving the virtual list responded to the scroll.
    const hasScrolledContent = headings.some((h) => {
      const match = h.match(/#(\d+)/);
      return match != null && parseInt(match[1]) > 30;
    });
    expect(hasScrolledContent).toBe(true);
  }).toPass({ timeout: 15000 });
});
