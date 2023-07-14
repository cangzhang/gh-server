use axum::{extract::Extension, routing::get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use std::{net::SocketAddr, time::Duration};

pub mod api;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(10))
        .timeout_write(Duration::from_secs(10))
        .build();

    let api_routes = Router::new()
        .route("/:user/:repo/:rev/*path", get(api::get_file))
        .layer(Extension(agent));

    let app = Router::new().nest("/gh", api_routes).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 4040));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
