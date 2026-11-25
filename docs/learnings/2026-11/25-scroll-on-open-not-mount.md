# A dialog's scroll position belongs to its opening, not its mount (2026-11-25)

The tech tree set `scrollLeft` to the player's era in a `useLayoutEffect` keyed on the era. The research dialog
stays mounted while closed, and a closed `<dialog>` has no layout, so the scroll ran at mount with nothing to
scroll and never ran again: the tree always opened on the Ancient era. A spec in the Medieval era passed only
because that column happened to fit at scroll 0.

Fix: key the effect on the dialog's open state too, and scroll again in the next animation frame once
`showModal` has laid the dialog out. Test it with a late-era fixture whose column is off the first screen, and by
closing and reopening after scrolling away (`e2e/research-tree.spec.ts`).

Related: the advisor's scroll-into-view only checked the window, so a target cut off by its own panel's scroll
area (a tree node in the bottom row) kept its ring half under the advisor strip. `Advisor.tsx` now also checks
the nearest `.tree-scroll` box.
