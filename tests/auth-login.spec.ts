import { test, expect } from './_helpers/fixtures';

/**
 * /auth/login behavioural tests.
 *
 * Covers:
 *  - All three brand buttons render (Google / Facebook / WhatsApp).
 *  - The WhatsApp click handler is wired via `addEventListener`, not
 *    inline `onclick=` (the pre-fix shape would have hit `FB is not
 *    defined` under our nonce-only CSP).
 *  - Clicking the button doesn't throw `FB is not defined` and does
 *    eventually call `FB.login` once the SDK is ready.
 */

test.beforeEach(async ({ page }) => {
  await page.goto('/auth/login');
});

test('shows Google, Facebook and WhatsApp brand buttons', async ({ page }) => {
  await expect(page.getByRole('link', { name: /continue with google/i })).toBeVisible();
  await expect(page.getByRole('link', { name: /continue with facebook/i })).toBeVisible();
  await expect(page.getByRole('button', { name: /continue with whatsapp/i })).toBeVisible();
});

test('WhatsApp button has no inline onclick attribute', async ({ page }) => {
  // Inline onclick would require `'unsafe-inline'` in script-src; the
  // click handler must be wired via addEventListener inside the nonced
  // module script.
  const btn = page.locator('#wa-signup-btn');
  const onclick = await btn.getAttribute('onclick');
  expect(onclick).toBeNull();
});

test('clicking WhatsApp does not throw "FB is not defined"', async ({ page, consoleErrors }) => {
  // Wait for SDK script to be present + give it a moment to load.
  await page.waitForFunction(() => typeof (window as any).FB === 'object');

  // Intercept FB.login so we don't navigate away — we only want to
  // confirm the handler reaches the SDK. Calling original after stub
  // also restores the cancel-error flow we care about.
  const calls = await page.evaluate(() => {
    const w = window as any;
    w.__fbLoginCalls = 0;
    const original = w.FB.login;
    w.FB.login = (cb: (r: any) => void) => {
      w.__fbLoginCalls += 1;
      cb({ authResponse: null }); // simulates "user cancelled"
    };
    return { stubbed: typeof original === 'function' };
  });
  expect(calls.stubbed).toBe(true);

  await page.click('#wa-signup-btn');
  await page.waitForTimeout(500);

  const fbLoginCalls = await page.evaluate(() => (window as any).__fbLoginCalls);
  expect(fbLoginCalls, 'FB.login should have been invoked once').toBe(1);

  const errorBlocking = consoleErrors.filter((e) => /FB is not defined/.test(e));
  expect(errorBlocking).toEqual([]);
});

test('cancelled login restores the button and shows the error', async ({ page }) => {
  await page.waitForFunction(() => typeof (window as any).FB === 'object');
  await page.evaluate(() => {
    (window as any).FB.login = (cb: (r: any) => void) => cb({ authResponse: null });
  });

  const btn = page.locator('#wa-signup-btn');
  const errDiv = page.locator('#wa-signup-error');
  await btn.click();
  await expect(errDiv).not.toHaveText('');
  // Button should have returned to enabled state.
  await expect(btn).toBeEnabled();
});
