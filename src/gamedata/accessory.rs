use num_derive::FromPrimitive;
use unity::prelude::*;
use crate::gamedata::god::GodData;
use crate::unit::Unit;
use super::{Gamedata, StructBaseFields};

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum AccessoryDataKinds {
    Body = 0,
    Head = 1,
    Face = 2,
    Back = 3,
    Sommie = 4,
    // Accessory Slot Expansion
    Battle = 5,
    Dye = 6,
    Style = 7,
}

#[unity::class("App", "AccessoryData")]
pub struct AccessoryData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub name_m: Option<&'static Il2CppString>,
    pub help_m: Option<&'static Il2CppString>,
    pub name_f: Option<&'static Il2CppString>,
    pub help_f: Option<&'static Il2CppString>,
    pub first: bool,
    pub amiibo: bool,
    pub condition_cid: &'static Il2CppString,
    pub condition_gender: i32,
    pub condition_skills: &'static [Il2CppString; 0],
    pub gid: &'static Il2CppString,
    pub asset: &'static Il2CppString,
    pub price: i32,
    pub iron: i32,
    pub steel: i32,
    pub silver: i32,
    pub mask: i32,
    pub kind: i32,
    pub god_data: Option<&'static GodData>,
    pub flag_name: &'static Il2CppString,
    // ...
}

impl Gamedata for AccessoryData { }

impl AccessoryData {
    pub fn get_shop_name(&self, gender: i32) -> &'static Il2CppString { AccessoryShopUtility::get_accessory_name(Some(&self), gender) }
    pub fn get_shop_help(&self, gender: i32) -> &'static Il2CppString { AccessoryShopUtility::get_accessory_help(Some(&self), gender) }
    #[unity::class_method(46)] pub fn regist_global_flags(); // Offset: 0x27B4F80 Flags: 0
    #[unity::class_method(47)] pub fn get_num(data: &AccessoryData) -> i32; // Offset: 0x27B5140 Flags: 0
    #[unity::class_method(48)] pub fn set_num(data: &AccessoryData, num: i32); // Offset: 0x27B5230 Flags: 0
    #[unity::class_method(49)] pub fn can_equip(&self, unit: &Unit) -> bool; // Offset: 0x27B5400 Flags: 0
    #[unity::class_method(50)] pub fn is_amiibo_open(&self) -> bool; // Offset: 0x27B5790 Flags: 0
    #[unity::class_method(51)] pub fn try_get_from_god_data(god_data: &GodData) -> Option<&'static AccessoryData>; // Offset: 0x27B5900 Flags: 0
}

#[unity::class("App", "AccessoryShopUtility")] pub struct AccessoryShopUtility {}
impl AccessoryShopUtility {
    #[unity::class_method(0)] pub fn get_private_dress_aid(unit: &Unit) -> &'static Il2CppString; // Offset: 0x27B6EA0 Flags: 0
    #[unity::class_method(1)] pub fn get_accessory_name(accessory_data: Option<&AccessoryData>, female: i32) -> &'static Il2CppString; // Offset: 0x27B5FE0 Flags: 0
    #[unity::class_method(2)] pub fn get_accessory_help(accessory_data: Option<&AccessoryData>, female: i32) -> &'static Il2CppString; // Offset: 0x27B60B0 Flags: 0
}