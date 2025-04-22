use slog::{info, Logger};
use sloggers::{terminal::TerminalLoggerBuilder, types::Severity, Build};
use tokio::net::TcpListener;
use webdrop::{controllers::MainController, repositories::session::SessionFsRepository};

const LISTENER_ADDR: &str = "0.0.0.0:8000";

const STORAGE_DIR: &str = "storage";

#[tokio::main]
async fn main() {
    let logger = create_logger();
    let repository = SessionFsRepository::new(STORAGE_DIR);
    let controller = MainController::new(repository);
    let router = controller.into_router();

    let listener = TcpListener::bind(LISTENER_ADDR).await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!(logger, "Listening at {addr}");

    axum::serve(listener, router).await.unwrap();
}

fn create_logger() -> Logger {
    TerminalLoggerBuilder::new()
        .level(Severity::Info)
        .build()
        .unwrap()
}
