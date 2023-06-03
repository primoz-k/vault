use std::{collections::HashMap, sync::Arc};

use crate::{
    common::state::Status,
    eventstream::service::MountSubscription,
    repo_files::{
        errors::{DeleteFileError, LoadFilesError},
        selectors as repo_files_selectors,
        state::{RepoFile, RepoFilesUploadResult},
    },
    repo_files_read::errors::GetFilesReaderError,
    store,
};

use super::{
    errors::{LoadContentError, SaveError},
    selectors,
    state::{
        RepoFilesDetails, RepoFilesDetailsContent, RepoFilesDetailsContentData,
        RepoFilesDetailsContentLoading, RepoFilesDetailsLocation, RepoFilesDetailsOptions,
        SaveInitiator,
    },
};

pub fn create_location(
    repo_id: String,
    path: String,
    eventstream_mount_subscription: Option<Arc<MountSubscription>>,
    is_editing: bool,
) -> RepoFilesDetailsLocation {
    RepoFilesDetailsLocation {
        repo_id,
        path,
        eventstream_mount_subscription,
        content: RepoFilesDetailsContent {
            status: Status::Initial,
            data: None,
            loading: None,
            version: 0,
        },
        is_editing,
        is_dirty: false,
        save_status: Status::Initial,
        delete_status: Status::Initial,
        should_destroy: false,
    }
}

pub fn create(
    state: &mut store::State,
    options: RepoFilesDetailsOptions,
    location: Result<RepoFilesDetailsLocation, LoadFilesError>,
    repo_files_subscription_id: u32,
) -> u32 {
    let details_id = state.repo_files_details.next_id;

    state.repo_files_details.next_id += 1;

    let status = match &location {
        Ok(location) => {
            if repo_files_selectors::select_file(
                state,
                &repo_files_selectors::get_file_id(&location.repo_id, &location.path),
            )
            .is_some()
            {
                Status::Reloading
            } else {
                Status::Loading
            }
        }
        Err(err) => Status::Error { error: err.clone() },
    };

    let details = RepoFilesDetails {
        options,
        location: location.ok(),
        status,
        repo_files_subscription_id,
    };

    state.repo_files_details.details.insert(details_id, details);

    details_id
}

pub fn destroy(state: &mut store::State, details_id: u32) -> Option<u32> {
    let repo_files_subscription_id = state
        .repo_files_details
        .details
        .get(&details_id)
        .map(|details| details.repo_files_subscription_id);

    state.repo_files_details.details.remove(&details_id);

    repo_files_subscription_id
}

pub fn loaded(
    state: &mut store::State,
    details_id: u32,
    repo_id: &str,
    path: &str,
    error: Option<&LoadFilesError>,
) {
    let details = match state.repo_files_details.details.get_mut(&details_id) {
        Some(details) => details,
        _ => return,
    };

    if details
        .location
        .as_ref()
        .filter(|loc| loc.repo_id == repo_id && loc.path == path)
        .is_some()
    {
        details.status = match error {
            Some(error) => Status::Error {
                error: error.clone(),
            },
            None => Status::Loaded,
        };
    }
}

pub fn content_loading(
    state: &mut store::State,
    notify: &store::Notify,
    details_id: u32,
) -> Result<RepoFile, LoadContentError> {
    let file = selectors::select_file(state, details_id)
        .map(|file| file.clone())
        .ok_or(LoadContentError::FileNotFound)?;

    match selectors::select_details(state, details_id) {
        Some(details)
            if details
                .options
                .load_content
                .matches(file.ext.as_deref(), &file.category) => {}
        _ => return Err(LoadContentError::LoadFilterMismatch),
    };

    let loading = selectors::select_remote_file(state, details_id).map(|remote_file| {
        RepoFilesDetailsContentLoading {
            remote_size: remote_file.size,
            remote_modified: remote_file.modified,
            remote_hash: remote_file.hash.clone(),
        }
    });

    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return Err(LoadContentError::FileNotFound),
    };

    location.content.status = match location.content.status {
        Status::Initial => Status::Loading,
        Status::Loaded | Status::Error { .. } => Status::Reloading,
        Status::Loading | Status::Reloading => return Err(LoadContentError::AlreadyLoading),
    };
    location.content.loading = loading;

    notify(store::Event::RepoFilesDetails);

    Ok(file)
}

pub fn file_reader_loading(
    state: &mut store::State,
    notify: &store::Notify,
    details_id: u32,
    file: RepoFile,
) -> Result<(), GetFilesReaderError> {
    let loading = repo_files_selectors::select_remote_file(state, &file).map(|remote_file| {
        RepoFilesDetailsContentLoading {
            remote_size: remote_file.size,
            remote_modified: remote_file.modified,
            remote_hash: remote_file.hash.clone(),
        }
    });

    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return Err(GetFilesReaderError::FileNotFound),
    };

    location.content.status = match location.content.status {
        Status::Initial | Status::Loading | Status::Reloading => Status::Loading,
        Status::Loaded | Status::Error { .. } => Status::Reloading,
    };
    location.content.loading = loading;

    notify(store::Event::RepoFilesDetails);

    Ok(())
}

pub fn content_loaded(
    state: &mut store::State,
    notify: &store::Notify,
    details_id: u32,
    repo_id: String,
    path: String,
    res: Result<Option<RepoFilesDetailsContentData>, GetFilesReaderError>,
) {
    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return,
    };

    if location.repo_id != repo_id || location.path != path {
        return;
    }

    notify(store::Event::RepoFilesDetails);

    location.content.loading = None;

    if location.is_dirty || matches!(location.save_status, Status::Loading) {
        location.content.status = Status::Loaded;
    } else {
        match res {
            Ok(data) => {
                location.content.status = Status::Loaded;
                location.content.data = data;
                location.content.version += 1;

                notify(store::Event::RepoFilesDetailsContentData);
            }
            Err(err) => {
                location.content.status = Status::Error { error: err };
            }
        }
    }
}

