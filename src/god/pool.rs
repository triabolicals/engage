use unity::prelude::*;
use unity::system::List;
use crate::gamedata::{Gamedata, god::GodData};
use crate::god::god_unit::GodUnit;

#[unity::class("App", "GodPool")]
pub struct GodPool {
    parent: [u8; 0x10], //  SingletonClass<GodPool>
    list: &'static List<GodUnit>,
    pub sort: &'static mut List<GodUnit>,
}

impl GodPool {
    pub fn get_instance() -> &'static Self { crate::util::get_instance::<GodPool>() }
    pub fn create_by_gid<'a>(gid: impl Into<&'a Il2CppString>) -> Option<&'static GodUnit> { GodData::get(gid.into()).and_then(|god| Self::create(god)) }
    pub fn try_get_gid<'a>(gid: impl Into<&'a Il2CppString>, include_reserved: bool) -> Option<&'static mut GodUnit> { Self::try_get_gid_(gid.into(), include_reserved) }
    #[unity::class_method(2)] pub fn try_get_gid_(gid: &Il2CppString, include_reserved: bool) -> Option<&'static mut GodUnit>; // Offset: 0x2334570 Flags: 0
    #[unity::class_method(3)] pub fn try_get(data: &GodData, include_reserved: bool) -> Option<&'static mut GodUnit>; // Offset: 0x2334600 Flags: 0
    #[unity::class_method(4)] pub fn try_get_impl(data: &GodData, include_reserved: bool) -> &'static mut GodUnit; // Offset: 0x2334720 Flags: 0
    #[unity::class_method(5)] pub fn create(data: &GodData) -> Option<&'static GodUnit>; // Offset: 0x23349C0 Flags: 0
    #[unity::class_method(6)] pub fn delete(god: &GodUnit); // Offset: 0x2334C30 Flags: 0
    #[unity::class_method(7)] pub fn delete_or_reserve(god: &GodUnit); // Offset: 0x2334DB0 Flags: 0
    #[unity::class_method(8)] pub fn delete_reserved(); // Offset: 0x2334ED0 Flags: 0
    #[unity::class_method(9)] pub fn delete_except_for_player(); // Offset: 0x2335130 Flags: 0
    #[unity::class_method(10)] pub fn append_enemy_god(god: &GodUnit) -> &'static GodUnit; // Offset: 0x2335420 Flags: 0
    #[unity::class_method(11)] pub fn has_armlet() -> bool; // Offset: 0x23359C0 Flags: 0
    // #[unity::class_method(12)] pub fn on_serialize(&self, stream: &Stream); // Offset: 0x2335BD0 Flags: 0
    // #[unity::class_method(13)] pub fn on_deserialize(&self, stream: &Stream, version: i32); // Offset: 0x2335C30 Flags: 0
}