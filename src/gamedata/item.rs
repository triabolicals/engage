use num_derive::FromPrimitive;
use unity::engine::Color;
pub use unity::prelude::*;
use crate::bit::{BitField32, BitField32Methods};
use super::{*, skill::SkillArray};
use crate::gamedata::StructBaseFields;
use crate::unit::{CapabilitySbyte, Unit, UnitAnimTypes, UnitItem};

#[unity::class("App", "ItemData")]
pub struct ItemData {
	pub parent: StructBaseFields,
	pub iid: &'static Il2CppString,
	pub name: &'static Il2CppString,
	pub help: &'static Il2CppString,
	pub tutorial: &'static Il2CppString,
	pub aid: &'static Il2CppString,
	pub kind: u32,
	pub attr: u32,
	pub use_type: u32,
	pub weapon_attr: u32,
	pub icon: Option<&'static Il2CppString>,
	pub endurance: u8,
	pub power: u8,
	pub weight: u8,
	pub range_i: u8,
	pub range_o: u8,
	pub distnace: u8,
	pub hit: i16,
	pub critical: i16,
	pub avoid: i16,
	pub secure: i16,
	__: i16,	//
	pub price: i32, 
	pub weapon_level_str: &'static Il2CppString,
	pub rod_type: i32, 
	pub rod_exp: u8,
	pub rate_arena: u8,
	pub shoot_effect: Option<&'static Il2CppString>,
	pub hit_effect: Option<&'static Il2CppString>, 
	pub cannon_effect: Option<&'static Il2CppString>,
	pub overlap_terrain: Option<&'static Il2CppString>,
	pub flag: &'static mut ItemDataFlags,
	pub enhance: &'static CapabilitySbyte,
	pub grow_ratio: &'static CapabilitySbyte,
	pub equip_condition: Option<&'static Il2CppString>,
	pub equip_sids: &'static Array<&'static Il2CppString>, // Offset 0xC8, Attr: 1
	pub passive_sids: &'static Array<&'static Il2CppString>, // Offset 0xD0, Attr: 1
	pub give_sids: &'static Array<&'static Il2CppString>, // Offset 0xD8, Attr: 1
	add_target: ItemDataAddTargets, // Offset 0xE0, Attr: 1
	add_type: ItemDataUseTypes, // Offset 0xE4, Attr: 1
	add_power: u8, // Offset 0xE8, Attr: 1
	add_range: u8, // Offset 0xE9, Attr: 1
	add_sids: &'static Array<&'static Il2CppString>, // Offset 0xF0, Attr: 1
	add_effect: &'static Il2CppString, // Offset 0xF8, Attr: 1
	add_help: &'static Il2CppString, // Offset 0x100, Attr: 1
	high_rank_item: &'static Il2CppString, // Offset 0x108, Attr: 1
	is_weapon: bool, // Offset 0x110, Attr: 1
	flag_name: &'static Il2CppString, // Offset 0x118, Attr: 1
	prefixless_iid: &'static Il2CppString, // Offset 0x120, Attr: 1
	enchant_hash: i32, // Offset 0x128, Attr: 1
	weapon_level: WeaponLevelKind, // Offset 0x12C, Attr: 1
	unit_item: &'static UnitItem, // Offset 0x130, Attr: 1
	pub equip_skills: &'static SkillArray, // Offset 0x138, Attr: 33
	pub passive_skills: &'static SkillArray, // Offset 0x140, Attr: 33
	pub give_skills: &'static SkillArray, // Offset 0x148, Attr: 33
	pub enchant_skills1: &'static SkillArray, // Offset 0x150, Attr: 33
	pub enchant_skills2: &'static SkillArray, // Offset 0x158, Attr: 33
	pub enchant_skills3: &'static SkillArray, // Offset 0x160, Attr: 33
	pub enchant_skills4: &'static SkillArray, // Offset 0x168, Attr: 33
	attack_motion: UnitAnimTypes, // Offset 0x170, Attr: 1
}
impl ItemData {
	#[unity::class_method(0)] pub fn get_kind_name(kind: ItemDataKinds) -> &'static Il2CppString; // Offset: 0x27AA800 Flags: 0
	// #[unity::class_method(1)] pub fn load(); // Offset: 0x27AA940 Flags: 0
	#[unity::class_method(2)] pub fn ctor(&self); // Offset: 0x27AA9F0 Flags: 0
	/*
	#[unity::class_method(3)] pub fn get_iid(&self) -> &'static Il2CppString; // Offset: 0x27AABF0 Flags: 0
	#[unity::class_method(4)] pub fn set_iid(&self, value: &'static Il2CppString); // Offset: 0x27AAC00 Flags: 0
	#[unity::class_method(5)] pub fn get_name(&self) -> &'static Il2CppString; // Offset: 0x27AAC10 Flags: 0
	#[unity::class_method(6)] pub fn set_name(&self, value: &'static Il2CppString); // Offset: 0x27AAC20 Flags: 0
	#[unity::class_method(7)] pub fn get_help(&self) -> &'static Il2CppString; // Offset: 0x27AAC30 Flags: 0
	#[unity::class_method(8)] pub fn set_help(&self, value: &'static Il2CppString); // Offset: 0x27AAC40 Flags: 0
	#[unity::class_method(9)] pub fn get_tutorial(&self) -> &'static Il2CppString; // Offset: 0x27AAC50 Flags: 0
	#[unity::class_method(10)] pub fn set_tutorial(&self, value: &'static Il2CppString); // Offset: 0x27AAC60 Flags: 0
	#[unity::class_method(11)] pub fn get_aid(&self) -> &'static Il2CppString; // Offset: 0x27AAC70 Flags: 0
	#[unity::class_method(12)] pub fn set_aid(&self, value: &'static Il2CppString); // Offset: 0x27AAC80 Flags: 0
	#[unity::class_method(13)] pub fn get_kind(&self) -> ItemDataKinds; // Offset: 0x27AAC90 Flags: 0
	#[unity::class_method(14)] pub fn set_kind(&self, value: ItemDataKinds); // Offset: 0x27AACA0 Flags: 0
	#[unity::class_method(15)] pub fn get_attr(&self) -> ItemDataAttrs; // Offset: 0x27AACB0 Flags: 0
	#[unity::class_method(16)] pub fn set_attr(&self, value: ItemDataAttrs); // Offset: 0x27AACC0 Flags: 0
	#[unity::class_method(17)] pub fn get_use_type(&self) -> ItemDataUseTypes; // Offset: 0x27AACD0 Flags: 0
	#[unity::class_method(18)] pub fn set_use_type(&self, value: ItemDataUseTypes); // Offset: 0x27AACE0 Flags: 0
	#[unity::class_method(19)] pub fn get_weapon_attr(&self) -> ItemDataWeaponAttrs; // Offset: 0x27AACF0 Flags: 0
	#[unity::class_method(20)] pub fn set_weapon_attr(&self, value: ItemDataWeaponAttrs); // Offset: 0x27AAD00 Flags: 0
	#[unity::class_method(21)] pub fn get_icon(&self) -> &'static Il2CppString; // Offset: 0x27AAD10 Flags: 0
	#[unity::class_method(22)] pub fn set_icon(&self, value: &'static Il2CppString); // Offset: 0x27AAD20 Flags: 0
	#[unity::class_method(23)] pub fn get_endurance(&self) -> u8; // Offset: 0x27AAD30 Flags: 0
	#[unity::class_method(24)] pub fn set_endurance(&self, value: u8); // Offset: 0x27AAD40 Flags: 0
	#[unity::class_method(25)] pub fn get_power(&self) -> u8; // Offset: 0x27AAD50 Flags: 0
	#[unity::class_method(26)] pub fn set_power(&self, value: u8); // Offset: 0x27AAD60 Flags: 0
	#[unity::class_method(27)] pub fn get_weight(&self) -> u8; // Offset: 0x27AAD70 Flags: 0
	#[unity::class_method(28)] pub fn set_weight(&self, value: u8); // Offset: 0x27AAD80 Flags: 0
	#[unity::class_method(29)] pub fn get_range_i(&self) -> u8; // Offset: 0x27AAD90 Flags: 0
	#[unity::class_method(30)] pub fn set_range_i(&self, value: u8); // Offset: 0x27AADA0 Flags: 0
	#[unity::class_method(31)] pub fn get_range_o(&self) -> u8; // Offset: 0x27AADB0 Flags: 0
	#[unity::class_method(32)] pub fn set_range_o(&self, value: u8); // Offset: 0x27AADC0 Flags: 0
	#[unity::class_method(33)] pub fn get_distance(&self) -> u8; // Offset: 0x27AADD0 Flags: 0
	#[unity::class_method(34)] pub fn set_distance(&self, value: u8); // Offset: 0x27AADE0 Flags: 0
	#[unity::class_method(35)] pub fn get_hit(&self) -> i16; // Offset: 0x27AADF0 Flags: 0
	#[unity::class_method(36)] pub fn set_hit(&self, value: i16); // Offset: 0x27AAE00 Flags: 0
	#[unity::class_method(37)] pub fn get_critical(&self) -> i16; // Offset: 0x27AAE10 Flags: 0
	#[unity::class_method(38)] pub fn set_critical(&self, value: i16); // Offset: 0x27AAE20 Flags: 0
	#[unity::class_method(39)] pub fn get_avoid(&self) -> i16; // Offset: 0x27AAE30 Flags: 0
	#[unity::class_method(40)] pub fn set_avoid(&self, value: i16); // Offset: 0x27AAE40 Flags: 0
	#[unity::class_method(41)] pub fn get_secure(&self) -> i16; // Offset: 0x27AAE50 Flags: 0
	#[unity::class_method(42)] pub fn set_secure(&self, value: i16); // Offset: 0x27AAE60 Flags: 0
	#[unity::class_method(43)] pub fn get_price(&self) -> i32; // Offset: 0x27AAE70 Flags: 0
	#[unity::class_method(44)] pub fn set_price(&self, value: i32); // Offset: 0x27AAE80 Flags: 0
	#[unity::class_method(45)] pub fn get_weapon_level(&self) -> &'static Il2CppString; // Offset: 0x27AAE90 Flags: 0
	#[unity::class_method(46)] pub fn set_weapon_level(&self, value: &'static Il2CppString); // Offset: 0x27AAEA0 Flags: 0
	#[unity::class_method(47)] pub fn get_rod_type(&self) -> ItemDataRodTypes; // Offset: 0x27AAEB0 Flags: 0
	#[unity::class_method(48)] pub fn set_rod_type(&self, value: ItemDataRodTypes); // Offset: 0x27AAEC0 Flags: 0
	#[unity::class_method(49)] pub fn get_rod_exp(&self) -> u8; // Offset: 0x27AAED0 Flags: 0
	#[unity::class_method(50)] pub fn set_rod_exp(&self, value: u8); // Offset: 0x27AAEE0 Flags: 0
	#[unity::class_method(51)] pub fn get_rate_arena(&self) -> u8; // Offset: 0x27AAEF0 Flags: 0
	#[unity::class_method(52)] pub fn set_rate_arena(&self, value: u8); // Offset: 0x27AAF00 Flags: 0
	#[unity::class_method(53)] pub fn get_shoot_effect(&self) -> &'static Il2CppString; // Offset: 0x27AAF10 Flags: 0
	#[unity::class_method(54)] pub fn set_shoot_effect(&self, value: &'static Il2CppString); // Offset: 0x27AAF20 Flags: 0
	#[unity::class_method(55)] pub fn get_hit_effect(&self) -> &'static Il2CppString; // Offset: 0x27AAF30 Flags: 0
	#[unity::class_method(56)] pub fn set_hit_effect(&self, value: &'static Il2CppString); // Offset: 0x27AAF40 Flags: 0
	#[unity::class_method(57)] pub fn get_cannon_effect(&self) -> &'static Il2CppString; // Offset: 0x27AAF50 Flags: 0
	#[unity::class_method(58)] pub fn set_cannon_effect(&self, value: &'static Il2CppString); // Offset: 0x27AAF60 Flags: 0
	#[unity::class_method(59)] pub fn get_overlap_terrain(&self) -> &'static Il2CppString; // Offset: 0x27AAF70 Flags: 0
	#[unity::class_method(60)] pub fn set_overlap_terrain(&self, value: &'static Il2CppString); // Offset: 0x27AAF80 Flags: 0
	#[unity::class_method(61)] pub fn get_flag(&self) -> &'static ItemDataFlags; // Offset: 0x27AAF90 Flags: 0
	#[unity::class_method(62)] pub fn set_flag(&self, value: &ItemDataFlags); // Offset: 0x27AAFA0 Flags: 0
	#[unity::class_method(63)] pub fn get_enhance(&self) -> &'static CapabilitySbyte; // Offset: 0x27AAFB0 Flags: 0
	#[unity::class_method(64)] pub fn set_enhance(&self, value: &CapabilitySbyte); // Offset: 0x27AAFC0 Flags: 0
	#[unity::class_method(65)] pub fn get_grow_ratio(&self) -> &'static CapabilitySbyte; // Offset: 0x27AAFD0 Flags: 0
	#[unity::class_method(66)] pub fn set_grow_ratio(&self, value: &CapabilitySbyte); // Offset: 0x27AAFE0 Flags: 0
	#[unity::class_method(67)] pub fn get_equip_condition(&self) -> &'static Il2CppString; // Offset: 0x27AAFF0 Flags: 0
	#[unity::class_method(68)] pub fn set_equip_condition(&self, value: &'static Il2CppString); // Offset: 0x27AB000 Flags: 0
	#[unity::class_method(69)] pub fn get_equip_sids(&self) -> &'static Array<String>; // Offset: 0x27AB010 Flags: 0
	#[unity::class_method(70)] pub fn set_equip_sids(&self, value: &'static Array<&'static Il2CppString>,); // Offset: 0x27AB020 Flags: 0
	#[unity::class_method(71)] pub fn get_passive_sids(&self) -> &'static Array<String>; // Offset: 0x27AB030 Flags: 0
	#[unity::class_method(72)] pub fn set_passive_sids(&self, value: &'static Array<&'static Il2CppString>,); // Offset: 0x27AB040 Flags: 0
	#[unity::class_method(73)] pub fn get_give_sids(&self) -> &'static Array<String>; // Offset: 0x27AB050 Flags: 0
	#[unity::class_method(74)] pub fn set_give_sids(&self, value: &'static Array<&'static Il2CppString>,); // Offset: 0x27AB060 Flags: 0
	#[unity::class_method(75)] pub fn get_add_target(&self) -> ItemDataAddTargets; // Offset: 0x27AB070 Flags: 0
	#[unity::class_method(76)] pub fn set_add_target(&self, value: ItemDataAddTargets); // Offset: 0x27AB080 Flags: 0
	#[unity::class_method(77)] pub fn get_add_type(&self) -> ItemDataUseTypes; // Offset: 0x27AB090 Flags: 0
	#[unity::class_method(78)] pub fn set_add_type(&self, value: ItemDataUseTypes); // Offset: 0x27AB0A0 Flags: 0
	#[unity::class_method(79)] pub fn get_add_power(&self) -> u8; // Offset: 0x27AB0B0 Flags: 0
	#[unity::class_method(80)] pub fn set_add_power(&self, value: u8); // Offset: 0x27AB0C0 Flags: 0
	#[unity::class_method(81)] pub fn get_add_range(&self) -> u8; // Offset: 0x27AB0D0 Flags: 0
	#[unity::class_method(82)] pub fn set_add_range(&self, value: u8); // Offset: 0x27AB0E0 Flags: 0
	#[unity::class_method(83)] pub fn get_add_sids(&self) -> &'static Array<String>; // Offset: 0x27AB0F0 Flags: 0
	#[unity::class_method(84)] pub fn set_add_sids(&self, value: &'static Array<&'static Il2CppString>,); // Offset: 0x27AB100 Flags: 0
	#[unity::class_method(85)] pub fn get_add_effect(&self) -> &'static Il2CppString; // Offset: 0x27AB110 Flags: 0
	#[unity::class_method(86)] pub fn set_add_effect(&self, value: &'static Il2CppString); // Offset: 0x27AB120 Flags: 0
	#[unity::class_method(87)] pub fn get_add_help(&self) -> &'static Il2CppString; // Offset: 0x27AB130 Flags: 0
	#[unity::class_method(88)] pub fn set_add_help(&self, value: &'static Il2CppString); // Offset: 0x27AB140 Flags: 0
	#[unity::class_method(89)] pub fn get_high_rank_item(&self) -> &'static Il2CppString; // Offset: 0x27AB150 Flags: 0
	#[unity::class_method(90)] pub fn set_high_rank_item(&self, value: &'static Il2CppString); // Offset: 0x27AB160 Flags: 0
	#[unity::class_method(91)] pub fn get_equip_skills(&self) -> &'static SkillArray; // Offset: 0x27AB170 Flags: 0
	#[unity::class_method(92)] pub fn get_passive_skills(&self) -> &'static SkillArray; // Offset: 0x27AB180 Flags: 0
	#[unity::class_method(93)] pub fn get_give_skills(&self) -> &'static SkillArray; // Offset: 0x27AB190 Flags: 0
	#[unity::class_method(94)] pub fn get_enchant_skills1(&self) -> &'static SkillArray; // Offset: 0x27AB1A0 Flags: 0
	#[unity::class_method(95)] pub fn get_enchant_skills2(&self) -> &'static SkillArray; // Offset: 0x27AB1B0 Flags: 0
	#[unity::class_method(96)] pub fn get_enchant_skills3(&self) -> &'static SkillArray; // Offset: 0x27AB1C0 Flags: 0
	#[unity::class_method(97)] pub fn get_enchant_skills4(&self) -> &'static SkillArray; // Offset: 0x27AB1D0 Flags: 0
	*/
	#[unity::class_method(98)] pub fn get_attack_motion(&self) -> UnitAnimTypes; // Offset: 0x27AB1E0 Flags: 0
	#[unity::class_method(99)] pub fn set_attack_motion(&self, value: UnitAnimTypes); // Offset: 0x27AB1F0 Flags: 0
	#[unity::class_method(100)] pub fn get_name(&self) -> &'static Il2CppString; // Offset: 0x27AB200 Flags: 0
	#[unity::class_method(101)] pub fn can_use(&self) -> bool; // Offset: 0x27AB400 Flags: 0
	#[unity::class_method(102)] pub fn can_self_target(&self, unit: &Unit) -> bool; // Offset: 0x27AB460 Flags: 0
	#[unity::class_method(103)] pub fn get_enchant_hash(&self) -> i32; // Offset: 0x27AB530 Flags: 0
	#[unity::class_method(104)] pub fn has_enchant_hash(&self) -> bool; // Offset: 0x27AB540 Flags: 0
	#[unity::class_method(105)] pub fn get_prefixless_iid(&self) -> &'static Il2CppString; // Offset: 0x27AB550 Flags: 0
	#[unity::class_method(106)] pub fn can_expend(&self) -> bool; // Offset: 0x27AB560 Flags: 0
	#[unity::class_method(107)] pub fn is_weapon(&self) -> bool; // Offset: 0x27AB580 Flags: 0
	#[unity::class_method(108)] pub fn is_physical(&self) -> bool; // Offset: 0x27AB590 Flags: 0
	#[unity::class_method(109)] pub fn is_magic(&self) -> bool; // Offset: 0x27AB5A0 Flags: 0
	#[unity::class_method(110)] pub fn is_breath(&self) -> bool; // Offset: 0x27AB5B0 Flags: 0
	#[unity::class_method(111)] pub fn is_flag(&self, flags: ItemDataFlags) -> bool; // Offset: 0x27AB610 Flags: 0
	#[unity::class_method(112)] pub fn is_sure_hit(&self) -> bool; // Offset: 0x27AB680 Flags: 0
	#[unity::class_method(113)] pub fn is_class_change(&self) -> bool; // Offset: 0x27AB6A0 Flags: 0
	#[unity::class_method(114)] pub fn is_material(&self) -> bool; // Offset: 0x27AB6D0 Flags: 0
	#[unity::class_method(115)] pub fn is_long_range(&self) -> bool; // Offset: 0x27AB700 Flags: 0
	#[unity::class_method(116)] pub fn is_range_target(&self) -> bool; // Offset: 0x27AB710 Flags: 0
	#[unity::class_method(117)] pub fn is_range_use_type(&self, use_type: ItemDataUseTypes) -> bool; // Offset: 0x27AB770 Flags: 0
	#[unity::class_method(118)] pub fn is_range_heal(&self) -> bool; // Offset: 0x27AB800 Flags: 0
	#[unity::class_method(119)] pub fn is_range_rest_heal(&self) -> bool; // Offset: 0x27AB880 Flags: 0
	#[unity::class_method(120)] pub fn is_range_again(&self) -> bool; // Offset: 0x27AB900 Flags: 0
	#[unity::class_method(121)] pub fn is_range_engage_add(&self) -> bool; // Offset: 0x27AB980 Flags: 0
	#[unity::class_method(122)] pub fn is_bless(&self) -> bool; // Offset: 0x27ABA00 Flags: 0
	#[unity::class_method(123)] pub fn is_download(&self) -> bool; // Offset: 0x27ABA20 Flags: 0
	#[unity::class_method(124)] pub fn is_unknown(&self) -> bool; // Offset: 0x27ABA80 Flags: 0
	#[unity::class_method(125)] pub fn is_single_rod(&self) -> bool; // Offset: 0x27ABB50 Flags: 0
	#[unity::class_method(127)] pub fn get_weapon_level(&self) -> i32; // Offset: 0x27ABBE0 Flags: 0
	/// Get item price at endurance/use count.
	#[unity::class_method(128)] pub fn get_price(&self, endurance: i32) -> i32; // Offset: 0x27ABBF0 Flags: 0
	#[unity::class_method(129)] pub fn get_unit_item(&self, unit: Option<&Unit>) -> &'static UnitItem; // Offset: 0x27ABC20 Flags: 0
	#[unity::class_method(130)] pub fn get_unit_item2(&self) -> &'static UnitItem; // Offset: 0x27ABCF0 Flags: 0
	#[unity::class_method(131)] pub fn get_font_color(&self, is_active: bool) -> Color; // Offset: 0x27ABD00 Flags: 0
	#[unity::class_method(134)] pub fn calc_enchant_hash(&self) -> i32; // Offset: 0x27AC270 Flags: 0
	#[unity::class_method(135)] pub fn has_flag_name(&self) -> bool; // Offset: 0x27AC220 Flags: 0
	#[unity::class_method(136)] pub fn calc_attr(&self) -> ItemDataAttrs; // Offset: 0x27AC400 Flags: 0
	#[unity::class_method(137)] pub fn get_attr_name_mid(&self) -> &'static Il2CppString; // Offset: 0x27AC4B0 Flags: 0
	#[unity::class_method(138)] pub fn get_attr_help_mid(&self) -> &'static Il2CppString; // Offset: 0x27AC590 Flags: 0
	#[unity::class_method(139)] pub fn try_get_skill(skills: &'static Array<&'static Il2CppString>, index: i32) -> &'static Il2CppString; // Offset: 0x27AC670 Flags: 0
	#[unity::class_method(140)] pub fn get_enchant_skills(&self, level: i32) -> &'static SkillArray; // Offset: 0x27AC6D0 Flags: 0

