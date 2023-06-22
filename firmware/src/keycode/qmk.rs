#![allow(non_snake_case)]

//! Aliases for keycodes based on the names used in QMK/TMK.

use usbd_human_interface_device::page::{Keyboard, Consumer};

use super::{Keycode, LayerAction, LayerKeycode, SystemKeycode};

pub const fn MO(layer: u8) -> Keycode {
    Keycode::Layer(LayerKeycode::new(LayerAction::Momentary, layer))
}

pub const fn OSL(layer: u8) -> Keycode {
    Keycode::Layer(LayerKeycode::new(LayerAction::Oneshot, layer))
}

pub const fn TG(layer: u8) -> Keycode {
    Keycode::Layer(LayerKeycode::new(LayerAction::Toggle, layer))
}

pub const fn TO(layer: u8) -> Keycode {
    Keycode::Layer(LayerKeycode::new(LayerAction::To, layer))
}

pub const KC_NO: Keycode = Keycode::System(SystemKeycode::None);
pub const KC_TRANSPARENT: Keycode = Keycode::System(SystemKeycode::Transparent);
pub const RESET: Keycode = Keycode::System(SystemKeycode::Reset);

pub const XXXXXXX: Keycode = KC_NO;
pub const _______: Keycode = KC_TRNS;

// Short names for ease of definition of keymap
pub const KC_LCTL: Keycode = KC_LCTRL;
pub const KC_RCTL: Keycode = KC_RCTRL;
pub const KC_LSFT: Keycode = KC_LSHIFT;
pub const KC_RSFT: Keycode = KC_RSHIFT;
pub const KC_ESC: Keycode = KC_ESCAPE;
pub const KC_BSPC: Keycode = KC_BSPACE;
pub const KC_ENT: Keycode = KC_ENTER;
pub const KC_DEL: Keycode = KC_DELETE;
pub const KC_INS: Keycode = KC_INSERT;
pub const KC_CAPS: Keycode = KC_CAPSLOCK;
pub const KC_CLCK: Keycode = KC_CAPSLOCK;
pub const KC_RGHT: Keycode = KC_RIGHT;
pub const KC_PGDN: Keycode = KC_PGDOWN;
pub const KC_PSCR: Keycode = KC_PSCREEN;
pub const KC_SLCK: Keycode = KC_SCROLLLOCK;
pub const KC_PAUS: Keycode = KC_PAUSE;
pub const KC_BRK: Keycode = KC_PAUSE;
pub const KC_NLCK: Keycode = KC_NUMLOCK;
pub const KC_SPC: Keycode = KC_SPACE;
pub const KC_MINS: Keycode = KC_MINUS;
pub const KC_EQL: Keycode = KC_EQUAL;
pub const KC_GRV: Keycode = KC_GRAVE;
pub const KC_RBRC: Keycode = KC_RBRACKET;
pub const KC_LBRC: Keycode = KC_LBRACKET;
pub const KC_COMM: Keycode = KC_COMMA;
pub const KC_BSLS: Keycode = KC_BSLASH;
pub const KC_SLSH: Keycode = KC_SLASH;
pub const KC_SCLN: Keycode = KC_SCOLON;
pub const KC_QUOT: Keycode = KC_QUOTE;
pub const KC_APP: Keycode = KC_APPLICATION;
pub const KC_NUHS: Keycode = KC_NONUS_HASH;
pub const KC_NUBS: Keycode = KC_NONUS_BSLASH;
pub const KC_LCAP: Keycode = KC_LOCKING_CAPS;
pub const KC_LNUM: Keycode = KC_LOCKING_NUM;
pub const KC_LSCR: Keycode = KC_LOCKING_SCROLL;
pub const KC_ERAS: Keycode = KC_ALT_ERASE;
pub const KC_CLR: Keycode = KC_CLEAR;
/* Japanese specific */
pub const KC_ZKHK: Keycode = KC_GRAVE;
pub const KC_RO: Keycode = KC_INT1;
pub const KC_KANA: Keycode = KC_INT2;
pub const KC_JYEN: Keycode = KC_INT3;
pub const KC_JPY: Keycode = KC_INT3;
pub const KC_HENK: Keycode = KC_INT4;
pub const KC_MHEN: Keycode = KC_INT5;
/* Korean specific */
pub const KC_HAEN: Keycode = KC_LANG1;
pub const KC_HANJ: Keycode = KC_LANG2;
/* Keypad */
pub const KC_P1: Keycode = KC_KP_1;
pub const KC_P2: Keycode = KC_KP_2;
pub const KC_P3: Keycode = KC_KP_3;
pub const KC_P4: Keycode = KC_KP_4;
pub const KC_P5: Keycode = KC_KP_5;
pub const KC_P6: Keycode = KC_KP_6;
pub const KC_P7: Keycode = KC_KP_7;
pub const KC_P8: Keycode = KC_KP_8;
pub const KC_P9: Keycode = KC_KP_9;
pub const KC_P0: Keycode = KC_KP_0;
// pub const KC_P00: Keycode = KC_KP_00;
// pub const KC_P000: Keycode = KC_KP_000;
pub const KC_PDOT: Keycode = KC_KP_DOT;
pub const KC_PCMM: Keycode = KC_KP_COMMA;
pub const KC_PSLS: Keycode = KC_KP_SLASH;
pub const KC_PAST: Keycode = KC_KP_ASTERISK;
pub const KC_PMNS: Keycode = KC_KP_MINUS;
pub const KC_PPLS: Keycode = KC_KP_PLUS;
pub const KC_PEQL: Keycode = KC_KP_EQUAL;
pub const KC_PENT: Keycode = KC_KP_ENTER;
/* Unix function key */
pub const KC_EXEC: Keycode = KC_EXECUTE;
pub const KC_SLCT: Keycode = KC_SELECT;
pub const KC_AGIN: Keycode = KC_AGAIN;
pub const KC_PSTE: Keycode = KC_PASTE;
/*TODO
    /* Mousekey */
    pub const KC_MS_U: Keycode = KC_MS_UP;
    pub const KC_MS_D: Keycode = KC_MS_DOWN;
    pub const KC_MS_L: Keycode = KC_MS_LEFT;
    pub const KC_MS_R: Keycode = KC_MS_RIGHT;
    pub const KC_BTN1: Keycode = KC_MS_BTN1;
    pub const KC_BTN2: Keycode = KC_MS_BTN2;
    pub const KC_BTN3: Keycode = KC_MS_BTN3;
    pub const KC_BTN4: Keycode = KC_MS_BTN4;
    pub const KC_BTN5: Keycode = KC_MS_BTN5;
    pub const KC_WH_U: Keycode = KC_MS_WH_UP;
    pub const KC_WH_D: Keycode = KC_MS_WH_DOWN;
    pub const KC_WH_L: Keycode = KC_MS_WH_LEFT;
    pub const KC_WH_R: Keycode = KC_MS_WH_RIGHT;
    pub const KC_ACL0: Keycode = KC_MS_ACCEL0;
    pub const KC_ACL1: Keycode = KC_MS_ACCEL1;
    pub const KC_ACL2: Keycode = KC_MS_ACCEL2;
    /* Sytem Control */
    pub const KC_PWR : Keycode = KC_SYSTEM_POWER;
    pub const KC_SLEP: Keycode = KC_SYSTEM_SLEEP;
    pub const KC_WAKE: Keycode = KC_SYSTEM_WAKE;
