use axum::Json;

use crate::compiler::compile;
use crate::model::FileResponse;

pub async fn post_compile(Json(payload): Json<Vec<FileResponse>>) -> Json<String> {
    Json(compile(payload))
}
