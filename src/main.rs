use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::{header, Response, StatusCode},
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

async fn upload_handler(multipart: Multipart) -> Response<String> {
    match do_upload(multipart).await {
        Ok(_) => {
            let html = upload_success_page().await.unwrap();
            Response::builder()
                .header(header::CONTENT_TYPE, "text/html")
                .body(html)
                .unwrap()
        }
        Err(err) => Response::builder()
            .header(header::CONTENT_TYPE, "text/plain")
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string())
            .unwrap(),
    }
}

async fn upload_success_page() -> std::io::Result<String> {
    let mut s = String::default();
    let mut file = File::open("web/success.html")?;
    file.read_to_string(&mut s)?;
    Ok(s)
}

async fn do_upload(mut multipart: Multipart) -> std::io::Result<()> {
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
