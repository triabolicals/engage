//! Types and methods to read the state of the inputs.


use modular_bitfield::{bitfield, specifiers::B4};
use unity::prelude::*;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PadMode {
    Switch = 0, // Attr: 17
    XBox = 1, // Attr: 17
    XBoxABSwap = 2, // Attr: 17
    PS3 = 3, // Attr: 17
    ELECOM = 4, // Attr: 17
    CyberGadget = 5, // Attr: 17
    Keyboard = 6, // Attr: 17
}

#[repr(C)]
#[unity::class("App", "Pad")]
pub struct Pad {
    base: [u8; 0x10],
    npad_id: i32,
    npad_style: i32,
    pub npad_state: NpadState,
    pub old_buttons: NpadButton,
}

impl Pad {
    #[unity::class_method(1)] pub fn get_mode(&self) -> PadMode; // Offset: 0x1F22980 Flags: 0
    #[unity::class_method(2)] pub fn get_input_mode(&self) -> PadMode; // Offset: 0x1F22990 Flags: 0
    #[unity::class_method(3)] pub fn get_stick_value(&self, key: &Il2CppString) -> i32; // Offset: 0x1F229A0 Flags: 0
    #[unity::class_method(4)] pub fn commit_stick_button(&self, state: NpadState, margin: f32); // Offset: 0x1F229F0 Flags: 0 0
    #[unity::class_method(9)] pub fn check_pad_state(&self) -> bool; // Offset: 0x1F22CB0 Flags: 0
    #[unity::class_method(10)] pub fn is_button(buttons: NpadButton) -> bool; // Offset: 0x1F22F60 Flags: 0
    #[unity::class_method(11)] pub fn is_trigger(buttons: NpadButton) -> bool; // Offset: 0x1F23010 Flags: 0
    #[unity::class_method(12)] pub fn is_repeat(buttons: NpadButton) -> bool; // Offset: 0x1F23100 Flags: 0
    #[unity::class_method(13)] pub fn is_release(buttons: NpadButton) -> bool; // Offset: 0x1F232B0 Flags: 0
    #[unity::class_method(14)] pub fn get_stick_lx() -> f32; // Offset: 0x1F233A0 Flags: 0
    #[unity::class_method(15)] pub fn get_stick_ly() -> f32; // Offset: 0x1F23470 Flags: 0
    #[unity::class_method(16)] pub fn get_stick_rx() -> f32; // Offset: 0x1F23540 Flags: 0
    #[unity::class_method(17)] pub fn get_stick_ry() -> f32; // Offset: 0x1F23610 Flags: 0
}

#[repr(C)]
pub struct NpadState {
    sampling_number: i64,
    pub buttons: NpadButton,
    base: [u8; 0x20],
}

#[repr(C)]
#[bitfield]
pub struct NpadButton {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub stick_l: bool,
    pub stick_r: bool,
    pub l: bool,
    pub r: bool,
    pub zl: bool,
    pub zr: bool,
    pub plus: bool,
    pub minus: bool,
    pub left: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub stick_l_left: bool,
    pub stick_l_up: bool,
    pub stick_l_right: bool,
    pub stick_l_down: bool,
    pub stick_r_left: bool,
    pub stick_r_up: bool,
    pub stick_r_right: bool,
    pub stick_r_down: bool,
    pub left_sl: bool,
    pub left_sr: bool,
    pub right_sl: bool,
    pub right_sr: bool,
    #[skip]
    __: B4,
}
impl NpadButton {
    pub fn left_key() -> Self { Self::new().with_left(true) }
    pub fn right_key() -> Self { Self::new().with_right(true) }
    pub fn up_key() -> Self { Self::new().with_up(true) }
    pub fn down_key() -> Self { Self::new().with_down(true) }
    pub fn plus_key() -> Self { Self::new().with_plus(true) }
    pub fn minus_key() -> Self { Self::new().with_minus(true) }
    pub fn a_key() -> Self { Self::new().with_a(true) }
    pub fn b_key() -> Self { Self::new().with_b(true) }
    pub fn x_key() -> Self { Self::new().with_x(true) }
    pub fn y_key() -> Self { Self::new().with_y(true) }
    pub fn l_key() -> Self { Self::new().with_l(true) }
    pub fn r_key() -> Self { Self::new().with_r(true) }
    pub fn zl_key() -> Self { Self::new().with_zl(true) }
    pub fn zr_key() -> Self { Self::new().with_zr(true) }
}

#[unity::from_offset("App", "Pad", "Vibration")]	
pub fn app_pad_vibration(method_info: OptionalMethod) -> *const u8;