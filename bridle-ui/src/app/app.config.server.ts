import { provideServerRendering } from '@angular/ssr';
import { mergeApplicationConfig, ApplicationConfig } from '@angular/core';
import { appConfig } from './app.config';

/** Server specific configuration */
const serverConfig: ApplicationConfig = {
  providers: [provideServerRendering()],
};

/** Merged application configuration */
export const config = mergeApplicationConfig(appConfig, serverConfig);
