mod model;

use anyhow::{Context, Result};
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::get,
    Router,
};
use base64::{engine::general_purpose, Engine};
use model::{
    Item, Metadata, Output, RaidItem, RaidResponse, RaidresResponse, ReservationData, SoftReserve,
};
use reqwest::Client;
use serde::Serialize;
use std::net::SocketAddr;

const RAIDRES_URL: &str = "https://raidres.fly.dev";

async fn fetch_raidres_data(id: &str, client: &Client) -> Result<RaidresResponse> {
    let url = format!("{}/api/events/{}", RAIDRES_URL, id);
    client
        .get(&url)
        .send()
        .await?
        .json::<RaidresResponse>()
        .await
        .context("Failed to fetch item reservation data.")
}

async fn fetch_raid_data(raid_id: i32, client: &Client) -> Result<RaidResponse> {
    let url = format!("{}/raids/raid_{}.json", RAIDRES_URL, raid_id);
    client
        .get(&url)
        .send()
        .await?
        .json::<RaidResponse>()
        .await
        .context("Failed to fetch raid item data.")
}

fn get_soft_reserves(
    reservations: &Vec<ReservationData>,
    raid_items: &[RaidItem],
) -> Vec<SoftReserve> {
    let mut result = Vec::new();

    for reservation in reservations {
        let item_id = reservation.raid_item_id;

        if item_id.is_none() {
            result.push(SoftReserve {
                name: reservation.character.name.clone(),
                items: vec![Item { id: 0, quality: 0 }],
            });
            continue;
        }

        if let Some(raid_item) = raid_items.iter().find(|item| item.id == item_id.unwrap()) {
            result.push(SoftReserve {
                name: reservation.character.name.clone(),
                items: vec![Item {
                    id: raid_item.turtle_db_item_id,
                    quality: raid_item.quality,
                }],
            });
        }
    }

    result
}

fn get_hard_reserves(reservations: &Vec<i32>, raid_items: &[RaidItem]) -> Vec<Item> {
    let mut result = Vec::new();

    for item_id in reservations {
        if let Some(raid_item) = raid_items.iter().find(|item| item.id == *item_id) {
            result.push(Item {
                id: raid_item.turtle_db_item_id,
                quality: raid_item.quality,
            });
        }
    }

    result
}

async fn api_handler(Path(id): Path<String>) -> impl IntoResponse {
    let client = reqwest::Client::new();

    let raidres_response = match fetch_raidres_data(&id, &client).await {
        Ok(r) => r,
        Err(e) => return (axum::http::StatusCode::BAD_REQUEST, e.to_string()),
    };

    let raid_response = match fetch_raid_data(raidres_response.raid_id, &client).await {
        Ok(r) => r,
        Err(e) => return (axum::http::StatusCode::BAD_REQUEST, e.to_string()),
    };

    let output = Output {
        metadata: Metadata {
            id: id.clone(),
            instance: raidres_response.raid_id,
            instances: vec![raid_response.name],
        },
        softreserves: get_soft_reserves(&raidres_response.reservations, &raid_response.raid_items),
        hardreserves: get_hard_reserves(
            &raidres_response.disabled_raid_item_ids,
            &raid_response.raid_items,
        ),
    };

    let json = match serde_json::to_string(&output) {
        Ok(j) => j,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    let encoded = general_purpose::STANDARD.encode(json);

    (axum::http::StatusCode::OK, encoded)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/:id", get(api_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8383));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}