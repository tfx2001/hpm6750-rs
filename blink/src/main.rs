#![no_std]
#![no_main]

extern crate panic_halt;

use hpm6750_rs::Peripherals;
use riscv_rt::entry;

fn board_turn_off_rgb_led(dp: &Peripherals) {
    let ioc = &dp.IOC;

    ioc.pad_pb18_func_ctl.write(|w| unsafe { w.alt_select().bits(0) });
    ioc.pad_pb19_func_ctl.write(|w| unsafe { w.alt_select().bits(0) });
    ioc.pad_pb20_func_ctl.write(|w| unsafe { w.alt_select().bits(0) });

    ioc.pad_pb18_pad_ctl.write(|w| w.ps().bit(true).pe().bit(true));
    ioc.pad_pb19_pad_ctl.write(|w| w.ps().bit(true).pe().bit(true));
    ioc.pad_pb20_pad_ctl.write(|w| w.ps().bit(true).pe().bit(true));
}

fn board_init_led_pins(dp: &Peripherals) {
    let gpio0 = &dp.GPIO0;

    gpio0.oe_gpiob_set
        .write(|w| unsafe { w.bits(1 << 18 | 1 << 19 | 1 << 20) });
    gpio0.do_gpiob_set
        .write(|w| unsafe { w.bits(1 << 18 | 1 << 19 | 1 << 20) });
}

fn board_led_b_toggle(dp: &Peripherals) {
    let gpio0 = &dp.GPIO0;

    gpio0.do_gpiob_toggle.write(|w| unsafe { w.bits(1 << 20) });
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    board_turn_off_rgb_led(&dp);
    board_init_led_pins(&dp);
    loop {
        board_led_b_toggle(&dp);
        for _ in 0..50000 {}
    }
}