use unity::prelude::*;
use crate::gamedata::{terrain::TerrainData, WeaponMask, skill::SkillArray};
use crate::unit::{CapabilityInt, Unit, UnitItem};

#[unity::class("App", "BattleDetail")]
pub struct BattleDetail {
    pub capability: &'static mut CapabilityInt,
}

#[unity::class("App", "BattleInfoSide")]
pub struct BattleInfoSide {
    info: &'static mut BattleInfo,
    pub side_type : i32,
    __ : i32,
    pub unit: Option<&'static Unit>,
    pub unit_item: Option<&'static UnitItem>,
    pub specified_item: &'static UnitItem,
    pub x: i32,
    pub z: i32,
    pub terrain: &'static TerrainData,
    pub overlap: &'static TerrainData,
    pub status: &'static mut WeaponMask,
    pub detail: &'static BattleDetail,
    hierarchy: u64,
    support: u64,
    pub parent: &'static BattleInfoSide,
    pub reverse: &'static BattleInfoSide,
    destroy: *const u8,
    pub mask_skill: &'static SkillArray,
    pub level: i32,
    pub hp: i32,
    pub gain_exp: i32,
    pub gain_gold: i32,
    pub drop_item_ratio: f32,
    pub pick_up_item: i32,
    pub damage: i32,
    pub heal: i32,
    pub battle_times: i32,
    pub total_order: i32,
    pub total_action: i32,
    pub total_attack: i32,
    pub total_damage: i32,
    pub total_result: i32,
    pub temporary: i32,
    pub stun: i32,
    pub engage_count: i32,
    pub engage_first_count: i32,
    pub blown_distance: i32,
    pub weapon_expend: i32,
    pub expend_count: i32,
}

#[unity::class("App", "BattleInfo")]
pub struct BattleInfo {}

impl BattleInfo {
    #[unity::class_method(53)] pub fn get_side(&self, side: BattleSideType) -> Option<&'static mut BattleInfoSide>; // Offset: 0x1E7F210 Flags: 0
    #[unity::class_method(59)] pub fn get_unit(&self, side: BattleSideType) -> Option<&'static mut Unit>; // Offset: 0x1E7F750 Flags: 0
}
#[unity::class("App", "BattleCalculator")]
pub struct BattleCalculator {
    pub mode: i32,
    pub info: &'static BattleInfo,
}
impl BattleCalculator {
    pub fn get_dead_side(&self) -> i32 { unsafe { battlecalcultor_get_deadside(self, None) }}
    pub fn get_side(&self, side: i32) -> Option<&'static mut BattleInfoSide> {
        unsafe { battle_calculator_get_side(self, side, None) }
    }
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum BattleSideType {
    None = -1,
    Offense = 0, // Attr: 17
    Defense = 1, // Attr: 17
    ChainOffense1 = 2, // Attr: 17
    ChainOffense2 = 3, // Attr: 17
    ChainOffense3 = 4, // Attr: 17
    ChainOffense4 = 5, // Attr: 17
    ChainOffense5 = 6, // Attr: 17
    ChainOffense6 = 7, // Attr: 17
    ChainOffense7 = 8, // Attr: 17
    ChainOffense8 = 9, // Attr: 17
    ChainOffense9 = 10, // Attr: 17
    ChainOffense10 = 11, // Attr: 17
    ChainOffense11 = 12, // Attr: 17
    ChainOffense12 = 13, // Attr: 17
    ChainOffense13 = 14, // Attr: 17
    ChainOffense14 = 15, // Attr: 17
    ChainOffense15 = 16, // Attr: 17
    ChainOffense16 = 17, // Attr: 17
    ChainOffense17 = 18, // Attr: 17
    ChainOffense18 = 19, // Attr: 17
    ChainOffense19 = 20, // Attr: 17
    ChainOffense20 = 21, // Attr: 17
    ChainOffense21 = 22, // Attr: 17
    ChainOffense22 = 23, // Attr: 17
    ChainOffense23 = 24, // Attr: 17
    ChainOffense24 = 25, // Attr: 17
    ChainDefense1 = 26, // Attr: 17
    ChainDefense2 = 27, // Attr: 17
    ChainDefense3 = 28, // Attr: 17
    ChainDefense4 = 29, // Attr: 17
}

#[unity::from_offset("App", "BattleCalculator", "GetDeadSide")]
fn battlecalcultor_get_deadside(this: &BattleCalculator,method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01e7f750)]
pub fn battle_info_get_unit(this: &BattleInfo, index: i32,  method_info: OptionalMethod) -> Option<&'static Unit>;

#[skyline::from_offset(0x0246f1a0)]
pub fn battle_calculator_get_side(this: &BattleCalculator, side: i32, method_info: OptionalMethod) -> Option<&'static mut BattleInfoSide>;
