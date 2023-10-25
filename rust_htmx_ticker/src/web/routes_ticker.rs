use crate::templates::{SearchResult, SearchResults, TickerValues, Values};
use askama::Template;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use serde::Deserialize;

pub fn routes_ticker() -> Router {
    Router::new()
        .route("/search", get(handle_search))
        .route("/values/:ticker", get(handle_values))
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    ticker: Option<String>,
}

async fn handle_search(Query(params): Query<SearchParams>) -> impl IntoResponse {
    println!("--> {:<12} - handle_search - {params:?}", "HANDLER");
    let ticker = params.ticker.as_deref().unwrap_or("ABCDEFG");

    let results = vec![SearchResult {
        name: "MappleSyrup".to_string(),
        ticker: ticker.to_string(),
    }];
    let template = SearchResults::new(results);
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn handle_values(Path(ticker): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handle_values - {ticker:?}", "HANDLER");
    let template = TickerValues {
        ticker,
        values: Values::default(),
    };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
