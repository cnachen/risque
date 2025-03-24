use std::{fs::File, io::Read, sync::Arc};

use axum::{Extension, Json};
use tokio::sync::Mutex;

use crate::{
    core::param::DRAM_BASE,
    model::{MemoryRangePayload, MemoryValueResponse, RegisterValueResponse, StepResponse},
    Cpu,
};

pub async fn post_memory(
    Extension(cpu): Extension<Arc<Mutex<Cpu>>>,
    Json(payload): Json<MemoryRangePayload>,
) -> Json<Vec<MemoryValueResponse>> {
    let cpu = cpu.lock().await;

    Json(cpu.read_memory_range(payload.begin, payload.end))
}

pub async fn post_registers(
    Extension(cpu): Extension<Arc<Mutex<Cpu>>>,
) -> Json<Vec<RegisterValueResponse>> {
    let cpu = cpu.lock().await;

    Json(cpu.read_registers())
}

pub async fn post_step(Extension(cpu): Extension<Arc<Mutex<Cpu>>>) -> Json<StepResponse> {
    let mut cpu = cpu.lock().await;

    let insn = match cpu.fetch() {
        Ok(inst) => inst,
        _ => 0xffffffff,
    };

    match cpu.execute(insn as u32) {
        Ok(new_pc) => cpu.pc = new_pc,
        _ => (),
    };

    cpu.running = false;

    Json(StepResponse::new(
        cpu.pc,
        insn as u32,
        format!("Instruction executed: 0x{:08x}.", insn),
    ))
}

pub async fn post_run(Extension(cpu): Extension<Arc<Mutex<Cpu>>>) -> Json<Vec<String>> {
    let mut cpu = cpu.lock().await;

    if cpu.running {
        return Json(vec!["Target is already running.".into()]);
    }

    let mut file = File::open("/tmp/risque-temp/payload.bin").unwrap();
    let mut code = Vec::new();
    file.read_to_end(&mut code).unwrap();

    cpu.bus.replace(code);
    cpu.pc = DRAM_BASE;
    cpu.running = true;
    Json(vec!["Target started to run.".into()])
}

pub async fn post_stop(Extension(cpu): Extension<Arc<Mutex<Cpu>>>) -> Json<Vec<String>> {
    let mut _cpu = cpu.lock().await;
    // cpu.running = false;
    Json(vec!["Target stopped.".into()])
}

pub async fn post_restart(Extension(cpu): Extension<Arc<Mutex<Cpu>>>) -> Json<Vec<String>> {
    let mut cpu = cpu.lock().await;
    cpu.pc = DRAM_BASE;
    cpu.running = false;
    Json(vec![format!("Target pc reseted to 0x{:016x}.", DRAM_BASE)])
}
