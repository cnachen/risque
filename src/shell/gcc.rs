use glob::glob;
use std::fs::{self, File as FsFile};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::model::FileResponse;
use crate::Cpu;

pub fn compile(payload: Vec<FileResponse>) -> String {
    // Create temporary directory
    let temp_dir = "/tmp/risque-temp";
    let _ = fs::create_dir_all(temp_dir);

    // Write all source files
    for file in payload {
        let path = Path::new(temp_dir).join(&file.name);
        let mut f = FsFile::create(&path).expect("Failed to create file");
        f.write_all(file.content.as_bytes())
            .expect("Failed to write file");
    }

    let cfiles: Vec<PathBuf> = glob(format!("{}/*.c", temp_dir).as_str())
        .unwrap()
        .flatten()
        .collect();
    let sfiles: Vec<PathBuf> = glob(format!("{}/*.s", temp_dir).as_str())
        .unwrap()
        .flatten()
        .collect();
    let upper_sfiles: Vec<PathBuf> = glob(format!("{}/*.S", temp_dir).as_str())
        .unwrap()
        .flatten()
        .collect();

    // Compile
    let status = Command::new("riscv64-unknown-elf-gcc")
        .args([
            "-march=rv64im",
            "-mabi=lp64",
            "-O0",
            "-nostdlib",
            // "-fno-elide-constructors",
            // "-mno-fdiv",
            "-T",
            "assets/link.ld",
            "-o",
            &format!("{}/payload.elf", temp_dir),
        ])
        .args(sfiles)
        .args(upper_sfiles)
        .args(cfiles)
        // .arg("/opt/homebrew/Cellar/riscv-gnu-toolchain/main/lib/gcc/riscv64-unknown-elf/12.2.0/libgcc.a")
        .status()
        .expect("Failed to execute gcc command");

    if !status.success() {
        return "Compilation failed".into();
    }

    // Convert to binary
    let status = Command::new("riscv64-unknown-elf-objcopy")
        .args([
            "-O",
            "binary",
            &format!("{}/payload.elf", temp_dir),
            &format!("{}/payload.bin", temp_dir),
        ])
        .status()
        .expect("Failed to execute objcopy command");

    if !status.success() {
        return "Binary conversion failed".into();
    }

    "OK".into()
}

pub fn decompile() -> String {
    let mut file = FsFile::open("/tmp/risque-temp/payload.bin").unwrap();
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
