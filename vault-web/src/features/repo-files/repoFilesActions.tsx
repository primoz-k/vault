import { downloadStream } from '../../utils/downloadStream';
import { WebVault } from '../../vault-wasm/vault-wasm';

export const downloadFile = async (
  webVault: WebVault,
  repoId: string,
  encryptedPath: string,
  isMobile: boolean,
) => {
  const forceBlob = isMobile;
  const stream = await webVault.repoFilesGetFileStream(
    repoId,
    encryptedPath,
    forceBlob,
  );

  if (stream === undefined) {
    return;
  }

  downloadStream(stream);
};

export const downloadSelected = async (
  webVault: WebVault,
  browserId: number,
  isMobile: boolean,
) => {
  const forceBlob = isMobile;
  const stream = await webVault.repoFilesBrowsersGetSelectedStream(
    browserId,
    forceBlob,
  );

  if (stream === undefined) {
    return;
  }

  downloadStream(stream);
};
