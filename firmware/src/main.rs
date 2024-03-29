#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

mod debounce;
#[macro_use]
mod digit;
mod changed;
mod emoji;
mod encoder;
mod key_macro;
mod keycode;
mod keymap;
mod report;
mod usb;

use crate::{
    debounce::Defer,
    digit::DIGITS,
    encoder::Encoder,
    keycode::{
        qmk::{KC_NO, KC_TRNS},
        Keycode, LayerAction,
    },
    keymap::LAYERS,
};
use changed::Changed;
use core::panic::PanicInfo;
use cortex_m::{asm::wfi, delay::Delay, peripheral::NVIC};
use digit::{CAPS, EMO, FU, HI, META, REV};
use embedded_hal::{
    blocking::spi::Write,
    digital::v2::{InputPin, OutputPin},
    spi::{self, Phase, Polarity},
    timer::CountDown,
    PwmPin,
};
use emoji::emoji_macro;
use fugit::{ExtU32, HertzU32, RateExtU32};
use heapless::Deque;
use key_macro::KeyMacro;
use keycode::{
    keycode_is_printable,
    qmk::{KC_AUDIO_VOL_DOWN, KC_AUDIO_VOL_UP, KC_LSFT},
    InputMode, KeyAction, SystemKeycode,
};
use keymap::{LAYER_FU, LAYER_META};
use packed_struct::PackedStruct;
use report::{ConsumerReport, KeyboardReport};
use rp2040_hal::{
    clocks, entry,
    gpio::{
        bank0::{Gpio13, Gpio23},
        DynPin, FunctionPwm, FunctionSpi, Pin, Pins, PushPullOutput, ReadableOutput,
    },
    pac::{interrupt, Interrupt, Peripherals, SPI1},
    pwm::Slices,
    rom_data::reset_to_usb_boot,
    timer::Alarm,
    Clock, Sio, Spi, Timer, Watchdog,
};
use usb_device::prelude::UsbDeviceState;
use usbd_human_interface_device::{
    device::{keyboard::KeyboardLedsReport, DeviceClass},
    page::Keyboard,
    UsbHidError,
};

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
        for _ in 0..10 {
            ctx.indicator.set_high().ok();
            delay.delay_ms(250);
            ctx.indicator.set_low().ok();
            delay.delay_ms(250);
        }
    }

    reset_to_usb_boot(1 << 25, 0);

    loop {
        wfi();
    }
}

struct PanicContext {
    indicator: Pin<Gpio23, ReadableOutput>,
    system_clock: HertzU32,
}

static mut PANIC_CTX: Option<PanicContext> = None;

struct System {
    rows: [DynPin; 10],
    columns: [DynPin; 6],
    pressed_keys: [u8; 10],
    deferred_release: Defer<(u8, u8, Keycode), 60, 5>,

    queued_keys: Deque<(Keycode, KeyAction), 128>,

    left_encoder: Encoder,
    right_encoder: Encoder,
    left_index: u8,
    right_index: u8,

    layer_mask: u8,

    input: Changed<KeyboardReport>,
    consumer: Changed<ConsumerReport>,
    leds: KeyboardLedsReport,

    spi: Spi<rp2040_hal::spi::Enabled, SPI1, 8>,
    led_latch: Pin<Gpio13, PushPullOutput>,

    press_buckets: [u8; 100],
    bucket_index: usize,

    mode: InputMode,
}

