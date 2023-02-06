use core::arch::asm;

fn interrupt_mask() -> u32 {
    let mut mask: u32;
    unsafe {
        asm!("csrr {}, {}", out(reg) mask, const 0xbc0);
    }

    mask
}

fn interrupt_pending() -> u32 {
    let mut pending: u32;
    unsafe {
        asm!("csrr {}, {}", out(reg) pending, const 0xfc0);
    }

    pending
}

pub fn usb_interrupt() -> bool {
    const USB_INTERRUPT: u32 = 3;
    interrupt_mask() & interrupt_pending() & (1 << USB_INTERRUPT) != 0
}
