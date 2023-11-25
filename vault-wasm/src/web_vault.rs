use std::{
    collections::{hash_map, HashMap},
    sync::{Arc, Mutex},
    time::Duration,
};

use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{AbortSignal, Storage};

use vault_core::{
    dir_pickers::state::DirPickerItemId,
    store::Event,
    transfers,
    types::{DecryptedName, EncryptedPath, RepoFileId, RepoId},
};

use crate::{
    browser_eventstream_websocket_client::{
        BrowserEventstreamWebSocketClient, BrowserEventstreamWebSocketDelegate,
    },
    browser_http_client::{BrowserHttpClient, BrowserHttpClientDelegate},
    browser_runtime::{now_ms, BrowserRuntime},
    browser_secure_storage::BrowserSecureStorage,
    browser_uploadable::BrowserUploadable,
    dto, helpers,
    web_errors::WebErrors,
    web_subscription::WebSubscription,
};

#[wasm_bindgen(typescript_custom_section)]
const FILE_STREAM: &'static str = r#"
export interface FileStream {
  name: string;
  stream?: ReadableStream;
  blob?: Blob;
  size: SizeInfo;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "number[]")]
    pub type IdVec;

    #[wasm_bindgen(typescript_type = "Status")]
    pub type Status;

    #[wasm_bindgen(typescript_type = "RelativeTime")]
    pub type RelativeTime;

    #[wasm_bindgen(typescript_type = "Notification[]")]
    pub type NotificationVec;

    #[wasm_bindgen(typescript_type = "Dialog | undefined")]
    pub type DialogOption;

    #[wasm_bindgen(typescript_type = "File | Blob")]
    pub type FileOrBlob;

    #[wasm_bindgen(typescript_type = "Uint8Array | undefined")]
    pub type FileBytes;

    #[wasm_bindgen(typescript_type = "User | undefined")]
    pub type UserOption;

    #[wasm_bindgen(typescript_type = "Uint8Array | undefined")]
    pub type UserProfilePicture;

    #[wasm_bindgen(typescript_type = "FileIconProps")]
    pub type FileIconProps;

    #[wasm_bindgen(typescript_type = "RepoInfo")]
    pub type RepoInfo;

    #[wasm_bindgen(typescript_type = "Repos")]
    pub type Repos;

    #[wasm_bindgen(typescript_type = "RepoCreateInfo | undefined")]
    pub type RepoCreateInfoOption;

    #[wasm_bindgen(typescript_type = "RepoUnlockOptions")]
    pub type RepoUnlockOptions;

    #[wasm_bindgen(typescript_type = "RepoUnlockInfo | undefined")]
    pub type RepoUnlockInfoOption;

    #[wasm_bindgen(typescript_type = "RepoRemoveInfo | undefined")]
    pub type RepoRemoveInfoOption;

    #[wasm_bindgen(typescript_type = "RepoConfigBackupInfo | undefined")]
    pub type RepoConfigBackupInfoOption;

    #[wasm_bindgen(typescript_type = "RepoSpaceUsageInfo | undefined")]
    pub type RepoSpaceUsageInfoOption;

    #[wasm_bindgen(typescript_type = "RepoFile | undefined")]
    pub type RepoFileOption;

    #[wasm_bindgen(typescript_type = "RepoFilesBreadcrumb[]")]
    pub type RepoFilesBreadcrumbVec;

    #[wasm_bindgen(typescript_type = "RepoFilesUploadResult | undefined")]
    pub type RepoFilesUploadResultOption;

    #[wasm_bindgen(typescript_type = "RepoFilesBrowserOptions")]
    pub type RepoFilesBrowserOptions;

    #[wasm_bindgen(typescript_type = "RepoFilesBrowserInfo | undefined")]
    pub type RepoFilesBrowserInfoOption;

    #[wasm_bindgen(typescript_type = "RepoFilesBrowserItem[]")]
    pub type RepoFilesBrowserItemVec;

    #[wasm_bindgen(typescript_type = "RepoFilesDetailsOptions")]
    pub type RepoFilesDetailsOptions;

    #[wasm_bindgen(typescript_type = "RepoFilesDetailsInfo | undefined")]
    pub type RepoFilesDetailsInfoOption;

    #[wasm_bindgen(typescript_type = "RepoFilesMoveInfo | undefined")]
    pub type RepoFilesMoveInfoOption;

    #[wasm_bindgen(typescript_type = "TransfersSummary")]
    pub type TransfersSummary;

    #[wasm_bindgen(typescript_type = "TransfersList")]
    pub type TransfersList;

    #[wasm_bindgen(typescript_type = "FileStream | undefined")]
    pub type FileStreamOption;

    #[wasm_bindgen(typescript_type = "DirPickerItem[]")]
    pub type DirPickerItemVec;

    #[wasm_bindgen(typescript_type = "SpaceUsage | undefined")]
    pub type SpaceUsageOption;
}

pub fn to_js<In: serde::ser::Serialize + ?Sized, Out: From<JsValue> + Into<JsValue>>(
    value: &In,
) -> Out {
    serde_wasm_bindgen::to_value(value).unwrap().into()
}

type Data<T> = Arc<Mutex<HashMap<u32, T>>>;

#[derive(Clone)]
struct VersionedFileBytes {
    value: JsValue,
    version: u32,
}

unsafe impl Send for VersionedFileBytes {}

#[derive(Default)]
struct SubscriptionData {
    notifications: Data<Vec<dto::Notification>>,
    dialogs: Data<Vec<u32>>,
    dialog: Data<Option<dto::Dialog>>,
    oauth2_status: Data<dto::Status>,
    user: Data<Option<dto::User>>,
    user_profile_picture_loaded: Data<bool>,
    repos: Data<dto::Repos>,
    repos_repo: Data<dto::RepoInfo>,
    repo_create_info: Data<Option<dto::RepoCreateInfo>>,
    repo_unlock_info: Data<Option<dto::RepoUnlockInfo>>,
    repo_remove_info: Data<Option<dto::RepoRemoveInfo>>,
    repo_config_backup_info: Data<Option<dto::RepoConfigBackupInfo>>,
    repo_space_usage_info: Data<Option<dto::RepoSpaceUsageInfo>>,
    repo_files_file: Data<Option<dto::RepoFile>>,
    transfers_is_active: Data<bool>,
    transfers_summary: Data<dto::TransfersSummary>,
    transfers_list: Data<dto::TransfersList>,
    dir_pickers_items: Data<Vec<dto::DirPickerItem>>,
    repo_files_browsers_info: Data<Option<dto::RepoFilesBrowserInfo>>,
    repo_files_details_info: Data<Option<dto::RepoFilesDetailsInfo>>,
    repo_files_details_file: Data<Option<dto::RepoFile>>,
    repo_files_details_content_bytes: Data<VersionedFileBytes>,
    repo_files_move_info: Data<Option<dto::RepoFilesMoveInfo>>,
    space_usage: Data<Option<dto::SpaceUsage>>,
}

