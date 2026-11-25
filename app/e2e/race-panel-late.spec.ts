// The original trapped-panel save stays a regression case alongside the staged T199 holds.
import { test, expect } from './fixtures';
import { openLateRace } from './lateRaceSave';
import { expectSidebarGlance, expectSidebarReachable } from './sidebarAssertions';
for (const viewport of [{ width: 1000, height: 600 }, { width: 1440, height: 900 }]) {
  test(`late race stays reachable ${viewport.width}`, async ({ page }) => {
    await page.setViewportSize(viewport);
    await openLateRace(page);
    // Rules 27+ cut the crowns, so this save's leader (Sparta, 7 VP) is no longer near the target and the rival alert row is absent;
    // the rivals table is what the panel must still keep whole (the race headline shows only while a team holds its target).
    await expect(page.getByRole('table', { name: 'Rivals' })).toBeVisible();
    await expectSidebarGlance(page);
    await expectSidebarReachable(page);
  });
}
