use num_derive::FromPrimitive;
pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::system::List;
use crate::gamedata::{Gamedata, StructBaseFields, WeaponMask};
use crate::gamedata::item::{ItemDataKinds, WeaponLevelKind};
use crate::gamedata::skill::{SkillArray, SkillDataAttrs};
use crate::unit::{Capability, CapabilitySbyte};

#[unity::class("App", "JobData")]
pub struct JobData {
    pub parent: StructBaseFields, //0x0
    pub jid: &'static Il2CppString, //0x10
    pub name: &'static Il2CppString, //0x18
    pub aid: &'static Il2CppString, //0x20
    pub help: &'static Il2CppString, //0x28
    pub unit_icon_id_m : Option<&'static Il2CppString>, //0x30
    pub unit_icon_id_f : Option<&'static Il2CppString>, //0x38
    pub unit_icon_weapon_id: &'static Il2CppString, //0x40
    pub rank: i32,  //0x48
    pub style_name: Option<&'static Il2CppString>, //0x50
    pub move_type: i32, //0x58
    pub step_frame: i32,    // 0x5c
    pub max_level: u8,  //0x60
    pub internal_level: i8, //0x61
    pub sort: u16,  //0x62
    pub flag: &'static mut JobDataFlag, //0x68
    cc_item: &'static Array<&'static Il2CppString>, //0x70
    unique_item: &'static Array<&'static Il2CppString>, //0x78
    pub style: i32, //0x80
    pub weapons: &'static mut Array<i8>,    // 0x88
    pub max_weapon_level: &'static mut Array<&'static Il2CppString>,    //0x90
    pub weapon_levels: &'static mut Array<i32>, //0x98
    pub weapon_mask_plus: &'static WeaponMask,  //0xa0
    pub high_jobs: &'static Array<&'static Il2CppString>,   //0xa8
    pub low_job: Option<&'static Il2CppString>, //0xb0
    pub base: &'static Capability,  //0xb8
    pub limit: &'static Capability, //0xc0
    pub base_grow: &'static CapabilitySbyte,    //0xc8
    pub diff_grow: &'static CapabilitySbyte,    //0xd0
    pub diff_grow_normal: &'static CapabilitySbyte, //0xd8
    pub diff_grow_hard: &'static CapabilitySbyte,   //0xe0
    pub diff_grow_lunatic: &'static CapabilitySbyte,    //0xe8
    pub short_name: &'static Il2CppString,  //0xf0
    pub skills: Option<&'static Array<&'static Il2CppString>>,  //0xf8
    pub learn_skill: Option<&'static Il2CppString>, // 0x100
    pub lunatic_skill: Option<&'static Il2CppString>, //0x108
    pub attrs: i32, //0x110
    pub mask_skills: &'static SkillArray,   //0x118
}
impl Gamedata for JobData { }

#[unity::class("App", "BattleStyle")]
pub struct BattleStyles{
    pub parent: StructBaseFields,
    pub style: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub skills: &'static Array<&'static Il2CppString>,
}
impl Gamedata for BattleStyles {}

impl BattleStyles{
    #[unity::class_method(1)] pub fn get_style(name: Option<&Il2CppString>) -> BattleStyleTypes; // Offset: 0x1E92EA0 Flags: 0
    #[unity::class_method(2)] pub fn get_name(style: BattleStyleTypes) -> &'static Il2CppString; // Offset: 0x1E92FA0 Flags: 0
    #[unity::class_method(3)] pub fn get_skills(style: BattleStyleTypes) -> Option<&'static Array<&'static Il2CppString>>; // Offset: 0x1E93030 Flags: 0
    #[unity::class_method(3)] pub fn get_skills2(style: i32) -> Option<&'static Array<&'static Il2CppString>>; // Offset: 0x1E93030 Flags: 0
    #[unity::class_method(4)] pub fn get_style_name(style: BattleStyleTypes) -> &'static Il2CppString; // Offset: 0x1E930D0 Flags: 0
}

#[unity::class("", "FlagField")]
#[nested_from_type(JobData)]
pub struct JobDataFlag {  pub value: i32, }

