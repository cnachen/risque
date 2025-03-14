use std::collections::HashMap;

use axum::Json;

use crate::model::MemoryRange;

pub async fn get_memory(Json(payload): Json<MemoryRange>) -> Json<HashMap<u64, u64>> {
    Json(HashMap::new())
}

pub async fn get_registers() -> Json<HashMap<String, u64>> {
    Json(HashMap::new())
}

pub async fn post_run(Json(payload): Json<Vec<i32>>) -> Json<Vec<i32>> {
    Json(payload.iter().map(|x| x * 2).collect())
}