	#[unity::class_method(146)] pub fn create_simple_weapon(kind: ItemDataKinds, is_bullet: bool) -> &'static ItemData; // Offset: 0x27B15F0 Flags: 0
	#[unity::class_method(147)] pub fn is_inventory(&self) -> bool; // Offset: 0x27B1910 Flags: 0
	#[unity::class_method(148)] pub fn get_inventory(&self) -> i32; // Offset: 0x27B1920 Flags: 0
	#[unity::class_method(149)] pub fn get_max_inventory(&self) -> i32; // Offset: 0x27B1A10 Flags: 0
	#[unity::class_method(150)] pub fn set_inventory(&self, count: i32); // Offset: 0x27B1A30 Flags: 0
	#[unity::class_method(151)] pub fn add_inventory(&self, count: i32); // Offset: 0x27B1C30 Flags: 0
	#[unity::class_method(152)] pub fn regist_global_flags(); // Offset: 0x27B1EA0 Flags: 0
	// #[unity::class_method(153)] pub fn get_shoot_effect2(&self) -> &'static EffectData; // Offset: 0x27B1FF0 Flags: 0
	// #[unity::class_method(154)] pub fn get_hit_effect2(&self) -> &'static EffectData; // Offset: 0x27B2080 Flags: 0
	// #[unity::class_method(155)] pub fn get_use_effect(&self) -> &'static EffectData; // Offset: 0x27B2110 Flags: 0
	// #[unity::class_method(156)] pub fn get_enchant_effect(&self) -> &'static EffectData; // Offset: 0x27B21D0 Flags: 0
	// #[unity::class_method(157)] pub fn get_cannon_effect2(&self) -> &'static EffectSequence; // Offset: 0x27B2290 Flags: 0
	#[unity::class_method(158)] pub fn is_dragon(&self) -> bool; // Offset: 0x27B2320 Flags: 0
	#[unity::class_method(159)] pub fn is_bullet(&self) -> bool; // Offset: 0x27B2380 Flags: 0
	#[unity::class_method(160)] pub fn can_enchant(&self) -> bool; // Offset: 0x27AC3F0 Flags: 0
	#[unity::class_method(161)] pub fn get_master_proof() -> &'static ItemData; // Offset: 0x27B23E0 Flags: 0
	#[unity::class_method(162)] pub fn get_change_proof() -> &'static ItemData; // Offset: 0x27B2470 Flags: 0
	#[unity::class_method(163)] pub fn get_enchant_proof() -> &'static ItemData; // Offset: 0x27B2500 Flags: 0
	#[unity::class_method(164)] pub fn get_gunner_proof() -> &'static ItemData; // Offset: 0x27B2590 Flags: 0
	#[unity::class_method(165)] pub fn replace_high_rank(iids: &Array<&Il2CppString>) -> &'static Array<&'static Il2CppString>; // Offset: 0x27B2620 Flags: 0
	#[unity::class_method(166)] pub fn get_enchant_range_i(&self) -> i32; // Offset: 0x27B2780 Flags: 0
	#[unity::class_method(167)] pub fn get_enchant_range_o(&self) -> i32; // Offset: 0x27B2790 Flags: 0
	#[unity::class_method(168)] pub fn get_gain_exp(&self) -> i32; // Offset: 0x27B27B0 Flags: 0
	#[unity::class_method(169)] pub fn get_gain_skill_point(&self) -> i32; // Offset: 0x27B27D0 Flags: 0
}
impl Gamedata for ItemData {}

