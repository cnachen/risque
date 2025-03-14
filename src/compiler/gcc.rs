use std::fs::{self, File as FsFile};
use std::io::Write;
use std::process::Command;
use std::path::Path;

use crate::model::File;

pub fn compile(payload: Vec<File>) -> String {
    // Create temporary directory
    let temp_dir = "temp";
    let _ = fs::create_dir_all(temp_dir);

    // Write all source files
    for file in payload {
        let path = Path::new(temp_dir).join(&file.name);
        let mut f = FsFile::create(&path).expect("Failed to create file");
        f.write_all(file.content.as_bytes()).expect("Failed to write file");
    }

    // Compile
    let status = Command::new("riscv64-unknown-elf-gcc")
        .args([
            "-march=rv64i",
            "-mabi=lp64",
            "-O0",
            "-nostdlib",
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
        .args([
            "-O",
            "binary",
            "temp/payload.elf",
            "temp/payload.bin",
        ])
        .status()
        .expect("Failed to execute objcopy command");

    if !status.success() {
        return "Binary conversion failed".into();
    }

    // Disassemble
    let output = Command::new("riscv64-unknown-elf-objdump")
        .args([
            "-m",
            "riscv",
            "-b",
            "binary",
            "-Mno-aliases",
            "-D",
            "temp/payload.bin",
        ])
        .output()
        .expect("Failed to execute objdump command");

    // Clean up temporary files
    let _ = fs::remove_dir_all(temp_dir);

    String::from_utf8_lossy(&output.stdout).into_owned()
}