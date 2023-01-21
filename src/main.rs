#![no_std]
#![no_main]

use core::arch::global_asm;
use core::include_str;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

global_asm!(include_str!("../start.s"));

const CSR_BASE: u32 = 0xe0000000;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn csr_write_simple(val: u32, addr: u32) {
    unsafe {
        let pointer: *mut u32 = (CSR_BASE + addr) as *mut u32;
        write_volatile(pointer, val);
    }
}

#[no_mangle]
extern "C" fn main() {
    csr_write_simple(0, 0x6810);
}

#[no_mangle]
extern "C" fn isr() {}
