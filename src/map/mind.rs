//! Types and methods used in the zoomed-out map view.

use num_derive::FromPrimitive;
use unity::prelude::*;

use crate::{gamedata::skill::SkillData, util::get_instance};
use crate::gamedata::ItemData;
use crate::map::inspectors::PokeInspector;
use crate::unit::{Unit, UnitItem};

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum MapMindType {
    None = 0, // Attr: 17
    Fixed = 1, // Attr: 17
    Talk = 2, // Attr: 17
    Attack = 3, // Attr: 17
    EngageStart = 4, // Attr: 17
    EngageLink = 5, // Attr: 17
    EngageAttack = 6, // Attr: 17
    EngageRod = 7, // Attr: 17
    EngageRewarp = 8, // Attr: 17
    EngageCharge = 9, // Attr: 17
    Cannon = 10, // Attr: 17
    Destroy = 11, // Attr: 17
    Rod = 12, // Attr: 17
    ItemUse = 13, // Attr: 17
    Item = 14, // Attr: 17
    Trade = 15, // Attr: 17
    Visit = 16, // Attr: 17
    Breakdown = 17, // Attr: 17
    BreakdownEnemy = 18, // Attr: 17
    Escape = 19, // Attr: 17
    Breakthrough = 20, // Attr: 17
    Door = 21, // Attr: 17
    Torch = 22, // Attr: 17
    TreasureBox = 23, // Attr: 17
    Transporter = 24, // Attr: 17
    RodWarp = 25, // Attr: 17
    RodWarpDest = 26, // Attr: 17
    RodRewarp = 27, // Attr: 17
    RodRewarpDest = 28, // Attr: 17
    RodRescue = 29, // Attr: 17
    RodInterference = 30, // Attr: 17
    RodTorch = 31, // Attr: 17
    RodCreation = 32, // Attr: 17
    RodNodus = 33, // Attr: 17
    Dance = 34, // Attr: 17
    Guard = 35, // Attr: 17
    DragonVein = 36, // Attr: 17
    OverlapSkill = 37, // Attr: 17
    CommandSkill = 38, // Attr: 17
    VisionCreate = 39, // Attr: 17
    VisionDelete = 40, // Attr: 17
    GodChange = 41, // Attr: 17
    DestroyVillage = 42, // Attr: 17
    TurnEnd = 43, // Attr: 17
    Surrender = 44, // Attr: 17
    Informal = 45, // Attr: 17
    RodHeal = 46, // Attr: 17
    RodMagicShield = 47, // Attr: 17
    FullBullet = 48, // Attr: 17
    EngageWait = 49, // Attr: 17
    EngageSummon = 50, // Attr: 17
    ItemMenu = 51, // Attr: 17
    EnchantMenu = 52, // Attr: 17
    Enchant = 53, // Attr: 17
    Contract = 54, // Attr: 17
    SubMenu = 55, // Attr: 17
}

#[repr(C)]
#[unity::class("App", "MapMind")]
pub struct MapMind {
    pub sup: [u8;0x9],
    pub unit_index: u8,
    pub first_unit_index: u8,
    pub first_x: i8,
    pub first_z: i8,
    pub unit_show_x: i8,
    pub unit_show_z: i8,
    pub x: i8,
    pub z: i8,
    pub mind: i32,
    pub attack_x: i8,
    pub attack_z: i8,
    pub item_index: i8,
    pub target_unit_index: u8,
    pub target_x: i8,
    pub target_z: i8,
    pub focus_x: i8,
    pub focus_z: i8,
    pub target_argument: i16,
    pub trade_unit_index: u8,
    pub event_unit_index: u8,
}

impl MapMind {
    pub fn get_instance() -> Option<&'static mut Self> { Some(get_instance::<MapMind>()) }
    pub fn get_unit() -> Option<&'static mut Unit> { Self::get_instance().and_then(|x| x.get_unit_()) }
    pub fn get_target_unit() -> Option<&'static mut Unit> { Self::get_instance().and_then(|x| x.get_target_unit_()) }
    pub fn get_trade_unit() -> Option<&'static mut Unit> { Self::get_instance().and_then(|x| x.get_trade_unit_()) }
    pub fn get_command_skill() -> Option<&'static SkillData> { Self::get_instance().and_then(|x| x.get_command_skill_() )}

