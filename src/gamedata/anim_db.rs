use super::*;

#[unity::class("Combat", "AnimSetDB")]
pub struct AnimSetDB {
    pub parent: StructBaseFields,
    pub name: &'static Il2CppString, // Offset 0x20, Attr: 1
    pub attack1: Option<&'static Il2CppString>, // Offset 0x28, Attr: 1
    pub attack2: Option<&'static Il2CppString>, // Offset 0x30, Attr: 1
    pub attack3: Option<&'static Il2CppString>, // Offset 0x38, Attr: 1
    pub attack4: Option<&'static Il2CppString>, // Offset 0x40, Attr: 1
    pub attack5: Option<&'static Il2CppString>, // Offset 0x48, Attr: 1
    pub attack_c: Option<&'static Il2CppString>, // Offset 0x50, Attr: 1
    pub attack_t: Option<&'static Il2CppString>, // Offset 0x58, Attr: 1
    pub damage_high: Option<&'static Il2CppString>, // Offset 0x60, Attr: 1
    pub damage_mid_b: Option<&'static Il2CppString>, // Offset 0x68, Attr: 1
    pub damage_mid_du: Option<&'static Il2CppString>, // Offset 0x70, Attr: 1
    pub damage_mid_ud: Option<&'static Il2CppString>, // Offset 0x78, Attr: 1
    pub die_b: Option<&'static Il2CppString>, // Offset 0x80, Attr: 1
    pub die_l: Option<&'static Il2CppString>, // Offset 0x88, Attr: 1
    pub die_r: Option<&'static Il2CppString>, // Offset 0x90, Attr: 1
    pub dive: Option<&'static Il2CppString>, // Offset 0x98, Attr: 1
    pub engage1: Option<&'static Il2CppString>, // Offset 0xA0, Attr: 1
    pub engage2: Option<&'static Il2CppString>, // Offset 0xA8, Attr: 1
    pub engage3: Option<&'static Il2CppString>, // Offset 0xB0, Attr: 1
    pub evasion_b: Option<&'static Il2CppString>, // Offset 0xB8, Attr: 1
    pub evasion_l: Option<&'static Il2CppString>, // Offset 0xC0, Attr: 1
    pub evasion_r: Option<&'static Il2CppString>, // Offset 0xC8, Attr: 1
    pub guard: Option<&'static Il2CppString>, // Offset 0xD0, Attr: 1
    pub hovering_loop: Option<&'static Il2CppString>, // Offset 0xD8, Attr: 1
    pub idle_dying: Option<&'static Il2CppString>, // Offset 0xE0, Attr: 1
    pub idle_normal: Option<&'static Il2CppString>, // Offset 0xE8, Attr: 1
    pub parry_l: Option<&'static Il2CppString>, // Offset 0xF0, Attr: 1
    pub parry_r: Option<&'static Il2CppString>, // Offset 0xF8, Attr: 1
    pub ready: Option<&'static Il2CppString>, // Offset 0x100, Attr: 1
    pub relax_loop: Option<&'static Il2CppString>, // Offset 0x108, Attr: 1
    pub repelled: Option<&'static Il2CppString>, // Offset 0x110, Attr: 1
    pub run_loop: Option<&'static Il2CppString>, // Offset 0x118, Attr: 1
    pub run_start: Option<&'static Il2CppString>, // Offset 0x120, Attr: 1
    pub special1: Option<&'static Il2CppString>, // Offset 0x128, Attr: 1
    pub start: Option<&'static Il2CppString>, // Offset 0x130, Attr: 1
    pub win: Option<&'static Il2CppString>, // Offset 0x138, Attr: 1
    pub win_loop: Option<&'static Il2CppString>, // Offset 0x140, Attr: 1
}
impl Gamedata for AnimSetDB {}