pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use crate::bit::BitField32Methods;
use crate::gamedata::god::GodData;
use super::{skill::*, WeaponMask, StructBaseFields, Gamedata, JobData};
use crate::unit::{Capability, CapabilitySbyte, Gender};

#[unity::class("App", "PersonData")]
pub struct PersonData {
    pub parent: StructBaseFields,
    pub pid: &'static Il2CppString,
    pub name: Option<&'static Il2CppString>,
    pub jid: Option<&'static Il2CppString>,
    pub fid: Option<&'static Il2CppString>,
    pub aid: Option<&'static Il2CppString>,
    pub help: Option<&'static Il2CppString>,
    pub die: Option<&'static Il2CppString>,
    pub belong: Option<&'static Il2CppString>,
    pub unit_icon_id: Option<&'static Il2CppString>,
    pub age: i16,
    pub birth_month: u8,
    pub birth_day: u8,
    pub gender: i32,
    pub level: i8,
    pub internal_level: i8,
    pub auto_grow_offset_n: i8,
    pub auto_grow_offset_h: i8,
    pub auto_grow_offset_l: i8,
    pub asset_force: i32,
    pub support_category: Option<&'static Il2CppString>,
    pub skill_point: i32,
    pub bmap_size: u8,
    pub items: Option<&'static Array<&'static Il2CppString>>,
    pub drop_item: Option<&'static Il2CppString>,
    pub drop_ratio: f32,
    pub flag: &'static PersonDataFlag,
    pub aptitude: &'static WeaponMask,
    pub sub_aptitude: &'static WeaponMask,
    pub offset_n: &'static mut CapabilitySbyte,
    pub offset_h: &'static mut CapabilitySbyte,
    pub offset_l: &'static mut CapabilitySbyte,
    pub limit: &'static mut CapabilitySbyte,
    pub grow: &'static mut Capability,
    pub common_sids: Option<&'static Array<&'static Il2CppString>>,
    pub normal_sids: Option<&'static Array<&'static Il2CppString>>,
    pub hard_sids: Option<&'static Array<&'static Il2CppString>>,
    pub lunatic_sids: Option<&'static Array<&'static Il2CppString>>,
    pub engage_sid: Option<&'static Il2CppString>,
    pub talk_pause_delay_min: f32,
    pub talk_pause_delay_max: f32,
    pub talk_pause_speed: f32,
    pub combat_bgm: Option<&'static Il2CppString>,
    pub ascii_name: Option<&'static Il2CppString>,
    pub link_god: Option<&'static GodData>,
    pub attrs: i32,
    pub exist_die_cid: Option<&'static Il2CppString>,
    pub exist_die_timing: i32,
    pub hometown: i32,
    pub net_ranking_index: u8,
    pub not_lvl_up_talk_pids: Option<&'static Array<&'static Il2CppString>>,
    pub summon_color: i32,
    pub summon_rank: i32,
    pub summon_god: Option<&'static Il2CppString>,
    pub summon_rate: i32,
    pub common_skills: &'static SkillArray,
    pub normal_skills: &'static SkillArray,
    pub hard_skills: &'static SkillArray,
    pub lunatic_skills: &'static SkillArray,
    pub engage_skill: Option<&'static SkillData>,
    pub face_data: &'static PersonData,
}
impl Gamedata for PersonData { }

#[unity::class("", "Flag")]
#[nested_from_type(PersonData)]
pub struct PersonDataFlag { pub value: i32, }

impl BitField32Methods for PersonDataFlag {}
#[allow(non_upper_case_globals)]
impl PersonDataFlag {
    pub const CandidateForFriend: i32 = 1;
    pub const BelongName: i32 = 2;
    pub const Talent: i32 = 4;
    pub const IgnoreJobSkillRemove: i32 = 8;
    pub const DarkWarp: i32 = 16;
    pub const DressReverse: i32 = 32;
    pub const SimpleUI: i32 = 64;
    pub const DerivedHero: i32 = 128;
    pub const SummonWarp: i32 = 256;
}

