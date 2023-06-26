use std::sync::Arc;

use futures::{
    future::{self, BoxFuture},
    io::Cursor,
};

use crate::{
    dialogs,
    eventstream::{self, service::MountSubscription},
    remote_files::errors::RemoteFilesErrors,
    repo_files::{
        errors::{
            self as repo_files_errors, CreateDirError, CreateFileError, DeleteFileError,
            LoadFilesError, RepoFilesErrors,
        },
        state::{RepoFile, RepoFilesSortField, RepoFilesUploadConflictResolution},
        RepoFilesService,
    },
    repo_files_move::{state::RepoFilesMoveMode, RepoFilesMoveService},
    repo_files_read::{
        errors::GetFilesReaderError, state::RepoFileReaderProvider, RepoFilesReadService,
    },
    repos::selectors as repos_selectors,
    store,
    utils::{
        name_utils,
        path_utils::{self, normalize_path},
    },
};

use super::{
    mutations, selectors,
    state::{RepoFilesBrowserLocation, RepoFilesBrowserOptions},
};

pub struct RepoFilesBrowsersService {
    repo_files_service: Arc<RepoFilesService>,
    repo_files_read_service: Arc<RepoFilesReadService>,
    repo_files_move_service: Arc<RepoFilesMoveService>,
    eventstream_service: Arc<eventstream::EventStreamService>,
    dialogs_service: Arc<dialogs::DialogsService>,
    store: Arc<store::Store>,
    repo_files_mutation_subscription_id: u32,
}

impl RepoFilesBrowsersService {
    pub fn new(
        repo_files_service: Arc<RepoFilesService>,
        repo_files_read_service: Arc<RepoFilesReadService>,
        repo_files_move_service: Arc<RepoFilesMoveService>,
        eventstream_service: Arc<eventstream::EventStreamService>,
        dialogs_service: Arc<dialogs::DialogsService>,
        store: Arc<store::Store>,
    ) -> Self {
        let repo_files_mutation_subscription_id = store.get_next_id();

        store.mutation_on(
            repo_files_mutation_subscription_id,
            &[store::MutationEvent::RepoFiles],
            Box::new(move |state, notify, _, _| {
                mutations::handle_repo_files_mutation(state, notify);
            }),
        );

        Self {
            repo_files_service,
            repo_files_read_service,
            repo_files_move_service,
            eventstream_service,
            dialogs_service,
            store: store.clone(),
            repo_files_mutation_subscription_id,
        }
    }

    pub fn create(
        self: Arc<Self>,
        repo_id: &str,
        path: &str,
        options: RepoFilesBrowserOptions,
    ) -> (
        u32,
        BoxFuture<'static, Result<(), repo_files_errors::LoadFilesError>>,
    ) {
        let location = self.clone().get_location(repo_id, path);

        let browser_id = self.store.mutate(|state, notify, _, _| {
            notify(store::Event::RepoFilesBrowsers);

            mutations::create(state, options, location)
        });

        let load_self = self.clone();

        let load_future: BoxFuture<'static, Result<(), repo_files_errors::LoadFilesError>> = if self
            .store
            .with_state(|state| selectors::select_is_unlocked(state, browser_id))
        {
            Box::pin(async move { load_self.load_files(browser_id).await })
        } else {
            Box::pin(future::ready(Ok(())))
        };

        (browser_id, load_future)
    }

    fn get_location(
        &self,
        repo_id: &str,
        path: &str,
    ) -> Result<RepoFilesBrowserLocation, repo_files_errors::LoadFilesError> {
        normalize_path(path)
            .map(|path| {
                let eventstream_mount_subscription =
                    self.clone().get_eventstream_mount_subscription(repo_id);

                RepoFilesBrowserLocation {
                    repo_id: repo_id.to_owned(),
                    path,
                    eventstream_mount_subscription,
                }
            })
            .map_err(|_| {
                repo_files_errors::LoadFilesError::RemoteError(RemoteFilesErrors::invalid_path())
            })
    }

