//! Types and methods to query the state of [`Unit`](crate::gamedata::unit::Unit)s in battle.
use bitflags::bitflags;
use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::ListFields;
use unity::system::action::Action;
use crate::battle::{BattleCalculator, BattleSideType};
use crate::force::Force;
use crate::unityengine::{Transform, UnityComponent, UnityObject};
use crate::gamedata::job::JobData;
use crate::gamedata::person::PersonData;
use crate::unit::{Unit, UnitItem};

mod character;
pub use character::*;

#[unity::class("Combat", "Kaneko")] pub struct Kaneko { }

impl Kaneko {
    #[unity::class_method(0)] pub fn find_in_children(this: &Transform, name: &Il2CppString) -> Option<&'static Transform>; // Offset: 0x205B910 Flags: 0
    #[unity::class_method(3)] pub fn ancestor(t: &Transform) -> &'static Transform; // Offset: 0x205C1E0 Flags: 0
    #[unity::class_method(6)] pub fn destroy<T>(obj: &T) where T: UnityObject; // Offset: 0x205C2A0 Flags: 0
    #[unity::class_method(7)] pub fn destroy_component<T>(obj: &T) where T: UnityComponent; // Offset: 0x205C360 Flags: 0
    #[unity::class_method(8)] pub fn is_destroyed<T>(obj: &T) -> bool where T: UnityObject; // Offset: 0x205C420 Flags: 0
}

#[unity::class("Combat", "Character")]
pub struct Character {
    monobehaviour_fields: [u8; 0x8],
    side: i32,
    chain_id: i32,
    prefetch: *const u8,
    effect: *const u8,
    observable: *const u8,
    idle_smb: *const u8,
    fsm: *const u8,
    brain: *const u8,
    pub game_status: Option<&'static mut CharacterGameStatus>,
    pub is_done_setup: bool,
    head_look_at_ik: *const u8,
    body_look_at_ik: *const u8,
    enemy_side: i32,
    ground_level: f32,
    world_hit_dir: [f32; 3],
    rush_dir: *const u8,
    combat_start_fade_disposable: *const u8,
    body_animator: *const u8,
    ride_animator: *const u8,
    face_animator: *const u8,
    play__idle__________: i32,
    ________________: f32,
    constant_speed_playback: bool,
    playing_hash: i32,
    playing_store: *const u8,
    pub playing_event: *const u8,
    play_end_world_pos: *const u8,
    dither_fade: *const u8,
    cached_dither_fade: bool,
    material_engage: *const u8,
    cached_material_engage: bool,
    signal: *const u8,
    lying: *const u8,
    joint: *const u8,
    cached_joint: bool,
    proportion: *const u8,
    cached_proportion: bool,
    config: *const u8,
    // too lazy to do the rest for now
}

impl Character {
    #[unity::class_method(0)] pub fn get_side(&self) -> i32; // Offset: 0x2AFC5A0 Flags: 0
    #[unity::class_method(18)] pub fn get_phase(&self) -> &'static Phase; // Offset: 0x2AFCB70 Flags: 0
    #[unity::class_method(19)] pub fn get_game_status(&self) -> &'static mut CharacterGameStatus; // Offset: 0x2AFCCC0 Flags: 0
    #[unity::class_method(24)] pub fn set_is_visible(&self, value: bool); // Offset: 0x2AFCE80 Flags: 0
    #[unity::class_method(100)] pub fn play_facial(&self, state_name: &Il2CppString); // Offset: 0x2B02AC0 Flags: 0
    #[unity::class_method(108)] pub fn get_joint(&self) -> &'static mut CharacterJoint; // Offset: 0x2AFD0A0 Flags: 0
    #[unity::class_method(109)] pub fn get_proportion(&self) -> &'static mut CharacterProportion; // Offset: 0x2B02A10 Flags: 0
    #[unity::class_method(116)] pub fn get_builder(&self) -> &'static mut CharacterBuilder; // Offset: 0x2AFC680 Flags: 0
    #[unity::class_method(124)] pub fn call_on_setup_done(&self, func: &Action); // Offset: 0x2AFE8E0 Flags: 0
    #[unity::class_method(125)] pub fn call_on_setup_done2<T>(&self, component: &T, my_start: Option<&Action>, my_update: Option<&Action>, my_late_update: Option<&Action>) where T: UnityComponent; // Offset: 0x2AFE330 Flags: 0
}
impl UnityComponent for Character {}
impl UnityObject for Character {}


