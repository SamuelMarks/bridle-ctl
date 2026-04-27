import { provideZoneChangeDetection } from '@angular/core';
import {
  BootstrapContext,
  bootstrapApplication,
} from '@angular/platform-browser';
import { AppComponent } from './app/app.component';
import { config } from './app/app.config.server';

/**
 * Bootstrap function for server-side rendering
 * @param context Bootstrap context
 * @returns ApplicationRef promise
 */
const bootstrap = (context: BootstrapContext) =>
  bootstrapApplication(
    AppComponent,
    {
      ...config,
      providers: [provideZoneChangeDetection(), ...config.providers],
    },
    context,
  );

export default bootstrap;
