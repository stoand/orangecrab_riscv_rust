#![no_std]
#![no_main]

use core::arch::global_asm;
use core::include_str;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

global_asm!(include_str!("../start.s"));


// constants taken from ~/orangecrab-examples/riscv/blink/generated/csr.h

const CSR_BASE: u32 = 0xe0000000;

const LED_RED: u32 = 0x6800;
const LED_BLUE: u32 = 0x6808;
const LED_GREEN: u32 = 0x6804;
const LED_RAW: u32 = 0x6810;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn mem_read(addr: u32) -> u32 {
    unsafe {
        let pointer: *mut u32 = (CSR_BASE + addr) as *mut u32;
        read_volatile(pointer)
    }
}

fn mem_write(addr: u32, val: u32) {
    unsafe {
        let pointer: *mut u32 = (CSR_BASE + addr) as *mut u32;
        write_volatile(pointer, val);
    }
}

#[no_mangle]
extern "C" fn main() {
    mem_write(LED_RAW, 0);

    mem_write(LED_RED, 0);
    mem_write(LED_BLUE, 0);

    loop {
        let val = if mem_read(0x8800) != 0 { 0 } else { 255 };
        mem_write(LED_GREEN, val);
    }
}

#[no_mangle]
extern "C" fn isr() {}
