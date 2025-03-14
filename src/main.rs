use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use risque::Cpu;

// fn main() -> io::Result<()> {
//     let args: Vec<String> = env::args().collect();

//     // if args.len() != 2 {
//     //     println!(
//     //         "Usage:\n\
//     //         - cargo run <filename>"
//     //     );
//     //     return Ok(());
//     // }

//     let mut file = File::open("assets/payload.bin")?;
//     let mut code = Vec::new();
//     file.read_to_end(&mut code)?;

//     let mut cpu = Cpu::new(code);

//     loop {
//         let inst = match cpu.fetch() {
//             Ok(inst) => inst,
//             Err(e) => {
//                 println!("{:?}", e);
//                 break;
//             }
//         };

//         match cpu.execute(inst as u32) {
//             Ok(new_pc) => cpu.pc = new_pc,
//             Err(e) => {
//                 println!("{:?}", e);
//                 break;
//             }
//         };
//     }

//     cpu.dump_registers();

//     Ok(())
// }

use risque::App;

#[tokio::main]
async fn main() {
    App::run().await
}