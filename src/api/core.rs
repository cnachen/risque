use std::{collections::HashMap, fs::File, io::Read, sync::Arc};

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

pub async fn post_step(Extension(cpu): Extension<Arc<Mutex<Cpu>>>) -> Json<Vec<String>> {
    let mut cpu = cpu.lock().await;

    let inst = match cpu.fetch() {
        Ok(inst) => inst,
        _ => 0
    };

    match cpu.execute(inst as u32) {
        Ok(new_pc) => cpu.pc = new_pc,
        _ => cpu.pc = 0,
    };

    Json(vec![format!("Instruction executed: {:x}.", inst)])
}

pub async fn post_run(Extension(cpu): Extension<Arc<Mutex<Cpu>>>) -> Json<Vec<String>> {
    let mut cpu = cpu.lock().await;
    let mut file = File::open("temp/payload.bin").unwrap();
    let mut code = Vec::new();
    file.read_to_end(&mut code).unwrap();
    cpu.bus.replace(code);
    Json(vec!("Target started to run.".into()))
}
