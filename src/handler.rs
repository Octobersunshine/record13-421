use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};

use crate::desensitize::desensitize_order;
use crate::export::{build_filename_stem, build_zip_from_csv, orders_to_csv_bytes};
use crate::model::OrderResponse;
use crate::store::OrderStore;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub start: Option<String>,
    pub end: Option<String>,
    pub format: Option<String>,
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

pub async fn export_orders(
    Query(params): Query<ExportQuery>,
    State(store): State<OrderStore>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let start = params.start.clone().unwrap_or_default();
    let end = params.end.clone().unwrap_or_default();
    let fmt = params
        .format
        .clone()
        .unwrap_or_else(|| "zip".to_string())
        .to_lowercase();

    let orders = store.list_by_date_range(&start, &end);
    if orders.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "指定日期区间内无订单数据".to_string(),
            }),
        ));
    }

    let csv_bytes = orders_to_csv_bytes(&orders).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("CSV 生成失败: {}", e),
            }),
        )
    })?;

    let filename_stem = build_filename_stem(&start, &end);

    match fmt.as_str() {
        "csv" => {
            let filename = format!("{}.csv", filename_stem);
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "text/csv; charset=utf-8".parse().unwrap(),
            );
            headers.insert(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename)
                    .parse()
                    .unwrap(),
            );
            Ok((headers, csv_bytes).into_response())
        }
        "zip" | _ => {
            let zip_bytes = build_zip_from_csv(&csv_bytes, &filename_stem).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("ZIP 打包失败: {}", e),
                    }),
                )
            })?;
            let filename = format!("{}.zip", filename_stem);
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "application/zip".parse().unwrap(),
            );
            headers.insert(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename)
                    .parse()
                    .unwrap(),
            );
            Ok((headers, zip_bytes).into_response())
        }
    }
}
