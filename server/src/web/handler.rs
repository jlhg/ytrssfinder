use axum::extract::{Json, Query};
use axum::http::StatusCode;
use regex::Regex;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::error;

use crate::web::object::*;

#[derive(Deserialize)]
pub struct Param {
    channel_url: String,
}

pub async fn get_youtube_channel_feed(q: Query<Param>) -> (StatusCode, Json<Value>) {
    let re = Regex::new(r#"href="(https://www\.youtube\.com/feeds/videos\.xml\?channel_id=.+?)""#)
        .unwrap();

    match reqwest::get(&q.channel_url).await {
        Ok(resp) => match resp.text().await {
            Ok(body) => match re.captures(&body) {
                Some(caps) => {
                    let feed_url = caps.get(1).unwrap().as_str();
                    render_data(StatusCode::OK, json!({"feed_url": feed_url}))
                }
                None => render_bad_request("Channel source can't be resolved."),
            },
            Err(e) => {
                error!("{}", e);
                render_bad_request("Failed to get the response body.")
            },
        },
        Err(e) => {
            error!("{}", e);
            render_bad_request("Invalid channel URL.")
        },
    }
}
