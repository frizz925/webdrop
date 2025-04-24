use std::{path::PathBuf, str::FromStr};

use log::info;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use webdrop::{
    controllers::MainController,
    models::session::SessionId,
    services::{object::ObjectService, session::SessionService},
    ConcreteObjectRepository, ConcreteObjectService, ConcreteServiceRepository,
};

const LISTENER_ADDR: &str = "0.0.0.0:8000";

const STORAGE_DIR: &str = "storage";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let repository = ConcreteServiceRepository::new(STORAGE_DIR);
    let service = SessionService::new(repository);
    let controller = MainController::new(service, object_service_factory);
    let router = controller.into_router().layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(LISTENER_ADDR).await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("Listening at {addr}");

    axum::serve(listener, router).await.unwrap();
}

fn object_service_factory(sid: &SessionId) -> ConcreteObjectService {
    let dir = PathBuf::from_str(STORAGE_DIR)
        .unwrap()
        .join(sid.to_string());
    let repository = ConcreteObjectRepository::new(dir);
    ObjectService::new(repository)
}