*/
/* Consumer Page */
pub const KC_MUTE: Keycode = KC_AUDIO_MUTE;
pub const KC_VOLU: Keycode = KC_AUDIO_VOL_UP;
pub const KC_VOLD: Keycode = KC_AUDIO_VOL_DOWN;
pub const KC_MNXT: Keycode = KC_MEDIA_NEXT_TRACK;
pub const KC_MPRV: Keycode = KC_MEDIA_PREV_TRACK;
pub const KC_MFFD: Keycode = KC_MEDIA_FAST_FORWARD;
pub const KC_MRWD: Keycode = KC_MEDIA_REWIND;
pub const KC_MSTP: Keycode = KC_MEDIA_STOP;
pub const KC_MPLY: Keycode = KC_MEDIA_PLAY_PAUSE;
pub const KC_EJCT: Keycode = KC_MEDIA_EJECT;
// pub const KC_MSEL: Keycode = KC_MEDIA_SELECT;
// pub const KC_MAIL: Keycode = KC_MAIL;
// pub const KC_CALC: Keycode = KC_CALCULATOR;
// pub const KC_MYCM: Keycode = KC_MY_COMPUTER;
// pub const KC_WSCH: Keycode = KC_WWW_SEARCH;
// pub const KC_WHOM: Keycode = KC_WWW_HOME;
// pub const KC_WBAK: Keycode = KC_WWW_BACK;
// pub const KC_WFWD: Keycode = KC_WWW_FORWARD;
// pub const KC_WSTP: Keycode = KC_WWW_STOP;
// pub const KC_WREF: Keycode = KC_WWW_REFRESH;
// pub const KC_WFAV: Keycode = KC_WWW_FAVORITES;
// pub const KC_BRTI: Keycode = KC_BRIGHTNESS_INC;
// pub const KC_BRTD: Keycode = KC_BRIGHTNESS_DEC;

