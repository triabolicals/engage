use unity::prelude::*;
use unity::system::Dictionary;
use crate::gamedata::god::GodData;
use crate::gamedata::skill::SkillData;
use crate::god::bond::{GodBond, GodBondHolder};
use crate::stream::Stream;
use crate::unit::Unit;

#[unity::class("App", "GodUnit")]
pub struct GodUnit {
    parent: [u8; 0x10],
    pub data: &'static GodData,
    pub parent_unit: Option<&'static Unit>,
    pub child: Option<&'static Unit>,
    pub bonds: u64,
    pub saved_parent: Option<&'static Unit>,
    pub is_temp_parent: bool,
    pub is_temp_changed: bool,
    pub reserved_deleting: bool,
    pub is_escaping: bool,
    pub dirty: u8,
    syncho_count: &'static Dictionary<'static, &'static Il2CppString, i32>,
    refine_level: u64,
    refine_result: u64,
    pub darkness: bool,
}
impl GodUnit {
    #[unity::class_method(131)] pub fn get_god_bonds(&self) -> Option<&'static mut GodBondHolder>; // Offset: 0x2342F90 Flags: 0
    #[unity::class_method(132)] pub fn get_bond(&self, unit: &Unit) -> Option<&'static mut GodBond>; // Offset: 0x23405B0 Flags: 0
    #[unity::class_method(133)] pub fn create_bond(&self, unit: &Unit); // Offset: 0x2340060 Flags: 0
    #[unity::class_method(0)] pub fn build(&self, data: &GodData) -> &'static GodUnit; // Offset: 0x2334B50 Flags: 0
    #[unity::class_method(14)] pub fn get_actual_gid(&self) -> &'static Il2CppString; // Offset: 0x233ED20 Flags: 0
    #[unity::class_method(50)] pub fn set_child(&self, unit: &Unit); // Offset: 0x2340140 Flags: 0
    #[unity::class_method(51)] pub fn clear_child(&self); // Offset: 0x2340150 Flags: 0
    #[unity::class_method(79)] pub fn set_level(&self, unit: &Unit, level: i32); // Offset: 0x2340A30 Flags: 0
    #[unity::class_method(112)] pub fn get_engage_atk(&self) -> &'static SkillData; // Offset: 0x2341630 Flags: 0
    #[unity::class_method(116)] pub fn get_engage_atk2(&self, unit: &Unit) -> &'static SkillData; // Offset: 0x2341640 Flags: 0
    #[unity::class_method(5)] pub fn get_escape(&self) -> bool; // Offset: 0x233EAE0 Flags: 0
    #[unity::class_method(6)] pub fn set_escape(&self, value: bool); // Offset: 0x233EAF0 Flags: 0
    #[unity::class_method(1)] pub fn delete(&self); // Offset: 0x2334D40 Flags: 0 
    #[unity::class_method(2)] pub fn reserve_deleting(&self); // Offset: 0x2334EC0 Flags: 0
    #[unity::class_method(3)] pub fn cancel_to_reserve_deleting(&self); // Offset: 0x2334C20 Flags: 0
    #[unity::class_method(4)] pub fn get_is_reserved_deleting(&self) -> bool; // Offset: 0x233EAD0 Flags: 0
    #[unity::class_method(7)] pub fn get_dirty(&self) -> i32; // Offset: 0x233EB00 Flags: 0
    #[unity::class_method(15)] pub fn is_hero(&self) -> bool; // Offset: 0x233ED40 Flags: 0
    #[unity::class_method(160)] pub fn on_serialize(&self, stream: &Stream); // Offset: 0x2345420 Flags: 0
    #[unity::class_method(161)] pub fn on_deserialize(&self, stream: &Stream, version: i32); // Offset: 0x23458F0 Flags: 0
    #[unity::class_method(83)] pub fn get_level(&self, unit: &Unit) -> i32; // Offset: 0x23302C0 Flags: 0
    #[unity::class_method(75)] pub fn get_diff_exp(&self, unit: &Unit) -> i32; // Offset: 0x2336970 Flags: 0
    #[unity::class_method(86)] pub fn get_max_level(&self, unit: &Unit) -> i32; // Offset: 0x2340B50 Flags: 0
    /*
     #[unity::class_method(8)] pub fn set_dirty(&self, value: i32); // Offset: 0x233EB10 Flags: 0
    #[unity::class_method(9)] pub fn get_data(&self) -> &'static GodData; // Offset: 0x233EB20 Flags: 0
    #[unity::class_method(10)] pub fn get_info_data(&self) -> &'static GodData; // Offset: 0x233EBB0 Flags: 0
    #[unity::class_method(11)] pub fn get_actual_data(&self) -> &'static GodData; // Offset: 0x233EC50 Flags: 0
    #[unity::class_method(12)] pub fn get_result_data(&self) -> &'static GodData; // Offset: 0x233EC60 Flags: 0
    #[unity::class_method(13)] pub fn get_gid(&self) -> &'static Il2CppString; // Offset: 0x233EC80 Flags: 0
    #[unity::class_method(16)] pub fn is_flag(&self, flag: GodDataFlags) -> bool; // Offset: 0x233EE80 Flags: 0
    #[unity::class_method(17)] pub fn get_name(&self) -> &'static Il2CppString; // Offset: 0x233EFC0 Flags: 0
    #[unity::class_method(18)] pub fn get_info_name(&self) -> &'static Il2CppString; // Offset: 0x233F190 Flags: 0
    #[unity::class_method(19)] pub fn get_main_name(&self) -> &'static Il2CppString; // Offset: 0x2330790 Flags: 0
    #[unity::class_method(20)] pub fn get_name2(data: &GodData) -> &'static Il2CppString; // Offset: 0x233F090 Flags: 0
    #[unity::class_method(21)] pub fn get_ascii_name(&self) -> &'static Il2CppString; // Offset: 0x23361D0 Flags: 0
    #[unity::class_method(22)] pub fn get_main_ascii_name(&self) -> &'static Il2CppString; // Offset: 0x233F270 Flags: 0
    #[unity::class_method(23)] pub fn get_face_icon_name(&self) -> &'static Il2CppString; // Offset: 0x233F360 Flags: 0
    #[unity::class_method(24)] pub fn get_info_face_icon_name(&self) -> &'static Il2CppString; // Offset: 0x233F3F0 Flags: 0
    #[unity::class_method(25)] pub fn get_face_icon_name_darkness(&self) -> &'static Il2CppString; // Offset: 0x233F490 Flags: 0
    #[unity::class_method(26)] pub fn get_face_icon_name_for_hero(&self) -> &'static Il2CppString; // Offset: 0x233F520 Flags: 0
    #[unity::class_method(27)] pub fn get_face_icon_name_for_heroine(&self) -> &'static Il2CppString; // Offset: 0x233F5B0 Flags: 0
    #[unity::class_method(28)] pub fn get_ring_name(&self) -> &'static Il2CppString; // Offset: 0x233F640 Flags: 0
    #[unity::class_method(29)] pub fn get_is_darkness(&self) -> bool; // Offset: 0x233F7C0 Flags: 0
    #[unity::class_method(30)] pub fn set_is_darkness(&self, value: bool); // Offset: 0x233F7D0 Flags: 0
    #[unity::class_method(31)] pub fn get_is_darkness_icon(&self) -> bool; // Offset: 0x233F7E0 Flags: 0
    #[unity::class_method(32)] pub fn get_is_darkness_gauge(&self) -> bool; // Offset: 0x233F8D0 Flags: 0
    #[unity::class_method(33)] pub fn get_is_only_engage_weapon(&self) -> bool; // Offset: 0x233F9C0 Flags: 0
    #[unity::class_method(34)] pub fn get_gender(&self) -> Gender; // Offset: 0x233FA90 Flags: 0
    #[unity::class_method(35)] pub fn is_female(&self) -> bool; // Offset: 0x233FC80 Flags: 0
    #[unity::class_method(36)] pub fn change_main(&self) -> bool; // Offset: 0x233FCA0 Flags: 0
    #[unity::class_method(37)] pub fn can_swap(&self) -> bool; // Offset: 0x233FD00 Flags: 0
    #[unity::class_method(38)] pub fn swap(&self) -> bool; // Offset: 0x233FD30 Flags: 0
    #[unity::class_method(39)] pub fn change(&self, index: i32) -> bool; // Offset: 0x233B740 Flags: 0
    #[unity::class_method(40)] pub fn change2(&self, data: &GodData) -> bool; // Offset: 0x233FDA0 Flags: 0
    #[unity::class_method(41)] pub fn set_god_data_for_rewind(&self, data: &GodData); // Offset: 0x233FDF0 Flags: 0
    #[unity::class_method(42)] pub fn get_is_engaging(&self) -> bool; // Offset: 0x233FE00 Flags: 0
    #[unity::class_method(43)] pub fn get_is_changed(&self) -> bool; // Offset: 0x233FE80 Flags: 0
    #[unity::class_method(44)] pub fn get_force_type(&self) -> ForceType; // Offset: 0x23353F0 Flags: 0
    #[unity::class_method(45)] pub fn get_engage_limit(&self) -> i32; // Offset: 0x233FF20 Flags: 0
    #[unity::class_method(46)] pub fn get_parent(&self) -> &'static Unit; // Offset: 0x233FFB0 Flags: 0
    #[unity::class_method(47)] pub fn get_child(&self) -> &'static Unit; // Offset: 0x233FFC0 Flags: 0
    #[unity::class_method(48)] pub fn set_parent(&self, unit: &Unit, god_state: GodState); // Offset: 0x233FFD0 Flags: 0
    #[unity::class_method(49)] pub fn clear_parent(&self); // Offset: 0x23400C0 Flags: 0
    #[unity::class_method(50)] pub fn set_child(&self, unit: &Unit); // Offset: 0x2340140 Flags: 0
    #[unity::class_method(51)] pub fn clear_child(&self); // Offset: 0x2340150 Flags: 0
    #[unity::class_method(52)] pub fn get_link(&self, unit: &Unit) -> &'static Unit; // Offset: 0x2340160 Flags: 0
    #[unity::class_method(53)] pub fn set_temporary_parent(&self, unit: &Unit); // Offset: 0x2340190 Flags: 0
    #[unity::class_method(54)] pub fn clear_temporary_parent(&self, unit: &Unit); // Offset: 0x2340240 Flags: 0
    #[unity::class_method(55)] pub fn get_synchro_counts(&self) -> Dictionary<&'static Il2CppString, i32>; // Offset: 0x2340360 Flags: 0
    #[unity::class_method(56)] pub fn add_synchro_count(&self, pid: &Il2CppString); // Offset: 0x2340370 Flags: 0
    #[unity::class_method(57)] pub fn set_synchro_count(&self, pid: &Il2CppString, count: i32); // Offset: 0x2340450 Flags: 0
    #[unity::class_method(58)] pub fn get_synchro_count(&self, pid: &Il2CppString) -> i32; // Offset: 0x2340520 Flags: 0
    #[unity::class_method(59)] pub fn dbg_set_exp(&self, unit: &Unit, exp: i32); // Offset: 0x23405A0 Flags: 0
    #[unity::class_method(60)] pub fn set_exp_for_rewind(&self, unit: &Unit, exp: i32); // Offset: 0x23405C0 Flags: 0
    #[unity::class_method(61)] pub fn can_add_exp(&self, unit: &Unit) -> bool; // Offset: 0x2340600 Flags: 0
    #[unity::class_method(62)] pub fn can_add_exp2(&self) -> bool; // Offset: 0x2340660 Flags: 0
    #[unity::class_method(63)] pub fn try_add_exp(&self, unit: &Unit, exp: i32) -> bool; // Offset: 0x23406C0 Flags: 0
    #[unity::class_method(64)] pub fn try_add_exp_current_level(&self, unit: &Unit, exp: i32) -> bool; // Offset: 0x2340700 Flags: 0
    #[unity::class_method(65)] pub fn get_exp(&self, unit: &Unit) -> i32; // Offset: 0x2340790 Flags: 0
    #[unity::class_method(66)] pub fn get_exp2(&self) -> i32; // Offset: 0x23407C0 Flags: 0
    #[unity::class_method(67)] pub fn get_next_level_exp(&self, unit: &Unit) -> i32; // Offset: 0x23407F0 Flags: 0
    #[unity::class_method(68)] pub fn get_next_level_exp2(&self) -> i32; // Offset: 0x2340820 Flags: 0
    #[unity::class_method(69)] pub fn get_next_level_cap(&self) -> i32; // Offset: 0x2340850 Flags: 0
    #[unity::class_method(70)] pub fn get_next_level_cap2(&self, unit: &Unit) -> i32; // Offset: 0x2340880 Flags: 0
    #[unity::class_method(71)] pub fn get_level_exp(&self, level: i32) -> i32; // Offset: 0x23408B0 Flags: 0
    #[unity::class_method(72)] pub fn get_level_exp2(&self, unit: &Unit, level: i32) -> i32; // Offset: 0x2340900 Flags: 0
    #[unity::class_method(73)] pub fn get_max_exp(&self, unit: &Unit) -> i32; // Offset: 0x2340940 Flags: 0
    #[unity::class_method(74)] pub fn get_max_exp2(&self) -> i32; // Offset: 0x2340970 Flags: 0
    #[unity::class_method(75)] pub fn get_diff_exp(&self, unit: &Unit) -> i32; // Offset: 0x2336970 Flags: 0
    #[unity::class_method(76)] pub fn get_diff_exp2(&self) -> i32; // Offset: 0x23409A0 Flags: 0
    #[unity::class_method(77)] pub fn get_max_diff_exp(&self, unit: &Unit) -> i32; // Offset: 0x2336920 Flags: 0
    #[unity::class_method(78)] pub fn get_max_diff_exp2(&self) -> i32; // Offset: 0x23409E0 Flags: 0

    #[unity::class_method(80)] pub fn set_level2(&self, level: i32); // Offset: 0x2340A70 Flags: 0
    #[unity::class_method(81)] pub fn set_level_from_unit_reliance(&self, unit: &Unit, unit_reliance_level: RelianceDataLevel); // Offset: 0x2340AC0 Flags: 0
    #[unity::class_method(82)] pub fn dbg_set_level(&self, unit: &Unit, level: i32); // Offset: 0x2340B00 Flags: 0
    #[unity::class_method(83)] pub fn get_level(&self, unit: &Unit) -> i32; // Offset: 0x23302C0 Flags: 0
    #[unity::class_method(84)] pub fn get_level2(&self) -> i32; // Offset: 0x2340B10 Flags: 0
    #[unity::class_method(85)] pub fn get_level_from_exp(&self, unit: &Unit) -> i32; // Offset: 0x23308F0 Flags: 0
    #[unity::class_method(86)] pub fn get_max_level(&self, unit: &Unit) -> i32; // Offset: 0x2340B50 Flags: 0
    #[unity::class_method(87)] pub fn level_up(&self, unit: &Unit); // Offset: 0x2334310 Flags: 0
    #[unity::class_method(88)] pub fn is_unlocked_level_cap(&self) -> bool; // Offset: 0x2340B80 Flags: 0
    #[unity::class_method(89)] pub fn is_unlocked_level_cap2(god_data: &GodData) -> bool; // Offset: 0x2340BF0 Flags: 0
    #[unity::class_method(90)] pub fn unlock_level_cap(&self); // Offset: 0x2340D40 Flags: 0
    #[unity::class_method(91)] pub fn unlock_level_cap2(god_data: &GodData); // Offset: 0x2340DB0 Flags: 0
    #[unity::class_method(92)] pub fn lock_level_cap(&self); // Offset: 0x2340FE0 Flags: 0
    #[unity::class_method(93)] pub fn lock_level_cap2(god_data: &GodData); // Offset: 0x2341050 Flags: 0
    #[unity::class_method(94)] pub fn is_level_cap_normal(&self, unit: &Unit) -> bool; // Offset: 0x2341250 Flags: 0
    #[unity::class_method(95)] pub fn is_level_cap_talk(&self, unit: &Unit) -> bool; // Offset: 0x23305A0 Flags: 0
    #[unity::class_method(96)] pub fn is_notified_level_cap_talk(&self, unit: &Unit) -> bool; // Offset: 0x2330570 Flags: 0
    #[unity::class_method(97)] pub fn set_notified_level_cap_talk(&self, unit: &Unit, flag: bool); // Offset: 0x2330800 Flags: 0
    #[unity::class_method(98)] pub fn get_reliance_level(&self, unit: &Unit) -> GodDataRelianceLevel; // Offset: 0x2336260 Flags: 0
    #[unity::class_method(99)] pub fn get_max_reliance_level(&self, unit: &Unit) -> GodDataRelianceLevel; // Offset: 0x2341280 Flags: 0
    #[unity::class_method(100)] pub fn can_be_reliance_level_s(&self, unit: &Unit) -> bool; // Offset: 0x23412B0 Flags: 0
    #[unity::class_method(101)] pub fn set_reliance_level_s(&self, unit: &Unit); // Offset: 0x23412E0 Flags: 0
    #[unity::class_method(102)] pub fn dbg_set_reliance_level_s(&self, unit: &Unit); // Offset: 0x2341310 Flags: 0
    #[unity::class_method(103)] pub fn get_count_of_reliance_level_a(&self) -> i32; // Offset: 0x2341320 Flags: 0
    #[unity::class_method(104)] pub fn update_state(&self); // Offset: 0x2334340 Flags: 0
    #[unity::class_method(105)] pub fn can_talk(&self, unit: &Unit) -> bool; // Offset: 0x23305D0 Flags: 0
    #[unity::class_method(106)] pub fn get_inherited_skills(&self, unit: &Unit) -> &'static GodInheritedSkills; // Offset: 0x2341330 Flags: 0
    #[unity::class_method(107)] pub fn can_inherit_skills(&self, unit: &Unit) -> bool; // Offset: 0x2341360 Flags: 0
    #[unity::class_method(108)] pub fn can_add_engage_turn_limit(&self, unit: &Unit) -> bool; // Offset: 0x23413F0 Flags: 0
    #[unity::class_method(109)] pub fn can_add_engage_turn_limit2(&self) -> bool; // Offset: 0x2341480 Flags: 0
    #[unity::class_method(110)] pub fn can_sub_engage_count_limit(&self, unit: &Unit) -> bool; // Offset: 0x2341510 Flags: 0
    #[unity::class_method(111)] pub fn can_sub_engage_count_limit2(&self) -> bool; // Offset: 0x23415A0 Flags: 0

    #[unity::class_method(113)] pub fn is_around(unit_a: &Unit, unit_b: &Unit) -> bool; // Offset: 0x2341AB0 Flags: 0
    #[unity::class_method(114)] pub fn is_around2(god_unit: &GodUnit, unit: &Unit) -> bool; // Offset: 0x2341FF0 Flags: 0
    #[unity::class_method(115)] pub fn get_link_god_unit(&self) -> &'static GodUnit; // Offset: 0x2342080 Flags: 0

    #[unity::class_method(117)] pub fn get_engage_skills(&self, unit: &Unit) -> &'static SkillArray; // Offset: 0x2342170 Flags: 0
    #[unity::class_method(118)] pub fn get_engage_skills2(&self) -> &'static SkillArray; // Offset: 0x2342210 Flags: 0
    #[unity::class_method(119)] pub fn get_engage_skill(&self, unit: &Unit, index: i32) -> &'static SkillData; // Offset: 0x23422B0 Flags: 0
    #[unity::class_method(120)] pub fn get_engage_skill2(&self, index: i32) -> &'static SkillData; // Offset: 0x2342450 Flags: 0
    #[unity::class_method(121)] pub fn get_engage_items(&self, unit: &Unit) -> List<&'static ItemData>; // Offset: 0x2342460 Flags: 0
    #[unity::class_method(122)] pub fn get_engage_items2(&self) -> List<&'static ItemData>; // Offset: 0x2342520 Flags: 0
    #[unity::class_method(123)] pub fn get_syncro_skills(&self, unit: &Unit) -> &'static SkillArray; // Offset: 0x2342530 Flags: 0
    #[unity::class_method(124)] pub fn get_syncro_skills2(&self) -> &'static SkillArray; // Offset: 0x2342ED0 Flags: 0
    #[unity::class_method(125)] pub fn get_aptitude(&self, unit: &Unit) -> &'static WeaponMask; // Offset: 0x2342EE0 Flags: 0
    #[unity::class_method(126)] pub fn get_aptitude2(&self) -> &'static WeaponMask; // Offset: 0x2342F10 Flags: 0
    #[unity::class_method(127)] pub fn can_update_for_god_state(&self) -> bool; // Offset: 0x2342F40 Flags: 0
    #[unity::class_method(128)] pub fn update_all_bonds_for_god_state(&self, god_state: GodState); // Offset: 0x2342F60 Flags: 0
    #[unity::class_method(129)] pub fn update_for_god_state(&self, unit: &Unit, god_state: GodState); // Offset: 0x2340070 Flags: 0
    #[unity::class_method(130)] pub fn delete_bonds_exluding(&self, pids: List<&Il2CppString>); // Offset: 0x2342F80 Flags: 0

    #[unity::class_method(134)] pub fn can_link(&self) -> bool; // Offset: 0x2337440 Flags: 0
    #[unity::class_method(135)] pub fn can_change(&self) -> bool; // Offset: 0x2342EB0 Flags: 0
    #[unity::class_method(136)] pub fn get_ring_prefab_path(&self) -> &'static Il2CppString; // Offset: 0x2342FA0 Flags: 0
    #[unity::class_method(137)] pub fn get_dirty_level(&self) -> i32; // Offset: 0x2343290 Flags: 0
    #[unity::class_method(138)] pub fn get_ring_dirty_sep1() -> f32; // Offset: 0x2343380 Flags: 0
    #[unity::class_method(139)] pub fn get_ring_dirty_sep2() -> f32; // Offset: 0x2343390 Flags: 0
    #[unity::class_method(140)] pub fn add_dirty(&self, dirty: i32); // Offset: 0x2330450 Flags: 0
    #[unity::class_method(141)] pub fn change_opponent(&self); // Offset: 0x23433A0 Flags: 0
    #[unity::class_method(142)] pub fn init_god_weapon_refine(&self); // Offset: 0x233E890 Flags: 0
    #[unity::class_method(143)] pub fn get_god_weapon_list(&self) -> List<&'static Il2CppString>; // Offset: 0x2343620 Flags: 0
    #[unity::class_method(144)] pub fn get_god_weapon_refine_level(&self, iid: &Il2CppString, kind: GodWeaponRefineDataKind) -> i32; // Offset: 0x2343CB0 Flags: 0
    #[unity::class_method(145)] pub fn add_god_weapon_refine_enhance(&self, iid: &Il2CppString, capability: &CapabilitySbyte); // Offset: 0x2343F00 Flags: 0
    #[unity::class_method(146)] pub fn get_god_weapon_refine_skill(&self, iid: &Il2CppString) -> &'static Il2CppString; // Offset: 0x23442A0 Flags: 0
    #[unity::class_method(147)] pub fn set_god_weapon_refine_level(&self, iid: &Il2CppString, kind: GodWeaponRefineDataKind, level: i32); // Offset: 0x2344330 Flags: 0
    #[unity::class_method(148)] pub fn reset_god_weapon_refine_level(&self, iid: &Il2CppString, kind: GodWeaponRefineDataKind); // Offset: 0x23445E0 Flags: 0
    #[unity::class_method(149)] pub fn inc_god_weapon_refine_level(&self, iid: &Il2CppString, kind: GodWeaponRefineDataKind); // Offset: 0x2344600 Flags: 0
    #[unity::class_method(150)] pub fn set_god_weapon_refine_skill(&self, iid: &Il2CppString, sid: &Il2CppString); // Offset: 0x2344850 Flags: 0
    #[unity::class_method(151)] pub fn reset_god_weapon_refine_level2(&self, iid: &Il2CppString); // Offset: 0x23448E0 Flags: 0
    #[unity::class_method(152)] pub fn get_using_capacity(&self, iid: &Il2CppString) -> i32; // Offset: 0x23449A0 Flags: 0
    #[unity::class_method(153)] pub fn set_using_capacity(&self, iid: &Il2CppString, capacity: i32); // Offset: 0x2344A30 Flags: 0
    #[unity::class_method(154)] pub fn add_using_capacity(&self, iid: &Il2CppString, capacity: i32); // Offset: 0x2344AC0 Flags: 0
    #[unity::class_method(155)] pub fn try_get_god_weapon_refine_result(&self, iid: &Il2CppString) -> &'static GodWeaponRefineResult; // Offset: 0x2344B50 Flags: 0
    #[unity::class_method(156)] pub fn try_get_god_weapon_refine_result_enhance(&self, iid: &Il2CppString) -> &'static CapabilitySbyte; // Offset: 0x2344BE0 Flags: 0
    #[unity::class_method(157)] pub fn try_get_god_weapon_refine_result_equip_skills(&self, iid: &Il2CppString, is_enchant: bool) -> &'static SkillArray; // Offset: 0x2344C90 Flags: 0
    #[unity::class_method(158)] pub fn copy_god_weapon_refine_from(&self, from: &GodUnit); // Offset: 0x23355B0 Flags: 0
    #[unity::class_method(159)] pub fn correct_bond(&self, instance_id: i32); // Offset: 0x2345270 Flags: 0
    #[unity::class_method(160)] pub fn on_serialize(&self, stream: &Stream); // Offset: 0x2345420 Flags: 0
    #[unity::class_method(161)] pub fn on_deserialize(&self, stream: &Stream, version: i32); // Offset: 0x23458F0 Flags: 0
    #[unity::class_method(162)] pub fn serialize(&self, stream: &Stream); // Offset: 0x2346B10 Flags: 0
    #[unity::class_method(163)] pub fn deserialize(&self, stream: &Stream); // Offset: 0x2346B60 Flags: 0
    #[unity::class_method(164)] pub fn clear(&self); // Offset: 0x233E7B0 Flags: 0
    #[unity::class_method(165)] pub fn get_sort_key(&self) -> i32; // Offset: 0x2346BB0 Flags: 0
    #[unity::class_method(166)] pub fn is_valid(&self) -> bool; // Offset: 0x2346BD0 Flags: 0
    #[unity::class_method(167)] pub fn op_implicit(p: &GodUnit) -> &'static GodData; // Offset: 0x2346BE0 Flags: 0
    #[unity::class_method(168)] pub fn change_cancel(&self); // Offset: 0x2346C70 Flags: 0
    #[unity::class_method(169)] pub fn change_resume(&self); // Offset: 0x2346D90 Flags: 0
    #[unity::class_method(170)] pub fn ctor(&self); // Offset: 0x2346E10 Flags: 0
    #[unity::class_method(171)] pub fn cctor(); // Offset: 0x2346FB0 Flags: 0
    #[unity::class_method(172)] pub fn <_get_god_weapon_list>g__add_each_iids|186_0(: GodUnit<>c__DisplayClass186_0, : GodUnit<>c__DisplayClass186_1); // Offset: 0x2343B90 Flags: 0
     */
}