import { Component } from '@angular/core';
import { RouterLink, RouterLinkActive } from '@angular/router';
import { AppToastComponent } from '../app-toast/app-toast.component';

/**
 * Global layout component serving as the application shell.
 */
@Component({
  selector: 'app-layout',
  imports: [RouterLink, RouterLinkActive, AppToastComponent],
  template: `
    <div class="layout-wrapper">
      <header class="Header">
        <div class="Header-item">
          <a routerLink="/" class="Header-link f4 d-flex flex-items-center">
            <svg
              height="32"
              aria-hidden="true"
              viewBox="0 0 16 16"
              version="1.1"
              width="32"
              class="octicon octicon-mark-github mr-2"
            >
              <path
                fill="currentColor"
                d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"
              ></path>
            </svg>
            <span>Bridle Control</span>
          </a>
        </div>
      </header>

      <div class="layout-body">
        <aside class="layout-sidebar">
          <nav class="SideNav" aria-label="Main">
            <a
              routerLink="/"
              routerLinkActive="active"
              [routerLinkActiveOptions]="{ exact: true }"
              ariaCurrentWhenActive="page"
              class="SideNav-item"
              >Dashboard</a
            >
            <a
              routerLink="/orgs"
              routerLinkActive="active"
              ariaCurrentWhenActive="page"
              class="SideNav-item"
              >Organizations & Repos</a
            >
            <a
              routerLink="/local-ops"
              routerLinkActive="active"
              ariaCurrentWhenActive="page"
              class="SideNav-item"
              >Local Operations</a
            >
            <a
              routerLink="/batch"
              routerLinkActive="active"
              ariaCurrentWhenActive="page"
              class="SideNav-item"
              >Batch Actions & Pipelines</a
            >
            <a
              routerLink="/prs"
              routerLinkActive="active"
              ariaCurrentWhenActive="page"
              class="SideNav-item"
              >PR Sync</a
            >
            <a
              routerLink="/dev"
              routerLinkActive="active"
              ariaCurrentWhenActive="page"
              class="SideNav-item"
              >Dev Tools & DB</a
            >
          </nav>
        </aside>

        <main class="layout-content">
          <ng-content></ng-content>
        </main>
      </div>
      <app-toast></app-toast>
    </div>
  `,
  styles: `
    .layout-wrapper {
      display: flex;
      flex-direction: column;
      min-height: 100vh;
    }

    .Header {
      display: flex;
      align-items: center;
      padding: 16px;
      background-color: #24292f;
      color: #ffffff;
    }

    .Header-item {
      display: flex;
      align-items: center;
    }

    .Header-link {
      color: #ffffff;
      font-weight: 600;
      text-decoration: none;
      display: flex;
      align-items: center;
      gap: 8px;
    }

    .Header-link:hover {
      color: rgba(255, 255, 255, 0.7);
      text-decoration: none;
    }

    .layout-body {
      display: flex;
      flex: 1;
    }

    .layout-sidebar {
      width: 260px;
      background-color: var(--color-canvas-default);
      border-right: 1px solid var(--color-border-default);
      padding: 16px;
    }

    .SideNav {
      display: flex;
      flex-direction: column;
      gap: 4px;
    }

    .SideNav-item {
      padding: 8px 12px;
      border-radius: var(--border-radius);
      color: var(--color-fg-default);
      font-size: 14px;
      font-weight: 500;
      text-decoration: none;
      transition: background-color 0.1s ease-in-out;
    }

    .SideNav-item:hover,
    .SideNav-item.active {
      background-color: var(--color-canvas-subtle);
      text-decoration: none;
    }

    .layout-content {
      flex: 1;
      padding: 24px;
      overflow-y: auto;
      background-color: var(--color-canvas-subtle);
    }
  `,
})
export class AppLayoutComponent {}
