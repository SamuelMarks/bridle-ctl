import { mergeApplicationConfig, ApplicationConfig } from '@angular/core';
import { provideServerRendering } from '@angular/platform-server';
import { appConfig } from './app.config';

/** Server specific configuration */
const serverConfig: ApplicationConfig = {
  providers: [
    provideServerRendering()
  ]
};

/** Merged application configuration */
export const config = mergeApplicationConfig(appConfig, serverConfig);