impl System {
    fn poll_matrix(&mut self) {
        // Handle deferred/debounced releases:
        self.deferred_release.tick();
        while let Some((row, col, keycode)) = self.deferred_release.poll() {
            // Make sure it is still released.
            if self.pressed_keys[row as usize] & (1 << col) == 0 {
                self.handle_user_key_event(keycode, false);
            }
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
                        self.handle_user_key_event(keycode, pressed);
                    } else {
                        match self.deferred_release.defer((i as u8, j as u8, keycode)) {
                            Ok(()) => {}
                            Err(..) => {
                                // Handle immediately if there are really too
                                // many releases deferred.
                                self.handle_user_key_event(keycode, pressed);
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

    fn handle_user_key_event(&mut self, keycode: Keycode, pressed: bool) {
        match keycode {
            Keycode::KeyboardPage(key) => match self.mode {
                InputMode::Normal => self.input.set(key, pressed),
                InputMode::RegionalIndicator => {
                    if let Some(macro_str) = emoji_macro(key) {
                        if pressed {
                            self.queue_str(macro_str);
                        }
                    } else {
                        self.input.set(key, pressed);
                    }
                }
                InputMode::Reverse => {
                    if keycode_is_printable(key) && pressed {
                        self.queue_oneshot(keycode);
                        self.queue_oneshot(Keycode::KeyboardPage(Keyboard::LeftArrow));
                    } else {
                        self.input.set(key, pressed);
                    }
                }
            },
            _ => self.handle_key_event(keycode, pressed),
        }
    }

    fn handle_key_event(&mut self, keycode: Keycode, pressed: bool) {
        match keycode {
            Keycode::KeyboardPage(key) => {
                self.input.set(key, pressed);
            }
            Keycode::ConsumerPage(key) => {
                self.consumer.set(key, pressed);
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
                        self.consumer.clear();
                    }
                    LayerAction::Toggle => {
                        if pressed {
                            self.layer_mask ^= 1 << layer_keycode.layer();
                            self.input.clear_all_but_mods();
                            self.consumer.clear();
                        }
                    }
                    LayerAction::Oneshot => {} //TODO
                    LayerAction::To => {}      //TODO
                }
            }
            Keycode::System(system_keycode) => match system_keycode {
                SystemKeycode::Reset => {
                    reset_to_usb_boot(1 << 25, 0);
                }
                SystemKeycode::None => {}
                SystemKeycode::Transparent => {}
                SystemKeycode::BacklightDown => {}
                SystemKeycode::BacklightUp => {}
                SystemKeycode::BacklightStep => {}
            },
            Keycode::InputMode(mode) => {
                if pressed {
                    self.mode = mode;
                    self.input.clear();
                    self.queued_keys.clear();
                }
            }
            Keycode::User(_) => {}
        }
    }

    fn queue_oneshot(&mut self, key: Keycode) {
        // Don't queue a press if you can't also queue a release
        if self.queued_keys.capacity() - self.queued_keys.len() >= 2 {
            self.queued_keys.push_back((key, KeyAction::Pressed)).ok();
            self.queued_keys.push_back((key, KeyAction::Released)).ok();
        }
    }

    fn queue_char(&mut self, c: char) {
        //TODO account for caps lock

        if let Some(key_macro) = KeyMacro::from_char(c) {
            // Don't queue a press if you can't also queue a release
            let required_length = if key_macro.shifted { 4 } else { 2 };
            if self.queued_keys.capacity() - self.queued_keys.len() < required_length {
                return;
            }

            if key_macro.shifted {
                self.queued_keys
                    .push_back((KC_LSFT, KeyAction::Pressed))
                    .ok();
            }
            self.queued_keys
                .push_back((key_macro.key, KeyAction::Pressed))
                .ok();
            self.queued_keys
                .push_back((key_macro.key, KeyAction::Released))
                .ok();
            if key_macro.shifted {
                self.queued_keys
                    .push_back((KC_LSFT, KeyAction::Released))
                    .ok();
            }
        }
    }

    fn queue_str(&mut self, s: &str) {
        for c in s.chars() {
            self.queue_char(c);
        }
    }

    fn poll_encoders(&mut self) {
        let left = self.left_encoder.poll();
        let right = self.right_encoder.poll();
        self.left_index = self.left_index.wrapping_add(left as u8);
        self.right_index = self.right_index.wrapping_add(right as u8);

        if left > 0 {
            self.queue_oneshot(KC_AUDIO_VOL_UP);
        } else if left < 0 {
            self.queue_oneshot(KC_AUDIO_VOL_DOWN);
        }
    }

    fn poll_hid(
        &mut self,
        usb: &mut usb::UsbContext,
        flags: &usb::UsbFlags,
    ) -> Result<(), UsbHidError> {
        usb.tick()?;

        if !self.input.is_changed() && !self.consumer.is_changed() {
            // Handle one queued event at a time, then wait until it has been
            // sent and the keyboard is idle.
            if let Some((key, action)) = self.queued_keys.pop_front() {
                self.handle_key_event(key, action.is_pressed())
            }
        }

        if self.input.is_changed() {
            let report = self
                .input
                .inner()
                .pack()
                .map_err(|_| UsbHidError::SerializationError)?;
            usb.keyboard().interface().write_report(&report)?;
            self.input.take();
        }
        if self.consumer.is_changed() {
            let report = self
                .consumer
                .inner()
                .pack()
                .map_err(|_| UsbHidError::SerializationError)?;
            usb.consumer().interface().write_report(&report)?;
            self.consumer.take();
        }
        if let Some(leds) = flags.leds.take() {
            self.leds = leds;
        }
        Ok(())
    }

    fn update_leds(&mut self) {
        let mut state = 0u16;

        let keys_per_second: u8 = self.press_buckets.iter().sum();
        self.bucket_index = (self.bucket_index + 1) % self.press_buckets.len();
        self.press_buckets[self.bucket_index] = 0;

        if (self.layer_mask & (1 << LAYER_META)) != 0 {
            state = META;
        } else if self.leds.caps_lock {
            state = CAPS;
        } else if self.mode != InputMode::Normal {
            match self.mode {
                InputMode::Normal => unreachable!(),
                InputMode::RegionalIndicator => {
                    state = EMO;
                }
                InputMode::Reverse => {
                    state = REV;
                }
            }
        } else if (self.layer_mask & (1 << LAYER_FU)) != 0 {
            state = FU;
        } else {
            if keys_per_second > 99 {
                state = 0xffff;
            } else {
                state |= DIGITS[keys_per_second as usize / 10];
                state |= DIGITS[keys_per_second as usize % 10] << 8;
            }
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

    let _sclk = pins.gpio14.into_mode::<FunctionSpi>();
    let _mosi = pins.gpio15.into_mode::<FunctionSpi>();

    let dim_pin = pins.gpio16.into_mode::<FunctionPwm>();
    let mut dim_slice = pwms.pwm0;
    dim_slice.enable();
    let mut dim_channel = dim_slice.channel_a;
    dim_channel.output_to(dim_pin);
    dim_channel.enable();
    dim_channel.set_duty(0xffff);

    let mut spi = Spi::new(dp.SPI1).init(
        &mut dp.RESETS,
        clocks.peripheral_clock.freq(),
        1.MHz(),
        &spi::Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
    );
    let mut led_latch = pins.gpio13.into_push_pull_output();
    spi.write(&HI.to_be_bytes()).ok();
    led_latch.set_high().ok();
    led_latch.set_low().ok();
    dim_channel.set_duty(0);

    unsafe {
        usb::init(
            dp.USBCTRL_REGS,
            dp.USBCTRL_DPRAM,
            clocks.usb_clock,
            &mut dp.RESETS,
        );
    }

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
        queued_keys: Deque::new(),

        left_encoder: Encoder::new(pins.gpio0.into(), pins.gpio1.into()),
        right_encoder: Encoder::new(pins.gpio27.into(), pins.gpio28.into()),
        left_index: 0,
        right_index: 0,

        layer_mask: 1,

        input: Changed::new(KeyboardReport::new()),
        consumer: Changed::new(ConsumerReport::new()),
        leds: Default::default(),

        spi,
        led_latch,

        press_buckets: [0; 100],
        bucket_index: 0,

        mode: InputMode::Normal,
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
        let active =
            cortex_m::interrupt::free(|cs| unsafe { usb::state(cs) }) == UsbDeviceState::Configured;
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
                let (usb, flags) = unsafe { usb::borrow(cs) };
                system.poll_hid(usb, flags).ok();
            })
        } else if encoder_timer.wait().is_ok() {
            system.poll_encoders();
            cortex_m::interrupt::free(|cs| {
                let (usb, flags) = unsafe { usb::borrow(cs) };
                system.poll_hid(usb, flags).ok();
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
