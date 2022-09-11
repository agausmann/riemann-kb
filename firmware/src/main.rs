#![no_std]
#![no_main]

mod digit;
mod keycode;
mod keymap;
mod nkro;

use core::{mem::MaybeUninit, panic::PanicInfo};

use cortex_m::{asm::wfi, delay::Delay, peripheral::NVIC, prelude::_embedded_hal_timer_CountDown};
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    spi::{self, Phase, Polarity},
    PwmPin,
};
use fugit::{ExtU32, HertzU32, RateExtU32};
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
    Clock, Sio, Spi, Timer, Watchdog,
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

struct System {
    rows: [DynPin; 10],
    columns: [DynPin; 6],
    left_encoder: [DynPin; 2],
    right_encoder: [DynPin; 2],
    pressed_keys: [u8; 10],
    layer_mask: u8,
    report: NkroKeyboardReport,
    changed: bool,
}

impl System {
    fn poll_matrix(&mut self) {
        let num_rows = self.rows.len();

        for (i, row) in self.rows.iter_mut().enumerate() {
            row.set_low().unwrap();
            for (j, col) in self.columns.iter().enumerate() {
                // Reverse index for right half
                let j = if i < num_rows / 2 {
                    j
                } else {
                    self.columns.len() - 1 - j
                };

                let prev_pressed = (self.pressed_keys[i] & (1 << j)) != 0;
                let pressed = col.is_low().unwrap();

                if prev_pressed != pressed {
                    let keycode = LAYERS
                        .iter()
                        .enumerate()
                        .rev()
                        .filter(|(k, _layer)| (self.layer_mask & (1 << k)) != 0)
                        .map(|(_k, layer)| layer[i][j])
                        .find(|kc| *kc != KC_TRNS)
                        .unwrap_or(KC_NO);
                    match keycode {
                        Keycode::Hid(hid_keycode) => {
                            if pressed {
                                self.report.press(hid_keycode as u8);
                            } else {
                                self.report.release(hid_keycode as u8);
                            }
                        }
                        Keycode::Layer(layer_keycode) => {
                            match layer_keycode.action() {
                                LayerAction::Momentary => {
                                    if pressed {
                                        self.layer_mask |= 1 << layer_keycode.layer();
                                    } else {
                                        self.layer_mask &= !(1 << layer_keycode.layer());
                                    }
                                    self.report.clear_all_but_mods();
                                }
                                LayerAction::Toggle => {
                                    if pressed {
                                        self.layer_mask ^= 1 << layer_keycode.layer();
                                        self.report.clear_all_but_mods();
                                    }
                                }
                                LayerAction::Oneshot => {} //TODO
                                LayerAction::To => {}      //TODO
                            }
                        }
                        _ => {}
                    }
                    self.changed = true;
                }

                if pressed {
                    self.pressed_keys[i] |= 1 << j;
                } else {
                    self.pressed_keys[i] &= !(1 << j);
                }
            }
            row.set_high().unwrap();
        }
    }

    fn poll_hid(&mut self, usb: &mut UsbContext) {
        if self.changed && usb.hid.push_input(&self.report).is_ok() {
            self.changed = false;
        }
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

    let mut system = System {
        rows: [
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
        ],
        columns: [
            pins.gpio7.into_pull_up_input().into(),
            pins.gpio8.into_pull_up_input().into(),
            pins.gpio9.into_pull_up_input().into(),
            pins.gpio10.into_pull_up_input().into(),
            pins.gpio11.into_pull_up_input().into(),
            pins.gpio12.into_pull_up_input().into(),
        ],
        left_encoder: [
            pins.gpio0.into_pull_up_input().into(),
            pins.gpio1.into_pull_up_input().into(),
        ],
        right_encoder: [
            pins.gpio27.into_pull_up_input().into(),
            pins.gpio28.into_pull_up_input().into(),
        ],
        pressed_keys: [0u8; 10],
        layer_mask: 1,
        report: NkroKeyboardReport::new(),
        changed: false,
    };

    let mut delay = Delay::new(cp.SYST, clocks.system_clock.freq().to_Hz());
    let timer = Timer::new(dp.TIMER, &mut dp.RESETS);

    let mut matrix_countdown = timer.count_down();
    matrix_countdown.start(200.micros());

    let mut indicator = pins.gpio25.into_readable_output();
    indicator.set_high().unwrap();

    unsafe { cortex_m::interrupt::enable() };

    loop {
        if matrix_countdown.wait().is_ok() {
            system.poll_matrix();
            cortex_m::interrupt::free(|_cs| {
                let usb = unsafe { USB_CTX.assume_init_mut() };
                system.poll_hid(usb);
            })
        } else {
            delay.delay_us(200);
        }
    }
}

#[interrupt]
fn USBCTRL_IRQ() {
    let ctx = unsafe { USB_CTX.assume_init_mut() };
    ctx.poll();
}
