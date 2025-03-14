use std::{collections::HashMap, sync::Arc};

use axum::{Extension, Json};
use tokio::sync::Mutex;

use crate::{model::{MemoryRange, MemoryValue, RegisterValue}, Cpu};

pub async fn post_memory(
    Extension(cpu): Extension<Arc<Mutex<Cpu>>>,
    Json(payload): Json<MemoryRange>,
) -> Json<Vec<MemoryValue>> {
    let cpu = cpu.lock().await;

    Json(cpu.read_memory_range(payload.begin, payload.end))
}

pub async fn post_registers(
    Extension(cpu): Extension<Arc<Mutex<Cpu>>>,
) -> Json<Vec<RegisterValue>> {
    let cpu = cpu.lock().await;

    Json(cpu.read_registers())
}

pub async fn post_run(Json(payload): Json<Vec<i32>>) -> Json<Vec<i32>> {
    Json(payload.iter().map(|x| x * 2).collect())
}