/* Jump to bootloader */
pub const KC_BTLD: Keycode = KC_BOOTLOADER;
/* Transparent */
pub const KC_TRNS: Keycode = KC_TRANSPARENT;

// Original names from `enum hid_keyboard_keypad_usage`
pub const KC_ROLL_OVER: Keycode = Keycode::KeyboardPage(Keyboard::ErrorRollOver);
pub const KC_POST_FAIL: Keycode = Keycode::KeyboardPage(Keyboard::POSTFail);
pub const KC_UNDEFINED: Keycode = Keycode::KeyboardPage(Keyboard::ErrorUndefine);
pub const KC_A: Keycode = Keycode::KeyboardPage(Keyboard::A);
pub const KC_B: Keycode = Keycode::KeyboardPage(Keyboard::B);
pub const KC_C: Keycode = Keycode::KeyboardPage(Keyboard::C);
pub const KC_D: Keycode = Keycode::KeyboardPage(Keyboard::D);
pub const KC_E: Keycode = Keycode::KeyboardPage(Keyboard::E);
pub const KC_F: Keycode = Keycode::KeyboardPage(Keyboard::F);
pub const KC_G: Keycode = Keycode::KeyboardPage(Keyboard::G);
pub const KC_H: Keycode = Keycode::KeyboardPage(Keyboard::H);
pub const KC_I: Keycode = Keycode::KeyboardPage(Keyboard::I);
pub const KC_J: Keycode = Keycode::KeyboardPage(Keyboard::J);
pub const KC_K: Keycode = Keycode::KeyboardPage(Keyboard::K);
pub const KC_L: Keycode = Keycode::KeyboardPage(Keyboard::L);
pub const KC_M: Keycode = Keycode::KeyboardPage(Keyboard::M);
pub const KC_N: Keycode = Keycode::KeyboardPage(Keyboard::N);
pub const KC_O: Keycode = Keycode::KeyboardPage(Keyboard::O);
pub const KC_P: Keycode = Keycode::KeyboardPage(Keyboard::P);
pub const KC_Q: Keycode = Keycode::KeyboardPage(Keyboard::Q);
pub const KC_R: Keycode = Keycode::KeyboardPage(Keyboard::R);
pub const KC_S: Keycode = Keycode::KeyboardPage(Keyboard::S);
pub const KC_T: Keycode = Keycode::KeyboardPage(Keyboard::T);
pub const KC_U: Keycode = Keycode::KeyboardPage(Keyboard::U);
pub const KC_V: Keycode = Keycode::KeyboardPage(Keyboard::V);
pub const KC_W: Keycode = Keycode::KeyboardPage(Keyboard::W);
pub const KC_X: Keycode = Keycode::KeyboardPage(Keyboard::X);
pub const KC_Y: Keycode = Keycode::KeyboardPage(Keyboard::Y);
pub const KC_Z: Keycode = Keycode::KeyboardPage(Keyboard::Z);
pub const KC_1: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard1);
pub const KC_2: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard2);
pub const KC_3: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard3);
pub const KC_4: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard4);
pub const KC_5: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard5);
pub const KC_6: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard6);
pub const KC_7: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard7);
pub const KC_8: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard8);
pub const KC_9: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard9);
pub const KC_0: Keycode = Keycode::KeyboardPage(Keyboard::Keyboard0);
pub const KC_ENTER: Keycode = Keycode::KeyboardPage(Keyboard::ReturnEnter);
pub const KC_ESCAPE: Keycode = Keycode::KeyboardPage(Keyboard::Escape);
pub const KC_BSPACE: Keycode = Keycode::KeyboardPage(Keyboard::DeleteBackspace);
pub const KC_TAB: Keycode = Keycode::KeyboardPage(Keyboard::Tab);
pub const KC_SPACE: Keycode = Keycode::KeyboardPage(Keyboard::Space);
pub const KC_MINUS: Keycode = Keycode::KeyboardPage(Keyboard::Minus);
pub const KC_EQUAL: Keycode = Keycode::KeyboardPage(Keyboard::Equal);
pub const KC_LBRACKET: Keycode = Keycode::KeyboardPage(Keyboard::LeftBrace);
pub const KC_RBRACKET: Keycode = Keycode::KeyboardPage(Keyboard::RightBrace);
pub const KC_BSLASH: Keycode = Keycode::KeyboardPage(Keyboard::Backslash);
pub const KC_NONUS_HASH: Keycode = Keycode::KeyboardPage(Keyboard::NonUSHash);
pub const KC_SCOLON: Keycode = Keycode::KeyboardPage(Keyboard::Semicolon);
pub const KC_QUOTE: Keycode = Keycode::KeyboardPage(Keyboard::Apostrophe);
pub const KC_GRAVE: Keycode = Keycode::KeyboardPage(Keyboard::Grave);
pub const KC_COMMA: Keycode = Keycode::KeyboardPage(Keyboard::Comma);
pub const KC_DOT: Keycode = Keycode::KeyboardPage(Keyboard::Dot);
pub const KC_SLASH: Keycode = Keycode::KeyboardPage(Keyboard::ForwardSlash);
pub const KC_CAPSLOCK: Keycode = Keycode::KeyboardPage(Keyboard::CapsLock);
pub const KC_F1: Keycode = Keycode::KeyboardPage(Keyboard::F1);
pub const KC_F2: Keycode = Keycode::KeyboardPage(Keyboard::F2);
pub const KC_F3: Keycode = Keycode::KeyboardPage(Keyboard::F3);
pub const KC_F4: Keycode = Keycode::KeyboardPage(Keyboard::F4);
pub const KC_F5: Keycode = Keycode::KeyboardPage(Keyboard::F5);
pub const KC_F6: Keycode = Keycode::KeyboardPage(Keyboard::F6);
pub const KC_F7: Keycode = Keycode::KeyboardPage(Keyboard::F7);
pub const KC_F8: Keycode = Keycode::KeyboardPage(Keyboard::F8);
pub const KC_F9: Keycode = Keycode::KeyboardPage(Keyboard::F9);
pub const KC_F10: Keycode = Keycode::KeyboardPage(Keyboard::F10);
pub const KC_F11: Keycode = Keycode::KeyboardPage(Keyboard::F11);
pub const KC_F12: Keycode = Keycode::KeyboardPage(Keyboard::F12);
pub const KC_PSCREEN: Keycode = Keycode::KeyboardPage(Keyboard::PrintScreen);
pub const KC_SCROLLLOCK: Keycode = Keycode::KeyboardPage(Keyboard::ScrollLock);
pub const KC_PAUSE: Keycode = Keycode::KeyboardPage(Keyboard::Pause);
pub const KC_INSERT: Keycode = Keycode::KeyboardPage(Keyboard::Insert);
pub const KC_HOME: Keycode = Keycode::KeyboardPage(Keyboard::Home);
pub const KC_PGUP: Keycode = Keycode::KeyboardPage(Keyboard::PageUp);
pub const KC_DELETE: Keycode = Keycode::KeyboardPage(Keyboard::DeleteForward);
pub const KC_END: Keycode = Keycode::KeyboardPage(Keyboard::End);
pub const KC_PGDOWN: Keycode = Keycode::KeyboardPage(Keyboard::PageDown);
pub const KC_RIGHT: Keycode = Keycode::KeyboardPage(Keyboard::RightArrow);
pub const KC_LEFT: Keycode = Keycode::KeyboardPage(Keyboard::LeftArrow);
pub const KC_DOWN: Keycode = Keycode::KeyboardPage(Keyboard::DownArrow);
pub const KC_UP: Keycode = Keycode::KeyboardPage(Keyboard::UpArrow);
pub const KC_NUMLOCK: Keycode = Keycode::KeyboardPage(Keyboard::KeypadNumLockAndClear);
pub const KC_KP_SLASH: Keycode = Keycode::KeyboardPage(Keyboard::KeypadDivide);
pub const KC_KP_ASTERISK: Keycode = Keycode::KeyboardPage(Keyboard::KeypadMultiply);
pub const KC_KP_MINUS: Keycode = Keycode::KeyboardPage(Keyboard::KeypadSubtract);
pub const KC_KP_PLUS: Keycode = Keycode::KeyboardPage(Keyboard::KeypadAdd);
pub const KC_KP_ENTER: Keycode = Keycode::KeyboardPage(Keyboard::KeypadEnter);
pub const KC_KP_1: Keycode = Keycode::KeyboardPage(Keyboard::Keypad1);
pub const KC_KP_2: Keycode = Keycode::KeyboardPage(Keyboard::Keypad2);
pub const KC_KP_3: Keycode = Keycode::KeyboardPage(Keyboard::Keypad3);
pub const KC_KP_4: Keycode = Keycode::KeyboardPage(Keyboard::Keypad4);
pub const KC_KP_5: Keycode = Keycode::KeyboardPage(Keyboard::Keypad5);
pub const KC_KP_6: Keycode = Keycode::KeyboardPage(Keyboard::Keypad6);
pub const KC_KP_7: Keycode = Keycode::KeyboardPage(Keyboard::Keypad7);
pub const KC_KP_8: Keycode = Keycode::KeyboardPage(Keyboard::Keypad8);
pub const KC_KP_9: Keycode = Keycode::KeyboardPage(Keyboard::Keypad9);
pub const KC_KP_0: Keycode = Keycode::KeyboardPage(Keyboard::Keypad0);
pub const KC_KP_DOT: Keycode = Keycode::KeyboardPage(Keyboard::KeypadDot);
pub const KC_NONUS_BSLASH: Keycode = Keycode::KeyboardPage(Keyboard::NonUSBackslash);
pub const KC_APPLICATION: Keycode = Keycode::KeyboardPage(Keyboard::Application);
pub const KC_POWER: Keycode = Keycode::KeyboardPage(Keyboard::Power);
pub const KC_KP_EQUAL: Keycode = Keycode::KeyboardPage(Keyboard::KeypadEqual);
pub const KC_F13: Keycode = Keycode::KeyboardPage(Keyboard::F13);
pub const KC_F14: Keycode = Keycode::KeyboardPage(Keyboard::F14);
pub const KC_F15: Keycode = Keycode::KeyboardPage(Keyboard::F15);
pub const KC_F16: Keycode = Keycode::KeyboardPage(Keyboard::F16);
pub const KC_F17: Keycode = Keycode::KeyboardPage(Keyboard::F17);
pub const KC_F18: Keycode = Keycode::KeyboardPage(Keyboard::F18);
pub const KC_F19: Keycode = Keycode::KeyboardPage(Keyboard::F19);
pub const KC_F20: Keycode = Keycode::KeyboardPage(Keyboard::F20);
pub const KC_F21: Keycode = Keycode::KeyboardPage(Keyboard::F21);
pub const KC_F22: Keycode = Keycode::KeyboardPage(Keyboard::F22);
pub const KC_F23: Keycode = Keycode::KeyboardPage(Keyboard::F23);
pub const KC_F24: Keycode = Keycode::KeyboardPage(Keyboard::F24);
pub const KC_EXECUTE: Keycode = Keycode::KeyboardPage(Keyboard::Execute);
pub const KC_HELP: Keycode = Keycode::KeyboardPage(Keyboard::Help);
pub const KC_MENU: Keycode = Keycode::KeyboardPage(Keyboard::Menu);
pub const KC_SELECT: Keycode = Keycode::KeyboardPage(Keyboard::Select);
pub const KC_STOP: Keycode = Keycode::KeyboardPage(Keyboard::Stop);
pub const KC_AGAIN: Keycode = Keycode::KeyboardPage(Keyboard::Again);
pub const KC_UNDO: Keycode = Keycode::KeyboardPage(Keyboard::Undo);
pub const KC_CUT: Keycode = Keycode::KeyboardPage(Keyboard::Cut);
pub const KC_COPY: Keycode = Keycode::KeyboardPage(Keyboard::Copy);
pub const KC_PASTE: Keycode = Keycode::KeyboardPage(Keyboard::Paste);
pub const KC_FIND: Keycode = Keycode::KeyboardPage(Keyboard::Find);
pub const KC_KB_MUTE: Keycode = Keycode::KeyboardPage(Keyboard::Mute);
pub const KC_KB_VOLUP: Keycode = Keycode::KeyboardPage(Keyboard::VolumeUp);
pub const KC_KB_VOLDOWN: Keycode = Keycode::KeyboardPage(Keyboard::VolumeDown);
pub const KC_LOCKING_CAPS: Keycode = Keycode::KeyboardPage(Keyboard::LockingCapsLock);
pub const KC_LOCKING_NUM: Keycode = Keycode::KeyboardPage(Keyboard::LockingNumLock);
pub const KC_LOCKING_SCROLL: Keycode = Keycode::KeyboardPage(Keyboard::LockingScrollLock);
pub const KC_KP_COMMA: Keycode = Keycode::KeyboardPage(Keyboard::KeypadComma);
pub const KC_KP_EQUAL_AS400: Keycode = Keycode::KeyboardPage(Keyboard::KeypadEqualSign);
pub const KC_INT1: Keycode = Keycode::KeyboardPage(Keyboard::Kanji1);
pub const KC_INT2: Keycode = Keycode::KeyboardPage(Keyboard::Kanji2);
pub const KC_INT3: Keycode = Keycode::KeyboardPage(Keyboard::Kanji3);
pub const KC_INT4: Keycode = Keycode::KeyboardPage(Keyboard::Kanji4);
pub const KC_INT5: Keycode = Keycode::KeyboardPage(Keyboard::Kanji5);
pub const KC_INT6: Keycode = Keycode::KeyboardPage(Keyboard::Kanji6);
pub const KC_INT7: Keycode = Keycode::KeyboardPage(Keyboard::Kanji7);
pub const KC_INT8: Keycode = Keycode::KeyboardPage(Keyboard::Kanji8);
pub const KC_INT9: Keycode = Keycode::KeyboardPage(Keyboard::Kanji9);
pub const KC_LANG1: Keycode = Keycode::KeyboardPage(Keyboard::LANG1);
pub const KC_LANG2: Keycode = Keycode::KeyboardPage(Keyboard::LANG2);
pub const KC_LANG3: Keycode = Keycode::KeyboardPage(Keyboard::LANG3);
pub const KC_LANG4: Keycode = Keycode::KeyboardPage(Keyboard::LANG4);
pub const KC_LANG5: Keycode = Keycode::KeyboardPage(Keyboard::LANG5);
pub const KC_LANG6: Keycode = Keycode::KeyboardPage(Keyboard::LANG6);
pub const KC_LANG7: Keycode = Keycode::KeyboardPage(Keyboard::LANG7);
pub const KC_LANG8: Keycode = Keycode::KeyboardPage(Keyboard::LANG8);
pub const KC_LANG9: Keycode = Keycode::KeyboardPage(Keyboard::LANG9);
pub const KC_ALT_ERASE: Keycode = Keycode::KeyboardPage(Keyboard::AlternateErase);
pub const KC_SYSREQ: Keycode = Keycode::KeyboardPage(Keyboard::SysReqAttention);
pub const KC_CANCEL: Keycode = Keycode::KeyboardPage(Keyboard::Cancel);
pub const KC_CLEAR: Keycode = Keycode::KeyboardPage(Keyboard::Clear);
pub const KC_PRIOR: Keycode = Keycode::KeyboardPage(Keyboard::Prior);
pub const KC_RETURN: Keycode = Keycode::KeyboardPage(Keyboard::Return);
pub const KC_SEPARATOR: Keycode = Keycode::KeyboardPage(Keyboard::Separator);
pub const KC_OUT: Keycode = Keycode::KeyboardPage(Keyboard::Out);
pub const KC_OPER: Keycode = Keycode::KeyboardPage(Keyboard::Oper);
pub const KC_CLEAR_AGAIN: Keycode = Keycode::KeyboardPage(Keyboard::ClearAgain);
pub const KC_CRSEL: Keycode = Keycode::KeyboardPage(Keyboard::CrSelProps);
pub const KC_EXSEL: Keycode = Keycode::KeyboardPage(Keyboard::ExSel);
// pub const KC_KP_00: Keycode = Keycode::KeyboardPage(Keyboard::Keypad00);
// pub const KC_KP_000: Keycode = Keycode::KeyboardPage(Keyboard::Keypad000);
// pub const KC_THOUSANDS_SEPARATOR: Keycode = Keycode::KeyboardPage(Keyboard::ThousandsSep);
// pub const KC_DECIMAL_SEPARATOR: Keycode = Keycode::KeyboardPage(Keyboard::DecimalSep);
// pub const KC_CURRENCY_UNIT: Keycode = Keycode::KeyboardPage(Keyboard::CurrencyUnit);
// pub const KC_CURRENCY_SUB_UNIT: Keycode = Keycode::KeyboardPage(Keyboard::CurrencySubUnit);
// pub const KC_KP_LPAREN: Keycode = Keycode::KeyboardPage(Keyboard::KeypadLeftParen);
// pub const KC_KP_RPAREN: Keycode = Keycode::KeyboardPage(Keyboard::KeypadRightParen);
// pub const KC_KP_LCBRACKET: Keycode = Keycode::KeyboardPage(Keyboard::KeypadLeftCBracket);
// pub const KC_KP_RCBRACKET: Keycode = Keycode::KeyboardPage(Keyboard::KeypadRightCBracket);
// pub const KC_KP_TAB: Keycode = Keycode::KeyboardPage(Keyboard::KeypadTab);
// pub const KC_KP_BSPACE: Keycode = Keycode::KeyboardPage(Keyboard::KeypadBackspace);
// pub const KC_KP_A: Keycode = Keycode::KeyboardPage(Keyboard::KeypadA);
// pub const KC_KP_B: Keycode = Keycode::KeyboardPage(Keyboard::KeypadB);
// pub const KC_KP_C: Keycode = Keycode::KeyboardPage(Keyboard::KeypadC);
// pub const KC_KP_D: Keycode = Keycode::KeyboardPage(Keyboard::KeypadD);
// pub const KC_KP_E: Keycode = Keycode::KeyboardPage(Keyboard::KeypadE);
// pub const KC_KP_F: Keycode = Keycode::KeyboardPage(Keyboard::KeypadF);
// pub const KC_KP_XOR: Keycode = Keycode::KeyboardPage(Keyboard::KeypadXor);
// pub const KC_KP_HAT: Keycode = Keycode::KeyboardPage(Keyboard::KeypadHat);
// pub const KC_KP_PERC: Keycode = Keycode::KeyboardPage(Keyboard::KeypadPercent);
// pub const KC_KP_LT: Keycode = Keycode::KeyboardPage(Keyboard::KeypadLess);
// pub const KC_KP_GT: Keycode = Keycode::KeyboardPage(Keyboard::KeypadGreater);
// pub const KC_KP_AND: Keycode = Keycode::KeyboardPage(Keyboard::KeypadAnd);
// pub const KC_KP_LAZYAND: Keycode = Keycode::KeyboardPage(Keyboard::KeypadLazyAnd);
// pub const KC_KP_OR: Keycode = Keycode::KeyboardPage(Keyboard::KeypadOr);
// pub const KC_KP_LAZYOR: Keycode = Keycode::KeyboardPage(Keyboard::KeypadLazyOr);
// pub const KC_KP_COLON: Keycode = Keycode::KeyboardPage(Keyboard::KeypadColon);
// pub const KC_KP_HASH: Keycode = Keycode::KeyboardPage(Keyboard::KeypadHash);
// pub const KC_KP_SPACE: Keycode = Keycode::KeyboardPage(Keyboard::KeypadSpace);
// pub const KC_ATMARK: Keycode = Keycode::KeyboardPage(Keyboard::KeypadAt);
// pub const KC_KP_EXCLAMATION: Keycode = Keycode::KeyboardPage(Keyboard::KeypadExclamation);
// pub const KC_KP_MEM_STORE: Keycode = Keycode::KeyboardPage(Keyboard::KeypadMemStore);
// pub const KC_KP_MEM_RECALL: Keycode = Keycode::KeyboardPage(Keyboard::KeypadMemRecall);
// pub const KC_KP_MEM_CLEAR: Keycode = Keycode::KeyboardPage(Keyboard::KeypadMemClear);
// pub const KC_KP_MEM_ADD: Keycode = Keycode::KeyboardPage(Keyboard::KeypadMemAdd);
// pub const KC_KP_MEM_SUB: Keycode = Keycode::KeyboardPage(Keyboard::KeypadMemSub);
// pub const KC_KP_MEM_MUL: Keycode = Keycode::KeyboardPage(Keyboard::KeypadMemMul);
// pub const KC_KP_MEM_DIV: Keycode = Keycode::KeyboardPage(Keyboard::KeypadMemDiv);
// pub const KC_KP_PLUS_MINUS: Keycode = Keycode::KeyboardPage(Keyboard::KeypadPlusMinus);
// pub const KC_KP_CLEAR: Keycode = Keycode::KeyboardPage(Keyboard::KeypadClear);
// pub const KC_KP_CLEAR_ENTRY: Keycode = Keycode::KeyboardPage(Keyboard::KeypadClearEntry);
// pub const KC_KP_BINARY: Keycode = Keycode::KeyboardPage(Keyboard::KeypadBinary);
// pub const KC_KP_OCTAL: Keycode = Keycode::KeyboardPage(Keyboard::KeypadOctal);
// pub const KC_KP_DECIMAL: Keycode = Keycode::KeyboardPage(Keyboard::KeypadDecimal);
// pub const KC_KP_HEXADECIMAL: Keycode = Keycode::KeyboardPage(Keyboard::KeypadHexadecimal);
pub const KC_LCTRL: Keycode = Keycode::KeyboardPage(Keyboard::LeftControl);
pub const KC_LSHIFT: Keycode = Keycode::KeyboardPage(Keyboard::LeftShift);
pub const KC_LALT: Keycode = Keycode::KeyboardPage(Keyboard::LeftAlt);
pub const KC_LGUI: Keycode = Keycode::KeyboardPage(Keyboard::LeftGUI);
pub const KC_RCTRL: Keycode = Keycode::KeyboardPage(Keyboard::RightControl);
pub const KC_RSHIFT: Keycode = Keycode::KeyboardPage(Keyboard::RightShift);
pub const KC_RALT: Keycode = Keycode::KeyboardPage(Keyboard::RightAlt);
pub const KC_RGUI: Keycode = Keycode::KeyboardPage(Keyboard::RightGUI);

