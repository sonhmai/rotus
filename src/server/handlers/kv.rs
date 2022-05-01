use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json
};

pub async fn get_value(
    key: String  // TODO - supports other types of key
) -> impl IntoResponse {
    let value = format!("{}{}", "value_of_", key);
    (StatusCode::CREATED, Json(value))
}

pub async fn put_value(
    key: String,
    value: String
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(value))  // TODO returned key and value
}
