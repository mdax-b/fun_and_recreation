pub use self::error::{Error, Result};
use crate::templates::Index;

use askama::Template;

use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;

use axum::middleware;
use axum::response::Response;

mod error;
mod templates;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .route("/", get(handle_main))
        .route("/_assets/*path", get(handle_assets))
        .merge(web::routes_hello::routes_hello())
        .merge(web::routes_ticker::routes_ticker())
        .layer(middleware::map_response(main_response_mapper));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENIGN on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

static BOOTSTRAP_MIN_CSS: &str = include_str!("../assets/bootstrap.min.css");
static HTMX_MIN_JS: &str = include_str!("../assets/htmx.min.js");

async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "bootstrap.min.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, BOOTSTRAP_MIN_CSS)
    } else if path == "htmx.min.js" {
        headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
        (StatusCode::OK, headers, HTMX_MIN_JS)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}

async fn handle_main() -> impl IntoResponse {
    let template = Index {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
