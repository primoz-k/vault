use std::collections::HashMap;

use crate::{
    eventstream,
    files::file_category::FileCategory,
    remote::{models, remote::RemoteFileTagsSetConditions},
    remote_files_tags::set_tags::set_tags,
    store,
    types::{
        MountId, RemoteFileId, RemoteName, RemoteNameLower, RemotePath, REMOTE_PATH_LOWER_ROOT,
    },
    utils::remote_path_utils,
};

use super::{
    selectors,
    state::{Mount, MountType, RemoteFile, RemoteFileType},
};

pub fn mount_to_remote_file(id: RemoteFileId, mount_id: MountId) -> RemoteFile {
    let path = RemotePath("/".into());
    let size = None;
    let modified = None;
    let hash = None;
    let unique_id = selectors::get_file_unique_id(
        &mount_id,
        &path.to_lowercase(),
        size,
        modified,
        hash.as_deref(),
    );
    let category = FileCategory::Folder;

    RemoteFile {
        id,
        mount_id,
        path,
        name: RemoteName("".into()),
        name_lower: RemoteNameLower("".into()),
        ext: None,
        typ: RemoteFileType::Dir,
        size,
        modified,
        hash,
        tags: HashMap::new(),
        unique_id,
        category,
    }
}

pub fn files_file_to_remote_file(
    id: RemoteFileId,
    mount_id: MountId,
    path: RemotePath,
    file: models::FilesFile,
) -> RemoteFile {
    let name_lower = file.name.to_lowercase();
    let typ = file.typ.as_str().into();
    let (ext, category) = match &typ {
        RemoteFileType::Dir => (None, FileCategory::Folder),
        RemoteFileType::File => selectors::get_file_ext_category(&name_lower),
    };
    let (size, modified) = match &typ {
        RemoteFileType::Dir => (None, None),
        RemoteFileType::File => (Some(file.size), Some(file.modified)),
    };
    let unique_id = selectors::get_file_unique_id(
        &mount_id,
        &path.to_lowercase(),
        size,
        modified,
        file.hash.as_deref(),
    );

    RemoteFile {
        id,
        mount_id,
        path,
        name: file.name,
        name_lower,
        ext,
        typ,
        size,
        modified,
        hash: file.hash,
        tags: file.tags,
        unique_id,
        category,
    }
}

pub fn bundle_file_to_remote_file(
    id: RemoteFileId,
    mount_id: MountId,
    path: RemotePath,
    file: models::BundleFile,
) -> RemoteFile {
    let name_lower = file.name.to_lowercase();
    let typ = file.typ.as_str().into();
    let (ext, category) = match &typ {
        RemoteFileType::Dir => (None, FileCategory::Folder),
        RemoteFileType::File => selectors::get_file_ext_category(&name_lower),
    };
    let (size, modified) = match &typ {
        RemoteFileType::Dir => (None, None),
        RemoteFileType::File => (Some(file.size), Some(file.modified)),
    };
    let unique_id = selectors::get_file_unique_id(
        &mount_id,
        &path.to_lowercase(),
        size,
        modified,
        file.hash.as_deref(),
    );

    RemoteFile {
        id,
        mount_id,
        path,
        name: file.name,
        name_lower,
        ext,
        typ,
        size,
        modified,
        hash: file.hash,
        tags: file.tags,
        unique_id,
        category,
    }
}

fn bookmark_to_remote_file(id: RemoteFileId, bookmark: models::Bookmark) -> RemoteFile {
    let name_lower = bookmark.name.to_lowercase();
    let size = None;
    let modified = None;
    let hash = None;
    let unique_id = selectors::get_file_unique_id(
        &bookmark.mount_id,
        &bookmark.path.to_lowercase(),
        size,
        modified,
        hash.as_deref(),
    );
    let category = FileCategory::Folder;

    RemoteFile {
        id,
        mount_id: bookmark.mount_id,
        path: bookmark.path,
        name: bookmark.name,
        name_lower,
        ext: None,
        typ: RemoteFileType::Dir,
        size,
        modified,
        hash,
        tags: HashMap::new(),
        unique_id,
        category,
    }
}

