use crate::{
    common::state::Status,
    repo_files::{
        errors::LoadFilesError,
        selectors as repo_files_selectors,
        state::{RepoFile, RepoFileSize, RepoFilesBreadcrumb},
    },
    repos::{
        errors::RepoLockedError,
        selectors as repo_selectors,
        state::{Repo, RepoState},
    },
    selection::{selectors as selection_selectors, state::SelectionSummary},
    store,
    types::{DecryptedPath, RepoFileId, RepoId},
};

use super::state::{
    RepoFilesBrowser, RepoFilesBrowserInfo, RepoFilesBrowserItem, RepoFilesBrowserLocation,
};

pub fn select_file_ids<'a>(
    state: &'a store::State,
    repo_id: &RepoId,
    path: &DecryptedPath,
) -> impl Iterator<Item = &'a RepoFileId> {
    repo_files_selectors::select_files(state, repo_id, path).map(|file| &file.id)
}

pub fn select_browser<'a>(
    state: &'a store::State,
    browser_id: u32,
) -> Option<&'a RepoFilesBrowser> {
    state.repo_files_browsers.browsers.get(&browser_id)
}

pub fn select_browser_location<'a>(
    state: &'a store::State,
    browser_id: u32,
) -> Option<&'a RepoFilesBrowserLocation> {
    select_browser(state, browser_id).and_then(|browser| browser.location.as_ref())
}

pub fn select_repo_id<'a>(state: &'a store::State, browser_id: u32) -> Option<&'a RepoId> {
    select_browser_location(state, browser_id).map(|loc| &loc.repo_id)
}

pub fn select_repo_id_path_owned(
    state: &store::State,
    browser_id: u32,
) -> Option<(RepoId, DecryptedPath)> {
    select_browser_location(state, browser_id).map(|loc| (loc.repo_id.clone(), loc.path.clone()))
}

pub fn select_repo<'a>(state: &'a store::State, browser_id: u32) -> Option<&'a Repo> {
    select_browser(state, browser_id)
        .and_then(|browser| browser.location.as_ref())
        .and_then(|loc| repo_selectors::select_repo(state, &loc.repo_id).ok())
}

pub fn select_repo_state<'a>(state: &'a store::State, browser_id: u32) -> Option<&'a RepoState> {
    select_repo(state, browser_id).map(|repo| &repo.state)
}

pub fn select_is_unlocked<'a>(state: &'a store::State, browser_id: u32) -> bool {
    select_repo_state(state, browser_id)
        .map(|repo_state| repo_state.is_unlocked())
        .unwrap_or(false)
}

pub fn select_is_selected(state: &store::State, browser_id: u32, file_id: &RepoFileId) -> bool {
    select_browser(state, browser_id)
        .map(|browser| browser.selection.selection.contains(file_id))
        .unwrap_or(false)
}

pub fn select_items<'a>(state: &'a store::State, browser_id: u32) -> Vec<RepoFilesBrowserItem<'a>> {
    select_browser(state, browser_id)
        .map(|browser| {
            repo_files_selectors::select_files_from_ids(state, &browser.file_ids)
                .map(|file| RepoFilesBrowserItem {
                    file,
                    is_selected: select_is_selected(state, browser_id, &file.id),
                })
                .collect()
        })
        .unwrap_or(vec![])
}

pub fn select_selection_summary(state: &store::State, browser_id: u32) -> SelectionSummary {
    select_browser(state, browser_id)
        .map(|browser| {
            selection_selectors::select_selection_summary(
                &browser.selection,
                repo_files_selectors::select_files_from_ids(state, &browser.file_ids).count(),
            )
        })
        .unwrap_or(SelectionSummary::None)
}

pub fn select_selected_file_ids<'a>(
    state: &'a store::State,
    browser_id: u32,
) -> Vec<&'a RepoFileId> {
    select_browser(state, browser_id)
        .map(|browser| browser.selection.selection.iter().collect())
        .unwrap_or_else(|| Vec::new())
}

