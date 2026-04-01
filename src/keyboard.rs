use unity::prelude::*;
use crate::proc::{Bindable, ProcInstFields};
use unity::system::action::Action1;

#[unity::class("App", "SoftwareKeyboard")]
pub struct SoftwareKeyboard {
    pub proc: ProcInstFields,
    arg: [u8; 0xb8],
    pub max_length: i32,
    pub header_text: Option<&'static Il2CppString>,
    pub sub_text: Option<&'static Il2CppString>,
    pub initial_text: Option<&'static Il2CppString>,
    pub preset: i32,
    call_back: *const (),
}

impl Bindable for SoftwareKeyboard {}

impl SoftwareKeyboard {
    #[unity::class_method(12)] pub fn get_result() -> Option<&'static Il2CppString>; // Offset: 0x1FDEA50 Flags: 0
    pub fn create_bind<B: Bindable>(
        proc: &B,
        max_length: i32,
        initial_text: Option<&'static Il2CppString>,
        header_text: Option<&'static Il2CppString>,
        sub_text: Option<&'static Il2CppString>,
        preset: i32,
        call_back: Option<&Action1<Il2CppString>>
    )
    {
        unsafe { keyboard_create_bind(proc, max_length, initial_text, header_text, sub_text, preset, call_back, None); }
    }
}

#[unity::from_offset("App", "SoftwareKeyboard", "GetResult")]
fn keyboard_get_result(method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "SoftwareKeyboard", "CreateBind")]
fn keyboard_create_bind<B: Bindable>(
    proc: &B,
    max_length: i32,
    initial_text: Option<&Il2CppString>,
    header_text: Option<&Il2CppString>,
    sub_text: Option<&Il2CppString>,
    preset: i32,
    call_back: Option<&Action1<Il2CppString>>,
    method_info: OptionalMethod
);