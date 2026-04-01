use unity::prelude::*;
use crate::bit::BitField32;
use crate::util::get_instance;

// Structure that control map win / lose conditions

#[unity::class("App", "MapSituation")]
pub struct MapSituation {
    junk: [u8; 0x20],
    pub status: &'static mut BitField32,
    pub current_force: i32,
    pub human_force: i32,
    force_cursor: *const u8,
    pub turn: i32,
    pub sub_phase: i32,
    pub win_rule_enemy_less_than_equal: i32, 
    pub win_rule_turn: i32,
    pub win_lose_result: i32,
    pub entrust: i32,
    pub win_rule_mid: &'static Il2CppString,
    pub win_rule_mid_arg: Option<&'static Il2CppString>,
    pub lose_rule_mid: &'static Il2CppString,
    pub lose_rule_arg: Option<&'static Il2CppString>,
    pub average_level: i32,
}
impl MapSituation {
    pub fn get_instance() -> &'static mut MapSituation { get_instance::<Self>() }
    #[unity::class_method(2)] pub fn set_status(&self, status: i32); // Offset: 0x1F30850 Flags: 0
    #[unity::class_method(3)] pub fn clear_status(&self, status: i32); // Offset: 0x1F2F870 Flags: 0
    #[unity::class_method(4)] pub fn check_status(&self, status: i32) -> bool; // Offset: 0x1F308C0 Flags: 0
    #[unity::class_method(5)] pub fn not_status(&self, status: i32) -> bool; // Offset: 0x1F307E0 Flags: 0
    
    #[unity::class_method(76)] pub fn get_average_level(&self) -> i32; // Offset: 0x1F4D880 Flags: 0
    #[unity::class_method(77)] pub fn set_average_level(&self, value: i32); // Offset: 0x1F4D890 Flags: 0
}

pub struct MapSituationStatus{}

#[allow(non_upper_case_globals)]
impl MapSituationStatus {
    pub const WinRuleBreakdown: i32 = 1; // Attr: 17
    pub const WinRuleDestroyBoss: i32 = 2; // Attr: 17
    pub const SequenceReplayCancel: i32 = 134217728; // Attr: 17
    pub const SequenceOpening: i32 = 268435456; // Attr: 17
    pub const SequenceAiEntrustCancel: i32 = 536870912; // Attr: 17
    pub const SequenceAi: i32 = 1073741824; // Attr: 17
    pub const SequenceMind: i32 = 1 << 31; // Attr: 17
    pub const WinRuleMask: i32 = 3; // Attr: 17
}
/*
#[unity::from_offset("App", "MapSituation", "SetStatus")]
fn mapsituation_set_status(this: &MapSituation, status: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "MapSituation", "CheckStatus")]
fn mapsituation_check_status(this: &MapSituation, status: i32, method_info: OptionalMethod) -> bool;

 */