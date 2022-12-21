import { memo } from 'react';

import { ReactComponent as FilesRenameHoverIcon } from '../../assets/images/files-rename-hover.svg';
import { ReactComponent as FilesRenameIcon } from '../../assets/images/files-rename.svg';
import { ReactComponent as FilesToolbarCopyHoverIcon } from '../../assets/images/files-toolbar-copy-hover.svg';
import { ReactComponent as FilesToolbarCopyIcon } from '../../assets/images/files-toolbar-copy.svg';
import { ReactComponent as FilesToolbarDeleteHoverIcon } from '../../assets/images/files-toolbar-delete-hover.svg';
import { ReactComponent as FilesToolbarDeleteIcon } from '../../assets/images/files-toolbar-delete.svg';
import { ReactComponent as FilesToolbarDownloadHoverIcon } from '../../assets/images/files-toolbar-download-hover.svg';
import { ReactComponent as FilesToolbarDownloadIcon } from '../../assets/images/files-toolbar-download.svg';
import { ReactComponent as FilesToolbarMoveHoverIcon } from '../../assets/images/files-toolbar-move-hover.svg';
import { ReactComponent as FilesToolbarMoveIcon } from '../../assets/images/files-toolbar-move.svg';
import { ConfirmModal, ConfirmPayload } from '../../components/ConfirmModal';
import {
  Toolbar,
  ToolbarCancelItem,
  ToolbarItem,
} from '../../components/toolbar/Toolbar';
import { useIsMobile } from '../../components/useIsMobile';
import { useModal } from '../../utils/useModal';
import {
  RepoFilesBrowserInfo,
  RepoFilesMoveMode,
} from '../../vault-wasm/vault-wasm';
import { useWebVault } from '../../webVault/useWebVault';

import { useRepoFilesBrowserId } from './RepoFilesBrowserId';
import { useRepoFilesRename } from './RepoFilesRename';
import { downloadFile } from './repoFilesActions';

export const RepoFilesToolbar = memo<{ info: RepoFilesBrowserInfo }>(
  ({ info }) => {
    const isMobile = useIsMobile();
    const webVault = useWebVault();
    const browserId = useRepoFilesBrowserId();
    const selectedFile =
      info.selectedFile !== undefined && info.path !== undefined
        ? info.selectedFile
        : undefined;
    const deleteFileModal = useModal<ConfirmPayload>();
    const renameFile = useRepoFilesRename();

    return (
      <Toolbar>
        {isMobile && selectedFile !== undefined ? (
          <ToolbarItem
            icon={<FilesRenameIcon />}
            iconHover={<FilesRenameHoverIcon />}
            onClick={() => {
              renameFile(selectedFile);
            }}
          >
            Rename
          </ToolbarItem>
        ) : null}
        {info.canCopySelected ? (
          <ToolbarItem
            icon={<FilesToolbarCopyIcon />}
            iconHover={<FilesToolbarCopyHoverIcon />}
            onClick={() => {
              webVault.repoFilesMoveShow(browserId, RepoFilesMoveMode.Copy);
            }}
          >
            Copy
          </ToolbarItem>
        ) : null}
        {info.canMoveSelected ? (
          <ToolbarItem
            icon={<FilesToolbarMoveIcon />}
            iconHover={<FilesToolbarMoveHoverIcon />}
            onClick={() => {
              webVault.repoFilesMoveShow(browserId, RepoFilesMoveMode.Move);
            }}
          >
            Move
          </ToolbarItem>
        ) : null}
        {selectedFile !== undefined && selectedFile.type === 'File' ? (
          <ToolbarItem
            icon={<FilesToolbarDownloadIcon />}
            iconHover={<FilesToolbarDownloadHoverIcon />}
            onClick={() => {
              downloadFile(webVault, selectedFile, isMobile);
            }}
          >
            Download
          </ToolbarItem>
        ) : null}
        {info.canDeleteSelected ? (
          <ToolbarItem
            icon={<FilesToolbarDeleteIcon />}
            iconHover={<FilesToolbarDeleteHoverIcon />}
            onClick={() => {
              deleteFileModal.show({
                title: 'Delete files',
                message: 'Do you really want to delete 1 item?',
                confirmText: 'Delete',
                onConfirm: () => {
                  webVault.repoFilesBrowsersDeleteSelected(browserId);
                },
              });
            }}
          >
            Delete
          </ToolbarItem>
        ) : null}
        {info.selectedCount > 0 ? (
          <ToolbarCancelItem
            onClick={() => webVault.repoFilesBrowsersClearSelection(browserId)}
          />
        ) : null}

        <ConfirmModal
          isVisible={deleteFileModal.isVisible}
          payload={deleteFileModal.payload}
          hide={deleteFileModal.hide}
        />
      </Toolbar>
    );
  }
);
