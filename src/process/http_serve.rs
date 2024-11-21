use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};

use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf, // PathBuf 相当于 String, Path 相当于 &str
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on  {}", path, addr);
    let state = HttpServeState { path };

    let app = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {:?} not found", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(err) => {
                warn!("Error reading file {:?}: {}", p.display(), err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error reading file: {}", err),
                )
            }
        }
    }
}
