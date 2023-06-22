use usbd_human_interface_device::{
    device::{consumer::FixedFunctionReport, keyboard::NKROBootKeyboardReport},
    page::{Consumer, Keyboard},
};

pub struct KeyboardReport {
    report: NKROBootKeyboardReport,
}

impl KeyboardReport {
    pub fn new() -> Self {
        Self {
            report: Default::default(),
        }
    }
    pub fn inner(&self) -> &NKROBootKeyboardReport {
        &self.report
    }

    pub fn set(&mut self, key: Keyboard, pressed: bool) {
        match key {
            Keyboard::RightGUI => {
                self.report.right_gui = pressed;
            }
            Keyboard::RightAlt => {
                self.report.right_alt = pressed;
            }
            Keyboard::RightShift => {
                self.report.right_shift = pressed;
            }
            Keyboard::RightControl => {
                self.report.right_ctrl = pressed;
            }
            Keyboard::LeftGUI => {
                self.report.left_gui = pressed;
            }
            Keyboard::LeftAlt => {
                self.report.left_alt = pressed;
            }
            Keyboard::LeftShift => {
                self.report.left_shift = pressed;
            }
            Keyboard::LeftControl => {
                self.report.left_ctrl = pressed;
            }
            _ => {
                if pressed {
                    if self.report.boot_keys.contains(&key) {
                        return;
                    }
                    self.report.boot_keys.copy_within(0.., 1);
                    self.report.boot_keys[0] = key;
                } else {
                    let mut new_keys = [Keyboard::default(); 6];
                    for (i, k) in self
                        .report
                        .boot_keys
                        .into_iter()
                        .filter(|&k| k != Keyboard::NoEventIndicated && k != key)
                        .enumerate()
                    {
                        new_keys[i] = k;
                    }
                    self.report.boot_keys = new_keys;
                }

                let byte_index = key as u8 / 8;
                let bit_index = key as u8 % 8;
                if let Some(byte) = self.report.nkro_keys.get_mut(byte_index as usize) {
                    if pressed {
                        *byte |= 1 << bit_index;
                    } else {
                        *byte &= !(1 << bit_index);
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn clear_all_but_mods(&mut self) {
        self.report.boot_keys = Default::default();
        self.report.nkro_keys = Default::default();
    }
}

pub struct ConsumerReport {
    report: FixedFunctionReport,
}

impl ConsumerReport {
    pub fn new() -> Self {
        Self {
            report: FixedFunctionReport {
                next: false,
                previous: false,
                stop: false,
                play_pause: false,
                mute: false,
                volume_increment: false,
                volume_decrement: false,
            },
        }
    }

    pub fn inner(&self) -> &FixedFunctionReport {
        &self.report
    }

    pub fn set(&mut self, key: Consumer, pressed: bool) {
        match key {
            Consumer::ScanNextTrack => {
                self.report.next = pressed;
            }
            Consumer::ScanPreviousTrack => {
                self.report.previous = pressed;
            }
            Consumer::Stop => {
                self.report.stop = pressed;
            }
            Consumer::PlayPause => {
                self.report.play_pause = pressed;
            }
            Consumer::Mute => {
                self.report.mute = pressed;
            }
            Consumer::VolumeIncrement => {
                self.report.volume_increment = pressed;
            }
            Consumer::VolumeDecrement => {
                self.report.volume_decrement = pressed;
            }
            _ => {}
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}
