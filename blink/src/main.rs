#![no_std]
#![no_main]

extern crate panic_halt;

use hpm6750_rs::Peripherals;
use riscv_rt::entry;

fn board_turn_off_rgb_led(dp: &Peripherals) {
    let ioc = &dp.IOC;

    ioc.pad_d[15]
        .function
        .write(|w| unsafe { w.alternate().bits(0).loop_back().enable() });
    ioc.pad_d[15]
        .pad
        .write(|w| w.pull_enable().enable().pull_direction().down());
}

fn board_init(dp: &Peripherals) {
    // Enable AXI_SRAM1 for stack, and link GPIO0_1 to GROUP0
    dp.SYSCTL
        .group0_0_value
        .write(|w| w.axi_sram1().link().gpio().link());

    board_turn_off_rgb_led(dp);
}

fn board_init_led_pins(dp: &Peripherals) {
    let gpio0 = &dp.GPIO0;

    gpio0.do_gpiod_set.write(|w| unsafe { w.bits(1 << 15) });
    gpio0.oe_gpiod_set.write(|w| unsafe { w.bits(1 << 15) });
}

fn board_led_b_toggle(dp: &Peripherals) {
    let gpio0 = &dp.GPIO0;

    gpio0.do_gpiod_toggle.write(|w| unsafe { w.bits(1 << 15) });
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    board_init(&dp);
    board_init_led_pins(&dp);
    loop {
        board_led_b_toggle(&dp);
        for _ in 0..50000 {}
    }
}