pub fn select_selected_files<'a>(state: &'a store::State, browser_id: u32) -> Vec<&'a RepoFile> {
    select_selected_file_ids(state, browser_id)
        .iter()
        .filter_map(|id| repo_files_selectors::select_file(state, id))
        .collect()
}

pub fn select_selected_paths(state: &store::State, browser_id: u32) -> Vec<DecryptedPath> {
    select_selected_files(state, browser_id)
        .into_iter()
        .filter_map(|file| file.decrypted_path().ok().cloned())
        .collect()
}

pub fn select_status<'a>(
    state: &store::State,
    browser: &RepoFilesBrowser,
) -> Status<LoadFilesError> {
    match &browser.location {
        Some(location) => match repo_selectors::select_repo(state, &location.repo_id) {
            Ok(repo) => {
                if matches!(repo.state, RepoState::Locked) {
                    Status::Error {
                        error: LoadFilesError::RepoLocked(RepoLockedError),
                        loaded: false,
                    }
                } else {
                    browser.status.clone()
                }
            }
            Err(err) => Status::Error {
                error: LoadFilesError::RepoNotFound(err.clone()),
                loaded: false,
            },
        },
        None => browser.status.clone(),
    }
}

pub fn select_info<'a>(state: &'a store::State, browser_id: u32) -> Option<RepoFilesBrowserInfo> {
    select_browser(state, browser_id).map(|browser| {
        let items = select_items(state, browser_id);
        let breadcrumbs = select_breadcrumbs(state, browser_id);
        let status = select_status(state, browser);
        let title = breadcrumbs.last().map(|x| x.name.clone());
        let total_count = items.len();
        let total_size = items
            .iter()
            .map(|item| match item.file.size {
                Some(RepoFileSize::Decrypted { size }) => size,
                Some(RepoFileSize::DecryptError { encrypted_size, .. }) => encrypted_size,
                None => 0,
            })
            .sum();
        let selected_count = items.iter().filter(|item| item.is_selected).count();
        let selected_size = items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| match item.file.size {
                Some(RepoFileSize::Decrypted { size }) => size,
                Some(RepoFileSize::DecryptError { encrypted_size, .. }) => encrypted_size,
                None => 0,
            })
            .sum();
        let selected_file = items
            .iter()
            .find(|item| item.is_selected)
            .map(|item| item.file)
            .filter(|_| selected_count == 1);
        let can_download_selected = selected_count > 0;
        let can_copy_selected = selected_count > 0;
        let can_move_selected = selected_count > 0;
        let can_delete_selected = selected_count > 0;

        RepoFilesBrowserInfo {
            repo_id: browser.location.as_ref().map(|loc| &loc.repo_id),
            path: browser.location.as_ref().map(|loc| &loc.path),
            selection_summary: select_selection_summary(state, browser_id),
            sort: browser.sort.clone(),
            status,
            title,
            total_count,
            total_size,
            selected_count,
            selected_size,
            selected_file,
            can_download_selected,
            can_copy_selected,
            can_move_selected,
            can_delete_selected,
            items,
            breadcrumbs,
        }
    })
}

pub fn select_breadcrumbs(state: &store::State, browser_id: u32) -> Vec<RepoFilesBreadcrumb> {
    select_browser_location(state, browser_id)
        .map(|loc| repo_files_selectors::select_breadcrumbs(state, &loc.repo_id, &loc.path))
        .unwrap_or_else(|| vec![])
}

pub fn select_root_file_id(state: &store::State, browser_id: u32) -> Option<RepoFileId> {
    select_browser_location(state, browser_id)
        .map(|loc| repo_files_selectors::get_file_id(&loc.repo_id, &loc.path))
}

pub fn select_root_file<'a>(state: &'a store::State, browser_id: u32) -> Option<&'a RepoFile> {
    select_root_file_id(state, browser_id)
        .and_then(|file_id| repo_files_selectors::select_file(state, &file_id))
}
