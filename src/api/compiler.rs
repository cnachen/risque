use axum::Json;

use crate::model::File;
use crate::compiler::compile;

pub async fn post_compile(Json(payload): Json<Vec<File>>) -> Json<String> {
    Json(compile(payload))
}