#[wasm_bindgen]
pub struct WebVault {
    vault: Arc<vault_core::Vault>,
    errors: Arc<WebErrors>,
    subscription_data: SubscriptionData,
    subscription: WebSubscription,
    file_icon_factory: vault_file_icon::FileIconFactory,
}

#[wasm_bindgen]
impl WebVault {
    #[wasm_bindgen(constructor)]
    pub fn new(
        base_url: String,
        oauth2_auth_base_url: String,
        oauth2_client_id: String,
        oauth2_client_secret: String,
        oauth2_redirect_uri: String,
        browser_http_client_delegate: BrowserHttpClientDelegate,
        browser_eventstream_websocket_delegate: BrowserEventstreamWebSocketDelegate,
        storage: Storage,
    ) -> Self {
        let oauth2_config = vault_core::oauth2::OAuth2Config {
            base_url: base_url.clone(),
            auth_base_url: oauth2_auth_base_url.clone(),
            client_id: oauth2_client_id,
            client_secret: oauth2_client_secret,
            redirect_uri: oauth2_redirect_uri,
        };

        let vault = Arc::new(vault_core::Vault::new(
            base_url,
            oauth2_config,
            Box::new(BrowserHttpClient::new(browser_http_client_delegate)),
            Box::new(BrowserEventstreamWebSocketClient::new(
                browser_eventstream_websocket_delegate,
            )),
            Box::new(BrowserSecureStorage::new(storage)),
            Box::new(BrowserRuntime::new()),
        ));

        let errors = Arc::new(WebErrors::new(vault.clone()));

        let subscription_data = SubscriptionData::default();
        let subscription = WebSubscription::new(vault.clone());

        let file_icon_theme = vault_file_icon::FileIconTheme::default();
        let file_icon_factory = vault_file_icon::FileIconFactory::new(&file_icon_theme);

        Self {
            vault: vault.clone(),
            errors,
            subscription_data,
            subscription,
            file_icon_factory,
        }
    }

    // errors

    fn handle_error(&self, user_error: impl vault_core::user_error::UserError) {
        self.errors.handle_error(user_error)
    }

    fn handle_result(&self, result: Result<(), impl vault_core::user_error::UserError>) {
        self.errors.handle_result(result)
    }

    // subscription

