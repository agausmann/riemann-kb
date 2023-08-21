use crate::keycode::{qmk::*, InputMode, Keycode, SystemKeycode};

pub const LAYER_LOWER: u8 = 1;
pub const LAYER_RAISE: u8 = 2;
pub const LAYER_FU: u8 = 3;
pub const LAYER_META: u8 = 4;

const MO_LOWR: Keycode = MO(LAYER_LOWER);
const MO_RAIS: Keycode = MO(LAYER_RAISE);
const MO_META: Keycode = MO(LAYER_META);
const TG_FU: Keycode = TG(LAYER_FU);
const SK_DFU: Keycode = Keycode::System(SystemKeycode::Reset);

const MD_NOR: Keycode = Keycode::InputMode(InputMode::Normal);
const MD_EMO: Keycode = Keycode::InputMode(InputMode::RegionalIndicator);
const MD_REV: Keycode = Keycode::InputMode(InputMode::Reverse);

#[rustfmt::skip]
pub const LAYERS: [[[Keycode; 6]; 10]; 5] = [
    // 0: Base
    [
        [KC_MUTE, KC_1   , KC_2   , KC_3   , KC_4   , KC_5   ],
        [KC_TAB , KC_Q   , KC_W   , KC_E   , KC_R   , KC_T   ],
        [KC_CLCK, KC_A   , KC_S   , KC_D   , KC_F   , KC_G   ],
        [KC_LSFT, KC_Z   , KC_X   , KC_C   , KC_V   , KC_B   ],
        [KC_LCTL, KC_LGUI, KC_LALT, MO_META, MO_LOWR, KC_ENT ],

        [KC_6   , KC_7   , KC_8   , KC_9   , KC_0   , KC_BSPC],
        [KC_Y   , KC_U   , KC_I   , KC_O   , KC_P   , KC_BSPC],
        [KC_H   , KC_J   , KC_K   , KC_L   , KC_SCLN, KC_QUOT],
        [KC_N   , KC_M   , KC_COMM, KC_DOT , KC_SLSH, KC_RSFT],
        [KC_SPC , MO_RAIS, XXXXXXX, KC_RALT, KC_RGUI, KC_RCTL],
    ],

    // 1: Lower
    [
        [_______, _______, _______, _______, _______, _______],
        [KC_ESC , KC_F1  , KC_F2  , KC_F3  , KC_F4  , _______],
        [_______, KC_F5  , KC_F6  , KC_F7  , KC_F8  , _______],
        [_______, KC_F9  , KC_F10 , KC_F11 , KC_F12 , _______],
        [_______, _______, _______, _______, _______, _______],

        [_______, _______, _______, _______, _______, _______],
        [KC_HOME, KC_PGDN, KC_PGUP, KC_END , KC_INS , _______],
        [KC_LEFT, KC_DOWN, KC_UP  , KC_RGHT, KC_DEL , _______],
        [_______, KC_PAUS, KC_PSCR, KC_SLCK, _______, _______],
        [_______, XXXXXXX, XXXXXXX, _______, _______, _______],
    ],

    // 2: Raise
    [
        [KC_MPLY, _______, _______, _______, _______, _______],
        [KC_GRV , KC_1   , KC_2   , KC_3   , KC_4   , KC_5   ],
        [_______, _______, KC_MPLY, KC_VOLD, KC_VOLU, KC_MUTE],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, XXXXXXX, XXXXXXX, _______],

        [_______, _______, _______, _______, _______, _______],
        [KC_6   , KC_7   , KC_8   , KC_9   , KC_0   , _______],
        [_______, KC_MINS, KC_EQL , KC_LBRC, KC_RBRC, KC_BSLS],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
    ],

    // 3: FU
    // Terraria does not let me unbind the Chat key (Enter)
    [
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, KC_SPC ],

        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
    ],

    // 4: Meta
    [
        [_______, _______, _______, _______, _______, _______],
        [SK_DFU , _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, _______, _______, _______, XXXXXXX, _______],

        [_______, _______, _______, _______, _______, TG_FU  ],
        [_______, _______, _______, _______, _______, _______],
        [_______, MD_NOR , MD_REV , MD_EMO , _______, _______],
        [_______, _______, _______, _______, _______, _______],
        [_______, XXXXXXX, _______, _______, _______, _______],
    ],

];
