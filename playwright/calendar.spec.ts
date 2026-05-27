import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  }); // Increase timeout to 20 minutes
  await page.waitForLoadState('networkidle');

  const calendar = page.locator("#component-preview-frame").first();
  const prevButton = calendar.getByRole("button").first();
  const nextButton = calendar.getByRole("button").nth(1);

  // Assert the calendar is displayed
  await expect(calendar).toBeVisible({ timeout: 30000 });
  // Assert the current month is displayed
  const currentMonth = calendar.locator("select").first();
  let currentMonthText = await currentMonth.inputValue();

  // Click the previous button to go to the previous month
  await prevButton.click();
  // Assert the month has changed
  let previousMonthText = await currentMonth.inputValue();
  expect(previousMonthText).not.toBe(currentMonthText);

  // Click the next button to go back to the current month
  await nextButton.click();
  // Assert the month has changed back to the current month
  await expect(currentMonth).toHaveValue(currentMonthText);

  // Move focus to the calendar with tab
  await page.keyboard.press("Tab");
  const focusedDay = calendar.locator(
    '[data-month="current"]:focus'
  );
  // Assert a day is focused
  const firstDay = focusedDay.first();
  // Get the day
  const day = await firstDay.textContent();
  const dayNumber = parseInt(day || "", 10);
  // Pressing right arrow should move focus to the next day
  await page.keyboard.press("ArrowRight");
  const nextDay = focusedDay.first();
  // Assert the next day is focused
  const nextDayNumber = parseInt((await nextDay.textContent()) || "", 10);
  let current_date = new Date();
  let daysInMonth = new Date(
    current_date.getFullYear(),
    current_date.getMonth() + 1,
    0
  ).getDate();
  if (dayNumber + 1 > daysInMonth) {
    // If the next day is in the next month, it should wrap around
    expect(nextDayNumber).toBe(1);
  } else {
    expect(nextDayNumber).toBe(dayNumber + 1);
  }
  // Pressing left arrow should move focus back to the original day
  await page.keyboard.press("ArrowLeft");
  await expect(focusedDay.first()).toContainText(day || "failure");
  // Pressing down arrow should move focus to the next week
  await page.keyboard.press("ArrowDown");
  const nextWeekDay = focusedDay.first();
  // Assert the next week day is focused
  const nextWeekDayNumber = parseInt(
    (await nextWeekDay.textContent()) || "",
    10
  );
  if (dayNumber + 7 > daysInMonth) {
    // If the next week day is in the next month, it should wrap around
    expect(nextWeekDayNumber).toBe(dayNumber + 7 - daysInMonth);
  } else {
    expect(nextWeekDayNumber).toBe(dayNumber + 7);
  }
  // Pressing up arrow should move focus back to the original day
  await page.keyboard.press("ArrowUp");
  await expect(focusedDay.first()).toContainText(day || "failure");
});

test("year navigation by moving 52 weeks with arrow keys", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  });

  const calendar = page.locator("#component-preview-frame").first();
  const monthSelect = calendar.locator("select").first();
  const yearSelect = calendar.locator("select").nth(1);

  // Assert the calendar is displayed
  await expect(calendar).toBeVisible({ timeout: 30000 });

  // Get the initial month and year
  const initialMonth = await monthSelect.inputValue();
  const initialYear = await yearSelect.inputValue();
  const initialYearNumber = parseInt(initialYear, 10);
  const initialMonthNumber = parseInt(initialMonth, 10);

  // Calculate the exact number of weeks needed to move to the next year
  // Start from the first day of the current month
  const startDate = new Date(initialYearNumber, initialMonthNumber - 1, 1);
  // Calculate the same date next year
  const targetDate = new Date(initialYearNumber + 1, initialMonthNumber - 1, 1);
  // Calculate the difference in days
  const daysDifference = Math.floor(
    (targetDate.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24)
  );
  // Calculate the number of weeks (round to nearest week)
  const weeksToMove = Math.ceil(daysDifference / 7);

  // Move focus to the calendar manually
  const firstDay = calendar
    .locator('[data-month="current"]')
    .first();
  await firstDay.focus();

  // Press ArrowDown the calculated number of times to move forward by one year
  for (let i = 0; i < weeksToMove; i++) {
    await page.keyboard.press("ArrowDown");
  }

  // Assert the year has changed to the next year
  const nextYear = await yearSelect.inputValue();
  const nextYearNumber = parseInt(nextYear, 10);
  expect(nextYearNumber).toBe(initialYearNumber + 1);

  // The month should be exactly the same
  const nextMonth = await monthSelect.inputValue();
  expect(nextMonth).toBe(initialMonth);

  // Press ArrowUp the same number of times to move back by one year
  for (let i = 0; i < weeksToMove; i++) {
    await page.keyboard.press("ArrowUp");
  }

  // Assert the year has changed back to the original year
  const currentYear = await yearSelect.inputValue();
  expect(currentYear).toBe(initialYear);

  // The month should be exactly the same
  const currentMonth = await monthSelect.inputValue();
  expect(currentMonth).toBe(initialMonth);
});

