use unity::prelude::*;

#[unity::class("App.Talk3D", "TalkPtr")]
pub struct TalkPtr {
    pub original_ptr: i64,
    pub now_ptr: i64,
}
impl TalkPtr {
    pub fn read_int16(&self) -> u16 { unsafe { talk_ptr_read_int16(self, None)} }
    pub fn read_int32(&self) -> i32 { unsafe { talk_ptr_read_int32(self, None) } }
    pub fn read_pid(&self) -> &'static mut Il2CppString { unsafe { talk_ptr_read_pid(self, None) } }
    pub fn read_string_param(&self) -> &'static mut Il2CppString { unsafe { talk_ptr_read_string_param(self, None) } }
    pub fn read_byte(&self) -> u8 { unsafe { talk_ptr_read_byte(self, None) } }
}

pub trait TalkTag: Il2CppClassData + Sized {
    #[unity::class_method(4, vtable)] fn initialize(&self, talk_ptr: &TalkPtr);
    #[unity::class_method(5, vtable)] fn execute(&self);
    #[unity::class_method(6, vtable)] fn execute_for_character_preload(&self);
    #[unity::class_method(7, vtable)] fn need_fade_wait(&self);
    #[unity::class_method(8, vtable)] fn get_result(&self) -> i32;
    #[unity::class_method(9, vtable)] fn get_result_for_head_text(&self) -> i32;
}

#[unity::class("App.Talk3D", "TalkTagParser")]
pub struct TalkTagParser {}

#[unity::class("App.Talk3D", "TalkTagArg")]
pub struct TalkTagArg {
    pub arg_index: i32
}

#[unity::class("App.Talk3D", "TalkTagWindow")]
pub struct TalkTagWindow {
    pub tag_id: i32,
    pub pid: Option<&'static Il2CppString>,
    pub pid_to_create: &'static mut Il2CppString,
    pub location_name: &'static mut Il2CppString,
    pub replacement_name: &'static mut Il2CppString,
}
impl TalkTagWindow {
    #[unity::class_method(0)] pub fn initialize_(&self, talk_ptr: &TalkPtr); // Offset: 0x21DB860 Flags: 0
}

#[unity::class("App.Talk3D", "TalkTagName")]
pub struct TalkTagName {
    pub tag_id: i32,
    pub pid: Option<&'static mut Il2CppString>,
    // pub pid_to_create: &'static mut Il2CppString,
    pub replacement_name: &'static mut Il2CppString,
}
impl TalkTagName {
    #[unity::class_method(0)] pub fn initialize_(&self, talk_ptr: &TalkPtr); // Offset: 0x21D9E70 Flags: 0
}

#[unity::class("App.Talk3D", "TalkTagAnimation")]
pub struct TalkTagAnimation {
    pub tag_id: i32,
    pub pid: Option<&'static Il2CppString>,
    pub pid_to_create: &'static mut Il2CppString,
    pub replacement_name: &'static mut Il2CppString,
}

impl TalkTagAnimation {
    #[unity::class_method(0)] pub fn initialize_(&self, talk_ptr: &TalkPtr); // Offset: 0x21D9030 Flags: 0
}


#[unity::class("App.Talk3D", "TalkTagPicture")]
pub struct TalkTagPicture {
    pub tag_id: i32,
    pub picture_index: i32,
    pub texture_name: &'static mut Il2CppString,
    pub anim_name: &'static mut Il2CppString,
}
impl TalkTagPicture {
    #[unity::class_method(0)] pub fn initialize_(&self, talk_ptr: &TalkPtr); // Offset:
}

#[unity::class("App.Talk3D", "TalkTagAddLetter")]
pub struct TalkTagAddLetter {
    pub is_line_feed_enabled: bool,
    pub add_letter: u16,
    pub result: i32,
}

impl TalkTagAddLetter {
    #[unity::class_method(2)] pub fn execute_(&self); // Offset: 0x21D86A0 Flags: 0
}

impl TalkTag for TalkTagArg {}
impl TalkTag for TalkTagWindow {}
impl TalkTag for TalkTagName {}
impl TalkTag for TalkTagAnimation {}
impl TalkTag for TalkTagAddLetter {}

#[unity::from_offset("App.Talk3D", "TalkPtr", "ReadInt16")]
fn talk_ptr_read_int16(this: &TalkPtr, method_info: OptionalMethod) -> u16;

#[unity::from_offset("App.Talk3D", "TalkPtr", "ReadByte")]
fn talk_ptr_read_byte(this: &TalkPtr, method_info: OptionalMethod) -> u8;

#[unity::from_offset("App.Talk3D", "TalkPtr", "ReadInt32")]
fn talk_ptr_read_int32(this: &TalkPtr, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App.Talk3D", "TalkPtr", "ReadStringParam")]
fn talk_ptr_read_string_param(this: &TalkPtr, method_info: OptionalMethod) -> &'static mut Il2CppString;

#[unity::from_offset("App.Talk3D", "TalkPtr", "ReadPID")]
fn talk_ptr_read_pid(this: &TalkPtr, method_info: OptionalMethod) -> &'static mut Il2CppString;