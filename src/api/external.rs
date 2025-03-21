use axum::Json;

use crate::model::FileResponse;
use crate::shell::{compile_v2, decompile};

pub async fn post_compile(Json(payload): Json<Vec<FileResponse>>) -> Json<String> {
    compile_v2(payload);
    Json(decompile())
}
