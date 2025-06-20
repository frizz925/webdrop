use std::{env, io::ErrorKind, path::PathBuf, str::FromStr, sync::Arc};

use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{event, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use webdrop::{
    controllers::MainController,
    models::session::SessionId,
    registries::{OBJECT_REPOSITORIES, OBJECT_SERVICES, SESSION_REPOSITORY, WEBSOCKET_SERVICES},
    services::{object::ObjectService, session::SessionService},
    ConcreteObjectRepository, ConcreteObjectService, ConcreteSessionRepository,
    ConcreteWebSocketService, STORAGE_DIR,
};

const LISTENER_ADDR: &str = "0.0.0.0:8000";
const NOTIFICATION_BACKLOG_SIZE: usize = 256;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .init();

    match std::fs::metadata(STORAGE_DIR) {
        Ok(meta) if !meta.is_dir() => panic!("Storage directory is not a directory"),
        Ok(_) => (),
        Err(e) if e.kind() == ErrorKind::NotFound => std::fs::create_dir(STORAGE_DIR).unwrap(),
        Err(e) => panic!("Failed to check for storage directory: {e}"),
    }

    let service = SessionService::new(session_repository_factory(), websocket_service_factory);
    let controller =
        MainController::new(service, websocket_service_factory, object_service_factory);
    let router = controller.into_router().layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)),
    );

    let addr = env::var("LISTENER_ADDR").unwrap_or(LISTENER_ADDR.to_owned());
    let listener = TcpListener::bind(addr).await.unwrap();
    let addr = listener.local_addr().unwrap();
    event!(Level::INFO, "Listening at {addr}");

    let local_ip = local_ip_address::local_ip().unwrap();
    event!(
        Level::INFO,
        "Access WebDrop via http://{}:{}/",
        local_ip.to_string(),
        addr.port()
    );

    axum::serve(listener, router).await.unwrap();
}

fn websocket_service_factory(sid: &SessionId) -> Arc<ConcreteWebSocketService> {
    let services = WEBSOCKET_SERVICES.read().unwrap();
    if let Some(service) = services.get(sid) {
        Arc::clone(service)
    } else {
        drop(services);
        let service = Arc::new(ConcreteWebSocketService::new(
            NOTIFICATION_BACKLOG_SIZE,
            session_repository_factory(),
        ));
        WEBSOCKET_SERVICES
            .write()
            .unwrap()
            .insert(*sid, service.clone());
        service
    }
}

fn object_service_factory(sid: &SessionId) -> Arc<ConcreteObjectService> {
    let services = OBJECT_SERVICES.read().unwrap();
    if let Some(service) = services.get(sid) {
        Arc::clone(service)
    } else {
        drop(services);
        let repository = object_repository_factory(sid);
        let websocket = websocket_service_factory(sid);
        let service = Arc::new(ObjectService::new(repository, websocket));
        OBJECT_SERVICES
            .write()
            .unwrap()
            .insert(*sid, service.clone());
        service
    }
}

fn session_repository_factory() -> Arc<ConcreteSessionRepository> {
    Arc::clone(&SESSION_REPOSITORY)
}

fn object_repository_factory(sid: &SessionId) -> Arc<ConcreteObjectRepository> {
    let repositories = OBJECT_REPOSITORIES.read().unwrap();
    if let Some(repository) = repositories.get(sid) {
        Arc::clone(repository)
    } else {
        drop(repositories);
        let dir = PathBuf::from_str(STORAGE_DIR)
            .unwrap()
            .join(sid.to_string());
        let repository = Arc::new(ConcreteObjectRepository::new(dir));
        OBJECT_REPOSITORIES
            .write()
            .unwrap()
            .insert(*sid, repository.clone());
        repository
    }
}
