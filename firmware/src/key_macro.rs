use crate::keycode::{qmk::*, Keycode};

pub struct KeyMacro {
    pub shifted: bool,
    pub key: Keycode,
}

impl KeyMacro {
    pub fn new(key: Keycode) -> Self {
        Self {
            shifted: false,
            key,
        }
    }

    pub fn shift(key: Keycode) -> Self {
        Self { shifted: true, key }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a' => Some(Self::new(KC_A)),
            'b' => Some(Self::new(KC_B)),
            'c' => Some(Self::new(KC_C)),
            'd' => Some(Self::new(KC_D)),
            'e' => Some(Self::new(KC_E)),
            'f' => Some(Self::new(KC_F)),
            'g' => Some(Self::new(KC_G)),
            'h' => Some(Self::new(KC_H)),
            'i' => Some(Self::new(KC_I)),
            'j' => Some(Self::new(KC_J)),
            'k' => Some(Self::new(KC_K)),
            'l' => Some(Self::new(KC_L)),
            'm' => Some(Self::new(KC_M)),
            'n' => Some(Self::new(KC_N)),
            'o' => Some(Self::new(KC_O)),
            'p' => Some(Self::new(KC_P)),
            'q' => Some(Self::new(KC_Q)),
            'r' => Some(Self::new(KC_R)),
            's' => Some(Self::new(KC_S)),
            't' => Some(Self::new(KC_T)),
            'u' => Some(Self::new(KC_U)),
            'v' => Some(Self::new(KC_V)),
            'w' => Some(Self::new(KC_W)),
            'x' => Some(Self::new(KC_X)),
            'y' => Some(Self::new(KC_Y)),
            'z' => Some(Self::new(KC_Z)),

            'A' => Some(Self::shift(KC_A)),
            'B' => Some(Self::shift(KC_B)),
            'C' => Some(Self::shift(KC_C)),
            'D' => Some(Self::shift(KC_D)),
            'E' => Some(Self::shift(KC_E)),
            'F' => Some(Self::shift(KC_F)),
            'G' => Some(Self::shift(KC_G)),
            'H' => Some(Self::shift(KC_H)),
            'I' => Some(Self::shift(KC_I)),
            'J' => Some(Self::shift(KC_J)),
            'K' => Some(Self::shift(KC_K)),
            'L' => Some(Self::shift(KC_L)),
            'M' => Some(Self::shift(KC_M)),
            'N' => Some(Self::shift(KC_N)),
            'O' => Some(Self::shift(KC_O)),
            'P' => Some(Self::shift(KC_P)),
            'Q' => Some(Self::shift(KC_Q)),
            'R' => Some(Self::shift(KC_R)),
            'S' => Some(Self::shift(KC_S)),
            'T' => Some(Self::shift(KC_T)),
            'U' => Some(Self::shift(KC_U)),
            'V' => Some(Self::shift(KC_V)),
            'W' => Some(Self::shift(KC_W)),
            'X' => Some(Self::shift(KC_X)),
            'Y' => Some(Self::shift(KC_Y)),
            'Z' => Some(Self::shift(KC_Z)),

            '1' => Some(Self::new(KC_1)),
            '2' => Some(Self::new(KC_2)),
            '3' => Some(Self::new(KC_3)),
            '4' => Some(Self::new(KC_4)),
            '5' => Some(Self::new(KC_5)),
            '6' => Some(Self::new(KC_6)),
            '7' => Some(Self::new(KC_7)),
            '8' => Some(Self::new(KC_8)),
            '9' => Some(Self::new(KC_9)),
            '0' => Some(Self::new(KC_0)),

            '!' => Some(Self::shift(KC_1)),
            '@' => Some(Self::shift(KC_2)),
            '#' => Some(Self::shift(KC_3)),
            '$' => Some(Self::shift(KC_4)),
            '%' => Some(Self::shift(KC_5)),
            '^' => Some(Self::shift(KC_6)),
            '&' => Some(Self::shift(KC_7)),
            '*' => Some(Self::shift(KC_8)),
            '(' => Some(Self::shift(KC_9)),
            ')' => Some(Self::shift(KC_0)),

            '\n' => Some(Self::new(KC_ENT)),
            ' ' => Some(Self::new(KC_SPC)),

            '-' => Some(Self::new(KC_MINS)),
            '=' => Some(Self::new(KC_EQL)),
            '[' => Some(Self::new(KC_LBRC)),
            ']' => Some(Self::new(KC_RBRC)),
            '`' => Some(Self::new(KC_GRV)),
            ',' => Some(Self::new(KC_COMM)),
            '.' => Some(Self::new(KC_DOT)),
            '/' => Some(Self::new(KC_SLSH)),
            '\'' => Some(Self::new(KC_QUOT)),
            '\\' => Some(Self::new(KC_BSLS)),
            ';' => Some(Self::new(KC_SCLN)),

            '_' => Some(Self::shift(KC_MINS)),
            '+' => Some(Self::shift(KC_EQL)),
            '{' => Some(Self::shift(KC_LBRC)),
            '}' => Some(Self::shift(KC_RBRC)),
            '~' => Some(Self::shift(KC_GRV)),
            '<' => Some(Self::shift(KC_COMM)),
            '>' => Some(Self::shift(KC_DOT)),
            '?' => Some(Self::shift(KC_SLSH)),
            '"' => Some(Self::shift(KC_QUOT)),
            '|' => Some(Self::shift(KC_BSLS)),
            ':' => Some(Self::shift(KC_SCLN)),

            _ => None,
        }
    }
}
