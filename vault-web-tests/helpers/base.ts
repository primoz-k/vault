import { test as base } from '@playwright/test';

import { oauth2RedirectUri } from './baseUrl';
import { Dialogs } from './dialogs';
import { KoofrApiClient } from './koofrApiClient';
import {
  getCurrentWorkerOAuth2Token,
  getCurrentWorkerStorageStatePath,
} from './storageState';
import { config, ignoreHTTPSErrors } from './vaultConfig';
import { WebVaultClient } from './webVaultClient';

export const test = base.extend<
  {
    koofrApiClient: KoofrApiClient;
    webVaultClient: WebVaultClient;
    dialogs: Dialogs;
  },
  {
    workerStorageState: string;
  }
>({
  storageState: ({ workerStorageState }, use) => use(workerStorageState),

  workerStorageState: [
    // eslint-disable-next-line no-empty-pattern
    async ({}, use) => {
      await use(getCurrentWorkerStorageStatePath());
    },
    { scope: 'worker' },
  ],

  koofrApiClient: [
    async ({ request }, use) => {
      const apiBaseUrl = config.baseUrl;
      const oauth2Token = getCurrentWorkerOAuth2Token();
      const oauth2ClientId = config.oauth2ClientId;
      const oauth2ClientSecret = config.oauth2ClientSecret;

      const koofrApiClient = new KoofrApiClient(
        request,
        apiBaseUrl,
        oauth2Token,
        oauth2ClientId,
        oauth2ClientSecret,
        oauth2RedirectUri,
        ignoreHTTPSErrors
      );

      await koofrApiClient.removeAllVaultRepos();

      await use(koofrApiClient);
    },
    { auto: true },
  ],

  // eslint-disable-next-line no-empty-pattern
  webVaultClient: async ({}, use) => {
    const apiBaseUrl = config.baseUrl;
    const oauth2Token = getCurrentWorkerOAuth2Token();
    const oauth2ClientId = config.oauth2ClientId;
    const oauth2ClientSecret = config.oauth2ClientSecret;

    const webVaultClient = new WebVaultClient(
      apiBaseUrl,
      oauth2Token,
      oauth2ClientId,
      oauth2ClientSecret,
      oauth2RedirectUri,
      ignoreHTTPSErrors
    );

    try {
      await use(webVaultClient);
    } finally {
      webVaultClient.destroy();
    }
  },

  dialogs: async ({ page }, use) => {
    await use(new Dialogs(page));
  },
});
