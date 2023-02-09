use crate::{mem_read, mem_write};

fn set_usb_pullup_out(enabled: bool) {
    mem_write(0x4800, if enabled { 1 } else { 0 });
}

fn set_usb_address(addr: u32) {
    mem_write(0x4804, addr);
}

const USB_OUT_CTRL_RESET: u32 = 5;

fn set_usb_out_ctrl(ctrl: u32) {
    mem_write(0x4840, ctrl);
}

fn set_usb_setup_ev_enable(enabled: bool) {
    mem_write(0x4820, if enabled { 1 } else { 0 });
}

fn set_usb_in_ev_enabled(enabled: bool) {
    mem_write(0x4838, if enabled { 1 } else { 0 });
}

fn set_usb_out_ev_enabled(enabled: bool) {
    mem_write(0x4850, if enabled { 1 } else { 0 });
}

const USB_IN_CTRL_RESET: u32 = 1;

fn set_usb_in_ctrl(ctrl: u32) {
    mem_write(0x4828, ctrl);
}

const USB_SETUP_CTRL_RESET: u32 = 5;

fn set_usb_setup_ctrl(ctrl: u32) {
    mem_write(0x4810, ctrl);
}

fn set_usb_setup_ev_pending(ctrl: u32) {
    mem_write(0x481c, ctrl);
}

fn usb_setup_ev_pending() -> u32 {
    mem_read(0x481c)
}

fn set_usb_in_ev_pending(ctrl: u32) {
    mem_write(0x4834, ctrl);
}

fn usb_in_ev_pending() -> u32 {
    mem_read(0x4834)
}

fn set_usb_out_ev_pending(ctrl: u32) {
    mem_write(0x484c, ctrl);
}

fn usb_out_ev_pending() -> u32 {
    mem_read(0x484c)
}

pub struct UsbConnection {
    out_buffer_length: u32,
}

impl UsbConnection {
    pub fn usb_connect(&self) {
        set_usb_setup_ev_pending(usb_setup_ev_pending());
        set_usb_in_ev_pending(usb_in_ev_pending());
        set_usb_out_ev_pending(usb_out_ev_pending());

        
    }

    pub fn usb_init(&mut self) {
        self.out_buffer_length = 0;

        set_usb_pullup_out(false);
        set_usb_address(0);
        set_usb_out_ctrl(0);

        set_usb_setup_ev_enable(false);
        set_usb_in_ev_enabled(false);
        set_usb_out_ev_enabled(false);

        set_usb_in_ctrl(1 << USB_IN_CTRL_RESET);

        set_usb_setup_ctrl(1 << USB_SETUP_CTRL_RESET);

        set_usb_out_ctrl(1 << USB_OUT_CTRL_RESET);
    }

    pub fn new() -> Self {
        UsbConnection {
            out_buffer_length: 0,
        }
    }
}