    fn subscribe<T: Clone + PartialEq + Send + 'static>(
        &self,
        events: &[vault_core::store::Event],
        js_callback: js_sys::Function,
        subscription_data: Arc<Mutex<HashMap<u32, T>>>,
        generate_data: impl Fn(Arc<vault_core::Vault>) -> T + 'static,
    ) -> u32 {
        self.subscription
            .subscribe(events, js_callback, subscription_data, generate_data)
    }

    fn subscribe_changed<T: Clone + Send + 'static>(
        &self,
        events: &[vault_core::store::Event],
        js_callback: js_sys::Function,
        subscription_data: Arc<Mutex<HashMap<u32, T>>>,
        generate_data: impl Fn(Arc<vault_core::Vault>, hash_map::Entry<'_, u32, T>) -> bool + 'static,
    ) -> u32 {
        self.subscription
            .subscribe_changed(events, js_callback, subscription_data, generate_data)
    }

    fn get_data<T: Clone + Send>(
        &self,
        id: u32,
        subscription_data: Arc<Mutex<HashMap<u32, T>>>,
    ) -> Option<T> {
        self.subscription.get_data(id, subscription_data)
    }

    fn get_data_js<T: Clone + Send + Serialize, Out: From<JsValue> + Into<JsValue>>(
        &self,
        id: u32,
        subscription_data: Arc<Mutex<HashMap<u32, T>>>,
    ) -> Out {
        to_js(&self.subscription.get_data(id, subscription_data))
    }

    #[wasm_bindgen(js_name = unsubscribe)]
    pub fn unsubscribe(&self, id: u32) {
        self.subscription.unsubscribe(id)
    }

    // lifecycle

    #[wasm_bindgen(js_name = load)]
    pub async fn load(&self) {
        self.handle_result(self.vault.load().await)
    }

    #[wasm_bindgen(js_name = logout)]
    pub fn logout(&self) {
        self.handle_result(self.vault.logout());
    }

    // relative_time

    #[wasm_bindgen(js_name = relativeTime)]
    pub fn relative_time(&self, value: f64, with_modifier: bool) -> RelativeTime {
        to_js(&dto::RelativeTime::from(
            self.vault.relative_time(value as i64, with_modifier),
        ))
    }

    // notifications

    #[wasm_bindgen(js_name = notificationsSubscribe)]
    pub fn notifications_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::Notifications],
            cb,
            self.subscription_data.notifications.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::notifications::selectors::select_notifications(state)
                        .into_iter()
                        .map(Into::into)
                        .collect()
                })
            },
        )
    }

    #[wasm_bindgen(js_name = notificationsData)]
    pub fn notifications_data(&self, id: u32) -> NotificationVec {
        self.get_data_js(id, self.subscription_data.notifications.clone())
    }

    #[wasm_bindgen(js_name = notificationsRemove)]
    pub fn notifications_remove(&self, notification_id: u32) {
        self.vault.notifications_remove(notification_id)
    }

    #[wasm_bindgen(js_name = notificationsRemoveAfter)]
    pub async fn notifications_remove_after(&self, notification_id: u32, duration_ms: u32) {
        self.vault
            .notifications_remove_after(notification_id, Duration::from_millis(duration_ms as u64))
            .await
    }

    #[wasm_bindgen(js_name = notificationsRemoveAll)]
    pub fn notifications_remove_all(&self) {
        self.vault.notifications_remove_all()
    }

    // dialogs

    #[wasm_bindgen(js_name = dialogsSubscribe)]
    pub fn dialogs_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::Dialogs],
            cb,
            self.subscription_data.dialogs.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::dialogs::selectors::select_dialogs(state)
                        .iter()
                        .map(|dialog| dialog.id)
                        .collect()
                })
            },
        )
    }

    #[wasm_bindgen(js_name = dialogsData)]
    pub fn dialogs_data(&self, id: u32) -> IdVec {
        self.get_data_js(id, self.subscription_data.dialogs.clone())
    }

    #[wasm_bindgen(js_name = dialogsDialogSubscribe)]
    pub fn dialogs_dialog_subscribe(&self, dialog_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::Dialogs],
            cb,
            self.subscription_data.dialog.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::dialogs::selectors::select_dialog_info(state, dialog_id)
                        .map(Into::into)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = dialogsDialogData)]
    pub fn dialogs_dialog_data(&self, id: u32) -> DialogOption {
        self.get_data_js(id, self.subscription_data.dialog.clone())
    }

    #[wasm_bindgen(js_name = dialogsConfirm)]
    pub fn dialogs_confirm(&self, dialog_id: u32) {
        self.vault.dialogs_confirm(dialog_id)
    }

    #[wasm_bindgen(js_name = dialogsCancel)]
    pub fn dialogs_cancel(&self, dialog_id: u32) {
        self.vault.dialogs_cancel(dialog_id)
    }

    #[wasm_bindgen(js_name = dialogsSetInputValue)]
    pub fn dialogs_set_input_value(&self, dialog_id: u32, value: String) {
        self.vault.dialogs_set_input_value(dialog_id, value);
    }

    // oauth2

    #[wasm_bindgen(js_name = oauth2StatusSubscribe)]
    pub fn oauth2_status_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::Auth],
            cb,
            self.subscription_data.oauth2_status.clone(),
            move |vault| {
                vault.with_state(|state| vault_core::oauth2::selectors::select_status(state).into())
            },
        )
    }

    #[wasm_bindgen(js_name = oauth2StatusData)]
    pub fn oauth2_status_data(&self, id: u32) -> Status {
        self.get_data_js(id, self.subscription_data.oauth2_status.clone())
    }

    #[wasm_bindgen(js_name = oauth2StartLoginFlow)]
    pub fn oauth2_start_login_flow(&self) -> Option<String> {
        match self.vault.oauth2_start_login_flow() {
            Ok(url) => Some(url),
            Err(err) => {
                self.handle_error(err);
                None
            }
        }
    }

    #[wasm_bindgen(js_name = oauth2StartLogoutFlow)]
    pub fn oauth2_start_logout_flow(&self) -> Option<String> {
        match self.vault.oauth2_start_logout_flow() {
            Ok(url) => Some(url),
            Err(err) => {
                self.handle_error(err);
                None
            }
        }
    }

    #[wasm_bindgen(js_name = oauth2FinishFlowUrl)]
    pub async fn oauth2_finish_flow_url(&self, url: &str) -> bool {
        let res = self.vault.oauth2_finish_flow_url(url).await;

        let success = res.is_ok();

        self.handle_result(res);

        success
    }

    // config

    #[wasm_bindgen(js_name = configGetBaseUrl)]
    pub fn config_get_base_url(&self) -> String {
        self.vault.with_state(|state| state.config.base_url.clone())
    }

    // user

    #[wasm_bindgen(js_name = userSubscribe)]
    pub fn user_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::User],
            cb,
            self.subscription_data.user.clone(),
            move |vault| vault.with_state(|state| state.user.user.as_ref().map(Into::into)),
        )
    }

    #[wasm_bindgen(js_name = userData)]
    pub fn user_data(&self, id: u32) -> UserOption {
        self.get_data_js(id, self.subscription_data.user.clone())
    }

    #[wasm_bindgen(js_name = userProfilePictureLoadedSubscribe)]
    pub fn user_profile_picture_loaded_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::User],
            cb,
            self.subscription_data.user_profile_picture_loaded.clone(),
            move |vault| {
                vault.with_state(|state| {
                    state
                        .user
                        .user
                        .as_ref()
                        .map(|user| match &user.profile_picture_status {
                            vault_core::common::state::Status::Loaded => true,
                            _ => false,
                        })
                        .unwrap_or(false)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = userProfilePictureLoadedData)]
    pub fn user_profile_picture_loaded_data(&self, id: u32) -> bool {
        self.get_data(
            id,
            self.subscription_data.user_profile_picture_loaded.clone(),
        )
        .unwrap_or(false)
    }

    #[wasm_bindgen(js_name = userGetProfilePicture)]
    pub fn user_get_profile_picture(&self) -> UserProfilePicture {
        UserProfilePicture::from(self.vault.with_state(|state| {
            match state
                .user
                .user
                .as_ref()
                .and_then(|user| user.profile_picture_bytes.as_ref())
            {
                Some(bytes) => helpers::bytes_to_array(&bytes),
                None => JsValue::UNDEFINED,
            }
        }))
    }

    #[wasm_bindgen(js_name = userEnsureProfilePicture)]
    pub async fn user_ensure_profile_picture(&self) {
        self.handle_result(self.vault.user_ensure_profile_picture().await)
    }

    // file_icon

    #[wasm_bindgen(js_name = fileIconSvg)]
    pub fn file_icon_svg(&self, props: FileIconProps) -> String {
        let props: dto::FileIconProps = serde_wasm_bindgen::from_value(props.into()).unwrap();
        let (svg, _, _) = self.file_icon_factory.generate_svg(&props.into());
        svg
    }

    // repos

    #[wasm_bindgen(js_name = reposSubscribe)]
    pub fn repos_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::Repos],
            cb,
            self.subscription_data.repos.clone(),
            move |vault| vault.with_state(|state| dto::Repos::from(state)),
        )
    }

    #[wasm_bindgen(js_name = reposData)]
    pub fn repos_data(&self, id: u32) -> Repos {
        self.get_data_js(id, self.subscription_data.repos.clone())
    }

    #[wasm_bindgen(js_name = reposRepoSubscribe)]
    pub fn repos_repo_subscribe(&self, repo_id: String, cb: js_sys::Function) -> u32 {
        let repo_id = RepoId(repo_id);

        self.subscribe(
            &[Event::Repos],
            cb,
            self.subscription_data.repos_repo.clone(),
            move |vault| {
                vault.with_state(|state| {
                    (&vault_core::repos::selectors::select_repo_info(state, &repo_id)).into()
                })
            },
        )
    }

    #[wasm_bindgen(js_name = reposRepoData)]
    pub fn repos_repo_data(&self, id: u32) -> RepoInfo {
        self.get_data_js(id, self.subscription_data.repos_repo.clone())
    }

    #[wasm_bindgen(js_name = reposLockRepo)]
    pub fn repos_lock_repo(&self, repo_id: String) {
        self.handle_result(self.vault.repos_lock_repo(&RepoId(repo_id)))
    }

    // repo_create

    #[wasm_bindgen(js_name = repoCreateCreate)]
    pub fn repo_create_create(&self) -> u32 {
        let (create_id, create_load_future) = self.vault.repo_create_create();

        spawn_local(async move {
            // error is displayed in the details component
            let _ = create_load_future.await;
        });

        create_id
    }

    #[wasm_bindgen(js_name = repoCreateInfoSubscribe)]
    pub fn repo_create_info_subscribe(&self, create_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoCreate, Event::DirPickers],
            cb,
            self.subscription_data.repo_create_info.clone(),
            move |vault| {
                use vault_core::{
                    remote_files::selectors as remote_files_selectors,
                    repo_create::{selectors, state::RepoCreate},
                };

                vault.with_state(|state| {
                    vault_core::repo_create::selectors::select_repo_create(state, create_id).map(
                        |repo_create| match repo_create {
                            vault_core::repo_create::state::RepoCreate::Form(form) => {
                                let location_breadcrumbs = form
                                    .location
                                    .as_ref()
                                    .map(|location| {
                                        remote_files_selectors::select_breadcrumbs(
                                            state,
                                            &location.mount_id,
                                            &location.path,
                                        )
                                    })
                                    .unwrap_or(Vec::new());

                                dto::RepoCreateInfo::Form(dto::RepoCreateForm {
                                    create_load_status: (&form.create_load_status).into(),
                                    location: form
                                        .location
                                        .as_ref()
                                        .map(|location| location.into()),
                                    location_breadcrumbs: location_breadcrumbs
                                        .iter()
                                        .map(dto::RemoteFilesBreadcrumb::from)
                                        .collect(),
                                    location_dir_picker_id: form.location_dir_picker_id,
                                    location_dir_picker_can_select:
                                        selectors::select_location_dir_picker_can_select(
                                            state, create_id,
                                        ),
                                    location_dir_picker_create_dir_enabled:
                                        selectors::select_location_dir_picker_create_dir_enabled(
                                            state, create_id,
                                        ),
                                    password: form.password.clone(),
                                    salt: form.salt.clone(),
                                    fill_from_rclone_config_error: form
                                        .fill_from_rclone_config_error
                                        .as_ref()
                                        .map(|e| e.to_string()),
                                    can_create: selectors::select_can_create(state, create_id),
                                    create_repo_status: (&form.create_repo_status).into(),
                                })
                            }
                            RepoCreate::Created(created) => {
                                dto::RepoCreateInfo::Created(created.into())
                            }
                        },
                    )
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoCreateInfoData)]
    pub fn repo_create_info_data(&self, id: u32) -> RepoCreateInfoOption {
        self.get_data_js(id, self.subscription_data.repo_create_info.clone())
    }

    #[wasm_bindgen(js_name = repoCreateSetPassword)]
    pub fn repo_create_set_password(&self, create_id: u32, password: String) {
        self.vault.repo_create_set_password(create_id, password)
    }

    #[wasm_bindgen(js_name = repoCreateSetSalt)]
    pub fn repo_create_set_salt(&self, create_id: u32, salt: Option<String>) {
        self.vault.repo_create_set_salt(create_id, salt)
    }

    #[wasm_bindgen(js_name = repoCreateFillFromRcloneConfig)]
    pub fn repo_create_fill_from_rclone_config(&self, create_id: u32, config: String) {
        let _ = self
            .vault
            .repo_create_fill_from_rclone_config(create_id, config);
    }

    #[wasm_bindgen(js_name = repoCreateLocationDirPickerShow)]
    pub async fn repo_create_location_dir_picker_show(&self, create_id: u32) {
        self.handle_result(
            self.vault
                .repo_create_location_dir_picker_show(create_id)
                .await,
        )
    }

    #[wasm_bindgen(js_name = repoCreateLocationDirPickerClick)]
    pub async fn repo_create_location_dir_picker_click(
        &self,
        create_id: u32,
        item_id: String,
        is_arrow: bool,
    ) {
        self.handle_result(
            self.vault
                .repo_create_location_dir_picker_click(
                    create_id,
                    &DirPickerItemId(item_id),
                    is_arrow,
                )
                .await,
        )
    }

    #[wasm_bindgen(js_name = repoCreateLocationDirPickerSelect)]
    pub fn repo_create_location_dir_picker_select(&self, create_id: u32) {
        self.vault.repo_create_location_dir_picker_select(create_id)
    }

    #[wasm_bindgen(js_name = repoCreateLocationDirPickerCancel)]
    pub fn repo_create_location_dir_picker_cancel(&self, create_id: u32) {
        self.vault.repo_create_location_dir_picker_cancel(create_id)
    }

    #[wasm_bindgen(js_name = repoCreateLocationDirPickerCreateDir)]
    pub async fn repo_create_location_dir_picker_create_dir(&self, create_id: u32) {
        self.handle_result(
            match self
                .vault
                .repo_create_location_dir_picker_create_dir(create_id)
                .await
            {
                Ok(_) => Ok(()),
                Err(vault_core::remote_files::errors::CreateDirError::Canceled) => Ok(()),
                Err(err) => Err(err),
            },
        )
    }

    #[wasm_bindgen(js_name = repoCreateCreateRepo)]
    pub async fn repo_create_create_repo(&self, create_id: u32) {
        self.vault.repo_create_create_repo(create_id).await
    }

    #[wasm_bindgen(js_name = repoCreateDestroy)]
    pub fn repo_create_destroy(&self, create_id: u32) {
        self.vault.repo_create_destroy(create_id);
    }

    // repo_unlock

    #[wasm_bindgen(js_name = repoUnlockCreate)]
    pub fn repo_unlock_create(&self, repo_id: String, options: RepoUnlockOptions) -> u32 {
        let options: dto::RepoUnlockOptions =
            serde_wasm_bindgen::from_value(options.into()).unwrap();

        self.vault
            .repo_unlock_create(RepoId(repo_id), options.into())
    }

    #[wasm_bindgen(js_name = repoUnlockInfoSubscribe)]
    pub fn repo_unlock_info_subscribe(&self, unlock_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoUnlock],
            cb,
            self.subscription_data.repo_unlock_info.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::repo_unlock::selectors::select_info(state, unlock_id).map(|info| {
                        dto::RepoUnlockInfo {
                            status: info.status.into(),
                            repo_name: info.repo_name.map(|x| x.0.clone()),
                        }
                    })
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoUnlockInfoData)]
    pub fn repo_unlock_info_data(&self, id: u32) -> RepoUnlockInfoOption {
        self.get_data_js(id, self.subscription_data.repo_unlock_info.clone())
    }

    #[wasm_bindgen(js_name = repoUnlockUnlock)]
    pub fn repo_unlock_unlock(&self, unlock_id: u32, password: &str) {
        let _ = self.vault.repo_unlock_unlock(unlock_id, password);
    }

    #[wasm_bindgen(js_name = repoUnlockDestroy)]
    pub fn repo_unlock_destroy(&self, unlock_id: u32) {
        self.vault.repo_unlock_destroy(unlock_id)
    }

    // repo_remove

    #[wasm_bindgen(js_name = repoRemoveCreate)]
    pub fn repo_remove_create(&self, repo_id: String) -> u32 {
        self.vault.repo_remove_create(RepoId(repo_id))
    }

    #[wasm_bindgen(js_name = repoRemoveInfoSubscribe)]
    pub fn repo_remove_info_subscribe(&self, remove_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoRemove],
            cb,
            self.subscription_data.repo_remove_info.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::repo_remove::selectors::select_info(state, remove_id)
                        .as_ref()
                        .map(Into::into)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoRemoveInfoData)]
    pub fn repo_remove_info_data(&self, id: u32) -> RepoRemoveInfoOption {
        self.get_data_js(id, self.subscription_data.repo_remove_info.clone())
    }

    #[wasm_bindgen(js_name = repoRemoveRemove)]
    pub async fn repo_remove_remove(&self, remove_id: u32, password: &str) -> bool {
        self.vault
            .repo_remove_remove(remove_id, password)
            .await
            .is_ok()
    }

    #[wasm_bindgen(js_name = repoRemoveDestroy)]
    pub fn repo_remove_destroy(&self, remove_id: u32) {
        self.vault.repo_remove_destroy(remove_id)
    }

    // repo_config_backup

    #[wasm_bindgen(js_name = repoConfigBackupCreate)]
    pub fn repo_config_backup_create(&self, repo_id: String) -> u32 {
        self.vault.repo_config_backup_create(RepoId(repo_id))
    }

    #[wasm_bindgen(js_name = repoConfigBackupInfoSubscribe)]
    pub fn repo_config_backup_info_subscribe(&self, backup_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoConfigBackup],
            cb,
            self.subscription_data.repo_config_backup_info.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::repo_config_backup::selectors::select_info(state, backup_id)
                        .as_ref()
                        .map(Into::into)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoConfigBackupInfoData)]
    pub fn repo_config_backup_info_data(&self, id: u32) -> RepoConfigBackupInfoOption {
        self.get_data_js(id, self.subscription_data.repo_config_backup_info.clone())
    }

    #[wasm_bindgen(js_name = repoConfigBackupGenerate)]
    pub fn repo_config_backup_generate(&self, backup_id: u32, password: &str) {
        let _ = self.vault.repo_config_backup_generate(backup_id, password);
    }

    #[wasm_bindgen(js_name = repoConfigBackupDestroy)]
    pub fn repo_config_backup_destroy(&self, backup_id: u32) {
        self.vault.repo_config_backup_destroy(backup_id)
    }

    // repo_space_usage

    #[wasm_bindgen(js_name = repoSpaceUsageCreate)]
    pub fn repo_space_usage_create(&self, repo_id: String) -> u32 {
        self.vault.repo_space_usage_create(RepoId(repo_id))
    }

    #[wasm_bindgen(js_name = repoSpaceUsageInfoSubscribe)]
    pub fn repo_space_usage_info_subscribe(&self, usage_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoSpaceUsage],
            cb,
            self.subscription_data.repo_space_usage_info.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::repo_space_usage::selectors::select_info(state, usage_id)
                        .as_ref()
                        .map(Into::into)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoSpaceUsageInfoData)]
    pub fn repo_space_usage_info_data(&self, id: u32) -> RepoSpaceUsageInfoOption {
        self.get_data_js(id, self.subscription_data.repo_space_usage_info.clone())
    }

    #[wasm_bindgen(js_name = repoSpaceUsageCalculate)]
    pub async fn repo_space_usage_calculate(&self, usage_id: u32) {
        let _ = self.vault.repo_space_usage_calculate(usage_id).await;
    }

    #[wasm_bindgen(js_name = repoSpaceUsageDestroy)]
    pub fn repo_space_usage_destroy(&self, usage_id: u32) {
        self.vault.repo_space_usage_destroy(usage_id)
    }

    // repo_files

    #[wasm_bindgen(js_name = repoFilesFileSubscribe)]
    pub fn repo_files_file_subscribe(&self, file_id: String, cb: js_sys::Function) -> u32 {
        let file_id = RepoFileId(file_id);

        self.subscribe(
            &[Event::RepoFiles],
            cb,
            self.subscription_data.repo_files_file.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::repo_files::selectors::select_file(state, &file_id).map(Into::into)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoFilesFileData)]
    pub fn repo_files_file_data(&self, id: u32) -> RepoFileOption {
        self.get_data_js(id, self.subscription_data.repo_files_file.clone())
    }

    async fn repo_file_reader_to_file_stream(
        &self,
        file_reader: Result<
            vault_core::repo_files_read::state::RepoFileReader,
            vault_core::repo_files_read::errors::GetFilesReaderError,
        >,
        force_blob: bool,
        abort_signal: Option<AbortSignal>,
    ) -> FileStreamOption {
        let reader = match file_reader {
            Ok(reader) => reader,
            Err(err) => {
                self.handle_error(err);

                return JsValue::UNDEFINED.into();
            }
        };

        let (transfer_id, file_reader) = self.vault.clone().transfers_download_reader(reader);

        let reader = match abort_signal {
            Some(abort_signal) => helpers::transfers_download_reader_abort_signal(
                self.vault.clone(),
                file_reader.reader,
                transfer_id,
                abort_signal,
            ),
            None => file_reader.reader,
        };

        let file_stream = match helpers::reader_to_file_stream(
            &file_reader.name.0,
            reader,
            file_reader.size,
            file_reader.content_type.as_deref(),
            force_blob,
        )
        .await
        {
            Ok(file_stream) => file_stream,
            Err(err) => {
                self.handle_error(err);

                return JsValue::UNDEFINED.into();
            }
        };

        FileStreamOption::from(file_stream)
    }

    #[wasm_bindgen(js_name = repoFilesGetFileStream)]
    pub async fn repo_files_get_file_stream(
        &self,
        repo_id: String,
        encrypted_path: String,
        force_blob: bool,
    ) -> FileStreamOption {
        self.repo_file_reader_to_file_stream(
            match self
                .vault
                .clone()
                .repo_files_get_file_reader(&RepoId(repo_id), &EncryptedPath(encrypted_path))
            {
                Ok(provider) => provider.reader().await,
                Err(err) => Err(err),
            },
            force_blob,
            None,
        )
        .await
    }

    #[wasm_bindgen(js_name = repoFilesDeleteFile)]
    pub async fn repo_files_delete_file(&self, repo_id: String, encrypted_path: String) {
        match self
            .vault
            .repo_files_delete_files(&[(RepoId(repo_id), EncryptedPath(encrypted_path))])
            .await
        {
            Ok(()) => {}
            Err(vault_core::repo_files::errors::DeleteFileError::Canceled) => {}
            Err(err) => self.handle_error(err),
        };
    }

    #[wasm_bindgen(js_name = repoFilesRenameFile)]
    pub async fn repo_files_rename_file(&self, repo_id: String, encrypted_path: String) {
        self.handle_result(
            self.vault
                .repo_files_rename_file(&RepoId(repo_id), &EncryptedPath(encrypted_path))
                .await,
        )
    }

    #[wasm_bindgen(js_name = repoFilesEncryptName)]
    pub fn repo_files_encrypt_name(&self, repo_id: String, name: String) -> Option<String> {
        self.vault
            .repo_files_service
            .encrypt_filename(&RepoId(repo_id), &DecryptedName(name))
            .map(|x| x.0)
            .ok()
    }

    // transfers

    #[wasm_bindgen(js_name = transfersIsActiveSubscribe)]
    pub fn transfers_is_active_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::Transfers],
            cb,
            self.subscription_data.transfers_is_active.clone(),
            move |vault| {
                vault.with_state(|state| vault_core::transfers::selectors::select_is_active(state))
            },
        )
    }

    #[wasm_bindgen(js_name = transfersIsActiveData)]
    pub fn transfers_is_active_data(&self, id: u32) -> bool {
        self.get_data(id, self.subscription_data.transfers_is_active.clone())
            .unwrap_or(false)
    }

    #[wasm_bindgen(js_name = transfersSummarySubscribe)]
    pub fn transfers_summary_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::Transfers],
            cb,
            self.subscription_data.transfers_summary.clone(),
            move |vault| {
                vault.with_state(|state| {
                    use vault_core::transfers::selectors;

                    let now = now_ms();

                    dto::TransfersSummary {
                        total_count: state.transfers.total_count,
                        done_count: state.transfers.done_count,
                        failed_count: state.transfers.failed_count,
                        size_progress_display: vault_core::files::file_size::size_of_display(
                            state.transfers.done_bytes,
                            state.transfers.total_bytes,
                        ),
                        percentage: selectors::select_percentage(state),
                        remaining_time_display: selectors::select_remaining_time(state, now)
                            .to_string(),
                        speed_display: vault_core::files::file_size::speed_display_bytes_duration(
                            selectors::select_bytes_done(state),
                            selectors::select_duration(state, now),
                        ),
                        is_transferring: selectors::select_is_transferring(state),
                        can_retry_all: selectors::select_can_retry_all(state),
                        can_abort_all: selectors::select_can_abort_all(state),
                    }
                })
            },
        )
    }

    #[wasm_bindgen(js_name = transfersSummaryData)]
    pub fn transfers_summary_data(&self, id: u32) -> TransfersSummary {
        self.get_data_js(id, self.subscription_data.transfers_summary.clone())
    }

    #[wasm_bindgen(js_name = transfersListSubscribe)]
    pub fn transfers_list_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::Transfers],
            cb,
            self.subscription_data.transfers_list.clone(),
            move |vault| {
                vault.with_state(|state| dto::TransfersList {
                    transfers: vault_core::transfers::selectors::select_transfers(state)
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                })
            },
        )
    }

    #[wasm_bindgen(js_name = transfersListData)]
    pub fn transfers_list_data(&self, id: u32) -> TransfersList {
        self.get_data_js(id, self.subscription_data.transfers_list.clone())
    }

    #[wasm_bindgen(js_name = transfersUpload)]
    pub async fn transfers_upload(
        &self,
        repo_id: String,
        encrypted_parent_path: String,
        name: String,
        file: FileOrBlob,
    ) -> RepoFilesUploadResultOption {
        let uploadable = Box::new(BrowserUploadable::from_value(file.into()).unwrap());

        let (_, create_future) = self.vault.transfers_upload(
            RepoId(repo_id),
            EncryptedPath(encrypted_parent_path),
            transfers::state::TransferUploadRelativeName(name),
            uploadable,
        );

        let future = match create_future.await {
            Ok(future) => future,
            Err(err) => {
                // create transfer errors have to be displayed
                self.handle_error(err);

                return JsValue::UNDEFINED.into();
            }
        };

        match future.await {
            Ok(res) => to_js(&dto::RepoFilesUploadResult::from(res)),
            Err(_) => {
                // transfer errors are displayed in transfers component
                JsValue::UNDEFINED.into()
            }
        }
    }

    #[wasm_bindgen(js_name = transfersAbort)]
    pub fn transfers_abort(&self, id: u32) {
        self.vault.transfers_abort(id);
    }

    #[wasm_bindgen(js_name = transfersAbortAll)]
    pub fn transfers_abort_all(&self) {
        self.vault.transfers_abort_all();
    }

    #[wasm_bindgen(js_name = transfersRetry)]
    pub fn transfers_retry(&self, id: u32) {
        self.vault.transfers_retry(id);
    }

    #[wasm_bindgen(js_name = transfersRetryAll)]
    pub fn transfers_retry_all(&self) {
        self.vault.transfers_retry_all();
    }

    // dir_pickers

    #[wasm_bindgen(js_name = dirPickersItemsSubscribe)]
    pub fn dir_pickers_items_subscribe(&self, picker_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::DirPickers],
            cb,
            self.subscription_data.dir_pickers_items.clone(),
            move |vault| {
                vault.with_state(|state| {
                    state
                        .dir_pickers
                        .pickers
                        .get(&picker_id)
                        .map(|picker| picker.items.iter().map(From::from).collect())
                        .unwrap_or_else(|| Vec::new())
                })
            },
        )
    }

    #[wasm_bindgen(js_name = dirPickersItemsData)]
    pub fn dir_pickers_items_data(&self, id: u32) -> DirPickerItemVec {
        self.get_data_js(id, self.subscription_data.dir_pickers_items.clone())
    }

    // repo_files_browsers

    #[wasm_bindgen(js_name = repoFilesBrowsersCreate)]
    pub fn repo_files_browsers_create(
        &self,
        repo_id: String,
        encrypted_path: String,
        options: RepoFilesBrowserOptions,
    ) -> u32 {
        let options: dto::RepoFilesBrowserOptions =
            serde_wasm_bindgen::from_value(options.into()).unwrap();

        let (browser_id, load_future) = self.vault.repo_files_browsers_create(
            RepoId(repo_id),
            &EncryptedPath(encrypted_path),
            options.into(),
        );

        let errors = self.errors.clone();

        spawn_local(async move { errors.handle_result(load_future.await) });

        browser_id
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersDestroy)]
    pub fn repo_files_browsers_destroy(&self, browser_id: u32) {
        self.vault.repo_files_browsers_destroy(browser_id)
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersInfo)]
    pub fn repo_files_browsers_info(&self, browser_id: u32) -> RepoFilesBrowserInfoOption {
        to_js(&self.vault.with_state(|state| {
            vault_core::repo_files_browsers::selectors::select_info(state, browser_id)
                .as_ref()
                .map(dto::RepoFilesBrowserInfo::from)
        }))
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersInfoSubscribe)]
    pub fn repo_files_browsers_info_subscribe(&self, browser_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoFilesBrowsers, Event::RepoFiles],
            cb,
            self.subscription_data.repo_files_browsers_info.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::repo_files_browsers::selectors::select_info(state, browser_id)
                        .as_ref()
                        .map(Into::into)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersInfoData)]
    pub fn repo_files_browsers_info_data(&self, id: u32) -> RepoFilesBrowserInfoOption {
        self.get_data_js(id, self.subscription_data.repo_files_browsers_info.clone())
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersLoadFiles)]
    pub async fn repo_files_browsers_load_files(&self, browser_id: u32) {
        self.handle_result(self.vault.repo_files_browsers_load_files(browser_id).await)
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersSelectFile)]
    pub fn repo_files_browsers_select_file(
        &self,
        browser_id: u32,
        file_id: String,
        extend: bool,
        range: bool,
        force: bool,
    ) {
        self.vault.repo_files_browsers_select_file(
            browser_id,
            RepoFileId(file_id),
            extend,
            range,
            force,
        )
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersSelectAll)]
    pub fn repo_files_browsers_select_all(&self, browser_id: u32) {
        self.vault.repo_files_browsers_select_all(browser_id)
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersClearSelection)]
    pub fn repo_files_browsers_clear_selection(&self, browser_id: u32) {
        self.vault.repo_files_browsers_clear_selection(browser_id)
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersSortBy)]
    pub fn repo_files_browsers_sort_by(&self, browser_id: u32, field: dto::RepoFilesSortFieldArg) {
        self.vault
            .repo_files_browsers_sort_by(browser_id, field.into(), None)
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersGetSelectedStream)]
    pub async fn repo_files_browsers_get_selected_stream(
        &self,
        browser_id: u32,
        force_blob: bool,
    ) -> FileStreamOption {
        self.repo_file_reader_to_file_stream(
            match self
                .vault
                .clone()
                .repo_files_browsers_get_selected_reader(browser_id)
            {
                Ok(provider) => provider.reader().await,
                Err(err) => Err(err),
            },
            force_blob,
            None,
        )
        .await
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersCreateDir)]
    pub async fn repo_files_browsers_create_dir(&self, browser_id: u32) {
        self.handle_result(
            match self.vault.repo_files_browsers_create_dir(browser_id).await {
                Ok(_) => Ok(()),
                Err(vault_core::repo_files::errors::CreateDirError::Canceled) => Ok(()),
                Err(err) => Err(err),
            },
        )
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersCreateFile)]
    pub async fn repo_files_browsers_create_file(
        &self,
        browser_id: u32,
        name: &str,
    ) -> Option<String> {
        match self
            .vault
            .repo_files_browsers_create_file(browser_id, name)
            .await
        {
            Ok((_, path)) => Some(path.0),
            Err(vault_core::repo_files::errors::CreateFileError::Canceled) => None,
            Err(err) => {
                self.handle_error(err);

                None
            }
        }
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersDeleteSelected)]
    pub async fn repo_files_browsers_delete_selected(&self, browser_id: u32) {
        match self
            .vault
            .repo_files_browsers_delete_selected(browser_id)
            .await
        {
            Ok(()) => {}
            Err(vault_core::repo_files::errors::DeleteFileError::Canceled) => {}
            Err(err) => self.handle_error(err),
        };
    }

    #[wasm_bindgen(js_name = repoFilesBrowsersMoveSelected)]
    pub async fn repo_files_browsers_move_selected(
        &self,
        browser_id: u32,
        mode: dto::RepoFilesMoveMode,
    ) {
        self.handle_result(
            self.vault
                .repo_files_browsers_move_selected(browser_id, mode.into())
                .await,
        )
    }

    // repo_files_details

    #[wasm_bindgen(js_name = repoFilesDetailsCreate)]
    pub fn repo_files_details_create(
        &self,
        repo_id: String,
        encrypted_path: String,
        is_editing: bool,
        options: RepoFilesDetailsOptions,
    ) -> u32 {
        let options: dto::RepoFilesDetailsOptions =
            serde_wasm_bindgen::from_value(options.into()).unwrap();

        let (details_id, load_future) = self.vault.repo_files_details_create(
            RepoId(repo_id),
            &EncryptedPath(encrypted_path),
            is_editing,
            options.into(),
        );

        spawn_local(async move {
            // error is displayed in the details component
            let _ = load_future.await;
        });

        details_id
    }

    #[wasm_bindgen(js_name = repoFilesDetailsDestroy)]
    pub async fn repo_files_details_destroy(&self, details_id: u32) {
        self.handle_result(
            self.vault
                .clone()
                .repo_files_details_destroy(details_id)
                .await,
        );
    }

    #[wasm_bindgen(js_name = repoFilesDetailsInfoSubscribe)]
    pub fn repo_files_details_info_subscribe(&self, details_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoFilesDetails, Event::RepoFiles],
            cb,
            self.subscription_data.repo_files_details_info.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::repo_files_details::selectors::select_info(state, details_id)
                        .as_ref()
                        .map(Into::into)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoFilesDetailsInfoData)]
    pub fn repo_files_details_info_data(&self, id: u32) -> RepoFilesDetailsInfoOption {
        self.get_data_js(id, self.subscription_data.repo_files_details_info.clone())
    }

    #[wasm_bindgen(js_name = repoFilesDetailsFileSubscribe)]
    pub fn repo_files_details_file_subscribe(&self, details_id: u32, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoFilesDetails, Event::RepoFiles],
            cb,
            self.subscription_data.repo_files_details_file.clone(),
            move |vault| {
                vault.with_state(|state| {
                    vault_core::repo_files_details::selectors::select_file(state, details_id)
                        .map(Into::into)
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoFilesDetailsFileData)]
    pub fn repo_files_details_file_data(&self, id: u32) -> RepoFileOption {
        self.get_data_js(id, self.subscription_data.repo_files_details_file.clone())
    }

    #[wasm_bindgen(js_name = repoFilesDetailsContentBytesSubscribe)]
    pub fn repo_files_details_content_bytes_subscribe(
        &self,
        details_id: u32,
        cb: js_sys::Function,
    ) -> u32 {
        self.subscribe_changed(
            &[Event::RepoFilesDetailsContentData],
            cb,
            self.subscription_data
                .repo_files_details_content_bytes
                .clone(),
            move |vault, entry| {
                vault.with_state(|state| {
                    let (bytes, version) =
                        vault_core::repo_files_details::selectors::select_content_bytes_version(
                            state, details_id,
                        );

                    vault_core::store::update_if(
                        entry,
                        || VersionedFileBytes {
                            value: (match bytes {
                                Some(bytes) => helpers::bytes_to_array(&bytes),
                                None => JsValue::UNDEFINED,
                            })
                            .into(),
                            version,
                        },
                        |x| x.version != version,
                    )
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoFilesDetailsContentBytesData)]
    pub fn repo_files_details_content_bytes_data(&self, id: u32) -> FileBytes {
        self.get_data(
            id,
            self.subscription_data
                .repo_files_details_content_bytes
                .clone(),
        )
        .map(|data| data.value)
        .unwrap_or(JsValue::UNDEFINED)
        .into()
    }

    #[wasm_bindgen(js_name = repoFilesDetailsLoadFile)]
    pub async fn repo_files_details_load_file(&self, details_id: u32) {
        // error is displayed in the details component
        let _ = self
            .vault
            .clone()
            .repo_files_details_load_file(details_id)
            .await;
    }

    #[wasm_bindgen(js_name = repoFilesDetailsLoadContent)]
    pub async fn repo_files_details_load_content(&self, details_id: u32) {
        // error is displayed in the details component
        let _ = self
            .vault
            .clone()
            .repo_files_details_load_content(details_id)
            .await;
    }

    #[wasm_bindgen(js_name = repoFilesDetailsGetFileStream)]
    pub async fn repo_files_details_get_file_stream(
        &self,
        details_id: u32,
        force_blob: bool,
        abort_signal: AbortSignal,
    ) -> FileStreamOption {
        self.repo_file_reader_to_file_stream(
            match self
                .vault
                .clone()
                .repo_files_details_get_file_reader(details_id)
                .await
            {
                Ok(provider) => provider.reader().await,
                Err(err) => Err(err),
            },
            force_blob,
            Some(abort_signal),
        )
        .await
    }

    #[wasm_bindgen(js_name = repoFilesDetailsEdit)]
    pub fn repo_files_details_edit(&self, details_id: u32) {
        self.vault.repo_files_details_edit(details_id);
    }

    #[wasm_bindgen(js_name = repoFilesDetailsEditCancel)]
    pub async fn repo_files_details_edit_cancel(&self, details_id: u32) {
        // error is displayed in the details component
        let _ = self
            .vault
            .clone()
            .repo_files_details_edit_cancel(details_id)
            .await;
    }

    #[wasm_bindgen(js_name = repoFilesDetailsSetContent)]
    pub fn repo_files_details_set_content(&self, details_id: u32, content: Vec<u8>) {
        self.vault
            .repo_files_details_set_content(details_id, content);
    }

    #[wasm_bindgen(js_name = repoFilesDetailsSave)]
    pub async fn repo_files_details_save(&self, details_id: u32) {
        // error is displayed in the details component
        let _ = self.vault.clone().repo_files_details_save(details_id).await;
    }

    #[wasm_bindgen(js_name = repoFilesDetailsDelete)]
    pub async fn repo_files_details_delete(&self, details_id: u32) {
        match self.vault.repo_files_details_delete(details_id).await {
            Ok(()) => {}
            Err(vault_core::repo_files::errors::DeleteFileError::Canceled) => {}
            Err(err) => self.handle_error(err),
        };
    }

    // repo_files_move

    #[wasm_bindgen(js_name = repoFilesMoveInfoSubscribe)]
    pub fn repo_files_move_info_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::RepoFilesMove, Event::RepoFiles, Event::DirPickers],
            cb,
            self.subscription_data.repo_files_move_info.clone(),
            move |vault| {
                vault.with_state(|state| {
                    state
                        .repo_files_move
                        .as_ref()
                        .map(|files_move| dto::RepoFilesMoveInfo {
                            src_files_count: files_move.src_paths.len(),
                            mode: (&files_move.mode).into(),
                            dir_picker_id: files_move.dir_picker_id,
                            dest_file_name:
                                vault_core::repo_files_move::selectors::select_dest_file(state)
                                    .and_then(|file| {
                                        vault_core::repo_files::selectors::select_file_name(
                                            state, file,
                                        )
                                        .ok()
                                    })
                                    .map(|x| x.0.to_owned()),
                            create_dir_enabled:
                                vault_core::repo_files_move::selectors::select_create_dir_enabled(
                                    state,
                                ),
                            can_move: vault_core::repo_files_move::selectors::select_check_move(
                                state,
                            )
                            .is_ok(),
                        })
                })
            },
        )
    }

    #[wasm_bindgen(js_name = repoFilesMoveInfoData)]
    pub fn repo_files_move_info_data(&self, id: u32) -> RepoFilesMoveInfoOption {
        self.get_data_js(id, self.subscription_data.repo_files_move_info.clone())
    }

    #[wasm_bindgen(js_name = repoFilesMoveDirPickerClick)]
    pub async fn repo_files_move_dir_picker_click(&self, item_id: String, is_arrow: bool) {
        self.handle_result(
            self.vault
                .repo_files_move_dir_picker_click(&DirPickerItemId(item_id), is_arrow)
                .await,
        )
    }

    #[wasm_bindgen(js_name = repoFilesMoveMoveFiles)]
    pub async fn repo_files_move_move_files(&self) {
        self.handle_result(self.vault.repo_files_move_move_files().await)
    }

    #[wasm_bindgen(js_name = repoFilesMoveCancel)]
    pub fn repo_files_move_cancel(&self) {
        self.vault.repo_files_move_cancel()
    }

    #[wasm_bindgen(js_name = repoFilesMoveCreateDir)]
    pub async fn repo_files_move_create_dir(&self) {
        self.handle_result(match self.vault.repo_files_move_create_dir().await {
            Ok(()) => Ok(()),
            Err(vault_core::repo_files::errors::CreateDirError::Canceled) => Ok(()),
            Err(err) => Err(err),
        })
    }

    // space_usage

    #[wasm_bindgen(js_name = spaceUsageSubscribe)]
    pub fn space_usage_subscribe(&self, cb: js_sys::Function) -> u32 {
        self.subscribe(
            &[Event::SpaceUsage],
            cb,
            self.subscription_data.space_usage.clone(),
            move |vault| {
                vault.with_state(|state| state.space_usage.space_usage.as_ref().map(Into::into))
            },
        )
    }

    #[wasm_bindgen(js_name = spaceUsageData)]
    pub fn space_usage_data(&self, id: u32) -> SpaceUsageOption {
        self.get_data_js(id, self.subscription_data.space_usage.clone())
    }
}
