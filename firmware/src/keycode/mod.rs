#![allow(dead_code)]

use usbd_human_interface_device::page;

pub mod qmk;

#[derive(Clone, Copy, PartialEq)]
pub enum KeyAction {
    Pressed,
    Released,
}

impl KeyAction {
    pub fn is_pressed(&self) -> bool {
        *self == Self::Pressed
    }

    pub fn is_released(&self) -> bool {
        *self == Self::Released
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keycode {
    KeyboardPage(page::Keyboard),
    ConsumerPage(page::Consumer),
    System(SystemKeycode),
    Layer(LayerKeycode),
    User(u8),
}

impl From<SystemKeycode> for Keycode {
    fn from(v: SystemKeycode) -> Self {
        Self::System(v)
    }
}

impl From<page::Keyboard> for Keycode {
    fn from(v: page::Keyboard) -> Self {
        Self::KeyboardPage(v)
    }
}

impl From<page::Consumer> for Keycode {
    fn from(v: page::Consumer) -> Self {
        Self::ConsumerPage(v)
    }
}

impl From<LayerKeycode> for Keycode {
    fn from(v: LayerKeycode) -> Self {
        Self::Layer(v)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKeycode {
    None,
    Transparent,
    Reset,
    BacklightDown,
    BacklightUp,
    BacklightStep,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LayerKeycode(u8);

impl LayerKeycode {
    const LAYER_MASK: u8 = 0x1f;

    pub const fn new(action: LayerAction, layer: u8) -> Self {
        assert!(layer & Self::LAYER_MASK == layer);
        Self(action.code() | layer)
    }

    pub const fn action(&self) -> LayerAction {
        LayerAction::from_code(self.0)
    }

    pub const fn layer(&self) -> u8 {
        self.0 & Self::LAYER_MASK
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayerAction {
    Momentary,
    Oneshot,
    Toggle,
    To,
}

impl LayerAction {
    const MASK: u8 = 0xe0;
    const MOMENTARY: u8 = 0x20;
    const ONESHOT: u8 = 0x40;
    const TOGGLE: u8 = 0x60;
    const TO: u8 = 0x80;

    const fn code(&self) -> u8 {
        match self {
            Self::Momentary => Self::MOMENTARY,
            Self::Oneshot => Self::ONESHOT,
            Self::Toggle => Self::TOGGLE,
            Self::To => Self::TO,
        }
    }

    const fn from_code(code: u8) -> Self {
        match code & Self::MASK {
            Self::MOMENTARY => Self::Momentary,
            Self::ONESHOT => Self::Oneshot,
            Self::TOGGLE => Self::Toggle,
            Self::TO => Self::To,
            _ => panic!(),
        }
    }
}
