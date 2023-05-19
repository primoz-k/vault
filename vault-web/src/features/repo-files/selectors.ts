import { To } from 'react-router-dom';

import { RepoFile } from '../../vault-wasm/vault-wasm';

export const repoFilesLink = (
  repoId: string,
  path?: string,
  name?: string
): To => {
  const search = new URLSearchParams();

  if (path !== undefined) {
    search.set('path', path);
  }

  if (name !== undefined) {
    search.set('name', name);
  }

  return {
    pathname: `/repos/${repoId}`,
    search: search.toString(),
  };
};

export const repoFilesDetailsLink = (repoId: string, path: string): To => ({
  pathname: `/repos/${repoId}/details`,
  search: `path=${encodeURIComponent(path)}`,
});

export const fileHasPdfViewer = (file: RepoFile): boolean =>
  file.ext === 'pdf' && !file.nameError;

export const fileHasTextEditor = (file: RepoFile): boolean =>
  (file.category === 'Text' || file.category === 'Code') && !file.nameError;

export const fileHasImageViewer = (file: RepoFile): boolean =>
  (file.ext === 'jpg' ||
    file.ext === 'jpeg' ||
    file.ext === 'gif' ||
    file.ext === 'png' ||
    file.ext === 'svg') &&
  !file.nameError;

export const fileHasDetails = (file: RepoFile): boolean =>
  fileHasPdfViewer(file) || fileHasTextEditor(file) || fileHasImageViewer(file);
