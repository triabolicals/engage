use unity::prelude::*;
use crate::unit::Unit;

#[unity::class("App", "UnitReliance")] pub struct UnitReliance {} 

impl UnitReliance {
    #[unity::class_method(5)] pub fn try_get(unit_a: &Unit, unit_b: &Unit) -> Option<&'static mut UnitRelianceData>; // Offset: 0x1C5A040 Flags: 0
    #[unity::class_method(14)] pub fn can_level_up2(unit_a: &Unit, unit_b: &Unit) -> bool; // Offset: 0x1C5A930 Flags: 0
    #[unity::class_method(18)] pub fn can_be_level_aplus2(unit_a: &Unit, unit_b: &Unit) -> bool; // Offset: 0x1C5AE10 Flags: 0
    #[unity::class_method(16)] pub fn level_up2(unit_a: &Unit, unit_b: &Unit); // Offset: 0x1C5ABF0 Flags: 0
    #[unity::class_method(20)] pub fn set_level_a_plus(unit_a: &Unit, unit_b: &Unit); // Offset: 0x1C5B020 Flags: 0
}

#[unity::class("App", "UnitRelianceData")]
pub struct UnitRelianceData {
    reliance_exp: *const u8,
    pub level: i32,
    pub exp: i8,
    pub score: i8,
}

impl UnitRelianceData {
    #[unity::class_method(15)] pub fn get_next_level_exp(&self, current_level: i32) -> i32; // Offset: 0x1C5C450 Flags: 0
}