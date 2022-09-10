#![no_std]
#![no_main]

mod digit;
mod keycode;
mod keymap;
mod nkro;

use core::{mem::MaybeUninit, panic::PanicInfo};

use cortex_m::{asm::wfi, delay::Delay, peripheral::NVIC};
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    spi::{self, Phase, Polarity},
    PwmPin,
};
use fugit::{HertzU32, RateExtU32};
use keycode::{
    qmk::{KC_NO, KC_TRNS},
    Keycode, LayerAction,
};
use keymap::LAYERS;
use nkro::NkroKeyboardReport;
use rp2040_hal::{
    clocks, entry,
    gpio::{bank0::Gpio23, DynPin, FunctionPwm, FunctionSpi, Pin, Pins, ReadableOutput},
    pac::{interrupt, Interrupt, Peripherals},
    pwm::Slices,
    usb::UsbBus,
    Clock, Sio, Spi, Watchdog,
};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    let _ = info;

    if let Some(ctx) = unsafe { PANIC_CTX.as_mut() } {
        let cp = unsafe { cortex_m::Peripherals::steal() };
        let mut delay = Delay::new(cp.SYST, ctx.system_clock.to_Hz());
        loop {
            ctx.indicator.set_high().ok();
            delay.delay_ms(250);
            ctx.indicator.set_low().ok();
            delay.delay_ms(250);
        }
    }
    loop {
        wfi();
    }
}

struct UsbContext {
    device: UsbDevice<'static, UsbBus>,
    hid: HIDClass<'static, UsbBus>,
}

impl UsbContext {
    fn poll(&mut self) {
        self.device.poll(&mut [&mut self.hid]);
    }
}

// Resources sent to the USB interrupt contexts.
static mut USB_CTX: MaybeUninit<UsbContext> = MaybeUninit::uninit();

struct PanicContext {
    indicator: Pin<Gpio23, ReadableOutput>,
    system_clock: HertzU32,
}

static mut PANIC_CTX: Option<PanicContext> = None;

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

    unsafe {
        PANIC_CTX = Some(PanicContext {
            indicator: pins.gpio23.into_readable_output(),
            system_clock: clocks.system_clock.freq(),
        })
    }
    let usb_bus = {
        static mut USB_BUS: MaybeUninit<UsbBusAllocator<UsbBus>> = MaybeUninit::uninit();
        unsafe {
            USB_BUS.write(UsbBusAllocator::new(UsbBus::new(
                dp.USBCTRL_REGS,
                dp.USBCTRL_DPRAM,
                clocks.usb_clock,
                true,
                &mut dp.RESETS,
            )))
        }
    };

    let usb_hid = HIDClass::new(usb_bus, NkroKeyboardReport::desc(), 1);
    // TODO allocate a PID code https://pid.codes
    let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Gaussian")
        .product("Riemann")
        .device_release(0x0001)
        .build();

    unsafe {
        USB_CTX.write(UsbContext {
            device: usb_device,
            hid: usb_hid,
        });
        NVIC::unmask(Interrupt::USBCTRL_IRQ);
    }

    let mut rows: [DynPin; 10] = [
        pins.gpio2.into_push_pull_output().into(),
        pins.gpio3.into_push_pull_output().into(),
        pins.gpio4.into_push_pull_output().into(),
        pins.gpio5.into_push_pull_output().into(),
        pins.gpio6.into_push_pull_output().into(),
        pins.gpio26.into_push_pull_output().into(),
        pins.gpio22.into_push_pull_output().into(),
        pins.gpio21.into_push_pull_output().into(),
        pins.gpio20.into_push_pull_output().into(),
        pins.gpio19.into_push_pull_output().into(),
    ];
    // avoid conflicting with mutable borrows:
    let num_rows = rows.len();
    let columns: [DynPin; 6] = [
        pins.gpio7.into_pull_up_input().into(),
        pins.gpio8.into_pull_up_input().into(),
        pins.gpio9.into_pull_up_input().into(),
        pins.gpio10.into_pull_up_input().into(),
        pins.gpio11.into_pull_up_input().into(),
        pins.gpio12.into_pull_up_input().into(),
    ];
    let left_encoder: [DynPin; 2] = [
        pins.gpio0.into_pull_up_input().into(),
        pins.gpio1.into_pull_up_input().into(),
    ];
    let right_encoder: [DynPin; 2] = [
        pins.gpio27.into_pull_up_input().into(),
        pins.gpio28.into_pull_up_input().into(),
    ];
    let mut layer_mask = 1u8;
    let mut pressed_keys = [0u8; 10];
    let mut report = NkroKeyboardReport::new();
    let mut sent = false;

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

    dim_channel.set_duty(0xffff);
    indicator.set_high().unwrap();

    unsafe { cortex_m::interrupt::enable() };

    loop {
        delay.delay_us(500);

        let mut changed = false;
        for (i, row) in rows.iter_mut().enumerate() {
            row.set_low().unwrap();
            for (j, col) in columns.iter().enumerate() {
                // Reverse index for right half
                let j = if i < num_rows / 2 {
                    j
                } else {
                    columns.len() - 1 - j
                };

                let prev_pressed = (pressed_keys[i] & (1 << j)) != 0;
                let pressed = col.is_low().unwrap();

                if prev_pressed != pressed {
                    let keycode = LAYERS
                        .iter()
                        .enumerate()
                        .rev()
                        .filter(|(k, _layer)| (layer_mask & (1 << k)) != 0)
                        .map(|(_k, layer)| layer[i][j])
                        .find(|kc| *kc != KC_TRNS)
                        .unwrap_or(KC_NO);
                    match keycode {
                        Keycode::Hid(hid_keycode) => {
                            if pressed {
                                report.press(hid_keycode as u8);
                            } else {
                                report.release(hid_keycode as u8);
                            }
                        }
                        Keycode::Layer(layer_keycode) => {
                            match layer_keycode.action() {
                                LayerAction::Momentary => {
                                    if pressed {
                                        layer_mask |= 1 << layer_keycode.layer();
                                    } else {
                                        layer_mask &= !(1 << layer_keycode.layer());
                                    }
                                    report.clear_all_but_mods();
                                }
                                LayerAction::Toggle => {
                                    if pressed {
                                        layer_mask ^= 1 << layer_keycode.layer();
                                        report.clear_all_but_mods();
                                    }
                                }
                                LayerAction::Oneshot => {} //TODO
                                LayerAction::To => {}      //TODO
                            }
                        }
                        _ => {}
                    }
                    changed = true;
                }

                if pressed {
                    pressed_keys[i] |= 1 << j;
                } else {
                    pressed_keys[i] &= !(1 << j);
                }
            }
            row.set_high().unwrap();
        }
        if changed {
            sent = false;
        }

        if !sent {
            cortex_m::interrupt::free(|_cs| {
                let usb = unsafe { USB_CTX.assume_init_mut() };
                if usb.hid.push_input(&report).is_ok() {
                    sent = true;
                }
            });
        }
    }
}

#[interrupt]
fn USBCTRL_IRQ() {
    let ctx = unsafe { USB_CTX.assume_init_mut() };
    ctx.poll();
}
