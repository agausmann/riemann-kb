#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::{asm::wfi, delay::Delay, interrupt};
use embedded_hal::digital::v2::ToggleableOutputPin;
use rp2040_hal::{clocks, entry, gpio::Pins, pac::Peripherals, Clock, Sio, Watchdog};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    let _ = info;

    loop {
        wfi();
    }
}

#[entry]
fn main() -> ! {
    let mut dp = Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(dp.WATCHDOG);

    let clocks = clocks::init_clocks_and_plls(
        12_000_000,
        dp.XOSC,
        dp.CLOCKS,
        dp.PLL_SYS,
        dp.PLL_USB,
        &mut dp.RESETS,
        &mut watchdog,
    )
    .map_err(|_| "failed to init clocks")
    .unwrap();

    let sio = Sio::new(dp.SIO);
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0, sio.gpio_bank0, &mut dp.RESETS);

    let mut indicator = pins.gpio25.into_readable_output();
    let mut delay = Delay::new(cp.SYST, clocks.system_clock.freq().to_Hz());

    loop {
        indicator.toggle().ok();
        delay.delay_ms(500);
    }
}
