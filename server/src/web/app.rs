use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

use crate::web::handler::get_youtube_channel_feed;

pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

impl ServerConfig {
    fn bind_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }
}

fn new_router() -> Router {
    Router::new()
        .route("/", get(get_youtube_channel_feed))
        .layer(CorsLayer::permissive())
}

pub async fn start_server(cfg: ServerConfig) -> Result<()> {
    let r = new_router().layer(
        ServiceBuilder::new().layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        ),
    );

    axum::Server::bind(&cfg.bind_addr())
        .serve(r.into_make_service())
        .await?;

    Ok(())
}
