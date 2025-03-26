use std::mem;

const PAGE_SIZE: usize = 4096;

struct JitMemory {
    code_buffer: *mut u8,
    size: usize,
}

impl JitMemory {
    fn new(numpages: usize) -> Self {
        use libc::{MAP_ANON, MAP_JIT, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
        let prot = PROT_READ | PROT_WRITE | PROT_EXEC;
        let flags = MAP_PRIVATE | MAP_ANON | MAP_JIT;

        unsafe {
            let mem = libc::mmap(
                std::ptr::null_mut(),
                numpages * PAGE_SIZE,
                prot,
                flags,
                -1,
                0,
            );
            if mem == libc::MAP_FAILED {
                panic!("mmap failed");
            }
            Self {
                code_buffer: mem as *mut u8,
                size: numpages * PAGE_SIZE,
            }
        }
    }

    fn write_protect(&self, enable: bool) {
        #[cfg(target_os = "macos")]
        unsafe {
            libc::pthread_jit_write_protect_np(enable as i32)
        }
    }

    fn write_u32(&mut self, index: usize, value: u32) {
        assert!((index + 1) * 4 <= self.size);
        unsafe {
            let ptr = self.code_buffer.add(index * 4) as *mut u32;
            *ptr = value;
        }
    }

    #[allow(dead_code)]
    fn read_u32(&self, index: usize) -> u32 {
        assert!((index + 1) * 4 <= self.size);
        unsafe {
            let ptr = self.code_buffer.add(index * 4) as *const u32;
            *ptr
        }
    }
}

fn run_jit() -> unsafe extern "C" fn() {
    let mut jit = JitMemory::new(PAGE_SIZE);

    let machine_code: [u32; 10] = [
        0xd2800020, // mov x0, #0x1
        0x100000e1, // adr x1, #0x38
        0xd28000e2, // mov x2, #0x7
        0xd2800090, // mov x16, #0x4
        0xd4001001, // svc #0x0
        0xd2800000, // mov x0, #0x0
        0xd2800030, // mov x16, #0x1
        0xd4001001, // svc #0x0
        0x35343131, // "1145"
        0x000a3431, // "41\n"
    ];

    jit.write_protect(false);
    for (i, &inst) in machine_code.iter().enumerate() {
        jit.write_u32(i, inst);
    }
    jit.write_protect(true);

    unsafe { mem::transmute(jit.code_buffer) }
}

pub fn jit_main() {
    let fun: unsafe extern "C" fn() = run_jit();
    unsafe { fun() };
}
