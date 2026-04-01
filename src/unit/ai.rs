use num_derive::FromPrimitive;
use unity::il2cpp::object::Array;
use unity::prelude::*;
use crate::bit::BitField32Methods;
use crate::gamedata::ai::AIValue;
use crate::gamedata::WeaponMask;
use super::*;
#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum UnitAIVersusTypes {
    NotMove = 0, // Attr: 17
    Defense = 1, // Attr: 17
    Rush = 2, // Attr: 17
    ResponseA = 3, // Attr: 17
    ResponseB = 4, // Attr: 17
    ResponseC = 5, // Attr: 17
    Num = 6, // Attr: 17
}

#[unity::class("App", "UnitAI")]
pub struct UnitAI {
    pub flag: &'static mut UnitAIFlag,
    pub band: u8,
    pub active: u8,
    pub priority: u8,
    pub heal_rate_a: u8,
    pub heal_rate_b: u8,
    pub battle_rate_type: u8,
    pub prohibit_engage_attack: u8,
    pub prohibit_rod: u8,
    pub prohibit_overlap: u8,
    pub rerewarp_count: u8,
    pub rerewarp_count_max: u8,
    pub rerewarp_last_x: u8,
    pub rerewarp_last_z: u8,
    pub rerwarp_event: Option<&'static Il2CppString>,
    pub random_flag: &'static mut WeaponMask,
    pub move_limit: &'static UnitAIMoveLimitRange,
    pub vs_type: u8,
    pub bullet_pattern: u8,
    pub sequence: &'static Array<&'static Il2CppString>,
    pub value: &'static Array<&'static mut AIValue>,
    pub unit: &'static Unit,
    pub vs_think: i32,
}

#[unity::class("", "MoveLimitRange")]
#[nested_from_type(UnitAI)]
pub struct UnitAIMoveLimitRange {
    pub m_type: u8,
    pub x: i8,
    pub z: i8,
    pub w: i8,
    pub h: i8,
}

impl UnitAI {
    pub fn set_sequence<'a>(&self, order: i32, name: impl Into<&'a Il2CppString>) { self.set_sequence_(order, name.into()) }
    #[unity::class_method(3)] pub fn set_active(&self, value: u8); // Offset: 0x1F5F2D0 Flags: 0
    #[unity::class_method(27)] pub fn set_versus_type(&self, value: UnitAIVersusTypes); // Offset: 0x1F5F4C0 Flags: 0
    #[unity::class_method(35)] pub fn set_flag(&self, flag: i32); // Offset: 0x1F5F540 Flags: 0
    #[unity::class_method(40)] pub fn set_sequence_(&self, order: i32, name: &Il2CppString); // Offset: 0x1F5F660 Flags: 0
    #[unity::class_method(42)] pub fn get_value(&self, order: i32, index: i32) -> &'static AIValue; // Offset: 0x1F5F880 Flags: 0
    #[unity::class_method(43)] pub fn set_value(&self, order: i32, index: i32, value: i32); // Offset: 0x1F5F8D0 Flags: 0
    #[unity::class_method(45)] pub fn set_value_str(&self, order: i32, index: i32, str: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1F5FA70 Flags: 0
    #[unity::class_method(56)] pub fn setup_versus_ai(&self); // Offset: 0x1F60810 Flags: 0

}
#[unity::class("", "Flag")]
#[nested_from_type(UnitAI)]
pub struct UnitAIFlag { pub value: i32, }
impl BitField32Methods for UnitAIFlag {}

#[allow(non_upper_case_globals)]
impl UnitAIFlag {
    pub const Ec_Attack: i32 = 1;
    pub const Ec_AttackLongRange: i32 = 2;
    pub const Hc_Vulnerary: i32 = 4;
    pub const Hc_Terrain: i32 = 8;
    pub const Rc_Deactivate: i32 = 16;
    pub const EnchantWeaponDone: i32 = 32;
    pub const AllowCrossfire: i32 = 64;
    pub const ThinkNoMove: i32 = 256;
    pub const RejectPower0Attack: i32 = 512;
    pub const MoveBreak: i32 = 1024;
    pub const MoveThrough: i32 = 2048;
    pub const MoveSlow: i32 = 4096;
    pub const MoveWithAttack: i32 = 8192;
    pub const ThinkBreak: i32 = 16384;
    pub const ThinkChain: i32 = 32768;
    pub const EquipShortAfterLongRange: i32 = 65536;
    pub const BandActivation: i32 = 131072;
    pub const BandActivationMove: i32 = 262144;
    pub const BandActivationAttacked: i32 = 524288;
    pub const AskHealA: i32 = 1048576;
    pub const AskHealB: i32 = 2097152;
    pub const Idle: i32 = 4194304;
    pub const MoveoverFailed: i32 = 8388608;
    pub const TargetToAttack: i32 = 16777216;
    pub const TargetToHeal: i32 = 33554432;
    pub const TargetToInterference: i32 = 67108864;
    pub const TargetToCannon: i32 = 134217728;
    pub const DoneAskHealB: i32 = 268435456;
    pub const EngageAttackOnce: i32 = 536870912;
    pub const EngageAttackOnceDone: i32 = 1073741824;
    pub const MagicShieldOnceDone: i32 = -1073741824;
    pub const MaskTarget: i32 = 251658240;
    pub const MaskThink: i32 = 535822336;
    pub const MaskBandActivation: i32 = 917504;
}