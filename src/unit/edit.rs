use unity::prelude::*;

#[unity::class("App","UnitEdit")]
pub struct UnitEdit {
    pub name : Option<&'static Il2CppString>,
    pub morph_name: Option<&'static Il2CppString>,
    pub gender: i32,
    pub language: i32,
    pub birth_month: u8,
    pub birth_day: u8,
}
impl UnitEdit {
    #[unity::class_method(0)] pub fn is_enabled(&self) -> bool; // Offset: 0x1F73A40 Flags: 0
    #[unity::class_method(3)] pub fn set_name(&self, name: &Il2CppString); // Offset: 0x1F73BE0 Flags: 0
    #[unity::class_method(6)] pub fn set_gender(&self, value: i32); // Offset: 0x1F73E50 Flags: 0
    #[unity::class_method(6)] pub fn set_gender2(&self, value: super::Gender); // Offset: 0x1F73E50 Flags: 0
}