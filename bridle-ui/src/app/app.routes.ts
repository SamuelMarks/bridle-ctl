import { Routes } from '@angular/router';

/** Main application routing definition */
export const routes: Routes = [
  { path: '', loadComponent: () => import('./features/dashboard/dashboard.component').then(m => m.DashboardComponent) },
  { path: 'orgs', loadComponent: () => import('./features/orgs/orgs-page/orgs-page.component').then(m => m.OrgsPageComponent) },
  { path: 'local-ops', loadComponent: () => import('./features/local-ops/local-ops-page/local-ops-page.component').then(m => m.LocalOpsPageComponent) },
  { path: 'batch', loadComponent: () => import('./features/batch/batch-actions-page/batch-actions-page.component').then(m => m.BatchActionsPageComponent) },
  { path: 'prs', loadComponent: () => import('./features/prs/pr-sync/pr-sync.component').then(m => m.PrSyncComponent) },
  { path: 'dev', loadComponent: () => import('./features/dev/dev-tools/dev-tools.component').then(m => m.DevToolsComponent) }
];