#[unity::class("Combat", "CharacterSound")]
pub struct CharacterSound { }

#[unity::class("Combat", "Phase")]
pub struct Phase {
    pub previous: Option<&'static mut Phase>,
    pub next: Option<&'static mut Phase>,
    pub kind: i32,
    pub hit_type: HitType,
    pub detail: Detail,
    pub attack_side: i32,
    pub attack_hash: i32,
    pub damage_hash: i32,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Bitflags for the type of hit. The combo flags (which are non-power of two) are provided by the game and included here for completeness.
    pub struct HitType: i32 {
        const Critical = 1;
        const Miss = 2;
        const Guard = 4;
        const Hit = 8;
        const Parry = 16;
        const Knockoff = 64;
        const Heal = 128;
        const ChainGuard = 256;
        const DualGuard = 512;
        const HitStop = 268;
        const GuardType = 260;
        const MissType = 82;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Bitflags for the detail of the hit. The combo flags (which are non-power of two) are provided by the game and included here for completeness.
    pub struct Detail: i32 {
        const FirstAttack = 1;
        const LastAttack = 2;
        const Rush = 4;
        const Efficacy = 8;
        const EngageAttack = 16;
        const Break = 32;
        const Smash = 64;
        const StandingDie = 128;
        const DamageDisplayed = 256;
        const ChainAtk = 4096;
        const ChainAtk2 = 8192;
        const ChainGrd1 = 16384;
        const ChainGrd2 = 32768;
        const ChainGrd3 = 65536;
        const ChainGrd4 = 131072;
        const ChainGrd = 245760;
    }
}


#[unity::class("Combat", "CharacterGameStatus")]
pub struct CharacterGameStatus {
    pub appearance: &'static mut CharacterAppearance,
    pub emblem_identifier: Option<&'static Il2CppString>,
    pub side: i32,
    pub stun: bool,
    pub unit: Option<&'static Unit>,
    pub person: Option<&'static PersonData>,
    pub job: Option<&'static JobData>,
    pub force: Option<&'static Force>,
    pub name: Option<&'static Il2CppString>,
    pub max_hp: i32,
    pub hp: i32,
    pub max_stun: i32,
    pub stun_value: i32,
    pub engage_count: i32,
    pub map_x: i32,
    pub map_y: i32,
    pub battle_x: i32,
    pub battle_y: i32,
    pub weapon: Option<&'static UnitItem>,
    pub engage_style: i32,
    // too lazy to do the rest for now
}
impl CharacterGameStatus {
    #[unity::class_method(4)] pub fn set_emblem_identifier(&self, value: &Il2CppString); // Offset: 0x27DEF80 Flags: 0
    #[unity::class_method(6)] pub fn get_unit(&self) -> Option<&'static mut Unit>; // Offset: 0x27DEFA0 Flags: 0
    #[unity::class_method(63)] pub fn import(&self, side_: i32, calc: &BattleCalculator, side_type: BattleSideType, map_distance: i32); // Offset: 0x27E0880 Flags: 0
}
#[unity::class("Combat", "SkillStack")]
pub struct SkillStack {}

impl SkillStack{
    pub fn has<'a>(&self, name: impl Into<&'a Il2CppString>) -> bool {
        unsafe { combat_skill_stack_has(self, name.into(), None) }
    }
}

#[unity::class("Combat", "PhaseArray")]
pub struct PhaseArray {
    pub parent: ListFields<Phase>,
}


