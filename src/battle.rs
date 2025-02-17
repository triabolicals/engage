use unity::il2cpp::object::Array;
use unity::prelude::*;
use crate::gamedata::{item::UnitItem, terrain::TerrainData, unit::Unit, WeaponMask};

#[unity::class("App", "CapabilityInt")]
pub struct CapabilityInt {
    pub data: &'static mut Array<i32>,
}

#[unity::class("App", "BattleDetail")]
pub struct BattleDetail {
    pub capability: &'static mut CapabilityInt,
}

#[unity::class("App", "BattleInfoSide")]
pub struct BattleInfoSide {
    //junk : [u8; 0x48],
    info: u64,
    pub side_type : i32,
    __ : i32,
    pub unit: Option<&'static Unit>,
    pub unit_item: &'static UnitItem,
    pub specified_item: &'static UnitItem,
    pub x: i32,
    pub z: i32,
    pub terrain: &'static TerrainData,
    pub overlap: &'static TerrainData,
    pub status: &'static WeaponMask,
    pub detail: &'static BattleDetail,
    hierarchy: u64,
    support: u64,
    pub parent: &'static BattleInfoSide,
    pub reverse: &'static BattleInfoSide,
}

#[unity::class("App", "BattleInfo")]
pub struct BattleInfo {}

#[unity::class("App", "BattleCalculator")]
pub struct BattleCalculator {
    pub mode: i32,
    pub info: &'static BattleInfo,
}
impl BattleCalculator {
    pub fn get_dead_side(&self) -> i32 { unsafe { battlecalcultor_get_deadside(self, None) }}
}

#[unity::from_offset("App", "BattleCalculator", "GetDeadSide")]
fn battlecalcultor_get_deadside(this: &BattleCalculator,method_info: OptionalMethod) -> i32;


#[skyline::from_offset(0x01e7f750)]
pub fn battle_info_get_unit(this: &BattleInfo, index: i32,  method_info: OptionalMethod) -> &Unit; 