    fn get_eventstream_mount_subscription(&self, repo_id: &str) -> Option<Arc<MountSubscription>> {
        self.store
            .with_state(|state| {
                repos_selectors::select_repo(state, repo_id)
                    .map(|repo| (repo.mount_id.clone(), repo.path.clone()))
            })
            .ok()
            .map(|(mount_id, mount_path)| {
                self.eventstream_service
                    .clone()
                    .get_mount_subscription(&mount_id, &mount_path)
            })
    }

    pub fn destroy(&self, browser_id: u32) {
        self.store.mutate(|state, notify, _, _| {
            notify(store::Event::RepoFilesBrowsers);

            mutations::destroy(state, browser_id);
        });
    }

    pub async fn set_location(
        &self,
        browser_id: u32,
        repo_id: &str,
        path: &str,
    ) -> Result<(), repo_files_errors::LoadFilesError> {
        let location = self.clone().get_location(repo_id, path);

        self.store.mutate(|state, notify, _, _| {
            notify(store::Event::RepoFilesBrowsers);

            mutations::set_location(state, browser_id, location);
        });

        if self
            .store
            .with_state(|state| selectors::select_is_unlocked(state, browser_id))
        {
            self.load_files(browser_id).await?;
        }

        Ok(())
    }

    pub async fn load_files(
        &self,
        browser_id: u32,
    ) -> Result<(), repo_files_errors::LoadFilesError> {
        if let Some((repo_id, path)) = self
            .store
            .with_state(|state| selectors::select_repo_id_path_owned(state, browser_id))
        {
            let res = self.repo_files_service.load_files(&repo_id, &path).await;

            self.store.mutate(|state, notify, _, _| {
                notify(store::Event::RepoFilesBrowsers);

                mutations::loaded(state, browser_id, &repo_id, &path, res.as_ref().err());
            });

            res?;
        }

        Ok(())
    }

    pub fn select_file(
        &self,
        browser_id: u32,
        file_id: &str,
        extend: bool,
        range: bool,
        force: bool,
    ) {
        self.store.mutate(|state, notify, _, _| {
            notify(store::Event::RepoFilesBrowsers);

            mutations::select_file(state, browser_id, file_id, extend, range, force);
        });
    }

    pub fn toggle_select_all(&self, browser_id: u32) {
        self.store.mutate(|state, notify, _, _| {
            notify(store::Event::RepoFilesBrowsers);

            mutations::toggle_select_all(state, browser_id);
        });
    }

    pub fn clear_selection(&self, browser_id: u32) {
        self.store.mutate(|state, notify, _, _| {
            notify(store::Event::RepoFilesBrowsers);

            mutations::clear_selection(state, browser_id);
        });
    }

    pub fn sort_by(&self, browser_id: u32, field: RepoFilesSortField) {
        self.store.mutate(|state, notify, _, _| {
            notify(store::Event::RepoFilesBrowsers);

            mutations::sort_by(state, browser_id, field);
        });
    }

    pub fn get_selected_reader(
        self: Arc<Self>,
        browser_id: u32,
    ) -> Result<RepoFileReaderProvider, GetFilesReaderError> {
        let files: Vec<RepoFile> = self.store.with_state(|state| {
            selectors::select_selected_files(state, browser_id)
                .into_iter()
                .map(|file| file.clone())
                .collect()
        });

        if files.is_empty() {
            return Err(GetFilesReaderError::FilesEmpty);
        }

        self.repo_files_read_service.clone().get_files_reader(files)
    }