#[unity::class("", "FlagField")]
#[nested_from_type(ItemData)]
pub struct ItemDataFlags { pub value: i32, }

impl BitField32Methods for ItemDataFlags {}
#[allow(non_upper_case_globals)]
impl ItemDataFlags {
	pub const Rarity: i32 = 1;
	pub const NotTrade: i32 = 2;
	pub const CanUse: i32 = 4;
	pub const OnlyChapter: i32 = 8;
	pub const OnlyEnemy: i32 = 16;
	pub const OnlyMale: i32 = 32;
	pub const OnlyFemale: i32 = 64;
	pub const Engage: i32 = 128;
	pub const IgnoreWeaponLevel: i32 = 256;
	pub const Unpublic: i32 = 512;
	pub const NotEntrust: i32 = 1024;
	pub const InvertInteract: i32 = 2048;
	pub const Download: i32 = 4096;
	pub const KeyDoor: i32 = 8192;
	pub const KeyTreasureBox: i32 = 16384;
	pub const AIUnequipable: i32 = 32768;
	pub const ReverseAttribute: i32 = 65536;
	pub const LunchBox: i32 = 131072;
	pub const SimpleHelp: i32 = 262144;
	pub const RangeTarget: i32 = 524288;
	pub const IgnoreCombat: i32 = 1048576;
	pub const ForcedCombat: i32 = 2097152;
	pub const Bless: i32 = 16777216;
	pub const Breath: i32 = 33554432;
	pub const Dragon: i32 = 67108864;
	pub const Bullet: i32 = 134217728;
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum ItemDataKinds {
	None = 0, // Attr: 17
	Sword = 1, // Attr: 17
	Lance = 2, // Attr: 17
	Axe = 3, // Attr: 17
	Bow = 4, // Attr: 17
	Dagger = 5, // Attr: 17
	Magic = 6, // Attr: 17
	Rod = 7, // Attr: 17
	Fist = 8, // Attr: 17
	Special = 9, // Attr: 17
	Tool = 10, // Attr: 17
	Shield = 11, // Attr: 17
	Accessory = 12, // Attr: 17
	Precious = 13, // Attr: 17
	RefineIron = 14, // Attr: 17
	RefineSteel = 15, // Attr: 17
	RefineSilver = 16, // Attr: 17
	PieceOfBond = 17, // Attr: 17
	Gold = 18, // Attr: 17
	Num = 19, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum ItemDataAttrs {
	None = 0, // Attr: 17
	Physical = 1, // Attr: 17
	Magic = 2, // Attr: 17
}
#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum ItemDataWeaponAttrs {
	None = 0, // Attr: 17
	Fire = 1, // Attr: 17
	Thunder = 2, // Attr: 17
	Wind = 3, // Attr: 17
	Ice = 4, // Attr: 17
	Light = 5, // Attr: 17
	Dark = 6, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum ItemDataUseTypes {
	None = 0, // Attr: 17
	Attack = 1, // Attr: 17
	Heal = 2, // Attr: 17
	RestHeal = 3, // Attr: 17
	Revive = 4, // Attr: 17
	Warp = 5, // Attr: 17
	Rescue = 6, // Attr: 17
	EngageAdd = 7, // Attr: 17
	Rewarp = 8, // Attr: 17
	Freeze = 9, // Attr: 17
	Sleep = 10, // Attr: 17
	Silence = 11, // Attr: 17
	Charm = 12, // Attr: 17
	Berserk = 13, // Attr: 17
	Weakness = 14, // Attr: 17
	Again = 15, // Attr: 17
	Torch = 16, // Attr: 17
	Food = 17, // Attr: 17
	Rest = 18, // Attr: 17
	SightUp = 19, // Attr: 17
	WeaponLevelUp = 20, // Attr: 17
	GrowUp = 21, // Attr: 17
	Enhance = 22, // Attr: 17
	CCMaster = 23, // Attr: 17
	CCChange = 24, // Attr: 17
	CCExtra = 25, // Attr: 17
	Creation = 26, // Attr: 17
	Draw = 27, // Attr: 17
	GainExp = 28, // Attr: 17
	Stun = 29, // Attr: 17
	Detox = 30, // Attr: 17
	GiveSkill = 31, // Attr: 17
	Foodstuff = 32, // Attr: 17
	Gift = 33, // Attr: 17
	Material = 34, // Attr: 17
	FishingRod = 35, // Attr: 17
	Bless = 36, // Attr: 17
	BlessRest = 37, // Attr: 17
	BlessPlus = 38, // Attr: 17
	BlessRestPlus = 39, // Attr: 17
	CCEnchant = 40, // Attr: 17
	CCGunner = 41, // Attr: 17
	GainSkillPoint = 42, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum ItemDataRodTypes {
	None = 0, // Attr: 17
	Basic = 1, // Attr: 17
	Heal = 2, // Attr: 17
	Interference = 3, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum ItemDataAddTargets {
	None = 0, // Attr: 17
	TargetSelf = 1, // Attr: 17
	Around = 2, // Attr: 17
	Whole = 3, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum WeaponLevelKind {
	None = 0, // Attr: 17
	D = 1, // Attr: 17
	C = 2, // Attr: 17
	B = 3, // Attr: 17
	A = 4, // Attr: 17
	S = 5, // Attr: 17
}




#[unity::class("App", "RewardData")]
pub struct RewardData {
	pub parent: StructDataArrayFields,
	pub iid: &'static Il2CppString,
	pub ratio: f32,
	pub factor: f32,
	pub min: f32,
	pub max: f32,
	pub is_show: bool,
}
impl GamedataArray for RewardData {}

#[unity::class("App", "ItemRefineData")]
pub struct ItemRefineData {
	pub parent: StructDataArrayFields,
	pub iron: u16,
	pub steel: u16,
	pub silver: u16,
	pub price: u16,
	pub refine_level: u8,
	pub weight: u8,
	pub hit: u8,
	pub critical: u8,
}
impl GamedataArray for ItemRefineData {}

#[unity::class("App", "ItemEvolveData")]
pub struct ItemEvolveData {
	pub parent: StructDataArrayFields,
	pub iid: &'static Il2CppString,
	pub iron: u16,
	pub steel: u16,
	pub silver: u16,
	pub price: u16,
	pub refine_level: u8,
}

impl ItemEvolveData {
	#[unity::class_method(0)] pub fn load(); // Offset: 0x203DA60 Flags: 0
	#[unity::class_method(1)] pub fn try_get_from_iid(iid: &Il2CppString) -> Option<&'static List<&'static ItemEvolveData>>; // Offset: 0x203DB10 Flags: 0
	#[unity::class_method(2)] pub fn try_get_from_item(item: &ItemData) -> Option<&'static List<&'static ItemEvolveData>>; // Offset: 0x203DC80 Flags: 0
	#[unity::class_method(3)] pub fn iid2_eid(iid: &Il2CppString) -> &'static Il2CppString; // Offset: 0x203DC00 Flags: 0
	#[unity::class_method(17)] pub fn get_flag_name(&self) -> &'static Il2CppString; // Offset: 0x203DEE0 Flags: 0
	#[unity::class_method(18)] pub fn regist_global_flags(); // Offset: 0x203DFD0 Flags: 0
	#[unity::class_method(19)] pub fn is_once_evolved(&self) -> bool; // Offset: 0x203E1B0 Flags: 0
	#[unity::class_method(20)] pub fn set_evolved(&self, evolved: bool); // Offset: 0x203E2A0 Flags: 0
	#[unity::class_method(21)] pub fn ctor(&self); // Offset: 0x203E5B0 Flags: 0
}
impl GamedataArray for ItemEvolveData  {}

#[unity::class("App", "InteractData")]
pub struct InteractData {
	pub parent: StructBaseFields,
	pub kind: &'static Il2CppString,
	pub flag: &'static mut BitField32,
}

impl Gamedata for InteractData {}