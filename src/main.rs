use std::{
    io,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    routing::post,
    Router,
};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    net::TcpListener,
};
use tower_http::services::ServeDir;

const SIZE_KIB: usize = 1024;
const SIZE_MIB: usize = 1024 * SIZE_KIB;
const SIZE_GIB: usize = 1024 * SIZE_MIB;

const UPLOAD_BASE_DIR: &str = "uploads";

#[tokio::main]
async fn main() {
    let upload_route = post(upload_handler).layer(DefaultBodyLimit::max(2 * SIZE_GIB));
    let app = Router::new()
        .route("/upload", upload_route)
        .fallback_service(ServeDir::new("web"));
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload_handler(multipart: Multipart) -> Result<String, (StatusCode, String)> {
    do_upload(multipart)
        .await
        .map(|_| "Upload success".to_string())
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

async fn do_upload(mut multipart: Multipart) -> io::Result<()> {
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let epoch_str = epoch.to_string();
    let upload_dir = Path::new(UPLOAD_BASE_DIR).join(epoch_str);
    fs::create_dir(&upload_dir).await?;
    while let Ok(Some(mut field)) = multipart.next_field().await {
        let filename = if let Some(file_name) = field.file_name() {
            upload_dir.join(file_name)
        } else {
            continue;
        };
        let mut file = OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(filename)
            .await?;
        while let Ok(Some(chunk)) = field.chunk().await {
            file.write(&chunk).await?;
        }
    }
    Ok(())
}
