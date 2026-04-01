use unity::prelude::*;
use crate::gamedata::{Gamedata, GamedataArray, StructBaseFields, StructDataArrayFields};

#[unity::class("App", "HubDisposData")]
pub struct HubDisposData {
    pub parent: StructDataArrayFields,
    pub locator: Option<&'static Il2CppString>,
    pub parent_locator: Option<&'static Il2CppString>,
    pub is_must_child: bool,
    pub fade_distance: f32,
    pub priority: i32,
    pub chapter: Option<&'static Il2CppString>,
    pub phase: i32,
}
impl HubDisposData {
    #[unity::class_method(12)] pub fn set_chapter(&self, value: &Il2CppString); // Offset: 0x2D87FE0 Flags: 0
    #[unity::class_method(23)] pub fn get_aid(&self) -> Option<&'static Il2CppString>; // Offset: 0x2D88090 Flags: 0
    #[unity::class_method(24)] pub fn set_aid(&self, value: &Il2CppString); // Offset: 0x2D880A0 Flags: 0
    #[unity::class_method(63)] pub fn get_optimize_type(&self) -> i32; // Offset: 0x2D88310 Flags: 0
    #[unity::class_method(64)] pub fn set_optimize_type(&self, value: i32); // Offset: 0x2D88320 Flags: 0
    #[unity::class_method(65)] pub fn is_chair(&self) -> bool; // Offset: 0x2D88330 Flags: 0
    #[unity::class_method(66)] pub fn is_pick(&self) -> bool; // Offset: 0x2D883A0 Flags: 0
    #[unity::class_method(67)] pub fn is_squat(&self) -> bool; // Offset: 0x2D88430 Flags: 0
    #[unity::class_method(68)] pub fn is_sit_up(&self) -> bool; // Offset: 0x2D884E0 Flags: 0
    #[unity::class_method(69)] pub fn is_sit(&self) -> bool; // Offset: 0x2D88550 Flags: 0
    #[unity::class_method(70)] pub fn is_sing(&self) -> bool; // Offset: 0x2D885C0 Flags: 0
    #[unity::class_method(71)] pub fn is_fish(&self) -> bool; // Offset: 0x2D88630 Flags: 0
}
impl GamedataArray for HubDisposData {}

#[unity::class("App", "HubRandomSet")]
pub struct HubRandomSet {
    pub parent: StructDataArrayFields,
    pub iid: &'static Il2CppString,
    pub rate: i32,
    pub count: i32,
}
impl GamedataArray for HubRandomSet {}

#[unity::class("App", "HubFacilityData")]
pub struct HubFacilityData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub mid: &'static Il2CppString,
    pub condition_cid: &'static Il2CppString,
    pub icon_name: &'static Il2CppString,
}
impl Gamedata for HubFacilityData {}

impl HubFacilityData {
    #[unity::class_method(15)] pub fn is_complete(&self) -> bool; // Offset: 0x28A80D0 Flags: 0
    #[unity::class_method(12)] pub fn is_first_access_flag(&self) -> bool; // Offset: 0x28A7A00 Flags: 0
    #[unity::class_method(13)] pub fn set_first_access_flag(&self); // Offset: 0x28A7B30 Flags: 0
}