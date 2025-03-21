use std::fs::{self, File as FsFile};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

use crate::model::FileResponse;
use crate::Cpu;

pub fn compile_v2(payload: Vec<FileResponse>) -> String {
    // Create temporary directory
    let temp_dir = "temp";
    let _ = fs::create_dir_all(temp_dir);

    // Write all source files
    for file in payload {
        let path = Path::new(temp_dir).join(&file.name);
        let mut f = FsFile::create(&path).expect("Failed to create file");
        f.write_all(file.content.as_bytes())
            .expect("Failed to write file");
    }

    // Compile
    let status = Command::new("riscv64-unknown-elf-gcc")
        .args([
            "-march=rv64i",
            "-mabi=lp64",
            "-O0",
            "-nostdlib",
            "-fno-elide-constructors",
            "-T",
            "assets/link.ld",
            "-o",
            "temp/payload.elf",
        ])
        .arg(format!("{}/entry.S", temp_dir))
        .status()
        .expect("Failed to execute gcc command");

    if !status.success() {
        return "Compilation failed".into();
    }

    // Convert to binary
    let status = Command::new("riscv64-unknown-elf-objcopy")
        .args(["-O", "binary", "temp/payload.elf", "temp/payload.bin"])
        .status()
        .expect("Failed to execute objcopy command");

    if !status.success() {
        return "Binary conversion failed".into();
    }

    "OK".into()
}

pub fn decompile() -> String {
    let mut file = FsFile::open("temp/payload.bin").unwrap();
    let mut code = Vec::new();
    file.read_to_end(&mut code).unwrap();
    let mut cpu = Cpu::new(code.clone());

    let mut ret = String::new();

    for _ in 0..code.len() / 4 {
        let inst = cpu.fetch().unwrap();
        ret.push_str(&cpu.explain(inst as u32));
        cpu.pc += 4;
        ret.push('\n');
    }

    ret
}
