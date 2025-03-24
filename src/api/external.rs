use axum::Json;

use crate::model::FileResponse;
use crate::shell::{compile, decompile};

pub async fn post_compile(Json(payload): Json<Vec<FileResponse>>) -> Json<String> {
    compile(payload);
    Json(decompile())
}
