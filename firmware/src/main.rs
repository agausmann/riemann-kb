#![no_std]
#![no_main]

mod debounce;
#[macro_use]
mod digit;
mod keycode;
mod keymap;
mod nkro;

use core::{cell::Cell, mem::MaybeUninit, panic::PanicInfo};

use cortex_m::{
    asm::wfi,
    delay::Delay,
    interrupt::Mutex,
    peripheral::NVIC,
    prelude::{_embedded_hal_blocking_spi_Write, _embedded_hal_timer_CountDown},
};
use debounce::{Debounce, Defer};
use digit::DIGITS;
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
    gpio::{
        bank0::{Gpio13, Gpio23},
        DynPin, FunctionPwm, FunctionSpi, Pin, Pins, PushPullOutput, ReadableOutput,
    },
    pac::{interrupt, Interrupt, Peripherals, SPI1},
    pwm::Slices,
    timer::Alarm,
    usb::UsbBus,
    Clock, Sio, Spi, Timer, Watchdog,
};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
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
    fn poll(&mut self, flags: &UsbFlags) {
        self.device.poll(&mut [&mut self.hid]);
        let mut leds = [0u8; 1];
        if self.hid.pull_raw_output(&mut leds).is_ok() {
            flags.output.set(Some(Leds {
                raw: leds[0],
                num: leds[0] & (1 << 0) != 0,
                caps: leds[0] & (1 << 1) != 0,
                scroll: leds[0] & (1 << 2) != 0,
            }));
        }
    }
}

struct UsbFlags {
    output: Cell<Option<Leds>>,
}

// Resources sent to the USB interrupt contexts.
static mut USB_CTX: MaybeUninit<UsbContext> = MaybeUninit::uninit();

// TODO more granular mutex based on which interrupts access this
static USB_FLAGS: Mutex<UsbFlags> = Mutex::new(UsbFlags {
    output: Cell::new(None),
});

struct PanicContext {
    indicator: Pin<Gpio23, ReadableOutput>,
    system_clock: HertzU32,
}

static mut PANIC_CTX: Option<PanicContext> = None;

struct Encoder {
    a: DynPin,
    b: DynPin,
    debounce: Debounce<Option<i8>, 1>,
    delta: i8,
}

impl Encoder {
    fn new(mut a: DynPin, mut b: DynPin) -> Self {
        a.into_pull_up_input();
        b.into_pull_up_input();
        Self {
            a,
            b,
            debounce: Debounce::new(None),
            delta: 0,
        }
    }

    fn poll(&mut self) -> i8 {
        // Map consecutive pin states to sequential numbers.
        // Clockwise is 0-1-2-3-0, counter-clockwise is 0-3-2-1-0
        let (a, b) = match (self.a.is_high(), self.b.is_high()) {
            (Ok(x), Ok(y)) => (x, y),
            _ => return 0,
        };
        let current_state = match (a, b) {
            (false, false) => 0,
            (false, true) => 1,
            (true, true) => 2,
            (true, false) => 3,
        };
        if let Some((Some(before), Some(after))) = self.debounce.update(Some(current_state)) {
            let diff = match (before, after) {
                (0, 0) | (1, 1) | (2, 2) | (3, 3) => 0,
                (0, 1) | (1, 2) | (2, 3) | (3, 0) => 1,
                (0, 3) | (1, 0) | (2, 1) | (3, 2) => -1,

                // Error - state was skipped:
                (0, 2) | (1, 3) | (2, 0) | (3, 1) => 0,

                _ => unreachable!(),
            };
            self.delta += diff;
            // NOTE: Signed integer division rounds toward zero,
            // effectively same as signum(delta) * floor(abs(delta) / 2))
            let detents = self.delta / 4;
            self.delta %= 4;
            detents
        } else {
            0
        }
    }
}

const ENCODER_ANIMATION: [u16; 4] = [
    digit!(
        1
      .   .
        .
      .   .
        .   .
    ),
    digit!(
        .
      .   1
        .
      .   .
        .   .
    ),
    digit!(
        .
      .   .
        1
      .   .
        .   .
    ),
    digit!(
        .
      1   .
        .
      .   .
        .   .
    ),
];

struct System {
    rows: [DynPin; 10],
    columns: [DynPin; 6],
    pressed_keys: [u8; 10],
    deferred_release: Defer<Keycode, 60, 5>,

