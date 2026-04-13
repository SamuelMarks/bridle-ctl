import { Component, ChangeDetectionStrategy } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { AppLayoutComponent } from './shared/ui/app-layout/app-layout.component';

/**
 * Root application component.
 */
@Component({
  selector: 'app-root',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [RouterOutlet, AppLayoutComponent],
  template: `
    <app-layout>
      <router-outlet></router-outlet>
    </app-layout>
  `,
  styles: [],
})
export class AppComponent {
  /** The title of the application */
  title = 'bridle-ui';
}
