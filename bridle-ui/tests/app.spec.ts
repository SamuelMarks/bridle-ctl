import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test.describe('Bridle UI End-to-End Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Mock the System Health API
    await page.route('**/api/health', async (route) => {
      const json = { rest: 'UP', rpc: 'UP', agent: 'UP' };
      await route.fulfill({ json });
    });

    // Mock Organizations API
    await page.route('**/api/orgs', async (route) => {
      const json = [
        { id: 'org-1', name: 'Acme Corp', provider: 'github' },
        { id: 'org-2', name: 'Open Source', provider: 'gitlab' },
      ];
      await route.fulfill({ json });
    });

    // Mock Jobs API
    await page.route('**/api/batch/jobs', async (route) => {
      const json = [
        {
          id: 'job-12345678',
          target: 'org-1',
          status: 'COMPLETED',
          createdAt: new Date().toISOString(),
        },
        {
          id: 'job-87654321',
          target: 'org-2',
          status: 'RUNNING',
          createdAt: new Date().toISOString(),
        },
      ];
      await route.fulfill({ json });
    });

    // Mock Repos API
    await page.route('**/api/orgs/org-1/repos', async (route) => {
      const json = [
        {
          id: 'repo-1',
          name: 'frontend',
          orgId: 'org-1',
          description: 'The UI',
          url: 'https://github.com/acme/frontend',
        },
      ];
      await route.fulfill({ json });
    });

    // Mock PRs API
    await page.route('**/api/prs?orgId=org-1', async (route) => {
      const json = [
        {
          id: 'pr-1',
          title: 'Fix bug',
          repoId: 'repo-1',
          status: 'LOCAL',
          createdAt: new Date().toISOString(),
        },
        {
          id: 'pr-2',
          title: 'Update deps',
          repoId: 'repo-1',
          status: 'SYNCED',
          createdAt: new Date().toISOString(),
        },
      ];
      await route.fulfill({ json });
    });

    // Mock Local Audit
    await page.route('**/api/local/audit', async (route) => {
      const json = { output: 'Found 2 matches in src/main.ts' };
      await route.fulfill({ json });
    });

    // Mock Local Fix
    await page.route('**/api/local/fix', async (route) => {
      const json = {
        output: 'Fixed 2 matches',
        diff: '--- a/src/main.ts\n+++ b/src/main.ts',
        modifiedFiles: ['src/main.ts'],
      };
      await route.fulfill({ json });
    });

    // Mock Dev Add
    await page.route('**/api/dev/add', async (route) => {
      const request = route.request();
      const body = await request.postDataJSON();
      const result = body.left + body.right;
      await route.fulfill({ json: { result } });
    });

    // Mock Dev DB
    await page.route('**/api/dev/db', async (route) => {
      const json = { status: 'success', rows: 42 };
      await route.fulfill({ json });
    });
  });

  test('Dashboard loads and displays system health', async ({ page }) => {
    await page.goto('/');

    // Check main title
    await expect(page.locator('h1')).toContainText('System Health Dashboard');

    // Check health badges are rendered
    const badges = page.locator('app-badge');
    await expect(badges).toHaveCount(3);
    await expect(badges.nth(0)).toContainText('UP');

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Organizations page lists orgs and repos', async ({ page }) => {
    await page.goto('/orgs');

    // Check tabs/header
    await expect(page.locator('h1')).toContainText(
      'Organizations & Repositories',
    );

    // Click on the first org link to load its repos
    const orgLink = page.getByRole('button', { name: 'Acme Corp' });
    if (await orgLink.isVisible()) {
      await orgLink.click();
    } else {
      // fallback if it's rendered as an anchor or text
      await page.locator('text=Acme Corp').first().click();
    }

    // Wait for repos to render and verify repo name is visible
    await expect(page.locator('text=frontend')).toBeVisible();
    await expect(page.locator('text=The UI')).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Batch Actions page displays jobs and allows toggling tabs', async ({
    page,
  }) => {
    await page.goto('/batch');

    await expect(page.locator('h1')).toContainText('Batch Actions');

    // Check job list rendered from the mock
    await expect(page.locator('text=job-1234')).toBeVisible();
    await expect(page.locator('text=COMPLETED')).toBeVisible();
    await expect(page.locator('text=RUNNING')).toBeVisible();

    // Test tabs
    const pipelineTab = page.getByRole('tab', { name: 'Pipeline Run' });
    await pipelineTab.click();

    // Check if the Pipeline Run form is visible
    await expect(
      page.locator('text=Pipeline Configuration (YAML)'),
    ).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('PR Sync page renders PR list and allows selection', async ({
    page,
  }) => {
    await page.goto('/prs');

    await expect(page.locator('h1')).toContainText(
      'Pull Requests Synchronization',
    );

    // Select an org from the dropdown
    await page.locator('select').first().selectOption('org-1');

    // Verify PRs from the mock are shown
    await expect(page.locator('text=Fix bug')).toBeVisible();
    await expect(page.locator('text=Update deps')).toBeVisible();

    // Verify Stats
    await expect(page.locator('text=Total PRs: 2')).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Local Ops page handles audit and fix workflows', async ({ page }) => {
    await page.goto('/local-ops');
    await page.waitForTimeout(500); // Wait for Angular hydration

    await expect(page.locator('h1')).toContainText('Local Operations');

    // Default tab should be Audit
    await expect(page.locator('h2', { hasText: 'Run Audit' })).toBeVisible();

    // Fill out and submit Audit form
    await page.getByLabel('Regex Pattern').fill('TODO.*');
    await page.getByRole('button', { name: 'Audit' }).click();

    // Check result
    await expect(page.locator('h2.Box-title')).toContainText('Audit Results');
    await expect(page.locator('pre.cli-output').first()).toContainText(
      'Found 2 matches in src/main.ts',
    );

    // Switch to Fix tab
    await page.getByRole('tab', { name: 'Fix' }).click();
    await expect(page.locator('h2', { hasText: 'Run Fix' })).toBeVisible();

    // Fill out and submit Fix form
    await page.getByLabel('Regex Pattern').fill('TODO.*');
    await page.getByRole('button', { name: 'Fix' }).click();

    // Check result
    await expect(page.locator('h2.Box-title')).toContainText('Fix Results');
    await expect(page.locator('pre.cli-output').first()).toContainText(
      'Fixed 2 matches',
    );
    await expect(
      page.locator('h3', { hasText: 'Modified Files' }),
    ).toBeVisible();
    await expect(page.locator('.file-list')).toContainText('src/main.ts');

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Developer Tools handles math and raw db commands', async ({ page }) => {
    await page.goto('/dev');
    await page.waitForTimeout(500); // Wait for Angular hydration

    await expect(page.locator('h1')).toContainText('Developer Tools');

    // Math Add utility
    await page.getByLabel('Left Integer').fill('5');
    await page.getByLabel('Right Integer').fill('7');
    await page.getByRole('button', { name: 'Add' }).click();

    // Check math result
    await expect(page.locator('.result-badge')).toContainText('Result: 12');

    // Raw DB Execution
    await page.getByLabel('Action').fill('read_orgs');
    await page.getByRole('button', { name: 'Execute Raw Command' }).click();

    // Check DB result
    await expect(page.locator('h3', { hasText: 'Result' })).toBeVisible();
    await expect(page.locator('pre.cli-output')).toContainText('success');

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });
});

test('Can ingest a new organization', async ({ page }) => {
  await page.route('**/api/orgs/ingest', async (route) => {
    const json = { id: 'org-new', name: 'new-org', provider: 'github' };
    await route.fulfill({ json });
  });

  await page.goto('/orgs');

  await page.getByLabel('Organization Name').fill('new-org');
  await page.getByLabel('Database URL').fill('postgres://user:pass@localhost:5432/db');
  await page.getByRole('button', { name: 'Ingest Org' }).click();

  await expect(page.locator('.Toast--success', { hasText: 'Organization new-org ingested successfully' })).toBeVisible();
});
