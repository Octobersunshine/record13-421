use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use serde::Serialize;

use crate::desensitize::desensitize_order;
use crate::model::OrderResponse;
use crate::store::OrderStore;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn get_order(
    Path(id): Path<String>,
    State(store): State<OrderStore>,
) -> Result<Json<OrderResponse>, (StatusCode, Json<ErrorResponse>)> {
    match store.get_by_id(&id) {
        Some(order) => {
            let desensitized = desensitize_order(&order);
            Ok(Json(desensitized))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("订单 {} 不存在", id),
            }),
        )),
    }
}
