use crate::debounce::Debounce;
use embedded_hal::digital::v2::InputPin;
use rp2040_hal::gpio::DynPin;

pub struct Encoder {
    a: DynPin,
    b: DynPin,
    debounce: Debounce<Option<i8>, 1>,
    delta: i8,
}

impl Encoder {
    pub fn new(mut a: DynPin, mut b: DynPin) -> Self {
        a.into_pull_up_input();
        b.into_pull_up_input();
        Self {
            a,
            b,
            debounce: Debounce::new(None),
            delta: 0,
        }
    }

    pub fn poll(&mut self) -> i8 {
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