    left_encoder: Encoder,
    right_encoder: Encoder,
    left_index: u8,
    right_index: u8,

    layer_mask: u8,

    input: NkroKeyboardReport,
    input_changed: bool,
    leds: Leds,

    spi: Spi<rp2040_hal::spi::Enabled, SPI1, 8>,
    led_latch: Pin<Gpio13, PushPullOutput>,

    press_buckets: [u8; 100],
    bucket_index: usize,
}

#[allow(dead_code)]
struct Leds {
    raw: u8,
    caps: bool,
    num: bool,
    scroll: bool,
}

impl System {
    fn poll_matrix(&mut self) {
        // Handle deferred/debounced releases:
        self.deferred_release.tick();
        while let Some(keycode) = self.deferred_release.poll() {
            self.handle_key_event(keycode, false);
        }

        for i in 0..self.rows.len() {
            self.rows[i].set_low().ok();
            for j in 0..self.columns.len() {
                // Reverse index for right half
                let col_pin_index = if i < self.rows.len() / 2 {
                    j
                } else {
                    self.columns.len() - 1 - j
                };

                let prev_pressed = (self.pressed_keys[i] & (1 << j)) != 0;
                let pressed = self.columns[col_pin_index].is_low().unwrap_or(false);

                if prev_pressed != pressed {
                    let keycode = LAYERS
                        .iter()
                        .enumerate()
                        .rev()
                        .filter(|(k, _layer)| (self.layer_mask & (1 << k)) != 0)
                        .map(|(_k, layer)| layer[i][j])
                        .find(|kc| *kc != KC_TRNS)
                        .unwrap_or(KC_NO);

                    if pressed {
                        self.handle_key_event(keycode, pressed);
                    } else {
                        match self.deferred_release.defer(keycode) {
                            Ok(()) => {}
                            Err(keycode) => {
                                // Handle immediately if there are really too
                                // many releases deferred.
                                self.handle_key_event(keycode, pressed);
                            }
                        }
                    }

                    if pressed {
                        self.press_buckets[self.bucket_index] += 1;
                        self.pressed_keys[i] |= 1 << j;
                    } else {
                        self.pressed_keys[i] &= !(1 << j);
                    }
                }
            }
            self.rows[i].set_high().ok();
        }
    }

    fn handle_key_event(&mut self, keycode: Keycode, pressed: bool) {
        match keycode {
            Keycode::Hid(hid_keycode) => {
                if pressed {
                    self.input.press(hid_keycode as u8);
                    self.input_changed = true;
                } else {
                    self.input.release(hid_keycode as u8);
                    self.input_changed = true;
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
                        self.input.clear_all_but_mods();
                        self.input_changed = true;
                    }
                    LayerAction::Toggle => {
                        if pressed {
                            self.layer_mask ^= 1 << layer_keycode.layer();
                            self.input.clear_all_but_mods();
                            self.input_changed = true;
                        }
                    }
                    LayerAction::Oneshot => {} //TODO
                    LayerAction::To => {}      //TODO
                }
            }
            _ => {}
        }
    }

    fn poll_encoders(&mut self) {
        let left = self.left_encoder.poll();
        let right = self.right_encoder.poll();
        self.left_index = self.left_index.wrapping_add(left as u8);
        self.right_index = self.right_index.wrapping_add(right as u8);
    }

    fn poll_hid(&mut self, usb: &mut UsbContext, flags: &UsbFlags) {
        if self.input_changed && usb.hid.push_input(&self.input).is_ok() {
            self.input_changed = false;
        }
        if let Some(leds) = flags.output.take() {
            self.leds = leds;
        }
    }

    fn update_leds(&mut self) {
        let mut state = 0u16;
        if self.leds.caps {
            // Turn on right DP
            state |= 1 << 15;
        }

        let keys_per_second: u8 = self.press_buckets.iter().sum();
        self.bucket_index = (self.bucket_index + 1) % self.press_buckets.len();
        self.press_buckets[self.bucket_index] = 0;

        if keys_per_second > 99 {
            state = 0xffff;
        } else {
            state |= DIGITS[keys_per_second as usize / 10];
            state |= DIGITS[keys_per_second as usize % 10] << 8;
        }
        // state |= ENCODER_ANIMATION[self.left_index as usize % ENCODER_ANIMATION.len()];
        // state |= ENCODER_ANIMATION[self.right_index as usize % ENCODER_ANIMATION.len()] << 8;
        self.spi.write(&state.to_be_bytes()).ok();
        self.led_latch.set_high().ok();
        self.led_latch.set_low().ok();
    }
}

