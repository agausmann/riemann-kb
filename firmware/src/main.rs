#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::{asm::wfi, delay::Delay, interrupt};
use embedded_hal::{
    blocking::spi::Write,
    digital::v2::{OutputPin, ToggleableOutputPin},
    spi::{self, Phase, Polarity},
    PwmPin,
};
use fugit::RateExtU32;
use rp2040_hal::{
    clocks, entry,
    gpio::{FunctionPwm, FunctionSpi, Pins, Pwm},
    pac::Peripherals,
    pwm::Slices,
    Clock, Sio, Spi, Watchdog,
};

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
    let pwms = Slices::new(dp.PWM, &mut dp.RESETS);

    let mut indicator = pins.gpio25.into_readable_output();
    let mut delay = Delay::new(cp.SYST, clocks.system_clock.freq().to_Hz());

    let _sclk = pins.gpio14.into_mode::<FunctionSpi>();
    let _mosi = pins.gpio15.into_mode::<FunctionSpi>();
    let mut latch = pins.gpio13.into_push_pull_output();
    let mut spi: Spi<_, _, 8> = Spi::new(dp.SPI1).init(
        &mut dp.RESETS,
        clocks.peripheral_clock.freq(),
        1.MHz(),
        &spi::Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
    );

    let dim_pin = pins.gpio16.into_mode::<FunctionPwm>();
    let mut dim_slice = pwms.pwm0;
    dim_slice.enable();
    let mut dim_channel = dim_slice.channel_a;
    dim_channel.output_to(dim_pin);
    dim_channel.enable();

    for (&segments, duty) in ANIMATION.iter().cycle().zip(
        (MIN_DUTY..=MAX_DUTY)
            .rev()
            .chain(MIN_DUTY..=MAX_DUTY)
            .cycle()
            .step_by(2000),
    ) {
        dim_channel.set_duty(duty);
        spi.write(&segments.to_le_bytes()).ok();
        delay.delay_us(10);
        latch.set_high().ok();
        delay.delay_us(10);
        latch.set_low().ok();
        delay.delay_ms(100);
    }

    loop {}
}

const MIN_DUTY: u16 = u16::MIN;
const MAX_DUTY: u16 = u16::MAX;

const ANIMATION: &[u16] = WIPER;

const CIRCLES: &[u16] = &[0x0101, 0x0202, 0x0404, 0x0808, 0x1010, 0x2020];
const MIRROR_CIRCLES: &[u16] = &[0x0101, 0x0220, 0x0410, 0x0808, 0x1004, 0x2002];
const WIPER: &[u16] = &[0x3000, 0x0600, 0x0030, 0x0006, 0x0030, 0x0600];
