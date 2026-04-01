use num_derive::FromPrimitive;
use unity::prelude::*;
use crate::gamedata::terrain::TerrainData;
use super::{GamedataArray, ItemData, JobData, PersonData, StructDataArrayFields};

#[unity::class("App", "AIData")]
pub struct AIData {
    pub parent: StructDataArrayFields,
    pub code: i8,
    pub mind: i8,
    pub active: i8,
    pub trans: i8,
    __: i32,
    pub str_value1: &'static Il2CppString,
    pub str_value2: &'static Il2CppString,
}
impl GamedataArray for AIData {}

#[unity::class("App", "AIValue")]
pub struct AIValue {
    pub v8_1: i8,
    pub v8_2: i8,
    pub v16: i16,
}
impl AIValue {
    pub fn set_str_value<'a>(&self, str: impl Into<&'a Il2CppString>) -> &'static Il2CppString { self.set_value2(str.into()) }
    #[unity::class_method(0)] pub fn get_x(&self) -> i32; // Offset: 0x27B2800 Flags: 0
    #[unity::class_method(1)] pub fn get_z(&self) -> i32; // Offset: 0x27B2810 Flags: 0
    #[unity::class_method(2)] pub fn get_value(&self) -> i32; // Offset: 0x27B2820 Flags: 0
    #[unity::class_method(3)] pub fn set_value(&self, v: i16); // Offset: 0x27B2830 Flags: 0
    #[unity::class_method(4)] pub fn set_position(&self, x: u8, z: u8); // Offset: 0x27B2840 Flags: 0
    #[unity::class_method(5)] pub fn get_person(&self) -> Option<&'static PersonData>; // Offset: 0x27B2850 Flags: 0
    #[unity::class_method(6)] pub fn get_job(&self) -> Option<&'static JobData>; // Offset: 0x27B2990 Flags: 0
    #[unity::class_method(7)] pub fn get_terrain(&self) -> Option<&'static TerrainData>; // Offset: 0x27B2A60 Flags: 0
    #[unity::class_method(8)] pub fn get_item(&self) -> Option<&'static ItemData>; // Offset: 0x27B2B30 Flags: 0 0
    #[unity::class_method(12)] pub fn get_flag_value(&self) -> i32; // Offset: 0x27B2D50 Flags: 0
    #[unity::class_method(13)] pub fn set_value2(&self, str: &Il2CppString) -> &'static Il2CppString; // Offset: 0x27B2E80 Flags: 0
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, FromPrimitive, PartialOrd, Ord)]
pub enum AIValueOrder {
    Cause = 0, // Attr: 17
    Mind = 1, // Attr: 17
    Attack = 2, // Attr: 17
    Move = 3, // Attr: 17
    Num = 4,
}