fn shared_file_to_remote_file(id: RemoteFileId, shared_file: models::SharedFile) -> RemoteFile {
    let path = RemotePath("/".into());
    let name_lower = shared_file.name.to_lowercase();
    let typ = shared_file.typ.as_str().into();
    let (ext, category) = match &typ {
        RemoteFileType::Dir => (None, FileCategory::Folder),
        RemoteFileType::File => selectors::get_file_ext_category(&name_lower),
    };
    let (size, modified) = match &typ {
        RemoteFileType::Dir => (None, None),
        RemoteFileType::File => (Some(shared_file.size), Some(shared_file.modified)),
    };
    let hash = None;
    let unique_id = selectors::get_file_unique_id(
        &shared_file.mount.id,
        &path.to_lowercase(),
        size,
        modified,
        hash.as_deref(),
    );

    RemoteFile {
        id,
        mount_id: shared_file.mount.id,
        path,
        name: shared_file.name.clone(),
        name_lower,
        ext,
        typ,
        size,
        modified,
        hash,
        tags: HashMap::new(),
        unique_id,
        category,
    }
}

pub fn mount_loaded(state: &mut store::State, mount: models::Mount) {
    state
        .remote_files
        .mounts
        .insert(mount.id.clone(), mount.into());
}

pub fn place_loaded(state: &mut store::State, mount: models::Mount) {
    let file_id = selectors::get_file_id(&mount.id, &REMOTE_PATH_LOWER_ROOT);

    state.remote_files.files.insert(
        file_id.clone(),
        mount_to_remote_file(file_id.clone(), mount.id.clone()),
    );

    mount_loaded(state, mount);
}

pub fn places_loaded(state: &mut store::State, mounts: Vec<models::Mount>) {
    for mount_id in state.remote_files.place_mount_ids.iter() {
        state.remote_files.mounts.remove(mount_id);
    }

    for mount in mounts {
        place_loaded(state, mount);
    }

    let mut place_mounts: Vec<&Mount> = state
        .remote_files
        .mounts
        .values()
        .filter(|mount| mount.typ == MountType::Device)
        .collect();

    place_mounts.sort_by(|x, y| selectors::mount_sort_key(&x).cmp(&selectors::mount_sort_key(&y)));

    state.remote_files.place_mount_ids =
        place_mounts.iter().map(|mount| mount.id.clone()).collect();

    state.remote_files.online_place_mount_ids = place_mounts
        .iter()
        .filter(|mount| mount.online)
        .map(|mount| mount.id.clone())
        .collect();

    state.remote_files.places_loaded = true;
}

pub fn bookmark_loaded(state: &mut store::State, bookmark: models::Bookmark) {
    let file_id = selectors::get_file_id(&bookmark.mount_id, &bookmark.path.to_lowercase());

    state.remote_files.files.insert(
        file_id.clone(),
        bookmark_to_remote_file(file_id.clone(), bookmark),
    );

    state.remote_files.bookmark_file_ids.push(file_id);
}

pub fn bookmarks_loaded(state: &mut store::State, bookmarks: Vec<models::Bookmark>) {
    state.remote_files.bookmark_file_ids.clear();

    for bookmark in bookmarks {
        bookmark_loaded(state, bookmark);
    }

    state.remote_files.bookmarks_loaded = true;
}

pub fn shared_file_loaded(state: &mut store::State, shared_file: models::SharedFile) {
    let file_id = selectors::get_file_id(&shared_file.mount.id, &REMOTE_PATH_LOWER_ROOT);

    mount_loaded(state, shared_file.mount.clone());

    state.remote_files.files.insert(
        file_id.clone(),
        shared_file_to_remote_file(file_id.clone(), shared_file),
    );

    state.remote_files.shared_file_ids.push(file_id);
}

pub fn shared_files_loaded(state: &mut store::State, shared_files: Vec<models::SharedFile>) {
    state.remote_files.shared_file_ids.clear();

    for shared_file in shared_files {
        shared_file_loaded(state, shared_file);
    }

    state.remote_files.shared_files_loaded = true;
}

pub fn sort_children(state: &mut store::State, file_id: &RemoteFileId) {
    if let Some(children_ids) = state.remote_files.children.get(file_id) {
        let mut children: Vec<&RemoteFile> = children_ids
            .iter()
            .filter_map(|id| state.remote_files.files.get(id))
            .collect();

        children.sort_by(|x, y| {
            selectors::remote_file_sort_key(x).cmp(&selectors::remote_file_sort_key(y))
        });

        let children_ids: Vec<RemoteFileId> = children.iter().map(|file| file.id.clone()).collect();

        state
            .remote_files
            .children
            .insert(file_id.to_owned(), children_ids);
    }
}