pub const KC_AUDIO_MUTE: Keycode = Keycode::ConsumerPage(Consumer::Mute);
pub const KC_AUDIO_VOL_UP: Keycode = Keycode::ConsumerPage(Consumer::VolumeIncrement);
pub const KC_AUDIO_VOL_DOWN: Keycode = Keycode::ConsumerPage(Consumer::VolumeDecrement);
pub const KC_MEDIA_NEXT_TRACK: Keycode = Keycode::ConsumerPage(Consumer::ScanNextTrack);
pub const KC_MEDIA_PREV_TRACK: Keycode = Keycode::ConsumerPage(Consumer::ScanPreviousTrack);
pub const KC_MEDIA_FAST_FORWARD: Keycode = Keycode::ConsumerPage(Consumer::FastForward);
pub const KC_MEDIA_REWIND: Keycode = Keycode::ConsumerPage(Consumer::Rewind);
pub const KC_MEDIA_STOP: Keycode = Keycode::ConsumerPage(Consumer::Stop);
pub const KC_MEDIA_PLAY_PAUSE: Keycode = Keycode::ConsumerPage(Consumer::PlayPause);
pub const KC_MEDIA_EJECT: Keycode = Keycode::ConsumerPage(Consumer::Eject);
// pub const KC_MEDIA_SELECT: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_MAIL: Keycode = Keycode::ConsumerPage(Consumer::Mail);
// pub const KC_CALCULATOR: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_MY_COMPUTER: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_WWW_SEARCH: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_WWW_HOME: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_WWW_BACK: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_WWW_FORWARD: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_WWW_STOP: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_WWW_REFRESH: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_WWW_FAVORITES: Keycode = Keycode::ConsumerPage(Consumer::);
// pub const KC_BRIGHTNESS_INC: Keycode = Keycode::ConsumerPage(Consumer::Brig);
// pub const KC_BRIGHTNESS_DEC: Keycode = Keycode::ConsumerPage(Consumer::);

// Backlight keycodes https://docs.qmk.fm/#/feature_backlight
pub const BL_DEC: Keycode = Keycode::System(SystemKeycode::BacklightDown);
pub const BL_INC: Keycode = Keycode::System(SystemKeycode::BacklightUp);
pub const BL_STEP: Keycode = Keycode::System(SystemKeycode::BacklightStep);

pub const KC_BOOTLOADER: Keycode = Keycode::System(SystemKeycode::Reset);
