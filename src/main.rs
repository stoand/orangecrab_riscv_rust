#![no_std]
#![no_main]
#![feature(asm_const)]

use core::arch::global_asm;
use core::include_str;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

global_asm!(include_str!("../start.s"));

mod interrupts;
mod usb;

use usb::UsbConnection;

use interrupts::{set_interrupt_ie_enabled, set_interrupt_mask, usb_interrupt};

use crate::interrupts::interrupt_mask;

// constants taken from ~/orangecrab-examples/riscv/blink/generated/csr.h

const CSR_BASE: u32 = 0xe0000000;

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

fn restart_to_bootloader() {
    const BOOTLOADER_IMAGE_INDEX: u32 = 0;
    mem_write(0x6000, 0xac | (BOOTLOADER_IMAGE_INDEX & 3) << 0);
}

fn button_pressed() -> bool {
    mem_read(0x8800) == 0
}

fn disable_rbg_special_effects() {
    mem_write(0x6810, 0);
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum RGB {
    Red,
    Green,
    Blue,
    Off,
}

fn set_rgb(rgb: RGB) {
    let set_bit = |b: bool| if b { 0xff } else { 0 };
    mem_write(0x6800, set_bit(rgb == RGB::Red));
    mem_write(0x6804, set_bit(rgb == RGB::Green));
    mem_write(0x6808, set_bit(rgb == RGB::Blue));
}

const USB_INTERRUPT: u32 = 3;

fn set_usb_interrupt_mask(enabled: bool) {
    if enabled {
        set_interrupt_mask(interrupt_mask() | (1 << USB_INTERRUPT));
    } else {
        set_interrupt_mask(interrupt_mask() & !(1 << USB_INTERRUPT));
    }
}

#[no_mangle]
extern "C" fn main() {
    disable_rbg_special_effects();
    // set_rgb(RGB::Blue);
    set_interrupt_mask(0);
    set_interrupt_ie_enabled();

    // set_usb_pullup_out(false);
    //
    let mut usb_connection = UsbConnection::new();

    usb_connection.usb_init();
    usb_connection.usb_connect();

    // usb_init();

    loop {
        if usb_interrupt() {
            set_rgb(RGB::Green);
        }

        if button_pressed() {
            restart_to_bootloader();
        }
    }
}

#[no_mangle]
extern "C" fn isr() {
    disable_rbg_special_effects();
    set_rgb(RGB::Green);
}
