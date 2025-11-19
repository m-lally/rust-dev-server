use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
    environment: String,
}

pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: state.config.environment.clone(),
    })
}

#[derive(Deserialize, Serialize)]
pub struct EchoRequest {
    message: String,
}

#[derive(Serialize)]
pub struct EchoResponse {
    echo: String,
    length: usize,
}

pub async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoResponse> {
    info!("Echo request received: {}", payload.message);
    Json(EchoResponse {
        length: payload.message.len(),
        echo: payload.message,
    })
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    id: u32,
    name: String,
    description: String,
}

// In-memory storage (for demo - use a real DB in production)
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ITEMS: Lazy<Mutex<Vec<Item>>> = Lazy::new(|| {
    Mutex::new(vec![
        Item {
            id: 1,
            name: "Example Item".to_string(),
            description: "This is an example item".to_string(),
        }
    ])
});

pub async fn list_items() -> Json<Vec<Item>> {
    let items = ITEMS.lock().unwrap();
    Json(items.clone())
}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    name: String,
    description: String,
}

pub async fn create_item(
    Json(payload): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let mut items = ITEMS.lock().unwrap();
    let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
    
    let item = Item {
        id: new_id,
        name: payload.name,
        description: payload.description,
    };
    
    items.push(item.clone());
    info!("Created new item with id: {}", new_id);
    
    Ok((StatusCode::CREATED, Json(item)))
}

// Custom error type
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
