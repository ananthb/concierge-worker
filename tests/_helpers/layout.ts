import type { Page } from '@playwright/test';

export type LayoutIssues = string[];

/**
 * In-page layout sanity check, used by `layout.spec.ts` and (for
 * fail-fast diagnostics) by `visual.spec.ts`. Three independent
 * checks — each returns its findings as human-readable strings:
 *
 *   1. Page-wide horizontal overflow — `documentElement.scrollWidth`
 *      must not exceed the viewport. A wider doc means *something*
 *      is poking past the right edge and dragging in a horizontal
 *      scrollbar.
 *
 *   2. Culprit hunt — when (1) fires, walk every element and return
 *      the deepest leaf nodes whose right edge crosses the viewport.
 *      Reporting only leaves keeps us from blaming `<body>` for
 *      a stray button two layers down.
 *
 *   3. Header health — even when (1) is clean, an individual nav
 *      button getting clipped *or* the header growing taller than
 *      one row (because `flex-wrap` kicked in) reads as broken UI.
 *
 * Returns an empty array on success.
 */
export async function checkLayout(page: Page, viewportWidth: number): Promise<LayoutIssues> {
  return await page.evaluate((vw: number) => {
    const issues: string[] = [];
    const docWidth = document.documentElement.scrollWidth;
    if (docWidth > vw + 1) {
      issues.push(`page overflows viewport: scrollWidth=${docWidth} viewport=${vw}`);
      const all = document.body.getElementsByTagName('*');
      const culprits: string[] = [];
      for (const el of Array.from(all)) {
        const cs = getComputedStyle(el);
        if (cs.display === 'none' || cs.visibility === 'hidden') continue;
        const r = el.getBoundingClientRect();
        if (r.width === 0 && r.height === 0) continue;
        if (r.right <= vw + 1) continue;
        let hasOverflowingChild = false;
        for (const child of Array.from(el.getElementsByTagName('*'))) {
          const cr = child.getBoundingClientRect();
          if (cr.right > r.right - 0.5) { hasOverflowingChild = true; break; }
        }
        if (hasOverflowingChild) continue;
        const tag = el.tagName.toLowerCase();
        const cls =
          el.className && typeof el.className === 'string'
            ? '.' + el.className.trim().split(/\s+/).slice(0, 3).join('.')
            : '';
        const id = el.id ? '#' + el.id : '';
        const label = (el.textContent || '').trim().replace(/\s+/g, ' ').slice(0, 50);
        culprits.push(`${tag}${id}${cls} right=${r.right.toFixed(0)} text="${label}"`);
        if (culprits.length >= 5) break;
      }
      for (const c of culprits) issues.push(`  culprit: ${c}`);
    }

    const header = document.querySelector('.site-header');
    if (header) {
      const headerRect = header.getBoundingClientRect();
      if (headerRect.right > vw + 1) {
        issues.push(`.site-header right edge ${headerRect.right.toFixed(0)} exceeds viewport ${vw}`);
      }
      // Single-row site header is ~58–73px depending on breakpoint;
      // anything over 80px means the nav wrapped to two lines or each
      // button's text wrapped, which is the historical "broken header"
      // signature.
      if (headerRect.height > 80) {
        issues.push(`.site-header is ${headerRect.height.toFixed(0)}px tall — nav likely wrapping at width ${vw}`);
      }
      for (const btn of Array.from(header.querySelectorAll('a, button'))) {
        const cs = getComputedStyle(btn);
        if (cs.display === 'none' || cs.visibility === 'hidden') continue;
        const r = btn.getBoundingClientRect();
        if (r.width === 0 && r.height === 0) continue;
        if (r.right > vw + 1) {
          const label = (btn.textContent || '').trim().slice(0, 40);
          issues.push(`nav item "${label}" extends to ${r.right.toFixed(0)}px (viewport ${vw})`);
        }
      }
    }
    return issues;
  }, viewportWidth);
}