test("shift + arrow keys navigation", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  });

  const calendar = page.locator("#component-preview-frame").first();
  const monthSelect = calendar.locator("select").first();
  const yearSelect = calendar.locator("select").nth(1);

  // Assert the calendar is displayed
  await expect(calendar).toBeVisible({ timeout: 30000 });

  // Get the initial month and year
  const initialMonth = await monthSelect.inputValue();
  const initialYear = await yearSelect.inputValue();
  const initialYearNumber = parseInt(initialYear, 10);
  const initialMonthNumber = parseInt(initialMonth, 10);

  // Move focus to the calendar
  const firstDay = calendar
    .locator('[data-month="current"]')
    .first();
  await firstDay.focus();

  // Test Shift + ArrowDown - should move forward by one month
  await page.keyboard.press("Shift+ArrowDown");

  let currentMonth = await monthSelect.inputValue();
  let currentYear = await yearSelect.inputValue();
  let expectedMonth = initialMonthNumber === 12 ? 1 : initialMonthNumber + 1;
  let expectedYear =
    initialMonthNumber === 12 ? initialYearNumber + 1 : initialYearNumber;

  expect(parseInt(currentMonth, 10)).toBe(expectedMonth);
  expect(parseInt(currentYear, 10)).toBe(expectedYear);

  // Test Shift + ArrowUp - should move back to the initial month
  await page.keyboard.press("Shift+ArrowUp");

  currentMonth = await monthSelect.inputValue();
  currentYear = await yearSelect.inputValue();
  expect(currentMonth).toBe(initialMonth);
  expect(currentYear).toBe(initialYear);
});

async function testArrowKeyNavigation(
  page: any,
  arrowKey: "ArrowRight" | "ArrowLeft",
  startPosition: "first" | "last",
  expectedOrder: "ascending" | "descending"
) {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  });

  const calendar = page.locator("#component-preview-frame").first();
  const monthSelect = calendar.locator("select").first();
  const yearSelect = calendar.locator("select").nth(1);

  // Assert the calendar is displayed
  await expect(calendar).toBeVisible({ timeout: 30000 });

  // Get the current month and year to calculate days in month
  const currentMonthValue = await monthSelect.inputValue();
  const currentYearValue = await yearSelect.inputValue();
  const monthNumber = parseInt(currentMonthValue, 10);
  const yearNumber = parseInt(currentYearValue, 10);

  // Calculate the number of days in the current month
  const daysInMonth = new Date(yearNumber, monthNumber, 0).getDate();

  // Move focus to the starting day of the current month
  const startDay = calendar
    .locator('[data-month="current"]')
    [startPosition]();
  await startDay.focus();

  // Get the focused day selector
  const focusedDay = calendar.locator(
    '[data-month="current"]:focus'
  );

  // Array to track all days visited
  const daysVisited = [];

  // Get the starting day number
  let dayText = await focusedDay.first().textContent();
  let dayNumber = parseInt(dayText || "", 10);
  daysVisited.push(dayNumber);

  // Press arrow key to navigate through all remaining days of the month
  for (let i = 1; i < daysInMonth; i++) {
    await page.keyboard.press(arrowKey);

    // Get the new focused day
    dayText = await focusedDay.first().textContent();
    dayNumber = parseInt(dayText || "", 10);
    daysVisited.push(dayNumber);
  }

  // Assert that we visited the correct number of days
  expect(daysVisited.length).toBe(daysInMonth);

  // Sort the days visited to check we got all days from 1 to daysInMonth
  const sortedDays = [...daysVisited].sort((a, b) => a - b);

  // Create the expected array [1, 2, 3, ..., daysInMonth]
  const expectedDays = Array.from({ length: daysInMonth }, (_, i) => i + 1);

  // Assert that we visited every day exactly once
  expect(sortedDays).toEqual(expectedDays);

  // Verify we traversed in the expected order
  if (expectedOrder === "ascending") {
    expect(daysVisited).toEqual(expectedDays);
  } else {
    const expectedReverseDays = Array.from(
      { length: daysInMonth },
      (_, i) => daysInMonth - i
    );
    expect(daysVisited).toEqual(expectedReverseDays);
  }
}

test("right arrow key navigates through all days of the month", async ({
  page,
}) => {
  await testArrowKeyNavigation(page, "ArrowRight", "first", "ascending");
});

test("left arrow key navigates through all days of the month in reverse", async ({
  page,
}) => {
  await testArrowKeyNavigation(page, "ArrowLeft", "last", "descending");
});