impl PersonData {
    pub fn get_offset_by_difficulty(&self) -> &'static mut CapabilitySbyte {
        match crate::gameuserdata::GameUserData::get_difficulty(false) {
            1 => { self.get_offset_h() },
            2 => { self.get_offset_l() },
            _ => { self.get_offset_n() },
        }
    }
    #[unity::class_method(1)] pub fn ctor(&self); // Offset: 0x1F259F0 Flags: 0 0
    #[unity::class_method(28)] pub fn get_level(&self) -> u8; // Offset: 0x1F25DC0 Flags: 0
    #[unity::class_method(29)] pub fn set_level(&self, value: u8); // Offset: 0x1F25DD0 Flags: 0
    #[unity::class_method(30)] pub fn get_internal_level(&self) -> i8; // Offset: 0x1F25DE0 Flags: 0
    #[unity::class_method(31)] pub fn set_internal_level(&self, value: i8); // Offset: 0x1F25DF0 Flags: 0
    #[unity::class_method(38)] pub fn get_asset_force(&self) -> i32; // Offset: 0x1F25E60 Flags: 0
    #[unity::class_method(42)] pub fn get_sp(&self) -> i32; // Offset: 0x1F25EA0 Flags: 0
    #[unity::class_method(43)] pub fn set_sp(&self, value: i32); // Offset: 0x1F25EB0 Flags: 0
    #[unity::class_method(44)] pub fn get_bmap_size(&self) -> u8; // Offset: 0x1F25EC0 Flags: 0
    #[unity::class_method(52)] pub fn get_flag(&self) -> &'static mut PersonDataFlag; // Offset: 0x1F25F40 Flags: 0
    #[unity::class_method(64)] pub fn get_limit(&self) -> &'static mut CapabilitySbyte; // Offset: 0x1F26000 Flags: 0
    #[unity::class_method(66)] pub fn get_grow(&self) -> &'static mut Capability; // Offset: 0x1F26020 Flags: 0
    #[unity::class_method(68)] pub fn get_common_sids(&self) -> Option<&'static mut Array<&'static mut Il2CppString>>; // Offset: 0x1F26050 Flags: 0
    #[unity::class_method(69)] pub fn set_common_sids(&self, value: &Array<&Il2CppString>); // Offset: 0x1F26050 Flags: 0
    #[unity::class_method(26)] pub fn get_gender(&self) -> i32; // Offset: 0x1F25DA0 Flags: 0
    #[unity::class_method(26)] pub fn get_gender2(&self) -> Gender; // Offset: 0x1F25DA0 Flags: 0
    #[unity::class_method(27)] pub fn set_gender(&self, value: Gender); // Offset: 0x1F25DB0 Flags: 0
    #[unity::class_method(58)] pub fn get_offset_n(&self) -> &'static mut CapabilitySbyte; // Offset: 0x1F25FA0 Flags: 0
    #[unity::class_method(60)] pub fn get_offset_h(&self) -> &'static mut CapabilitySbyte; // Offset: 0x1F25FC0 Flags: 0
    #[unity::class_method(62)] pub fn get_offset_l(&self) -> &'static mut CapabilitySbyte; // Offset: 0x1F25FE0 Flags: 0
    #[unity::class_method(86)] pub fn get_ascii_name(&self) -> Option<&'static Il2CppString>; // Offset: 0x1F26160 Flags: 0
    #[unity::class_method(87)] pub fn set_ascii_name(&self, value: &Il2CppString); // Offset: 0x1F26170 Flags: 0
    #[unity::class_method(88)] pub fn get_link_god(&self) -> Option<&'static GodData>; // Offset: 0x1F26180 Flags: 0
    #[unity::class_method(89)] pub fn set_link_god(&self, value: Option<&GodData>); // Offset: 0x1F26190 Flags: 0
    #[unity::class_method(118)] pub fn get_name(&self) -> &'static mut Il2CppString; // Offset: 0x1F29C20 Flags: 0
    #[unity::class_method(119)] pub fn get_job(&self) -> Option<&'static JobData>; // Offset: 0x1F29E30 Flags: 0
    #[unity::class_method(112)] pub fn get_dress_gender(&self) -> Gender; // Offset: 0x1F266A0 Flags: 0
    #[unity::class_method(120)] pub fn get_mask_skill(&self) -> &'static SkillArray; // Offset: 0x1F29ED0 Flags: 0
    #[unity::class_method(121)] pub fn is_hero(&self) -> bool; // Offset: 0x1F2A0B0 Flags: 0
    #[unity::class_method(133)] pub fn get_common_skills(&self) -> &'static mut SkillArray; // Offset: 0x1F2A6F0 Flags: 0
    #[unity::class_method(134)] pub fn set_common_skills(&self, value: &SkillArray); // Offset: 0x1F2A700 Flags: 0
    #[unity::class_method(135)] pub fn get_normal_skills(&self) -> &'static mut SkillArray; // Offset: 0x1F2A710 Flags: 0
    #[unity::class_method(136)] pub fn set_normal_skills(&self, value: &SkillArray); // Offset: 0x1F2A720 Flags: 0
    #[unity::class_method(137)] pub fn get_hard_skills(&self) -> &'static mut SkillArray; // Offset: 0x1F2A730 Flags: 0
    #[unity::class_method(138)] pub fn set_hard_skills(&self, value: &SkillArray); // Offset: 0x1F2A740 Flags: 0
    #[unity::class_method(139)] pub fn get_lunatic_skills(&self) -> &'static mut SkillArray; // Offset: 0x1F2A750 Flags: 0
    #[unity::class_method(140)] pub fn set_lunatic_skills(&self, value: &SkillArray); // Offset: 0x1F2A760 Flags: 0
    #[unity::class_method(141)] pub fn get_engage_skill(&self) -> Option<&'static SkillData>; // Offset: 0x1F2A770 Flags: 0
    #[unity::class_method(142)] pub fn set_engage_skill(&self, value: Option<&SkillData>); // Offset: 0x1F2A780 Flags: 0
    #[unity::class_method(143)] pub fn get_face_data(&self) -> &'static PersonData; // Offset: 0x1F2A790 Flags: 0
    #[unity::class_method(144)] pub fn set_face_data(&self, value: &PersonData); // Offset: 0x1F2A7A0 Flags: 0
    #[unity::class_method(131)] pub fn is_veyre(person: &PersonData) -> bool; // Offset: 0x1F2A590 Flags: 0
}