pub fn edit(state: &mut store::State, details_id: u32) {
    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return,
    };

    location.is_editing = true;
}

pub fn edit_cancel(
    state: &mut store::State,
    notify: &store::Notify,
    details_id: u32,
    is_discarded: bool,
) {
    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return,
    };

    if location.is_editing {
        location.is_editing = false;
        location.is_dirty = false;
        location.save_status = Status::Initial;

        if is_discarded {
            // this will reload the content
            location.content.status = Status::Initial;
            location.content.data = None;
            location.content.loading = None;
        }

        notify(store::Event::RepoFilesDetails);
    }
}

pub fn set_content(
    state: &mut store::State,
    notify: &store::Notify,
    details_id: u32,
    content: Vec<u8>,
) {
    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return,
    };

    if let Some(data) = &mut location.content.data {
        if data.bytes != content {
            data.bytes = content;

            location.content.version += 1;

            notify(store::Event::RepoFilesDetailsContentData);

            if !location.is_dirty {
                location.is_dirty = true;

                notify(store::Event::RepoFilesDetails);
            }
        }
    }
}

pub fn saving(
    state: &mut store::State,
    notify: &store::Notify,
    details_id: u32,
    initiator: SaveInitiator,
) -> Result<(String, String, RepoFilesDetailsContentData, u32, bool), SaveError> {
    if !selectors::select_is_dirty(state, details_id) {
        return Err(SaveError::NotDirty);
    }

    let file = selectors::select_file(state, details_id);

    let remote_file = file.and_then(|file| repo_files_selectors::select_remote_file(state, file));

    let content = selectors::select_content(state, details_id).ok_or(SaveError::InvalidState)?;

    let data = content.data.clone().ok_or(SaveError::InvalidState)?;

    let location = match selectors::select_details_location(state, details_id) {
        Some(location) => location,
        _ => return Err(SaveError::InvalidState),
    };

    let is_deleted = file.is_none();

    if matches!(initiator, SaveInitiator::Autosave)
        && (selectors::get_is_conflict(true, Some(&data), remote_file, &location.save_status)
            || is_deleted)
    {
        return Err(SaveError::AutosaveNotPossible);
    }

    let version = content.version;

    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return Err(SaveError::InvalidState),
    };

    location.save_status = Status::Loading;

    notify(store::Event::RepoFilesDetails);

    Ok((
        location.repo_id.clone(),
        location.path.clone(),
        data,
        version,
        is_deleted,
    ))
}

pub fn saved(
    state: &mut store::State,
    notify: &store::Notify,
    details_id: u32,
    saved_version: u32,
    res: Result<(String, RepoFilesUploadResult, bool), SaveError>,
) {
    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return,
    };

    notify(store::Event::RepoFilesDetails);

    match res {
        Ok((saved_path, result, should_destroy)) => {
            if location.path != saved_path {
                location.path = saved_path;
            }
            if let Some(data) = &mut location.content.data {
                data.remote_size = result.remote_file.size;
                data.remote_modified = result.remote_file.modified;
                data.remote_hash = result.remote_file.hash;
            }
            if location.content.version == saved_version {
                location.is_dirty = false;
            }
            location.save_status = Status::Initial;
            if should_destroy {
                location.should_destroy = true;
            }
        }
        Err(err) => {
            match err {
                SaveError::Canceled => {
                    location.save_status = Status::Initial;
                }
                SaveError::DiscardChanges { should_destroy } => {
                    location.save_status = Status::Initial;
                    if should_destroy {
                        location.should_destroy = true;
                    }
                }
                err => {
                    location.save_status = Status::Error { error: err };
                }
            };
        }
    }
}

pub fn deleting(state: &mut store::State, notify: &store::Notify, details_id: u32) {
    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return,
    };

    notify(store::Event::RepoFilesDetails);

    location.delete_status = Status::Loading;
}

pub fn deleted(
    state: &mut store::State,
    notify: &store::Notify,
    details_id: u32,
    res: Result<(), DeleteFileError>,
) {
    let location = match selectors::select_details_location_mut(state, details_id) {
        Some(location) => location,
        _ => return,
    };

    notify(store::Event::RepoFilesDetails);

    match res {
        Ok(()) => {
            location.delete_status = Status::Loaded;
            location.should_destroy = true;
        }
        Err(DeleteFileError::Canceled) => {
            location.delete_status = Status::Initial;
        }
        Err(err) => {
            location.delete_status = Status::Error { error: err };
        }
    }
}

pub fn handle_repo_files_mutation(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
) {
    if !mutation_state.repo_files.moved_files.is_empty() {
        let moved_files = mutation_state
            .repo_files
            .moved_files
            .iter()
            .map(|(repo_id, old_path, new_path)| {
                (
                    (repo_id.to_owned(), old_path.to_owned()),
                    new_path.to_owned(),
                )
            })
            .collect::<HashMap<_, _>>();

        for (_, details) in state.repo_files_details.details.iter_mut() {
            if let Some(location) = &mut details.location {
                if let Some(new_path) =
                    moved_files.get(&(location.repo_id.clone(), location.path.clone()))
                {
                    location.path = new_path.to_owned();

                    notify(store::Event::RepoFilesDetails);
                }
            }
        }
    }
}
