use axum::Json;

use crate::compiler::compile;
use crate::model::File;

pub async fn post_compile(Json(payload): Json<Vec<File>>) -> Json<String> {
    Json(compile(payload))
}