    #[unity::class_method(0)] pub fn ctor(&self); // Offset: 0x1DF42C0 Flags:
    #[unity::class_method(1)] pub fn reset(&self, unit: &Unit, for_remove: bool); // Offset: 0x1DF4460 Flags: 0
    #[unity::class_method(2)] pub fn reset_mind(&self); // Offset: 0x1DF4950 Flags: 0
    #[unity::class_method(3)] pub fn reset_target(&self); // Offset: 0x1DF4A60 Flags: 0
    #[unity::class_method(4)] pub fn reset_route(&self); // Offset: 0x1DF4A00 Flags: 0
    #[unity::class_method(5)] pub fn is_moved(&self) -> bool; // Offset: 0x1DF4AF0 Flags: 0
    #[unity::class_method(6)] pub fn is_done(&self) -> bool; // Offset: 0x1DF4B70 Flags: 0
    // #[unity::class_method(7)] pub fn is_done2(&self, mask: MapMindDone) -> bool; // Offset: 0x1DF4B90 Flags: 0
    // #[unity::class_method(8)] pub fn set_done(&self, mask: MapMindDone); // Offset: 0x1DF4C00 Flags: 0
    #[unity::class_method(9)] pub fn is_aiengage_rewarp(&self) -> bool; // Offset: 0x1DF4C80 Flags: 0
    #[unity::class_method(10)] pub fn get_unit_(&self) -> Option<&'static mut Unit>; // Offset: 0x1DEE2B0 Flags: 0
    #[unity::class_method(11)] pub fn get_target_unit_(&self) -> Option<&'static mut Unit>; // Offset: 0x1DF4CB0 Flags: 0
    #[unity::class_method(12)] pub fn get_trade_unit_(&self) -> Option<&'static mut Unit>; // Offset: 0x1DEE9E0 Flags: 0
    #[unity::class_method(13)] pub fn get_unit_index(&self) -> i32; // Offset: 0x1DF4CC0 Flags: 0
    #[unity::class_method(14)] pub fn get_first_unit_index(&self) -> i32; // Offset: 0x1DF4CD0 Flags: 0
    #[unity::class_method(15)] pub fn get_first_x(&self) -> i32; // Offset: 0x1DF4CE0 Flags: 0
    #[unity::class_method(16)] pub fn get_first_z(&self) -> i32; // Offset: 0x1DF4CF0 Flags: 0
    #[unity::class_method(17)] pub fn get_unit_x(&self) -> i32; // Offset: 0x1DF4D00 Flags: 0
    #[unity::class_method(18)] pub fn set_unit_x(&self, value: i32); // Offset: 0x1DF4D10 Flags: 0
    #[unity::class_method(19)] pub fn get_unit_z(&self) -> i32; // Offset: 0x1DF4D20 Flags: 0
    #[unity::class_method(20)] pub fn set_unit_z(&self, value: i32); // Offset: 0x1DF4D30 Flags: 0
    #[unity::class_method(21)] pub fn set_unit_pos(&self, x: i32, z: i32); // Offset: 0x1DF4D40 Flags: 0
    #[unity::class_method(22)] pub fn get_x(&self) -> i32; // Offset: 0x1DF4D50 Flags: 0
    #[unity::class_method(23)] pub fn set_x(&self, value: i32); // Offset: 0x1DF4D60 Flags: 0
    #[unity::class_method(24)] pub fn get_z(&self) -> i32; // Offset: 0x1DF4D70 Flags: 0
    #[unity::class_method(25)] pub fn set_z(&self, value: i32); // Offset: 0x1DF4D80 Flags: 0
    #[unity::class_method(26)] pub fn get_mind(&self) -> MapMindType; // Offset: 0x1DF4D90 Flags: 0
    #[unity::class_method(27)] pub fn set_mind(&self, value: MapMindType); // Offset: 0x1DF4DA0 Flags: 0
    #[unity::class_method(28)] pub fn get_attack_x(&self) -> i32; // Offset: 0x1DF4DB0 Flags: 0
    #[unity::class_method(29)] pub fn set_attack_x(&self, value: i32); // Offset: 0x1DF4DD0 Flags: 0
    #[unity::class_method(30)] pub fn get_attack_z(&self) -> i32; // Offset: 0x1DF4DE0 Flags: 0
    #[unity::class_method(31)] pub fn set_attack_z(&self, value: i32); // Offset: 0x1DF4E00 Flags: 0
    #[unity::class_method(32)] pub fn get_item_index(&self) -> i32; // Offset: 0x1DF4E10 Flags: 0
    #[unity::class_method(33)] pub fn set_item_index(&self, value: i32); // Offset: 0x1DF4E20 Flags: 0
    #[unity::class_method(34)] pub fn get_target_unit_index(&self) -> i32; // Offset: 0x1DF4E30 Flags: 0
    #[unity::class_method(35)] pub fn set_target_unit_index(&self, value: i32); // Offset: 0x1DF4E40 Flags: 0
    #[unity::class_method(36)] pub fn get_target_x(&self) -> i32; // Offset: 0x1DF4E50 Flags: 0
    #[unity::class_method(37)] pub fn set_target_x(&self, value: i32); // Offset: 0x1DF4E60 Flags: 0
    #[unity::class_method(38)] pub fn get_target_z(&self) -> i32; // Offset: 0x1DF4E70 Flags: 0
    #[unity::class_method(39)] pub fn set_target_z(&self, value: i32); // Offset: 0x1DF4E80 Flags: 0
    #[unity::class_method(40)] pub fn get_focus_x(&self) -> i32; // Offset: 0x1DF4E90 Flags: 0
    #[unity::class_method(41)] pub fn set_focus_x(&self, value: i32); // Offset: 0x1DF4EA0 Flags: 0
    #[unity::class_method(42)] pub fn get_focus_z(&self) -> i32; // Offset: 0x1DF4EB0 Flags: 0
    #[unity::class_method(43)] pub fn set_focus_z(&self, value: i32); // Offset: 0x1DF4EC0 Flags: 0
    #[unity::class_method(44)] pub fn get_target_argument(&self) -> i32; // Offset: 0x1DF4ED0 Flags: 0
    #[unity::class_method(45)] pub fn set_target_argument(&self, value: i32); // Offset: 0x1DF4EE0 Flags: 0
    #[unity::class_method(46)] pub fn get_trade_unit_index(&self) -> i32; // Offset: 0x1DF4EF0 Flags: 0
    #[unity::class_method(47)] pub fn set_trade_unit_index(&self, value: i32); // Offset: 0x1DF4F00 Flags: 0
    #[unity::class_method(48)] pub fn get_move_power(&self) -> i32; // Offset: 0x1DF4F10 Flags: 0
    #[unity::class_method(49)] pub fn set_move_power(&self, value: i32); // Offset: 0x1DF4F20 Flags: 0
    // #[unity::class_method(50)] pub fn get_routes(&self) -> &'static Array<Type>; // Offset: 0x1DF4F30 Flags: 0
    // #[unity::class_method(51)] pub fn set_routes(&self, value: &Array<Type>); // Offset: 0x1DF4F40 Flags: 0
    #[unity::class_method(52)] pub fn get_transporter_index(&self) -> i32; // Offset: 0x1DF4FB0 Flags: 0
    #[unity::class_method(53)] pub fn set_transporter_index(&self, value: i32); // Offset: 0x1DF4FC0 Flags: 0
    #[unity::class_method(54)] pub fn get_event_unit_index(&self) -> i32; // Offset: 0x1DF4FD0 Flags: 0
    #[unity::class_method(55)] pub fn set_event_unit_index(&self, value: i32); // Offset: 0x1DF4FE0 Flags: 0
    #[unity::class_method(56)] pub fn get_command_skill_(&self) -> Option<&'static SkillData>; // Offset: 0x1DF4FF0 Flags: 0
    #[unity::class_method(57)] pub fn set_command_skill(&self, value: &SkillData); // Offset: 0x1DF5000 Flags: 0
    #[unity::class_method(58)] pub fn get_specified_item(&self) -> &'static ItemData; // Offset: 0x1DF5010 Flags: 0
    #[unity::class_method(59)] pub fn set_specified_item(&self, value: &ItemData); // Offset: 0x1DF5020 Flags: 0
    #[unity::class_method(60)] pub fn get_aiengage_rewarp_x(&self) -> i32; // Offset: 0x1DF5030 Flags: 0
    #[unity::class_method(61)] pub fn set_aiengage_rewarp_x(&self, value: i32); // Offset: 0x1DF5040 Flags: 0
    #[unity::class_method(62)] pub fn get_aiengage_rewarp_z(&self) -> i32; // Offset: 0x1DF5050 Flags: 0
    #[unity::class_method(63)] pub fn set_aiengage_rewarp_z(&self, value: i32); // Offset: 0x1DF5060 Flags: 0
    /*
    #[unity::class_method(64)] pub fn get_summon_color(&self) -> PersonDataColors; // Offset: 0x1DF5070 Flags: 0
    #[unity::class_method(65)] pub fn set_summon_color(&self, value: PersonDataColors); // Offset: 0x1DF5080 Flags: 0
    #[unity::class_method(66)] pub fn get_multi_target(&self) -> &'static MapMindMultiTargets; // Offset: 0x1DF5090 Flags: 0
    #[unity::class_method(67)] pub fn get_targets(&self) -> &'static MapMindMultiTargets; // Offset: 0x1DF50A0 Flags: 0
    #[unity::class_method(68)] pub fn get_stack(&self) -> &'static MapMindCommandStack; // Offset: 0x1DF50B0 Flags: 0

     */
    #[unity::class_method(69)] pub fn get_target_unit_by_index(&self, index: i32) -> Option<&'static Unit>; // Offset: 0x1DF50C0 Flags: 0
    #[unity::class_method(70)] pub fn get_target_x2(&self, index: i32) -> i32; // Offset: 0x1DF5160 Flags: 0
    #[unity::class_method(71)] pub fn get_target_z2(&self, index: i32) -> i32; // Offset: 0x1DF51F0 Flags: 0
    #[unity::class_method(72)] pub fn get_target_last_x(&self) -> i32; // Offset: 0x1DF5280 Flags: 0
    #[unity::class_method(73)] pub fn get_target_last_z(&self) -> i32; // Offset: 0x1DF5300 Flags: 0
    #[unity::class_method(74)] pub fn get_destroy_target(&self) -> Option<&'static PokeInspector>; // Offset: 0x1DF5380 Flags: 0
    #[unity::class_method(75)] pub fn get_item(&self) -> &'static ItemData; // Offset: 0x1DF5420 Flags: 0
    #[unity::class_method(76)] pub fn get_unit_item(&self) -> &'static UnitItem; // Offset: 0x1DF56A0 Flags: 0
    #[unity::class_method(77)] pub fn get_unit_item_index(&self, index: i32) -> Option<&'static UnitItem>; // Offset: 0x1DF54A0 Flags: 0
    #[unity::class_method(78)] pub fn can_god_change(&self) -> bool; // Offset: 0x1DF56B0 Flags: 0
    #[unity::class_method(79)] pub fn is_focus(&self) -> bool; // Offset: 0x1DF5710 Flags: 0
    #[unity::class_method(80)] pub fn get_mind2(&self) -> MapMindType; // Offset: 0x1DF5740 Flags: 0
    #[unity::class_method(81)] pub fn get_mind3(mind: MapMindType, skill: &SkillData) -> MapMindType; // Offset: 0x1DEE1B0 Flags: 0
}
