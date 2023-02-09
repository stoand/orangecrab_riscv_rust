use crate::interrupts::{interrupt_mask, set_interrupt_mask, USB_INTERRUPT};
use crate::{mem_read, mem_write};
use core::mem::transmute;

fn set_usb_pullup_out(enabled: bool) {
    mem_write(0x4800, if enabled { 1 } else { 0 });
}

fn set_usb_address(addr: u32) {
    mem_write(0x4804, addr);
}

const USB_OUT_CTRL_RESET: u32 = 5;
const USB_OUT_CTRL_ENABLE: u32 = 4;

fn set_usb_out_ctrl(ctrl: u32) {
    mem_write(0x4840, ctrl);
}

fn set_usb_setup_ev_enable(ctrl: u32) {
    mem_write(0x4820, ctrl);
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

const USB_SETUP_STATUS_HAVE: u32 = 4;

fn usb_setup_status() -> u32 {
    mem_read(0x4814)
}

fn usb_setup_data() -> u8 {
    mem_read(0x480c) as u8
}

fn set_usb_interrupt_mask(enabled: bool) {
    if enabled {
        set_interrupt_mask(interrupt_mask() | (1 << USB_INTERRUPT));
    } else {
        set_interrupt_mask(interrupt_mask() & !(1 << USB_INTERRUPT));
    }
}

struct UsbSetupRequest {
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    length: u16,
}

pub struct UsbConnection {
    out_buffer_length: u32,
    setup_length: u32,
    out_have: u32,
    current_data: u32,
    current_length: u32,
    previous_setup_length: u32,
    setup_packet: [u8; 10],
    previous_setup_packet: [u8; 10],
    setup_packet_count: u32,
    next_address: u32,
}

impl UsbConnection {
    pub fn usb_connect(&self) {
        set_usb_setup_ev_pending(usb_setup_ev_pending());
        set_usb_in_ev_pending(usb_in_ev_pending());
        set_usb_out_ev_pending(usb_out_ev_pending());

        set_usb_setup_ev_enable(3);
        set_usb_in_ev_enabled(true);
        set_usb_out_ev_enabled(true);

        set_usb_in_ctrl(1 << USB_IN_CTRL_RESET);

        set_usb_setup_ctrl(1 << USB_SETUP_CTRL_RESET);

        set_usb_out_ctrl(1 << USB_OUT_CTRL_RESET);
        set_usb_out_ctrl(1 << USB_OUT_CTRL_ENABLE);

        set_usb_address(0);

        set_usb_pullup_out(true);

        set_usb_interrupt_mask(true);
    }

    pub fn usb_init(&mut self) {
        self.out_buffer_length = 0;

        set_usb_pullup_out(false);
        set_usb_address(0);
        set_usb_out_ctrl(0);

        set_usb_setup_ev_enable(0);
        set_usb_in_ev_enabled(false);
        set_usb_out_ev_enabled(false);

        set_usb_in_ctrl(1 << USB_IN_CTRL_RESET);

        set_usb_setup_ctrl(1 << USB_SETUP_CTRL_RESET);

        set_usb_out_ctrl(1 << USB_OUT_CTRL_RESET);
    }

    pub fn usb_isr(&mut self) {
        let setup_pending = usb_setup_ev_pending() as u8;
        let in_pending = usb_in_ev_pending() as u8;
        let out_pending = usb_out_ev_pending() as u8;

        if setup_pending & 2 != 0 {
            self.setup_length = 0;
            self.out_buffer_length = 0;
            self.out_have = 0;
            self.current_data = 0;
            self.current_length = 0;
            self.usb_connect();
            return;
        }

        if setup_pending & 1 != 0 {
            self.previous_setup_length = self.setup_length;
            self.previous_setup_packet = self.setup_packet;
            self.setup_length = 0;
            self.setup_packet = [0; 10];

            while usb_setup_status() & (1 << USB_SETUP_STATUS_HAVE) != 0 {
                self.setup_packet[self.setup_length as usize] = usb_setup_data();
                self.setup_length += 1;
            }

            if self.setup_length == 10 {
                self.setup_packet_count += 1;
            } else {
                self.setup_length = 0;
            }
        }

        if in_pending != 0 {
            self.process_tx();

            if self.next_address != 0 {
                set_usb_address(self.next_address);
                self.next_address = 0;
            }
        }

        if out_pending != 0 {
            self.process_rx();
        }

        set_usb_setup_ev_pending(setup_pending as u32);
        set_usb_in_ev_pending(in_pending as u32);
        set_usb_out_ev_pending(out_pending as u32);
    }

    fn process_tx(&mut self) {}

    fn process_rx(&mut self) {}

    pub fn usb_setup(&mut self, usb_setup_request: UsbSetupRequest) {
        
    }

    pub fn usb_poll(&mut self) {
        if self.setup_length != 0 {
            self.setup_length = 0;
            let p = self.setup_packet;
            let get_u16 = |index| unsafe { transmute::<[u8; 2], u16>([p[index], p[index + 1]]) };
            let usb_setup_request = UsbSetupRequest {
                request_type: p[0],
                request: p[1],
                value: get_u16(2),
                index: get_u16(4),
                length: get_u16(6),
            };
            self.usb_setup(usb_setup_request);
        }
    }

    pub const fn new() -> Self {
        UsbConnection {
            out_buffer_length: 0,
            setup_length: 0,
            out_have: 0,
            current_data: 0,
            current_length: 0,
            previous_setup_length: 0,
            setup_packet: [0; 10],
            previous_setup_packet: [0; 10],
            setup_packet_count: 0,
            next_address: 0,
        }
    }
}
