use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum_server::{tls_rustls::RustlsConfig, Handle};
use tokio::{sync::oneshot, task::JoinHandle};

use super::{
    app_state::AppState, errors::FakeRemoteStartError, eventstream, files::service::FilesService,
    router::build_router, state::FakeRemoteState,
};

pub struct FakeRemoteServer {
    addr: SocketAddr,
    app_state: AppState,
    cert_pem: Vec<u8>,
    key_pem: Vec<u8>,
    tokio_runtime: Arc<tokio::runtime::Runtime>,

    handle: Arc<RwLock<Option<Handle>>>,
    serve_join_handle: Arc<RwLock<Option<JoinHandle<()>>>>,
    started: Arc<tokio::sync::Mutex<bool>>,
}

impl FakeRemoteServer {
    pub fn new(
        state: Arc<RwLock<FakeRemoteState>>,
        files_service: Arc<FilesService>,
        eventstream_listeners: Arc<eventstream::Listeners>,
        addr: SocketAddr,
        cert_pem: Vec<u8>,
        key_pem: Vec<u8>,
        tokio_runtime: Arc<tokio::runtime::Runtime>,
    ) -> Self {
        let app_state = AppState {
            state,
            files_service,
            eventstream_listeners,
        };

        Self {
            addr,
            app_state,
            cert_pem,
            key_pem,
            tokio_runtime,

            handle: Arc::new(RwLock::new(None)),
            serve_join_handle: Arc::new(RwLock::new(None)),
            started: Arc::new(tokio::sync::Mutex::new(false)),
        }
    }

    pub async fn start(&self) -> Result<String, FakeRemoteStartError> {
        let mut started_guard = self.started.lock().await;

        if *started_guard {
            return Err(FakeRemoteStartError::AlreadyStarted(addr_to_url(self.addr)));
        }

        let rustls_config = RustlsConfig::from_pem(self.cert_pem.clone(), self.key_pem.clone())
            .await
            .map_err(|err| FakeRemoteStartError::InvalidCertOrKey(Arc::new(err)))?;

        let router = build_router(self.app_state.clone());

        let handle = Handle::new();

        *self.handle.write().unwrap() = Some(handle.clone());

        let (serve_error_tx, serve_error_rx) = oneshot::channel();

        let serve_handle = handle.clone();
        let serve_addr = self.addr;

        let serve_join_handle = self.tokio_runtime.spawn(async move {
            if let Err(err) = axum_server::bind_rustls(serve_addr, rustls_config)
                .handle(serve_handle)
                .serve(router.into_make_service())
                .await
            {
                let _ = serve_error_tx.send(err);
            }
        });

        match handle.listening().await {
            Some(addr) => {
                *started_guard = true;

                log::info!("FakeRemoteServer listening on {}", addr);

                *self.serve_join_handle.write().unwrap() = Some(serve_join_handle);

                drop(started_guard);

                Ok(addr_to_url(addr))
            }
            None => {
                let _ = serve_join_handle.await;

                let err = serve_error_rx.await.unwrap();

                log::warn!("FakeRemoteServer listen error: {:?}", err);

                drop(started_guard);

                Err(FakeRemoteStartError::ListenError(Arc::new(err)))
            }
        }
    }

    pub async fn ensure_started(&self) -> Result<String, FakeRemoteStartError> {
        match self.start().await {
            Ok(proxy_url) => Ok(proxy_url),
            Err(FakeRemoteStartError::AlreadyStarted(proxy_url)) => Ok(proxy_url),
            Err(err) => Err(err),
        }
    }

    pub async fn stop(&self) {
        let mut started_guard = self.started.lock().await;

        if let Some(handle) = self.handle.write().unwrap().take() {
            // TODO graceful shutdown
            handle.shutdown();
        }

        if let Some(join_handle) = self.serve_join_handle.write().unwrap().take() {
            let _ = join_handle.await;
        }

        *started_guard = false;
    }
}

pub fn addr_to_url(addr: SocketAddr) -> String {
    format!("https://{}", addr)
}