pub fn bundle_loaded(
    state: &mut store::State,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    path: &RemotePath,
    bundle: models::Bundle,
) {
    let root_file_id = selectors::get_file_id(mount_id, &path.to_lowercase());

    let models::Bundle {
        file: bundle_file,
        files: bundle_files,
    } = bundle;

    state.remote_files.files.insert(
        root_file_id.clone(),
        bundle_file_to_remote_file(
            root_file_id.clone(),
            mount_id.to_owned(),
            path.to_owned(),
            bundle_file,
        ),
    );

    if let Some(files) = bundle_files {
        let mut children = Vec::with_capacity(files.len());

        for file in files {
            let file_path = remote_path_utils::join_path_name(path, &file.name);
            let file_id = selectors::get_file_id(mount_id, &file_path.to_lowercase());
            let remote_file =
                bundle_file_to_remote_file(file_id.clone(), mount_id.to_owned(), file_path, file);

            children.push(file_id.clone());

            state.remote_files.files.insert(file_id, remote_file);
        }

        state
            .remote_files
            .children
            .insert(root_file_id.clone(), children);

        sort_children(state, &root_file_id);
    }

    state.remote_files.loaded_roots.insert(root_file_id.clone());

    mutation_state
        .remote_files
        .loaded_roots
        .push((mount_id.to_owned(), path.to_owned()));

    mutation_notify(store::MutationEvent::RemoteFiles, state, mutation_state);
}

pub fn file_loaded(
    state: &mut store::State,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    path: &RemotePath,
    file: models::FilesFile,
) {
    let path_lower = path.to_lowercase();

    let root_file_id = selectors::get_file_id(mount_id, &path_lower);

    state.remote_files.files.insert(
        root_file_id.clone(),
        files_file_to_remote_file(
            root_file_id.clone(),
            mount_id.to_owned(),
            path.to_owned(),
            file,
        ),
    );

    if let Some(parent_path) = remote_path_utils::parent_path(path) {
        let parent_id = selectors::get_file_id(mount_id, &parent_path.to_lowercase());

        add_child(state, &parent_id, root_file_id.clone());
    }

    state.remote_files.loaded_roots.insert(root_file_id);

    mutation_state
        .remote_files
        .loaded_roots
        .push((mount_id.to_owned(), path.to_owned()));

    mutation_notify(store::MutationEvent::RemoteFiles, state, mutation_state);
}

pub fn add_child(state: &mut store::State, parent_id: &RemoteFileId, child_id: RemoteFileId) {
    if let Some(children) = state.remote_files.children.get_mut(parent_id) {
        if !children.contains(&child_id) {
            children.push(child_id);

            sort_children(state, &parent_id);
        }
    }
}

pub fn handle_eventstream_events_mutation(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
) {
    if !mutation_state.eventstream_events.events.is_empty() {
        let events = mutation_state.eventstream_events.events.clone();

        for (mount_listener, event) in events {
            match event {
                eventstream::Event::FileCreatedEvent {
                    mount_id,
                    path,
                    file,
                    ..
                } => {
                    file_created(
                        state,
                        notify,
                        mutation_state,
                        mutation_notify,
                        &mount_id,
                        &remote_path_utils::join_paths(&mount_listener.path, &path),
                        file,
                    );
                }
                eventstream::Event::FileRemovedEvent { mount_id, path, .. } => {
                    file_removed(
                        state,
                        notify,
                        mutation_state,
                        mutation_notify,
                        &mount_id,
                        &remote_path_utils::join_paths(&mount_listener.path, &path),
                    );
                }
                eventstream::Event::FileCopiedEvent {
                    mount_id,
                    new_path,
                    file,
                    ..
                } => {
                    file_copied(
                        state,
                        notify,
                        mutation_state,
                        mutation_notify,
                        &mount_id,
                        &remote_path_utils::join_paths(&mount_listener.path, &new_path),
                        file,
                    );
                }
                eventstream::Event::FileMovedEvent {
                    mount_id,
                    path,
                    new_path,
                    file,
                    ..
                } => {
                    file_moved(
                        state,
                        notify,
                        mutation_state,
                        mutation_notify,
                        &mount_id,
                        &remote_path_utils::join_paths(&mount_listener.path, &path),
                        &remote_path_utils::join_paths(&mount_listener.path, &new_path),
                        file,
                    );
                }
                eventstream::Event::FileTagsUpdatedEvent {
                    mount_id,
                    path,
                    file,
                    ..
                } => {
                    file_tags_updated(
                        state,
                        notify,
                        mutation_state,
                        mutation_notify,
                        &mount_id,
                        &remote_path_utils::join_paths(&mount_listener.path, &path),
                        file,
                    );
                }
                _ => {}
            }
        }
    }
}

