use unity::prelude::*;
use crate::bit::BitField32Methods;
use crate::force::ForceType;
use crate::gamedata::*;
use crate::gamedata::{job::JobData, person::PersonData, terrain::TerrainData, item::ItemData};

// Contains DisposData
#[unity::class("App", "DisposData")]
pub struct DisposData {
    pub parent: StructDataArrayFields,
    pub group: &'static Il2CppString,
    pub pid: &'static Il2CppString,
    pub tid: &'static Il2CppString,
    pub flag: &'static mut DisposDataFlagField,
    pub jid: &'static Il2CppString,
    pub sid: Option<&'static Il2CppString>,
    bid: &'static Il2CppString,
    pub appear_x: i8,
    pub appear_y: i8,
    pub dispos_x: i8,
    pub dispos_y: i8,
    pub direction: i32,
    pub rotation: i8,
    pub level_n: u8,
    pub level_h: u8,
    pub level_l: u8,
    __: i32,
    pub items: &'static mut Array<&'static mut DisposDataItem>,
    pub item1: &'static DisposDataItem,
    pub item2: &'static DisposDataItem,
    pub item3: &'static DisposDataItem,
    pub item4: &'static DisposDataItem,
    pub item5: &'static DisposDataItem,
    pub item6: &'static DisposDataItem,
    pub gid: Option<&'static Il2CppString>,
    pub hp_stock_count: u32,
    state0: i32,
    state1: i32,
    state2: i32,
    state3: i32,
    state4: i32,
    state5: i32,
    ___: i32,
    pub ai_action_name: &'static Il2CppString,
    pub ai_action_value: Option<&'static Il2CppString>,
    pub ai_mind_name: &'static Il2CppString,
    pub ai_mind_value: Option<&'static Il2CppString>,
    pub ai_attack_name: &'static Il2CppString,
    pub ai_attack_value: Option<&'static Il2CppString>,
    pub ai_move_name: &'static Il2CppString,
    pub ai_move_value: Option<&'static Il2CppString>,
    ai_battle_rate: Option<&'static Il2CppString>, // Offset 0x110, Attr: 1
    ai_priority: u8, // Offset 0x118, Attr: 1
    ai_heal_rate_a: i8, // Offset 0x119, Attr: 1
    ai_heal_rate_b: i8, // Offset 0x11A, Attr: 1
    ai_band_no: u32, // Offset 0x11C, Attr: 1
    ai_move_limit: Option<&'static Il2CppString>, // Offset 0x120, Attr: 1
    ai_flag: u64,
    pub force: i8,
}
#[unity::class("", "FlagField")]
#[nested_from_type(DisposData)]
pub struct DisposDataFlagField { pub value: i32, }

#[unity::class("", "Item")]
#[nested_from_type(DisposData)]
pub struct DisposDataItem {
    pub iid: Option<&'static mut Il2CppString>,
    pub drop: i32,
}

impl DisposData {
    #[unity::class_method(1)] pub fn ctor(&self); // Offset: 0x1CFA220 Flags: 0
    #[unity::class_method(5)] pub fn set_pid(&self, value: Option<&Il2CppString>); // Offset: 0x1CFA570 Flags: 0
    #[unity::class_method(11)] pub fn set_jid(&self, value: &Il2CppString); // Offset: 0x1CFA5D0 Flags: 0
    #[unity::class_method(13)] pub fn set_sid(&self, value: Option<&Il2CppString>); // Offset: 0x1CFA5F0 Flags: 0
    #[unity::class_method(8)] pub fn get_flag(&self) -> &'static mut DisposDataFlagField; // Offset: 0x1CFA5A0 Flags: 0
    #[unity::class_method(9)] pub fn set_flag(&self, value: &DisposDataFlagField); // Offset: 0x1CFA5B0 Flags: 0
    #[unity::class_method(98)] pub fn get_force(&self) -> i8; // Offset: 0x1CFAB40 Flags: 0
    #[unity::class_method(99)] pub fn set_force(&self, value: i8); // Offset: 0x1CFAB50 Flags: 0
    #[unity::class_method(100)] pub fn get_force_type(&self) -> ForceType; // Offset: 0x1CFAB60 Flags:
    #[unity::class_method(101)] pub fn get_person(&self) -> Option<&'static PersonData>; // Offset: 0x1CFAB70 Flags: 0
    #[unity::class_method(102)] pub fn get_terrain(&self) -> Option<&'static TerrainData>; // Offset: 0x1CFAC30 Flags: 0
    #[unity::class_method(103)] pub fn get_job(&self) -> Option<&'static JobData>; // Offset: 0x1CFAC90 Flags: 0
    #[unity::class_method(104)] pub fn get_item(&self, index: i32) -> Option<&'static ItemData>; // Offset: 0x1CFAD10 Flags: 0
}
impl GamedataArray for DisposData {}

impl BitField32Methods for DisposDataFlagField {}
#[allow(non_upper_case_globals)]
impl DisposDataFlagField {
    pub const Normal: i32 = 1;
    pub const Hard: i32 = 2;
    pub const Lunatic: i32 = 4;
    pub const Create: i32 = 8;
    pub const Leader: i32 = 16;
    pub const NotMove: i32 = 32;
    pub const Edge: i32 = 64;
    pub const Pos: i32 = 128;
    pub const Must: i32 = 256;
    pub const Fix: i32 = 512;
    pub const Guest: i32 = 1024;
    pub const MaskSortie: i32 = 896;
    pub const MaskDifficulty: i32 = 7;
}

impl DisposDataItem {
    #[unity::class_method(0)] pub fn get_iid(&self) -> Option<&'static Il2CppString>; // Offset: 0x1BD3DC0 Flags: 0
    #[unity::class_method(1)] pub fn set_iid(&self, value: &Il2CppString); // Offset: 0x1BD3DD0 Flags: 0
}

