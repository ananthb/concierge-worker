import { test as base, expect } from '@playwright/test';

/**
 * Auto-collected console errors / page errors / CSP violations.
 *
 * Every test gets a `consoleErrors` array populated as the page runs;
 * specs assert it's empty (or filter what they expect to ignore) at
 * the relevant point. CSP-related console messages are routed here
 * too so a fresh `'unsafe-inline'`-needing line in a template fails
 * the suite instead of slipping past unnoticed.
 */
export const test = base.extend<{ consoleErrors: string[] }>({
  consoleErrors: async ({ page }, use) => {
    const errors: string[] = [];
    page.on('pageerror', (e) => errors.push(`pageerror: ${e.message}`));
    page.on('console', (msg) => {
      const text = msg.text();
      const lower = text.toLowerCase();
      if (msg.type() === 'error') errors.push(`console: ${text}`);
      else if (lower.includes('content security policy') || lower.includes('refused to')) {
        errors.push(`csp: ${text}`);
      }
    });
    await use(errors);
  },
});

export { expect };