#[unity::class("Combat", "CombatRecord")]
pub struct CombatRecord {
    pub is_enemy_attack: i32,
    pub combat_style: i32,
    pub calculator: &'static mut BattleCalculator,
    pub sim_calculator: &'static mut BattleCalculator,
    pub game_status: &'static mut Array<&'static mut CharacterGameStatus>,
    pub chain_atk: &'static mut Array<&'static mut CharacterGameStatus>,
    pub dragonize: &'static mut Array<&'static mut CharacterGameStatus>,
    location: u64,
    pub passive_skills: &'static mut SkillStack,
    pub phase_array: &'static mut PhaseArray,
    pub map_distance: i32,
    pub chain_attack_count: i32,
    pub finish_style: i32,
}

impl CombatRecord {
    #[unity::class_method(5)] pub fn get_calculator(&self) -> &'static BattleCalculator; // Offset: 0x2922C10 Flags: 0
    #[unity::class_method(11)] pub fn get_game_status_chain_atk(&self) -> &'static mut Array<&'static mut CharacterGameStatus>; // Offset: 0x2922C70 Flags: 0
    #[unity::class_method(13)] pub fn get_game_status_dragonize(&self) -> &'static mut Array<&'static mut CharacterGameStatus>; // Offset: 0x2922C90 Flags: 0
    #[unity::class_method(50)] pub fn import_from_game(&self, calc: &BattleCalculator, sim_calc: &BattleCalculator); // Offset: 0x2925900 Flags: 0
}


#[repr(C)]
#[derive(Debug)]
/// Used by the game to determine the sound effects to play during damage for zoomed-in combat.
pub enum DamageEffectLevel {
    Low,
    Medium,
    High,
}

#[repr(C)]
#[unity::class("Combat", "MagicSignalProcessor")]
pub struct MagicSignalProcessor {
    monobehaviour_fields: [u8; 0x8],
    pub character: &'static Character,
}

#[repr(C)]
#[derive(Debug)]
/// Describes how the magic projectile will arrive at the target.
pub enum ArrivalType {
    /// The magic projectile will fly to the target, such as fireballs and wind attacks.
    Flying,
    /// The magic projectile will arrive at the target immediately, such as lightning.
    ConstantTime,
}

#[unity::class("Combat", "MagicBulletSettings")]
pub struct MagicBulletSettings {
    home_node_name: &'static Il2CppString,
    target_node_name: &'static Il2CppString,
    float: f32,
    pub arrival_type: ArrivalType,
    move_speed: f32,
}

#[unity::class("Combat", "Magic")]
pub struct Magic<'a> {
    base: [u8; 0x28],
    pub magic_bullet_settings: &'a MagicBulletSettings,
}

#[unity::class("Combat", "MagicSignal")]
pub struct MagicSignal {
    pub level: i32,
    pub frame: f32,
    pub command: i32,
    pub prefab: *const u8,
    pub parent_name: Option<&'static Il2CppString>,
    pub connect: i32,
    pub int_parameter: i32,
    pub float_parameter: f32,
    pub string_parameter: Option<&'static Il2CppString>,
}

#[unity::class("Combat", "AnimAsset")]
pub struct AnimAsset {
    pub asset_type: i32,
    pub name: Option<&'static Il2CppString>,
    pub addr_path: Option<&'static Il2CppString>,
    other_fields: [u64; 3],
    pub hash: i32,
}
impl AnimAsset {
    pub fn new<'a>(name: impl Into<&'a Il2CppString>, hash: i32) -> &'static mut AnimAsset {
        let asset = Self::instantiate().unwrap();
        unsafe {
            anim_asset_ctor(asset, None);
            anim_asset_set_name_and_hash(asset, name.into(), hash, None);
        }
        asset
    }
}

