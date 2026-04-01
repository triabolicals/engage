use unity::{engine::Color, prelude::*};
use crate::force::ForceType;
use crate::gamedata::skill::SkillData;
use crate::unit::Unit;
use super::{StructBaseFields, Gamedata};

#[unity::class("App", "TerrainData")]
pub struct TerrainData {
    pub parent: StructBaseFields,
    pub tid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub cost_name: &'static Il2CppString,
    pub cost_type: i32,
    pub layer: i32,
    pub prohibition: i32,
    pub command: i32,
    pub sight: u8,
    pub destroyer: i32,
    pub hp_normal: i32,
    pub hp_hard: i32,
    pub hp_lunatic: i32,
    pub defense: i8,
    pub avoid: i8,
    pub player_defense: i8,
    pub enemy_defense: i8,
    pub player_avoid: i8,
    pub enemy_avoid: i8,
    pub heal: i8,
    pub life: u8,
    pub move_cost: u8,
    pub fly_cost: u8,
    pub move_first: i8,
    pub offset: f32,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub color: Color,
    pub change_tid: Option<&'static Il2CppString>,
    pub change_encount: Option<&'static Il2CppString>,
    pub height: f32,
    pub put_effect: Option<&'static Il2CppString>,
    pub minimap: Option<&'static Il2CppString>,
    pub cannon_skill: Option<&'static Il2CppString>,
    pub cannon_shell_normal: u8,
    pub cannon_shell_hard: u8,
    pub cannon_shell_lunatic: u8,
    pub flag: i32,
    pub put_allow: u8,
    pub ascii_name: Option<&'static Il2CppString>,
}
impl Gamedata for TerrainData {}

impl TerrainData {
    #[unity::class_method(81)] pub fn is_door(&self) -> bool; // Offset: 0x21E32A0 Flags: 0
    #[unity::class_method(82)] pub fn is_treasure(&self) -> bool; // Offset: 0x21E32B0 Flags: 0
    #[unity::class_method(83)] pub fn is_visit(&self) -> bool; // Offset: 0x21E32C0 Flags: 0
    #[unity::class_method(84)] pub fn is_bow_cannon(&self) -> bool; // Offset: 0x21E32D0 Flags: 0
    #[unity::class_method(85)] pub fn is_magic_cannon(&self) -> bool; // Offset: 0x21E32E0 Flags: 0
    #[unity::class_method(86)] pub fn is_fire_cannon(&self) -> bool; // Offset: 0x21E32F0 Flags: 0
    #[unity::class_method(87)] pub fn is_not_shadow(&self) -> bool; // Offset: 0x21E3300 Flags: 0
    #[unity::class_method(88)] pub fn is_foot_smoke(&self) -> bool; // Offset: 0x21E3310 Flags: 0
    #[unity::class_method(89)] pub fn is_footprint(&self) -> bool; // Offset: 0x21E3320 Flags: 0
    #[unity::class_method(90)] pub fn is_roof(&self) -> bool; // Offset: 0x21E3330 Flags: 0
    #[unity::class_method(91)] pub fn is_sight_masking(&self) -> bool; // Offset: 0x21E3340 Flags: 0
    #[unity::class_method(92)] pub fn is_torch(&self) -> bool; // Offset: 0x21E3350 Flags: 0
    #[unity::class_method(93)] pub fn is_cannon(&self) -> bool; // Offset: 0x21E3370 Flags: 0
    #[unity::class_method(94)] pub fn is_not_stun(&self) -> bool; // Offset: 0x21E3380 Flags: 0
    #[unity::class_method(95)] pub fn is_not_engage_add(&self) -> bool; // Offset: 0x21E3390 Flags: 0
    #[unity::class_method(96)] pub fn is_fly_enable(&self) -> bool; // Offset: 0x21E33A0 Flags: 0
    #[unity::class_method(97)] pub fn is_help_spot(&self) -> bool; // Offset: 0x21E33B0 Flags: 0
    #[unity::class_method(98)] pub fn is_engage_heal(&self) -> bool; // Offset: 0x21E33C0 Flags: 0
    #[unity::class_method(99)] pub fn is_not_target(&self) -> bool; // Offset: 0x21E33D0 Flags: 0
    #[unity::class_method(100)] pub fn is_not_warp(&self) -> bool; // Offset: 0x21E33F0 Flags: 0
    #[unity::class_method(101)] pub fn is_hide_break_icon(&self) -> bool; // Offset: 0x21E3410 Flags: 0
    #[unity::class_method(102)] pub fn is_show_phase_icon(&self) -> bool; // Offset: 0x21E3420 Flags: 0
    #[unity::class_method(103)] pub fn is_immobile(&self) -> bool; // Offset: 0x21E3430 Flags: 0
    #[unity::class_method(104)] pub fn is_damage_half_display(&self) -> bool; // Offset: 0x21E3440 Flags: 0
    #[unity::class_method(105)] pub fn get_name2(&self) -> &'static Il2CppString; // Offset: 0x21E3450 Flags: 0
    #[unity::class_method(106)] pub fn is_flight_only(&self) -> bool; // Offset: 0x21E34C0 Flags: 0
    #[unity::class_method(107)] pub fn is_breakable(&self) -> bool; // Offset: 0x21E3520 Flags: 0
    #[unity::class_method(108)] pub fn can_put_allow(&self, cost_type: i32) -> bool; // Offset: 0x21E3540 Flags: 0
    #[unity::class_method(109)] pub fn can_put_allow2(&self) -> bool; // Offset: 0x21E3550 Flags: 0
    #[unity::class_method(110)] pub fn can_breakable(&self, force: ForceType) -> bool; // Offset: 0x21E3560 Flags: 0
    #[unity::class_method(111)] pub fn get_hp(&self) -> i32; // Offset: 0x21E35B0 Flags: 0
    #[unity::class_method(112)] pub fn can_unit_command(&self, unit: &Unit) -> bool; // Offset: 0x21E3780 Flags: 0
    #[unity::class_method(113)] pub fn get_cannon_skill(&self) -> Option<&'static SkillData>; // Offset: 0x21E37E0 Flags: 0
    #[unity::class_method(114)] pub fn get_cannon_shells(&self) -> i32; // Offset: 0x21E3880 Flags: 0
}