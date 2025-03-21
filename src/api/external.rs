use axum::Json;

use crate::shell::{compile_v2, decompile};
use crate::model::FileResponse;

pub async fn post_compile(Json(payload): Json<Vec<FileResponse>>) -> Json<String> {
    compile_v2(payload);
    Json(decompile())
}