#[entry]
fn main() -> ! {
    let mut dp = Peripherals::take().unwrap();

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

    let dim_pin = pins.gpio16.into_mode::<FunctionPwm>();
    let mut dim_slice = pwms.pwm0;
    dim_slice.enable();
    let mut dim_channel = dim_slice.channel_a;
    dim_channel.output_to(dim_pin);
    dim_channel.enable();

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
        pressed_keys: [0u8; 10],
        deferred_release: Defer::new(),

        left_encoder: Encoder::new(pins.gpio0.into(), pins.gpio1.into()),
        right_encoder: Encoder::new(pins.gpio27.into(), pins.gpio28.into()),
        left_index: 0,
        right_index: 0,

        layer_mask: 1,

        input: NkroKeyboardReport::new(),
        input_changed: false,
        leds: Leds {
            raw: 0,
            caps: false,
            num: false,
            scroll: false,
        },

        spi: Spi::new(dp.SPI1).init(
            &mut dp.RESETS,
            clocks.peripheral_clock.freq(),
            1.MHz(),
            &spi::Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
        ),
        led_latch: pins.gpio13.into_push_pull_output(),

        press_buckets: [0; 100],
        bucket_index: 0,
    };

    let mut timer = Timer::new(dp.TIMER, &mut dp.RESETS);

    let mut wakeup_alarm = timer.alarm_0().unwrap();
    wakeup_alarm.enable_interrupt();
    unsafe { NVIC::unmask(Interrupt::TIMER_IRQ_0) };

    let mut matrix_timer = timer.count_down();
    let mut encoder_timer = timer.count_down();
    let mut leds_timer = timer.count_down();

    let mut indicator = pins.gpio25.into_readable_output();
    let mut prev_active = false;

    unsafe { cortex_m::interrupt::enable() };

    loop {
        let active = cortex_m::interrupt::free(|_cs| {
            unsafe { USB_CTX.assume_init_mut() }.device.state() == UsbDeviceState::Configured
        });
        if prev_active != active {
            indicator.set_state(active.into()).ok();
            if active {
                matrix_timer.start(1.millis());
                encoder_timer.start(500.micros());
                leds_timer.start(10.millis());

                dim_channel.set_duty(0xcccc);
            } else {
                dim_channel.set_duty(0xffff);
            }
        }
        prev_active = active;

        if !active {
            if wakeup_alarm.schedule::<1, 1_000_000>(1.millis()).is_ok() {
                wfi();
            }
            continue;
        }

        if matrix_timer.wait().is_ok() {
            system.poll_matrix();
            cortex_m::interrupt::free(|cs| {
                let usb = unsafe { USB_CTX.assume_init_mut() };
                let flags = USB_FLAGS.borrow(cs);
                system.poll_hid(usb, flags);
            })
        } else if encoder_timer.wait().is_ok() {
            system.poll_encoders();
            cortex_m::interrupt::free(|cs| {
                let usb = unsafe { USB_CTX.assume_init_mut() };
                let flags = USB_FLAGS.borrow(cs);
                system.poll_hid(usb, flags);
            })
        } else if leds_timer.wait().is_ok() {
            system.update_leds();
        } else {
            if wakeup_alarm.schedule::<1, 1_000_000>(100.micros()).is_ok() {
                wfi();
            }
        }
    }
}

#[interrupt]
fn USBCTRL_IRQ() {
    cortex_m::interrupt::free(|cs| {
        let ctx = unsafe { USB_CTX.assume_init_mut() };
        let flags = USB_FLAGS.borrow(cs);
        ctx.poll(flags);
    })
}

#[interrupt]
fn TIMER_IRQ_0() {
    // Just used to periodically wake CPU.
    // TODO less hacky way to clear this interrupt?
    unsafe {
        Peripherals::steal()
            .TIMER
            .intr
            .write(|w| w.alarm_0().set_bit());
    }
}
