use unity::engine::Color;
use unity::il2cpp::object::Array;
use unity::prelude::*;

use crate::gamedata::{Gamedata, StructBaseFields, skill::SkillArray};
use crate::unit::CapabilitySbyte;

#[unity::class("App", "RingData")]
pub struct RingData {
    pub parent: StructBaseFields,
    pub rid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub gid: Option<&'static Il2CppString>,
    ring_model: &'static Il2CppString,
    pub kind: i32,
    pub rank: i32,
    pub icon: &'static Il2CppString,
    pub enhance: &'static mut CapabilitySbyte,
    equip_sids: Option<&'static Array<&'static Il2CppString>>, // Offset 0x60, Attr: 1
    equip_skills: &'static SkillArray, // Offset 0x68, Attr: 1
    is_single_rank: bool, // Offset 0x70, Attr: 1
    jewel_color_r: u8, // Offset 0x71, Attr: 1
    jewel_color_g: u8, // Offset 0x72, Attr: 1
    jewel_color_b: u8, // Offset 0x73, Attr: 1
    rim_color_r: u8, // Offset 0x74, Attr: 1
    rim_color_g: u8, // Offset 0x75, Attr: 1
    rim_color_b: u8, // Offset 0x76, Attr: 1
    m_group: &'static Il2CppString, // Offset 0x78, Attr: 1
    m_flag_name: &'static Il2CppString, // Offset 0x80, Attr: 1
    pub jewel_color: Color, // Offset 0x88, Attr: 1
    pub rim_color: Color, // Offset 0x98, Attr: 1
}

impl Gamedata for RingData {}

impl RingData {
    #[unity::class_method(23)] pub fn get_equip_skills(&self) -> &'static SkillArray; // Offset: 0x24246F0 Flags: 0
    #[unity::class_method(24)] pub fn set_equip_skills(&self, value: &SkillArray); // Offset: 0x2424700 Flags: 0
    pub fn get_pool_ring_stock(&self) -> i32 { crate::unit::UnitRingPool::get_all_stock_count(self) }
}
