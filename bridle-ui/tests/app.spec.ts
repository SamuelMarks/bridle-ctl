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
        },
        {
          id: 'pr-2',
          title: 'Update deps',
          repoId: 'repo-1',
          status: 'SYNCED',
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
        diff: `--- a/src/main.ts
+++ b/src/main.ts`,
        modifiedFiles: ['src/main.ts'],
      };
      await route.fulfill({ json });
    });

    // Mock Dev Add
    await page.route('**/api/dev/add', async (route) => {
      const request = route.request();
      const body = await request.postDataJSON();
      const result = body.a + body.b;
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

    await expect(page.locator('h1')).toContainText('Dashboard');

    const badges = page.locator('app-badge');
    await expect(badges).toHaveCount(3);
    await expect(badges.nth(0)).toContainText('UP');

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Organizations page lists orgs and repos', async ({ page }) => {
    await page.goto('/orgs');

    await expect(page.locator('h1')).toContainText(
      'Organizations & Repositories',
    );

    const orgLink = page.getByRole('button', { name: 'Acme Corp' });
    await orgLink.click();

    await expect(page.locator('text=frontend')).toBeVisible();
    await expect(page.locator('text=The UI')).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Batch Actions page displays jobs and allows toggling tabs', async ({
    page,
  }) => {
    await page.goto('/batch');

    await expect(page.locator('h1')).toContainText('Batch Actions & Pipelines');

    await expect(page.locator('text=job-12345678')).toBeVisible();
    await expect(page.locator('text=COMPLETED')).toBeVisible();
    await expect(page.locator('text=RUNNING')).toBeVisible();

    const pipelineTab = page.getByRole('tab', { name: 'Run Pipeline' });
    await pipelineTab.click();

    await expect(page.locator('text=Pipeline Configuration')).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('PR Sync page renders PR list and allows selection', async ({
    page,
  }) => {
    await page.goto('/prs');

    await expect(page.locator('h1')).toContainText('PR Synchronization');

    await page.locator('select').first().selectOption('org-1');

    await expect(page.locator('text=Fix bug')).toBeVisible();
    await expect(page.locator('text=Update deps')).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Local Ops page handles audit and fix workflows', async ({ page }) => {
    await page.goto('/local-ops');
    await page.waitForTimeout(500);

    await expect(page.locator('h1')).toContainText('Local Operations');

    await expect(
      page.locator('h2', { hasText: 'Audit Configuration' }),
    ).toBeVisible();

    await page.getByLabel('Target Regex Pattern').fill('TODO.*');
    await page.getByRole('button', { name: 'Run Audit' }).click();

    await expect(
      page.locator('h2.Box-title', { hasText: 'Audit Output' }),
    ).toBeVisible();
    await expect(page.locator('pre.terminal-output').first()).toContainText(
      'Found 2 matches in src/main.ts',
    );

    await page.getByRole('tab', { name: 'Fix Issues' }).click();
    await expect(
      page.locator('h2', { hasText: 'Fix Configuration' }),
    ).toBeVisible();

    await page.getByLabel('Target Regex Pattern').fill('TODO.*');
    await page.getByRole('button', { name: 'Run Fix' }).click();

    await expect(
      page.locator('h2.Box-title', { hasText: 'Fix Output & Diff' }),
    ).toBeVisible();
    await expect(page.locator('pre.terminal-output').first()).toContainText(
      'Fixed 2 matches',
    );
    await expect(page.locator('text=Modified Files:')).toBeVisible();
    await expect(page.locator('.file-list')).toContainText('src/main.ts');

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Developer Tools handles math and raw db commands', async ({ page }) => {
    await page.goto('/dev');
    await page.waitForTimeout(500);

    await expect(page.locator('h1')).toContainText(
      'Developer Tools & Database',
    );

    await page.getByLabel('First Number (a)').fill('5');
    await page.getByLabel('Second Number (b)').fill('7');
    await page.getByRole('button', { name: 'Calculate Sum' }).click();

    await expect(page.locator('.result-box')).toContainText('Result: 12');

    await page.getByRole('tab', { name: 'DB Explorer' }).click();
    await page.getByRole('button', { name: 'Execute DB Command' }).click();

    await expect(page.locator('pre.json-output')).toContainText('success');

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('Can ingest a new organization', async ({ page }) => {
    await page.route('**/api/orgs/ingest', async (route) => {
      const json = { id: 'org-new', name: 'new-org', provider: 'github' };
      await route.fulfill({ json });
    });

    await page.goto('/orgs');

    await page.getByLabel('Organization Name').fill('new-org');
    await page
      .getByLabel('Database URL')
      .fill('postgres://user:pass@localhost:5432/db');
    await page.getByRole('button', { name: 'Ingest Org' }).click();

    await expect(
      page.locator('.Toast--success', {
        hasText: 'Organization new-org ingested successfully',
      }),
    ).toBeVisible();
  });
});