impl JobData {
    #[unity::class_method(2)] pub fn ctor(&self); // Offset: 0x2053A20 Flags: 0
    #[unity::class_method(7)] pub fn get_aid(&self) -> &'static Il2CppString; // Offset: 0x2053D60 Flags: 0
    #[unity::class_method(8)] pub fn set_aid(&self, value: &Il2CppString); // Offset: 0x2053D70 Flags: 0
    #[unity::class_method(9)] pub fn get_help(&self) -> &'static Il2CppString; // Offset: 0x2053D80 Flags: 0
    #[unity::class_method(10)] pub fn set_help(&self, value: &Il2CppString); // Offset: 0x2053D90 Flags: 0
    #[unity::class_method(17)] pub fn get_rank(&self) -> i8; // Offset: 0x2053E00 Flags: 0
    #[unity::class_method(18)] pub fn set_rank(&self, value: i8); // Offset: 0x2053E10 Flags: 0
    #[unity::class_method(19)] pub fn get_style_name(&self) -> &'static Il2CppString; // Offset: 0x2053E20 Flags: 0
    #[unity::class_method(20)] pub fn set_style_name(&self, value: &Il2CppString); // Offset: 0x2053E30 Flags: 0
    #[unity::class_method(21)] pub fn get_move_type(&self) -> JobDataMoveTypes; // Offset: 0x2053E40 Flags: 0
    #[unity::class_method(22)] pub fn set_move_type(&self, value: JobDataMoveTypes); // Offset: 0x2053E50 Flags: 0
    #[unity::class_method(23)] pub fn get_step_frame(&self) -> i32; // Offset: 0x2053E60 Flags: 0
    #[unity::class_method(24)] pub fn set_step_frame(&self, value: i32); // Offset: 0x2053E70 Flags: 0
    #[unity::class_method(25)] pub fn get_max_level(&self) -> u8; // Offset: 0x2053E80 Flags: 0
    #[unity::class_method(26)] pub fn set_max_level(&self, value: u8); // Offset: 0x2053E90 Flags: 0
    #[unity::class_method(27)] pub fn get_internal_level(&self) -> i8; // Offset: 0x2053EA0 Flags: 0
    #[unity::class_method(28)] pub fn set_internal_level(&self, value: i8); // Offset: 0x2053EB0 Flags: 0
    #[unity::class_method(29)] pub fn get_sort(&self) -> u16; // Offset: 0x2053EC0 Flags: 0
    #[unity::class_method(30)] pub fn set_sort(&self, value: u16); // Offset: 0x2053ED0 Flags: 0
    #[unity::class_method(31)] pub fn get_flag(&self) -> &'static JobDataFlag; // Offset: 0x2053EE0 Flags: 0
    #[unity::class_method(32)] pub fn set_flag(&self, value: &JobDataFlag); // Offset: 0x2053EF0 Flags: 0
    #[unity::class_method(37)] pub fn get_style(&self) -> BattleStyleTypes; // Offset: 0x2053F40 Flags: 0
    #[unity::class_method(38)] pub fn set_style(&self, value: BattleStyleTypes); // Offset: 0x2053F50 Flags: 0
    #[unity::class_method(81)] pub fn get_high_job1(&self) -> Option<&'static Il2CppString>; // Offset: 0x2054980 Flags: 0
    #[unity::class_method(82)] pub fn set_high_job1(&self, value: &Il2CppString); // Offset: 0x20549B0 Flags: 0
    #[unity::class_method(83)] pub fn get_high_job2(&self) -> Option<&'static Il2CppString>; // Offset: 0x2054A20 Flags: 0
    #[unity::class_method(84)] pub fn set_high_job2(&self, value: &Il2CppString); // Offset: 0x2054A50 Flags: 0
    #[unity::class_method(85)] pub fn get_low_job(&self) -> &'static Il2CppString; // Offset: 0x2054AC0 Flags: 0
    #[unity::class_method(86)] pub fn set_low_job(&self, value: &Il2CppString); // Offset: 0x2054AD0 Flags: 0
    #[unity::class_method(87)] pub fn get_base(&self) -> &'static Capability; // Offset: 0x2054AE0 Flags: 0
    #[unity::class_method(88)] pub fn set_base(&self, value: &Capability); // Offset: 0x2054AF0 Flags: 0
    #[unity::class_method(89)] pub fn get_limit(&self) -> &'static mut Capability; // Offset: 0x2054B00 Flags: 0
    #[unity::class_method(90)] pub fn set_limit(&self, value: &Capability); // Offset: 0x2054B10 Flags: 0
    #[unity::class_method(91)] pub fn get_base_grow(&self) -> &'static mut Capability; // Offset: 0x2054B20 Flags: 0
    #[unity::class_method(92)] pub fn set_base_grow(&self, value: &Capability); // Offset: 0x2054B30 Flags: 0
    #[unity::class_method(93)] pub fn get_diff_grow(&self) -> &'static mut CapabilitySbyte; // Offset: 0x2054B40 Flags: 0
    #[unity::class_method(94)] pub fn set_diff_grow(&self, value: &CapabilitySbyte); // Offset: 0x2054B50 Flags: 0
    #[unity::class_method(95)] pub fn get_diff_grow_normal(&self) -> &'static mut CapabilitySbyte; // Offset: 0x2054B60 Flags: 0
    #[unity::class_method(96)] pub fn set_diff_grow_normal(&self, value: &CapabilitySbyte); // Offset: 0x2054B70 Flags: 0
    #[unity::class_method(97)] pub fn get_diff_grow_hard(&self) -> &'static mut CapabilitySbyte; // Offset: 0x2054B80 Flags: 0
    #[unity::class_method(98)] pub fn set_diff_grow_hard(&self, value: &CapabilitySbyte); // Offset: 0x2054B90 Flags: 0
    #[unity::class_method(99)] pub fn get_diff_grow_lunatic(&self) -> &'static mut CapabilitySbyte; // Offset: 0x2054BA0 Flags: 0
    #[unity::class_method(100)] pub fn set_diff_grow_lunatic(&self, value: &CapabilitySbyte); // Offset: 0x2054BB0 Flags: 0
    #[unity::class_method(101)] pub fn get_short_name(&self) -> &'static Il2CppString; // Offset: 0x2054BC0 Flags: 0
    #[unity::class_method(102)] pub fn set_short_name(&self, value: &Il2CppString); // Offset: 0x2054BD0 Flags: 0
    #[unity::class_method(105)] pub fn get_learning_skill(&self) -> Option<&'static Il2CppString>; // Offset: 0x2054C00 Flags: 0
    #[unity::class_method(106)] pub fn set_learning_skill(&self, value: Option<&Il2CppString>); // Offset: 0x2054C10 Flags: 0
    #[unity::class_method(107)] pub fn get_lunatic_skill(&self) -> Option<&'static Il2CppString>; // Offset: 0x2054C20 Flags: 0
    #[unity::class_method(108)] pub fn set_lunatic_skill(&self, value: Option<&Il2CppString>); // Offset: 0x2054C30 Flags: 0
    #[unity::class_method(109)] pub fn get_attrs(&self) -> SkillDataAttrs; // Offset: 0x2054C40 Flags: 0
    #[unity::class_method(110)] pub fn set_attrs(&self, value: SkillDataAttrs); // Offset: 0x2054C50 Flags: 0
    #[unity::class_method(111)] pub fn on_build(&self); // Offset: 0x2054C60 Flags: 0
    #[unity::class_method(112)] pub fn on_completed(&self); // Offset: 0x2054D80 Flags: 0
    #[unity::class_method(113)] pub fn get_prefixless_jid(&self) -> &'static Il2CppString; // Offset: 0x2055B60 Flags: 0
    #[unity::class_method(114)] pub fn get_name(&self) -> &'static Il2CppString; // Offset: 0x2055B70 Flags: 0
    #[unity::class_method(115)] pub fn get_unit_icon_id(&self, is_female: bool) -> Option<&'static Il2CppString>; // Offset: 0x2055CF0 Flags: 0
    #[unity::class_method(116)] pub fn is_high(&self) -> bool; // Offset: 0x2055D10 Flags: 0
    #[unity::class_method(117)] pub fn is_low(&self) -> bool; // Offset: 0x2055D20 Flags: 0
    #[unity::class_method(118)] pub fn is_fly(&self) -> bool; // Offset: 0x2055D30 Flags: 0
    #[unity::class_method(119)] pub fn is_rider(&self) -> bool; // Offset: 0x2055D40 Flags: 0
    #[unity::class_method(120)] pub fn is_download(&self) -> bool; // Offset: 0x2055D60 Flags: 0
    #[unity::class_method(121)] pub fn is_unknown(&self) -> bool; // Offset: 0x2055C20 Flags: 0
    #[unity::class_method(122)] pub fn has_high_jobs(&self) -> bool; // Offset: 0x2055D70 Flags: 0
    #[unity::class_method(123)] pub fn get_high_jobs(&self) -> &'static List<JobData>; // Offset: 0x2055E70 Flags: 0
    #[unity::class_method(124)] pub fn get_low_jobs(&self) -> &'static List<JobData>; // Offset: 0x2055FE0 Flags: 0
    #[unity::class_method(125)] pub fn is_enchant(&self) -> bool; // Offset: 0x2056240 Flags: 0
    #[unity::class_method(126)] pub fn is_gunner(&self) -> bool; // Offset: 0x20562A0 Flags: 0
    #[unity::class_method(127)] pub fn get_weapon_mask(&self, mask: &WeaponMask, selected: &WeaponMask) -> &'static WeaponMask; // Offset: 0x2056300 Flags: 0
    #[unity::class_method(128)] pub fn get_weapon_mask2(&self) -> &'static WeaponMask; // Offset: 0x20563E0 Flags: 0
    #[unity::class_method(129)] pub fn get_selectable_weapon_mask(&self, required_select_count: &mut i32) -> Option<&'static WeaponMask>; // Offset: 0x20564F0 Flags: 0
    #[unity::class_method(130)] pub fn get_equipable_weapon_kinds(&self) -> &'static Array<ItemDataKinds>; // Offset: 0x2056610 Flags: 0
    #[unity::class_method(131)] pub fn get_max_level_weapons(&self, weapon_mask: &WeaponMask, original_aptitude: &WeaponMask) -> &'static Array<ItemDataKinds>; // Offset: 0x2056830 Flags: 0
    #[unity::class_method(132)] pub fn is_equipable(&self, kind: ItemDataKinds) -> bool; // Offset: 0x2056AC0 Flags: 0
    #[unity::class_method(133)] pub fn get_max_weapon_level(&self, index: i32) -> i32; // Offset: 0x2056BF0 Flags: 0
    #[unity::class_method(133)] pub fn get_max_weapon_level2(&self, index: i32) -> WeaponLevelKind; // Offset: 0x2056BF0 Flags: 0
    #[unity::class_method(134)] pub fn get_max_weapon_level_original_apt(&self, index: i32, original_aptitude: &WeaponMask) -> WeaponLevelKind; // Offset: 0x2056C30 Flags: 0
    #[unity::class_method(134)] pub fn get_max_weapon_level_original_apt2(&self, index: i32, original_aptitude: &WeaponMask) -> i32; // Offset: 0x2056C30 Flags: 0
    #[unity::class_method(135)] pub fn get_level_plus_weapon_mask(&self) -> &'static WeaponMask; // Offset: 0x2056C90 Flags: 0
    #[unity::class_method(136)] pub fn get_learn_job_skill_level(&self) -> i32; // Offset: 0x2056CA0 Flags: 0
    #[unity::class_method(142)] pub fn get_mask_skill(&self) -> &'static SkillArray; // Offset: 0x2057070 Flags: 0
    #[unity::class_method(143)] pub fn set_mask_skill(&self, value: &SkillArray); // Offset: 0x2057080 Flags: 0
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum BattleStyleTypes {
    None = 0, // Attr: 17
    Cooperation = 1, // Attr: 17
    Horse = 2, // Attr: 17
    Covert = 3, // Attr: 17
    Heavy = 4, // Attr: 17
    Fly = 5, // Attr: 17
    Magic = 6, // Attr: 17
    Prana = 7, // Attr: 17
    Dragon = 8, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum JobDataMoveTypes {
    None = 0, // Attr: 17
    Foot = 1, // Attr: 17
    Horse = 2, // Attr: 17
    Fly = 3, // Attr: 17
    Dragon = 4, // Attr: 17
    Pad = 5, // Attr: 17
    Num = 6, // Attr: 17
}