pub fn file_created(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    path: &RemotePath,
    file: models::FilesFile,
) {
    notify(store::Event::RemoteFiles);

    let file_id = selectors::get_file_id(mount_id, &path.to_lowercase());

    state.remote_files.files.insert(
        file_id.clone(),
        files_file_to_remote_file(file_id.clone(), mount_id.to_owned(), path.to_owned(), file),
    );

    if let Some(parent_path) = remote_path_utils::parent_path(path) {
        let parent_id = selectors::get_file_id(mount_id, &parent_path.to_lowercase());

        add_child(state, &parent_id, file_id);
    }

    mutation_state
        .remote_files
        .created_files
        .push((mount_id.to_owned(), path.to_owned()));

    mutation_notify(store::MutationEvent::RemoteFiles, state, mutation_state);
}

pub fn remove_child(state: &mut store::State, parent_id: &RemoteFileId, child_id: &RemoteFileId) {
    if let Some(children) = state.remote_files.children.get_mut(parent_id) {
        children.retain(|id| &id != &child_id);
    }
}

pub fn file_removed(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    path: &RemotePath,
) {
    notify(store::Event::RemoteFiles);

    let file_id = selectors::get_file_id(mount_id, &path.to_lowercase());

    if let Some(parent_path) = remote_path_utils::parent_path(path) {
        let parent_id = selectors::get_file_id(mount_id, &parent_path.to_lowercase());

        remove_child(state, &parent_id, &file_id);
    }

    cleanup_file(state, &file_id);

    mutation_state
        .remote_files
        .removed_files
        .push((mount_id.to_owned(), path.to_owned()));

    mutation_notify(store::MutationEvent::RemoteFiles, state, mutation_state);
}

pub fn cleanup_file(state: &mut store::State, file_id: &RemoteFileId) {
    state.remote_files.files.remove(file_id);

    let file_id_prefix = if file_id.0.ends_with('/') {
        file_id.0.to_owned()
    } else {
        format!("{}/", file_id.0)
    };

    state
        .remote_files
        .files
        .retain(|file_id, _| !file_id.0.starts_with(&file_id_prefix));

    state.remote_files.children.remove(file_id);

    state
        .remote_files
        .children
        .retain(|file_id, _| !file_id.0.starts_with(&file_id_prefix));
}

pub fn file_copied(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    new_path: &RemotePath,
    new_file: models::FilesFile,
) {
    notify(store::Event::RemoteFiles);

    let new_file_id = selectors::get_file_id(mount_id, &new_path.to_lowercase());
    let new_parent_path = match remote_path_utils::parent_path(new_path) {
        Some(new_parent_path) => new_parent_path,
        None => {
            return;
        }
    };
    let new_parent_id = selectors::get_file_id(mount_id, &new_parent_path.to_lowercase());

    state.remote_files.files.insert(
        new_file_id.clone(),
        files_file_to_remote_file(
            new_file_id.clone(),
            mount_id.to_owned(),
            new_path.to_owned(),
            new_file,
        ),
    );

    add_child(state, &new_parent_id, new_file_id.clone());

    mutation_state
        .remote_files
        .created_files
        .push((mount_id.to_owned(), new_path.to_owned()));

    mutation_notify(store::MutationEvent::RemoteFiles, state, mutation_state);
}

pub fn file_moved(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    old_path: &RemotePath,
    new_path: &RemotePath,
    new_file: models::FilesFile,
) {
    notify(store::Event::RemoteFiles);

    let old_file_id = selectors::get_file_id(mount_id, &old_path.to_lowercase());
    let old_parent_path = match remote_path_utils::parent_path(old_path) {
        Some(old_parent_path) => old_parent_path,
        None => {
            return;
        }
    };
    let old_parent_id = selectors::get_file_id(mount_id, &old_parent_path.to_lowercase());

    let new_file_id = selectors::get_file_id(mount_id, &new_path.to_lowercase());
    let new_parent_path = match remote_path_utils::parent_path(new_path) {
        Some(new_parent_path) => new_parent_path,
        None => {
            return;
        }
    };

    ensure_dirs(
        state,
        notify,
        mutation_state,
        mutation_notify,
        mount_id,
        &new_parent_path,
    );

    let new_parent_id = selectors::get_file_id(mount_id, &new_parent_path.to_lowercase());

    if let Some(_) = state.remote_files.files.remove(&old_file_id) {
        file_children_change_parent_path(state, &old_file_id, new_path);
    }

    state.remote_files.files.insert(
        new_file_id.clone(),
        files_file_to_remote_file(
            new_file_id.clone(),
            mount_id.to_owned(),
            new_path.to_owned(),
            new_file,
        ),
    );

    remove_child(state, &old_parent_id, &old_file_id);

    add_child(state, &new_parent_id, new_file_id.clone());

    mutation_state.remote_files.moved_files.push((
        mount_id.to_owned(),
        old_path.to_owned(),
        new_path.to_owned(),
    ));

    mutation_notify(store::MutationEvent::RemoteFiles, state, mutation_state);
}

