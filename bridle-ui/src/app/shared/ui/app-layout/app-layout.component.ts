import { Component, ChangeDetectionStrategy } from '@angular/core';
import { RouterLink } from '@angular/router';

/**
 * Global layout component serving as the app shell.
 */
@Component({
  selector: 'app-layout',
  imports: [RouterLink],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="layout-wrapper">
      <header class="Header">
        <div class="Header-item">
          <a routerLink="/" class="Header-link f4 d-flex flex-items-center">
            <svg height="32" aria-hidden="true" viewBox="0 0 16 16" version="1.1" width="32" class="octicon octicon-mark-github v-align-middle color-fg-default">
              <path fill="currentColor" d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path>
            </svg>
            <span class="ml-2 text-bold Header-title">Bridle</span>
          </a>
        </div>
        <div class="Header-item Header-item--full"></div>
      </header>
      
      <div class="layout-main">
        <aside class="layout-sidebar">
          <nav class="SideNav">
            <a routerLink="/" class="SideNav-item">Dashboard</a>
            <a routerLink="/orgs" class="SideNav-item">Organizations</a>
            <a routerLink="/local-ops" class="SideNav-item">Local Operations</a>
            <a routerLink="/batch" class="SideNav-item">Batch Actions</a>
            <a routerLink="/prs" class="SideNav-item">Pull Requests</a>
            <a routerLink="/dev" class="SideNav-item">Dev Tools</a>
          </nav>
        </aside>
        
        <main class="layout-content">
          <ng-content></ng-content>
        </main>
      </div>
    </div>
  `,
  styles: `
    .layout-wrapper {
      display: flex;
      flex-direction: column;
      min-height: 100vh;
    }
    
    .Header {
      z-index: 32;
      display: flex;
      padding: 16px;
      font-size: 14px;
      line-height: 1.5;
      color: var(--color-header-text);
      background-color: var(--color-header-bg);
      align-items: center;
      flex-wrap: nowrap;
    }
    
    .Header-item {
      display: flex;
      margin-right: 16px;
      align-self: stretch;
      align-items: center;
      flex-wrap: nowrap;
    }
    
    .Header-item--full {
      flex: auto;
      margin-right: 0;
    }
    
    .Header-link {
      font-weight: 600;
      color: rgba(255, 255, 255, 1);
      white-space: nowrap;
      text-decoration: none;
      display: flex;
      align-items: center;
    }
    
    .Header-link:hover {
      color: rgba(255, 255, 255, 0.7);
    }
    
    .Header-title {
      margin-left: 8px;
    }
    
    .layout-main {
      display: flex;
      flex: 1;
      overflow: hidden;
    }
    
    .layout-sidebar {
      width: 256px;
      background-color: var(--color-canvas-default);
      border-right: 1px solid var(--color-border-default);
      overflow-y: auto;
    }
    
    .SideNav {
      display: flex;
      flex-direction: column;
    }
    
    .SideNav-item {
      padding: 8px 16px;
      font-size: 14px;
      color: var(--color-fg-default);
      text-decoration: none;
      border-bottom: 1px solid var(--color-border-default);
    }
    
    .SideNav-item:hover {
      background-color: var(--color-canvas-subtle);
      text-decoration: none;
    }
    
    .layout-content {
      flex: 1;
      padding: 24px;
      overflow-y: auto;
      background-color: var(--color-canvas-subtle);
    }
  `
})
export class AppLayoutComponent {}
