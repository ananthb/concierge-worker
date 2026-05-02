import { test, expect } from './_helpers/fixtures';

const PUBLIC_PAGES = [
  { path: '/', title: /Concierge/i },
  { path: '/features', title: /features/i },
  { path: '/pricing', title: /pricing/i },
  { path: '/auth/login', title: /sign in/i },
  { path: '/terms', title: /terms/i },
  { path: '/privacy', title: /privacy/i },
];

for (const { path, title } of PUBLIC_PAGES) {
  test(`${path} renders cleanly`, async ({ page, consoleErrors }) => {
    await page.goto(path);
    await expect(page).toHaveTitle(title);
    await expect(page.locator('main#main')).toBeVisible();
    await expect(page.locator('footer.site-footer')).toBeVisible();
    expect(consoleErrors).toEqual([]);
  });
}

test('public nav has Features / Pricing / Open source / Sign in (no Docs)', async ({ page }) => {
  await page.goto('/');
  const nav = page.locator('header.site-header nav.site-nav');
  await expect(nav.getByRole('link', { name: 'Features' })).toBeVisible();
  await expect(nav.getByRole('link', { name: 'Pricing' })).toBeVisible();
  await expect(nav.getByRole('link', { name: 'Sign in' })).toBeVisible();
  // Open source is in the DOM on every viewport; it's just `display:none`
  // below 760px (`.nav-ext`). Match the underlying anchor so the
  // assertion holds for both desktop and mobile projects.
  await expect(nav.locator('a[href*="github.com/ananthb/concierge"]')).toHaveCount(1);
  // Docs was deliberately removed from the top nav (still in the footer).
  await expect(nav.locator('a[href*="ananthb.github.io"]')).toHaveCount(0);
});

test('footer carries all seven links', async ({ page }) => {
  await page.goto('/');
  const footer = page.locator('footer.site-footer');
  for (const name of ['Features', 'Pricing', 'Docs', 'Open-source', 'AGPL-3.0', 'Terms of Service', 'Privacy Policy']) {
    await expect(footer.getByRole('link', { name })).toBeVisible();
  }
});

test('brand link returns home', async ({ page }) => {
  await page.goto('/pricing');
  const brand = page.locator('header.site-header a.brand');
  await expect(brand).toHaveAttribute('href', '/');
});