#[unity::class("Combat", "Side")]
pub struct CombatSide {}
impl CombatSide {
    #[unity::class_method(0)] pub fn get_name(i: i32) -> &'static Il2CppString; // Offset: 0x247C130 Flags: 0
    #[unity::class_method(1)] pub fn from_name(name: &Il2CppString) -> i32; // Offset: 0x247C6E0 Flags: 0
    #[unity::class_method(2)] pub fn is_master(i: i32) -> bool; // Offset: 0x247CAD0 Flags: 0
    #[unity::class_method(3)] pub fn is_paired_grandew(i: i32) -> bool; // Offset: 0x247CAE0 Flags: 0
    #[unity::class_method(4)] pub fn is_chain(i: i32) -> bool; // Offset: 0x247CAF0 Flags: 0
    #[unity::class_method(5)] pub fn is_chain_atk(i: i32) -> bool; // Offset: 0x247CB00 Flags: 0
    #[unity::class_method(6)] pub fn is_chain_grd(i: i32) -> bool; // Offset: 0x247CB10 Flags: 0
    #[unity::class_method(7)] pub fn is_player_side(i: i32) -> bool; // Offset: 0x247CB30 Flags: 0
    #[unity::class_method(8)] pub fn is_enemy_side(i: i32) -> bool; // Offset: 0x247CB40 Flags: 0
    #[unity::class_method(9)] pub fn get_enemy(i: i32) -> i32; // Offset: 0x247CB50 Flags: 0
    #[unity::class_method(10)] pub fn get_enemy_grandew(i: i32) -> i32; // Offset: 0x247CB60 Flags: 0
    #[unity::class_method(11)] pub fn get_grandew(i: i32) -> i32; // Offset: 0x247CB70 Flags: 0
    #[unity::class_method(12)] pub fn get_master(i: i32) -> i32; // Offset: 0x247CB90 Flags: 0
    #[unity::class_method(13)] pub fn get_partner(i: i32) -> i32; // Offset: 0x247CBB0 Flags: 0
    #[unity::class_method(14)] pub fn get_mirror_side(i: i32) -> i32; // Offset: 0x247CBD0 Flags: 0
    #[unity::class_method(15)] pub fn get_enemy_chr(i: i32) -> Option<&'static Character>; // Offset: 0x247CBF0 Flags: 0
    #[unity::class_method(16)] pub fn get_grandew_chr(i: i32) -> Option<&'static Character>; // Offset: 0x247CC50 Flags: 0
    #[unity::class_method(17)] pub fn get_master_chr(i: i32) -> Option<&'static Character>; // Offset: 0x247CCE0 Flags: 0
    #[unity::class_method(18)] pub fn get_partner_chr(i: i32) ->Option<&'static Character>; // Offset: 0x247CD50 Flags: 0
    #[unity::class_method(19)] pub fn get_enemy_grandew_chr(i: i32) -> Option<&'static Character>; // Offset: 0x247CDC0 Flags: 0
    #[unity::class_method(20)] pub fn convert_from(side_type: BattleSideType, is_reversed: bool) -> i32; // Offset: 0x247CE20 Flags: 0
}

#[unity::from_offset("Combat", "AnimAsset", ".ctor")]
fn anim_asset_ctor(this: &AnimAsset, method_info: OptionalMethod);

#[unity::from_offset("Combat", "AnimAsset", "SetNameAndHash")]
fn anim_asset_set_name_and_hash(this: &AnimAsset, name: &Il2CppString, hash: i32, method_info: OptionalMethod);

// Combat.MagicSignalProcessor$$get_Magic	7101bf31a0	Combat_Magic_o * Combat.MagicSignalProcessor$$get_Magic(Combat_MagicSignalProcessor_o * __this, MethodInfo * method)	8
#[unity::from_offset("Combat", "MagicSignalProcessor", "get_Magic")]
pub fn magicsignalprocessor_get_magic(
    this: &MagicSignalProcessor,
    method_info: OptionalMethod,
) -> &Magic;

#[unity::class("UnityEngine", "AnimationEvent")]
pub struct AnimationEvent { }