    pub async fn create_dir(&self, browser_id: u32) -> Result<(), CreateDirError> {
        let (repo_id, parent_path) =
            self.store
                .with_state::<_, Result<_, CreateDirError>>(|state| {
                    let root_file = selectors::select_root_file(state, browser_id)
                        .ok_or_else(RepoFilesErrors::not_found)?;

                    let root_path = root_file.decrypted_path()?;

                    Ok((root_file.repo_id.clone(), root_path.to_owned()))
                })?;

        let input_value_validator_store = self.store.clone();

        if let Some(name) = self
            .dialogs_service
            .show(dialogs::state::DialogShowOptions {
                input_value_validator: Some(Box::new(move |value| {
                    input_value_validator_store
                        .with_state(|state| {
                            selectors::select_check_create_file(state, browser_id, value)
                        })
                        .is_ok()
                })),
                input_placeholder: Some(String::from("Folder name")),
                confirm_button_text: String::from("Create folder"),
                ..self
                    .dialogs_service
                    .build_prompt(String::from("Enter new folder name"))
            })
            .await
        {
            self.repo_files_service
                .create_dir(&repo_id, &parent_path, &name)
                .await?;
        }

        Ok(())
    }

    pub async fn create_file(
        &self,
        browser_id: u32,
        name: &str,
    ) -> Result<String, CreateFileError> {
        let (repo_id, parent_path) =
            self.store
                .with_state::<_, Result<_, CreateFileError>>(|state| {
                    let root_file = selectors::select_root_file(state, browser_id)
                        .ok_or_else(RepoFilesErrors::not_found)?;

                    let root_path = root_file.decrypted_path()?;

                    Ok((root_file.repo_id.clone(), root_path.to_owned()))
                })?;

        let input_value = name.to_owned();
        let input_value_validator_store = self.store.clone();
        let input_value_selected = Some(name_utils::split_name_ext(&input_value).0.to_owned());

        if let Some(name) = self
            .dialogs_service
            .show(dialogs::state::DialogShowOptions {
                input_value,
                input_value_validator: Some(Box::new(move |value| {
                    input_value_validator_store
                        .with_state(|state| {
                            selectors::select_check_create_file(state, browser_id, value)
                        })
                        .is_ok()
                })),
                input_value_selected,
                input_placeholder: Some(String::from("File name")),
                confirm_button_text: String::from("Create file"),
                ..self
                    .dialogs_service
                    .build_prompt(String::from("Enter new file name"))
            })
            .await
        {
            Ok(self
                .repo_files_service
                .clone()
                .upload_file_reader(
                    &repo_id,
                    &parent_path,
                    &name,
                    Box::pin(Cursor::new(vec![])),
                    Some(0),
                    RepoFilesUploadConflictResolution::Error,
                    None,
                )
                .await
                .map(|_| path_utils::join_path_name(&parent_path, &name))?)
        } else {
            Err(CreateFileError::Canceled)
        }
    }

    pub async fn delete_selected(&self, browser_id: u32) -> Result<(), DeleteFileError> {
        let files = self.store.with_state(|state| {
            selectors::select_selected_files(state, browser_id)
                .into_iter()
                .filter_map(|file| {
                    file.path
                        .decrypted_path()
                        .ok()
                        .map(|path| (file.repo_id.clone(), path.to_owned()))
                })
                .collect::<Vec<_>>()
        });

        self.repo_files_service.delete_files(&files, None).await?;

        Ok(())
    }

    pub async fn move_selected(
        &self,
        browser_id: u32,
        mode: RepoFilesMoveMode,
    ) -> Result<(), LoadFilesError> {
        let repo_id = match self
            .store
            .with_state(|state| selectors::select_repo_id(state, browser_id).map(str::to_string))
        {
            Some(repo_id) => repo_id,
            None => {
                return Ok(());
            }
        };

        let src_file_ids = self.store.with_state(|state| {
            selectors::select_selected_file_ids(state, browser_id)
                .into_iter()
                .map(|id| id.to_owned())
                .collect::<Vec<String>>()
        });

        self.repo_files_move_service
            .show(&repo_id, src_file_ids, mode)
            .await
    }
}

impl Drop for RepoFilesBrowsersService {
    fn drop(&mut self) {
        self.store
            .remove_listener(self.repo_files_mutation_subscription_id)
    }
}