pub fn file_tags_set(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    path: &RemotePath,
    tags: HashMap<String, Vec<String>>,
    conditions: &RemoteFileTagsSetConditions,
) {
    let file_id = selectors::get_file_id(mount_id, &path.to_lowercase());

    let Some(file) = state.remote_files.files.get_mut(&file_id) else {
        return;
    };

    if set_tags(
        file.size,
        file.modified,
        file.hash.as_deref(),
        &mut file.tags,
        tags,
        conditions,
    )
    .is_ok()
    {
        notify(store::Event::RemoteFiles);

        mutation_state
            .remote_files
            .tags_updated
            .push((mount_id.to_owned(), path.to_owned()));

        mutation_notify(store::MutationEvent::RemoteFiles, state, mutation_state);
    }
}

pub fn file_tags_updated(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    path: &RemotePath,
    file: models::FilesFile,
) {
    notify(store::Event::RemoteFiles);

    let file_id = selectors::get_file_id(mount_id, &path.to_lowercase());

    state.remote_files.files.insert(
        file_id.clone(),
        files_file_to_remote_file(file_id.clone(), mount_id.to_owned(), path.to_owned(), file),
    );

    if let Some(parent_path) = remote_path_utils::parent_path(path) {
        let parent_id = selectors::get_file_id(mount_id, &parent_path.to_lowercase());

        add_child(state, &parent_id, file_id);
    }

    mutation_state
        .remote_files
        .tags_updated
        .push((mount_id.to_owned(), path.to_owned()));

    mutation_notify(store::MutationEvent::RemoteFiles, state, mutation_state);
}

pub fn ensure_dirs(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    path: &RemotePath,
) {
    for path in remote_path_utils::paths_chain(path) {
        ensure_dir(
            state,
            notify,
            mutation_state,
            mutation_notify,
            mount_id,
            &path,
        );
    }
}

pub fn ensure_dir(
    state: &mut store::State,
    notify: &store::Notify,
    mutation_state: &mut store::MutationState,
    mutation_notify: &store::MutationNotify,
    mount_id: &MountId,
    path: &RemotePath,
) {
    let file_id = selectors::get_file_id(mount_id, &path.to_lowercase());

    if state.remote_files.files.contains_key(&file_id) {
        return;
    }

    let name = match remote_path_utils::path_to_name(path) {
        Some(name) => name,
        None => return,
    };

    file_created(
        state,
        notify,
        mutation_state,
        mutation_notify,
        mount_id,
        &path,
        models::FilesFile {
            name,
            typ: "dir".into(),
            modified: 0,
            size: 0,
            content_type: "".into(),
            hash: None,
            tags: HashMap::new(),
        },
    );
}

pub fn file_children_change_parent_path(
    state: &mut store::State,
    parent_file_id: &RemoteFileId,
    new_parent_path: &RemotePath,
) {
    if let Some(old_children_ids) = state
        .remote_files
        .children
        .get(parent_file_id)
        .map(|ids| ids.clone())
    {
        let new_children_ids: Vec<RemoteFileId> = Vec::with_capacity(old_children_ids.len());

        for old_child_id in &old_children_ids {
            if let Some(mut child) = state.remote_files.files.remove(old_child_id) {
                let new_child_path =
                    remote_path_utils::join_path_name(new_parent_path, &child.name);
                let new_child_id =
                    selectors::get_file_id(&child.mount_id, &new_child_path.to_lowercase());

                file_children_change_parent_path(state, old_child_id, &new_child_path);

                child.id = new_child_id.clone();
                child.path = new_child_path;

                state.remote_files.files.insert(new_child_id.clone(), child);
            }
        }

        state
            .remote_files
            .children
            .insert(parent_file_id.to_owned(), new_children_ids);
    }
}