#[unity::from_offset("Combat", "Phase", "get_DamageEffectLevel")]
pub fn phase_get_damage_effect_level(
    this: &Phase,
    method_info: OptionalMethod,
) -> DamageEffectLevel;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsEvasion")]
pub fn runtimeanimutil_is_evasion(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsParry")]
pub fn runtimeanimutil_is_parry(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsGuard")]
pub fn runtimeanimutil_is_guard(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "Phase", "get_IsCritical")]
pub fn phase_get_is_critical(this: &Phase, method_info: OptionalMethod) -> bool;



//Combat.Phase$$IsDeadSomeone	7101f2abe0	bool Combat.Phase$$IsDeadSomeone(Combat_Phase_o * __this, MethodInfo * method)	136
#[unity::from_offset("Combat", "Phase", "IsDeadSomeone")]
pub fn phase_is_dead_someone(this: &Phase, method_info: OptionalMethod) -> bool;

//Combat.Phase$$IsDeadDamager	7101f2ad80	bool Combat.Phase$$IsDeadDamager(Combat_Phase_o * __this, MethodInfo * method)	136
#[unity::from_offset("Combat", "Phase", "IsDeadDamager")]
pub fn phase_is_dead_damager(this: &Phase, method_info: OptionalMethod) -> bool;

//Combat.Phase$$IsDead	7101f2ad10	bool Combat.Phase$$IsDead(Combat_Phase_o * __this, int32_t side, MethodInfo * method)	108
#[unity::from_offset("Combat", "Phase", "IsDead")]
pub fn phase_is_dead(this: &Phase, side: i32, method_info: OptionalMethod) -> bool;

// Combat.Character$$get_Phase	7102afcb70	Combat_Phase_o * Combat.Character$$get_Phase(Combat_Character_o * __this, MethodInfo * method)	336
#[unity::from_offset("Combat", "Character", "get_Phase")]
pub fn character_get_phase(
    this: &Character,
    method_info: OptionalMethod,
) -> &Phase;

// Combat.CharacterSound$$get_CP	71025efef0	Combat_Character_o * Combat.CharacterSound$$get_CP(Combat_CharacterSound_o * __this, MethodInfo * method)	180
#[unity::from_offset("Combat", "CharacterSound", "get_CP")]
pub fn charactersound_get_cp(
    this: &CharacterSound,
    method_info: OptionalMethod,
) -> &Character;

// Combat.Phase$$get_IsPlayerSideAttack	7101f2b2d0	bool Combat.Phase$$get_IsPlayerSideAttack(Combat_Phase_o * __this, MethodInfo * method)	12
#[unity::from_offset("Combat", "Phase", "get_IsPlayerSideAttack")]
pub fn phase_get_is_player_side_attack(
    this: &Phase,
    method_info: OptionalMethod,
) -> bool;

// Combat.Phase$$get_IsEnemySideAttack	7101f2b2e0	bool Combat.Phase$$get_IsEnemySideAttack(Combat_Phase_o * __this, MethodInfo * method)	12
#[unity::from_offset("Combat", "Phase", "get_IsEnemySideAttack")]
pub fn phase_get_is_enemy_side_attack(
    this: &Phase,
    method_info: OptionalMethod,
) -> bool;

// Combat.Side$$IsMaster	710247cad0	bool Combat.Side$$IsMaster(int32_t i, MethodInfo * method)	12
#[unity::from_offset("Combat", "Side", "IsMaster")]
pub fn side_is_master(i: i32, method_info: OptionalMethod) -> bool;

// Combat.Side$$IsChainAtk	710247cb00	bool Combat.Side$$IsChainAtk(int32_t i, MethodInfo * method)	16
#[unity::from_offset("Combat", "Side", "IsChainAtk")]
pub fn side_is_chain_atk(i: i32, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2940b90)]
fn combat_skill_stack_has(ss: &SkillStack, name: &Il2CppString, optional_method: OptionalMethod) -> bool;