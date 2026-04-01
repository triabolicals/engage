use std::fmt::{Display, Formatter};
use unity::il2cpp::object::Array;
use crate::force::{Force, ForceType};
use unity::prelude::*;
use crate::gamedata::{
    person::PersonData,
    skill::{SkillArray, SkillData},
    item::ItemData,
    job::{BattleStyleTypes, JobData, JobDataMoveTypes},
    WeaponMask,
    dispos::DisposData,
    terrain::TerrainData,
};

mod capability;
mod record;
mod edit;
mod status;
mod accessory;
mod utils;
mod enhance;
mod model;
mod items;
mod pool;
mod ring;
mod ai;

pub use record::*;
pub use edit::*;
pub use capability::*;
pub use status::*;
pub use accessory::*;
pub use utils::*;
pub use model::*;
pub use enhance::*;
pub use items::*;
pub use ai::*;
pub use ring::*;
pub use pool::*;


use crate::god::*;

use crate::random::Random;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Gender {
    None = 0,
    Male = 1,
    Female = 2,
    Other = 3,
}
impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::None => write!(f, "None"),
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Other => write!(f, "Other"),
        }
    }
}

#[unity::class("App", "Unit")]
pub struct Unit {
    pub status: &'static mut UnitStatusField,
    pub prev: Option<&'static Unit>,
    pub next: Option<&'static Unit>,
    pub ai: &'static mut UnitAI,
    pub edit: &'static UnitEdit,
    pub ident: i32,
    pub person: &'static mut PersonData,
    pub job : &'static mut JobData,
    pub force : Option<&'static Force>,
    pub base_capability : &'static mut UnitBaseCapability,
    pub grow_capability: &'static mut Capability,
    pub level_capability: &'static mut UnitBaseCapability,
    pub grow_seed :i32,
    pub drop_seed :i32,
    pub actor : &'static UnitActor,
    pub info :u64,
    pub index :u8,
    pub level :u8,
    pub exp :u8,
    pub hp_value: u8,
    pub hp_display: u8,
    pub hp_stock_count :u8,
    pub hp_stock_count_max :u8,
    pub extra_hp_stock_count :u8,
    pub extra_hp_stock_count_max :u8,
    pub engage_count :u8,
    pub engage_turn :u8,
    pub engage_count_view :u8,
    pub god_states :u64,
    pub x :u8,
    pub z :u8,
    pub dispos_y :u8,
    pub dispos_z :u8,
    pub angle :f32,
    pub dont_attack_person :u64,
    pub dont_attack_force_mask :i32,
    pub item_list : &'static mut UnitItemList,
    pub item_selected :u64,
    pub accessory_list : &'static mut UnitAccessoryList,
    pub god_unit :Option<&'static GodUnit>,
    pub god_link :Option<&'static GodUnit>,
    pub ring :Option<&'static UnitRing>,
    pub extra_sight :i32,
    pub move_distance :i32,
    pub mask_skill: Option<&'static SkillArray>,
    pub equip_skill: &'static mut SkillArray,
    pub private_skill: &'static SkillArray,
    pub receive_skill: &'static SkillArray,
    pub supported_skill: &'static SkillArray,
    pub equip_skill_pool: &'static SkillArray,
    pub learned_job_skill: Option<&'static SkillData>,
    pub original_aptitude: &'static mut WeaponMask,
    pub aptitude: &'static mut WeaponMask,
    pub weapon_mask: &'static mut WeaponMask,
    pub selected_weapon_mask :&'static mut WeaponMask,
    pub enhance_factors: Option<&'static UnitEnhanceFactors>,
    pub enhance_calculator: Option<&'static UnitEnhanceCalculator>,
    pub internal_level: i8,
    pub last_pick_voice :u8,
    pub attack_image :u64,
    pub rod_image :u64,
    pub heal_image :u64,
    pub support_image :u64,
    pub interference_image :u64,
    pub engage_image :u64,
    pub move_image :u64,
    pub record: &'static mut UnitRecord,
    pub map_history_index :u8,
    pub mask_skill_lock :u64,
    pub fortune_target :u64,
    pub fortune_seed :i32,
    pub relay_player_index :u8,
    pub skill_point :i16,
    pub owner_unit :i32,
    pub lock_target_x :u8,
    pub lock_target_z :u8,
    pub total_order :i32,
    pub total_action :i32,
    pub total_attack :i32,
    pub total_damage :i32,
    pub total_result :i32,
    pub side_type :i32,
    pub battle_temporary :i32,
    pub calc_info :i32
}

impl Unit {
    pub fn class_change(&self, job: &JobData) { self.class_change_with_item(job, None) }

    #[unity::class_method(0)] pub fn ctor(&self, use_image: bool); // Offset: 0x1A07F20 Flags: 0
    #[unity::class_method(1)] pub fn initialize(&self, index: i32); // Offset: 0x1A08670 Flags: 0
    #[unity::class_method(2)] pub fn create(&self, person: &PersonData, job: &JobData, level: i32, random: &Random); // Offset: 0x1A086E0 Flags: 0
    #[unity::class_method(3)] pub fn create_from_dispos(&self, data: &DisposData); // Offset: 0x1A08DE0 Flags: 0
    #[unity::class_method(4)] pub fn calc_encount_level(dispos: &DisposData, level: i32) -> i32; // Offset: 0x1A09190 Flags: 0
    #[unity::class_method(5)] pub fn calc_encount_job(dispos: &DisposData, random: &Random, level: i32, job: &JobData) -> &'static JobData; // Offset: 0x1A09240 Flags: 0
    #[unity::class_method(6)] pub fn create_challenge_enemy(&self, dispos: &DisposData, random: &Random, level: i32, ejid: &Il2CppString); // Offset: 0x1A09320 Flags: 0
    // #[unity::class_method(7)] pub fn create_encount_enemy(&self, dispos: &DisposData, unit_rare_type: EncountUnitDataRareType, job: &JobData); // Offset: 0x1A0AD50 Flags: 0
    // #[unity::class_method(8)] pub fn set_encount_hp_stock(&self, dispos: &DisposData, difficulty: Difficulty); // Offset: 0x1A0B0E0 Flags: 0
    #[unity::class_method(9)] pub fn set_hp_stock_count(&self, count: i32); // Offset: 0x1A0B1A0 Flags: 0
    //#[unity::class_method(10)] pub fn create_encount_post(&self, dispos: &DisposData, level: i32, offset: bool, rare_type: EncountUnitDataRareType); // Offset: 0x1A09520 Flags: 0
    #[unity::class_method(11)] pub fn create_encount_mob(&self, pid: &Il2CppString, jid: &Il2CppString, equip_weapon_iid: &Il2CppString); // Offset: 0x1A0C780 Flags: 0
    #[unity::class_method(12)] pub fn create_dlc_god_enemy(&self, data: &DisposData, is_clear_all: bool); // Offset: 0x1A0CA80 Flags: 0
    #[unity::class_method(13)] pub fn create_dlc_god_enemy_impl1(&self, dispos: &DisposData, level: i32, random: &Random); // Offset: 0x1A0CBB0 Flags: 0
    #[unity::class_method(14)] pub fn create_for_vision(&self, original: &Unit, person: &PersonData); // Offset: 0x1A0D340 Flags: 0
    // #[unity::class_method(15)] pub fn create_for_summon(&self, original: &Unit, rank: PersonDataRanks, person: &PersonData); // Offset: 0x1A0D4E0 Flags: 0
    // #[unity::class_method(16)] pub fn create_for_summon_telop(&self, original: &Unit, rank: PersonDataRanks, person: &PersonData); // Offset: 0x1A0D5F0 Flags: 0
    // #[unity::class_method(17)] pub fn create_for_summon_common(&self, original: &Unit, rank: PersonDataRanks, person: &PersonData); // Offset: 0x1A0D5A0 Flags: 0
    // #[unity::class_method(19)] pub fn create_for_versus(&self, data: &DisposData); // Offset: 0x1A0DEC0 Flags: 0
    /// Autolevel unit to target level
    #[unity::class_method(20)] pub fn auto_grow_capability(&self, level: i32, target_level: i32); // Offset: 0x1A0B1B0 Flags: 0
    // #[unity::class_method(21)] pub fn auto_grow_capability2(&self, level: i32, percents: &CapabilityInt); // Offset: 0x1A0DED0 Flags: 0
    #[unity::class_method(22)] pub fn clear(&self); // Offset: 0x1A086D0 Flags: 0
    #[unity::class_method(23)] pub fn clear_except_map_history_index(&self); // Offset: 0x1A0CBA0 Flags: 0
    #[unity::class_method(24)] pub fn clear_impl(&self, is_clear_map_history_index: bool); // Offset: 0x1A0E1F0 Flags: 0
    #[unity::class_method(25)] pub fn clear_for_rewind_preview(&self, changed_gid: &Il2CppString); // Offset: 0x1A0EC30 Flags: 0
    #[unity::class_method(26)] pub fn copy_from(&self, from: &Unit); // Offset: 0x1A0EFF0 Flags: 0
    // #[unity::class_method(27)] pub fn copy_from_for_level_up(&self, from: &Unit); // Offset: 0x1A10070 Flags: 0
    // #[unity::class_method(28)] pub fn copy_from_for_arena_result(&self, from: &Unit); // Offset: 0x1A10180 Flags: 0
    // #[unity::class_method(29)] pub fn copy_from_for_versus(&self, from: &Unit); // Offset: 0x1A10C10 Flags: 0
    #[unity::class_method(30)] pub fn reset_phase_begin(&self, phase: ForceType); // Offset: 0x1A11C70 Flags: 0
    #[unity::class_method(31)] pub fn is_engage_owner(&self) -> bool; // Offset: 0x1A197A0 Flags: 0
    #[unity::class_method(32)] pub fn reset_phase_begin_after(&self, phase: ForceType); // Offset: 0x1A19810 Flags: 0
    #[unity::class_method(33)] pub fn reset_phase_end(&self, phase: ForceType); // Offset: 0x1A19EF0 Flags: 0
    #[unity::class_method(34)] pub fn reset_sub_phase_charm_confusion_begin(&self); // Offset: 0x1A1A390 Flags: 0
    #[unity::class_method(35)] pub fn reset_dead(&self); // Offset: 0x1A1A410 Flags: 0
    #[unity::class_method(36)] pub fn reset_map_begin(&self); // Offset: 0x1A1A680 Flags: 0
    #[unity::class_method(37)] pub fn reset_map_end(&self, is_reset_position: bool); // Offset: 0x1A1A850 Flags: 0
    #[unity::class_method(38)] pub fn map_completed(&self); // Offset: 0x1A1AA90 Flags: 0
    #[unity::class_method(39)] pub fn fixed(&self); // Offset: 0x1A1E7C0 Flags: 0
    #[unity::class_method(40)] pub fn transfer(&self, force: ForceType, is_last: bool); // Offset: 0x1A1E8C0 Flags: 0
    #[unity::class_method(41)] pub fn transfer_for_sortie(&self, force: ForceType, is_last: bool); // Offset: 0x1A1F510 Flags: 0
    #[unity::class_method(42)] pub fn transfer_for_rewind(&self, force: ForceType, prev_unit: &Unit); // Offset: 0x1A1F520 Flags: 0
    #[unity::class_method(43)] pub fn transfer_for_rewind_latest(&self, force: ForceType); // Offset: 0x1A1F530 Flags: 0
    #[unity::class_method(44)] pub fn transfer_impl(&self, force: ForceType, is_last: bool, prev_unit: &Unit, is_delete_actor: bool); // Offset: 0x1A1E8D0 Flags: 0
    #[unity::class_method(45)] pub fn update(&self); // Offset: 0x1A0C730 Flags: 0
    #[unity::class_method(46)] pub fn auto_equip(&self); // Offset: 0x1A0CA70 Flags: 0
    #[unity::class_method(47)] pub fn update_state_with_equipped(&self, equipped: &UnitItem); // Offset: 0x1A1F550 Flags: 0
    #[unity::class_method(48)] pub fn try_update_state_with_equipped(&self, equipped: &UnitItem) -> bool; // Offset: 0x1A1F560 Flags: 0
    #[unity::class_method(49)] pub fn add_skills(&self, array: &SkillArray); // Offset: 0x1A1F620 Flags: 0
    // #[unity::class_method(50)] pub fn add_skill2(&self, array: &SkillArray, category: SkillDataCategorys); // Offset: 0x1A1FF40 Flags: 0
    // #[unity::class_method(51)] pub fn add_skill3(&self, skill: &SkillData, category: SkillDataCategorys, age: i32); // Offset: 0x1A20820 Flags: 0
    // #[unity::class_method(52)] pub fn add_skill_without_update(&self, skill: &SkillData, category: SkillDataCategorys, age: i32) -> bool; // Offset: 0x1A20EC0 Flags: 0
    // #[unity::class_method(53)] pub fn is_engaging2(&self, god_unit: &GodUnit) -> bool; // Offset: 0x1A213A0 Flags: 0
    #[unity::class_method(54)] pub fn update_state_impl(&self, is_auto_equip: bool, equipped: &UnitItem); // Offset: 0x1A12020 Flags: 0
    // #[unity::class_method(55)] pub fn commit_enhance(&self, equipped: &UnitItem); // Offset: 0x1A1AAE0 Flags: 0
    // #[unity::class_method(56)] pub fn update_place(&self); // Offset: 0x1A21B80 Flags: 0
    #[unity::class_method(57)] pub fn create_actor(&self); // Offset: 0x1A22000 Flags: 0
    #[unity::class_method(58)] pub fn try_create_actor(&self) -> bool; // Offset: 0x1A22010 Flags: 0
    #[unity::class_method(59)] pub fn reload_actor(&self); // Offset: 0x1A19ED0 Flags: 0
    #[unity::class_method(60)] pub fn delete_actor(&self); // Offset: 0x1A0EA70 Flags: 0
    #[unity::class_method(61)] pub fn update_actor(&self); // Offset: 0x1A220B0 Flags: 0
    #[unity::class_method(62)] pub fn tick_actor(&self); // Offset: 0x1A22150 Flags: 0
    #[unity::class_method(63)] pub fn check_status(&self, status: i64) -> bool; // Offset: 0x1A222A0 Flags: 0
    #[unity::class_method(64)] pub fn not_status(&self, status: i64) -> bool; // Offset: 0x1A22310 Flags: 0
    #[unity::class_method(65)] pub fn set_status(&self, status: i64); // Offset: 0x1A0C6C0 Flags: 0
    #[unity::class_method(66)] pub fn clear_status(&self, status: i64); // Offset: 0x1A11FB0 Flags: 0
    #[unity::class_method(67)] pub fn change_status(&self, status: i64); // Offset: 0x1A22380 Flags: 0
    #[unity::class_method(68)] pub fn get_status(&self) -> &'static UnitStatusField; // Offset: 0x1A223F0 Flags: 0
    #[unity::class_method(69)] pub fn set_dead(&self); // Offset: 0x1A22400 Flags: 0
    #[unity::class_method(70)] pub fn get_name(&self) -> &'static mut Il2CppString; // Offset: 0x1A22770 Flags: 0
    #[unity::class_method(71)] pub fn get_talk_name(&self) -> &'static Il2CppString; // Offset: 0x1A22990 Flags: 0
    #[unity::class_method(72)] pub fn get_name_impl(&self, is_morph: bool) -> &'static Il2CppString; // Offset: 0x1A22880 Flags: 0
    #[unity::class_method(73)] pub fn get_name_for_relay_data(edit: &UnitEdit, person: &PersonData, job: &JobData, is_morph: bool, relay_player_index: i32) -> &'static Il2CppString; // Offset: 0x1A22CB0 Flags: 0
    #[unity::class_method(74)] pub fn get_name_impl2(edit: &UnitEdit, person: &PersonData, job: &JobData, is_morph: bool, is_guest: bool, relay_player_index: i32, force_type: ForceType) -> &'static Il2CppString; // Offset: 0x1A229A0 Flags: 0
    #[unity::class_method(75)] pub fn is_default_name_for_net(is_guest: bool, relay_player_index: i32, force_type: ForceType) -> bool; // Offset: 0x1A22D50 Flags: 0
    #[unity::class_method(76)] pub fn get_ascii_name(&self) -> &'static Il2CppString; // Offset: 0x1A22EB0 Flags: 0
    #[unity::class_method(77)] pub fn get_job_name(&self) -> &'static Il2CppString; // Offset: 0x1A22EC0 Flags: 0
    // #[unity::class_method(78)] pub fn set_position(&self, pos: Vector3, update: bool); // Offset: 0x1A22ED0 Flags: 0
    // #[unity::class_method(79)] pub fn set_position2(&self, x: i32, z: i32, update: bool); // Offset: 0x1A22FC0 Flags: 0
    #[unity::class_method(80)] pub fn set_rotation(&self, angle: f32); // Offset: 0x1A23000 Flags: 0
    #[unity::class_method(81)] pub fn set_force_for_stand_alone(&self, force: &Force); // Offset: 0x1A23030 Flags: 0
    #[unity::class_method(82)] pub fn is_hero(&self) -> bool; // Offset: 0x1A23040 Flags: 0
    #[unity::class_method(83)] pub fn is_dead(&self) -> bool; // Offset: 0x1A23050 Flags: 0
    #[unity::class_method(84)] pub fn get_gender(&self) -> Gender; // Offset: 0x1A230B0 Flags: 0
    #[unity::class_method(85)] pub fn get_dress_gender(&self) -> Gender; // Offset: 0x1A232E0 Flags: 0
    // #[unity::class_method(86)] pub fn get_gender(edit: &UnitEdit, person: &PersonData) -> Gender; // Offset: 0x1A23560 Flags: 0
    // #[unity::class_method(87)] pub fn get_dress_gender2(edit: &UnitEdit, person: &PersonData) -> Gender; // Offset: 0x1A235A0 Flags: 0
    #[unity::class_method(88)] pub fn is_female(&self) -> bool; // Offset: 0x1A235F0 Flags: 0
    #[unity::class_method(89)] pub fn is_leader(&self) -> bool; // Offset: 0x1A23740 Flags: 0
    #[unity::class_method(90)] pub fn is_morph(&self) -> bool; // Offset: 0x1A23840 Flags: 0
    #[unity::class_method(91)] pub fn has_fang_curse(&self) -> bool; // Offset: 0x1A23940 Flags: 0
    #[unity::class_method(92)] pub fn is_enchantment(&self) -> bool; // Offset: 0x1A23A40 Flags: 0
    #[unity::class_method(93)] pub fn is_move_not_allow(&self) -> bool; // Offset: 0x1A23B40 Flags: 0
    #[unity::class_method(94)] pub fn is_unique(&self) -> bool; // Offset: 0x1A23BB0 Flags: 0
    #[unity::class_method(95)] pub fn is_show(&self) -> bool; // Offset: 0x1A23C20 Flags: 0
    #[unity::class_method(96)] pub fn can_sortie(&self) -> bool; // Offset: 0x1A23C80 Flags: 0
    // #[unity::class_method(97)] pub fn is_play_area(&self) -> bool; // Offset: 0x1A23CF0 Flags: 0
    // #[unity::class_method(98)] pub fn is_play_area2(&self, x: i32, z: i32) -> bool; // Offset: 0x1A23DF0 Flags: 0
    #[unity::class_method(99)] pub fn is_allied(&self, unit: &Unit) -> bool; // Offset: 0x1A23EE0 Flags: 0
    #[unity::class_method(100)] pub fn is_operate(&self) -> bool; // Offset: 0x1A23FB0 Flags: 0
    #[unity::class_method(101)] pub fn is_efficacy(&self, target: &Unit) -> bool; // Offset: 0x1A24040 Flags: 0
    // #[unity::class_method(102)] pub fn get_attrs(&self) -> SkillDataAttrs; // Offset: 0x1A24090 Flags: 0
    #[unity::class_method(103)] pub fn can_transporter(&self) -> bool; // Offset: 0x1A240C0 Flags: 0
    // #[unity::class_method(104)] pub fn can_breakable(&self, target: &Unit) -> bool; // Offset: 0x1A241F0 Flags: 0
    // #[unity::class_method(105)] pub fn can_sky_battle(&self) -> bool; // Offset: 0x1A258E0 Flags: 0
    #[unity::class_method(106)] pub fn get_force_absent_type(&self) -> ForceType; // Offset: 0x1A25AA0 Flags: 0
    #[unity::class_method(107)] pub fn get_force_dead_type(&self) -> ForceType; // Offset: 0x1A25B60 Flags: 0
    #[unity::class_method(108)] pub fn is_same_force_type(&self, force_type: ForceType) -> bool; // Offset: 0x1A25CB0 Flags: 0
    #[unity::class_method(109)] pub fn set_engage_impl(&self, enable: bool, link: bool); // Offset: 0x1A25D10 Flags: 0
    // #[unity::class_method(110)] pub fn set_engage_link_simulation(&self, enable: bool, link: &Unit); // Offset: 0x1A261E0 Flags: 0
    // #[unity::class_method(111)] pub fn set_engage_simulation(&self, enable: bool, link: &Unit); // Offset: 0x1A26280 Flags: 0
    #[unity::class_method(112)] pub fn set_god_link_impl(&self, god_link: &GodUnit); // Offset: 0x1A0EDE0 Flags: 0
    #[unity::class_method(113)] pub fn play_engage(&self, enable: bool); // Offset: 0x1A26520 Flags: 0
    #[unity::class_method(114)] pub fn is_engaging(&self) -> bool; // Offset: 0x1A265E0 Flags: 0
    #[unity::class_method(115)] pub fn is_hero_engaging(&self) -> bool; // Offset: 0x1A26640 Flags: 0
    #[unity::class_method(116)] pub fn can_engage_heal(&self) -> bool; // Offset: 0x1A266F0 Flags: 0
    #[unity::class_method(117)] pub fn set_engage_link(&self, enable: bool, god_link: &GodUnit, parent: &Unit, child: &Unit); // Offset: 0x1A26830 Flags: 0
    #[unity::class_method(118)] pub fn set_engage(&self, enable: bool, link_unit: Option<&Unit>); // Offset: 0x1A19BA0 Flags: 0
    // #[unity::class_method(119)] pub fn try_relocation(&self) -> bool; // Offset: 0x1A268B0 Flags: 0
    // #[unity::class_method(120)] pub fn is_active_route(&self) -> bool; // Offset: 0x1A26C80 Flags: 0
    // #[unity::class_method(121)] pub fn get_first_x(&self) -> i32; // Offset: 0x1A26B40 Flags: 0
    // #[unity::class_method(122)] pub fn get_first_z(&self) -> i32; // Offset: 0x1A26BE0 Flags: 0
    #[unity::class_method(123)] pub fn force_set_engage_link_for_rewind(&self, link_unit: &Unit); // Offset: 0x1A26DB0 Flags: 0
    #[unity::class_method(124)] pub fn set_engage_attack(&self, enable: bool); // Offset: 0x1A26E40 Flags: 0
    #[unity::class_method(125)] pub fn can_engage_impl(&self, god_unit: &GodUnit) -> bool; // Offset: 0x1A26F70 Flags: 0
    #[unity::class_method(126)] pub fn can_engage_cancel(&self, pickup: bool) -> bool; // Offset: 0x1A271B0 Flags: 0
    #[unity::class_method(127)] pub fn can_engage_start(&self) -> bool; // Offset: 0x1A273E0 Flags: 0
    #[unity::class_method(128)] pub fn can_engage_link(&self) -> bool; // Offset: 0x1A273F0 Flags: 0
    #[unity::class_method(129)] pub fn get_linkable_god_unit(&self) -> Option<&'static GodUnit>; // Offset: 0x1A274B0 Flags: 0
    #[unity::class_method(130)] pub fn get_engage_link_unit(&self) -> Option<&'static Unit>; // Offset: 0x1A274D0 Flags: 0
    #[unity::class_method(131)] pub fn pre_start_target_select(&self); // Offset: 0x1A27500 Flags: 0
    #[unity::class_method(132)] pub fn post_start_target_select(&self); // Offset: 0x1A27630 Flags: 0
    // #[unity::class_method(133)] pub fn try_add_achieve_engage(&self); // Offset: 0x1A27710 Flags: 0
    // #[unity::class_method(134)] pub fn decide_target_select(&self, is_redoer: bool); // Offset: 0x1A277F0 Flags: 0
    // #[unity::class_method(135)] pub fn cancel_target_select(&self); // Offset: 0x1A27A30 Flags: 0
    #[unity::class_method(136)] pub fn can_change_engage(&self, target_unit: &Unit) -> bool; // Offset: 0x1A27C50 Flags: 0
    #[unity::class_method(137)] pub fn try_change_engage(&self, target_unit: &Unit, item_index: i32) -> bool; // Offset: 0x1A27F50 Flags: 0
    #[unity::class_method(138)] pub fn change_engage(&self, target_unit: &Unit, item_index: i32); // Offset: 0x1A27FB0 Flags: 0
    #[unity::class_method(139)] pub fn can_engage_attack(&self) -> bool; // Offset: 0x1A288F0 Flags: 0
    #[unity::class_method(140)] pub fn can_engage_target(&self, target: &Unit) -> bool; // Offset: 0x1A291A0 Flags: 0
    #[unity::class_method(141)] pub fn engage_target_exists(&self, target: &Unit, skill: &SkillData) -> bool; // Offset: 0x1A291D0 Flags: 0
    #[unity::class_method(142)] pub fn can_enemy_engage_attack(&self) -> bool; // Offset: 0x1A289E0 Flags: 0
    #[unity::class_method(143)] pub fn can_execute_engage_attack(&self) -> bool; // Offset: 0x1A29AB0 Flags: 0
    #[unity::class_method(144)] pub fn can_act(&self, is_fixed: bool, is_charged: bool) -> bool; // Offset: 0x1A29B10 Flags: 0
    #[unity::class_method(145)] pub fn can_act_without_engage_charge(&self) -> bool; // Offset: 0x1A29C10 Flags: 0
    #[unity::class_method(146)] pub fn can_be_target(&self) -> bool; // Offset: 0x1A29CF0 Flags: 0
    #[unity::class_method(147)] pub fn can_external_move(&self) -> bool; // Offset: 0x1A29F10 Flags: 0
    #[unity::class_method(148)] pub fn is_sight(&self, x: i32, z: i32) -> bool; // Offset: 0x1A29FB0 Flags: 0
    #[unity::class_method(149)] pub fn can_warp(&self, rod_unit: &Unit, x: i32, z: i32) -> bool; // Offset: 0x1A2A0E0 Flags: 0
    // #[unity::class_method(150)] pub fn can_skill(&self, skill: &SkillData) -> bool; // Offset: 0x1A2A420 Flags: 0
    // #[unity::class_method(151)] pub fn can_change_god(&self) -> bool; // Offset: 0x1A2A590 Flags: 0
    // #[unity::class_method(152)] pub fn change_god(&self) -> bool; // Offset: 0x1A2A690 Flags: 0
    // #[unity::class_method(153)] pub fn get_cells(&self, cells: &Array<MapPos>) -> i32; // Offset: 0x1A2A6F0 Flags: 0
    // #[unity::class_method(154)] pub fn get_cells2(&self, cells: &Array<MapPos>, base_x: i32, base_z: i32) -> i32; // Offset: 0x1A2A700 Flags: 0
    #[unity::class_method(155)] pub fn set_base_capability(&self, index: i32, value: i32); // Offset: 0x1A2A7A0 Flags: 0
    #[unity::class_method(156)] pub fn add_base_capability(&self, index: i32, value: i32); // Offset: 0x1A2A800 Flags: 0
    // #[unity::class_method(157)] pub fn set_capability_just(&self, index: i32, value: i32); // Offset: 0x1A2A8B0 Flags: 0
    // #[unity::class_method(158)] pub fn get_god_enhance_rating(&self, god: &GodUnit) -> i32; // Offset: 0x1A2B6B0 Flags: 0
    #[unity::class_method(159)] pub fn change_hp(&self, hp: i32); // Offset: 0x1A2B770 Flags: 0
    #[unity::class_method(160)] pub fn play_set_hp(&self, hp: i32); // Offset: 0x1A2B920 Flags: 0
    #[unity::class_method(161)] pub fn play_set_heal(&self, heal: i32); // Offset: 0x1A2BCA0 Flags: 0
    #[unity::class_method(162)] pub fn play_set_damage(&self, damage: i32, can_die: bool, is_multi: bool); // Offset: 0x1A2B940 Flags: 0
    /*
    #[unity::class_method(163)] pub fn is_changing_hp(&self) -> bool; // Offset: 0x1A2BCB0 Flags: 0
    #[unity::class_method(164)] pub fn get_max_hp(&self) -> i32; // Offset: 0x1A2BCD0 Flags: 0
    #[unity::class_method(165)] pub fn get_str(&self) -> i32; // Offset: 0x1A2BE50 Flags: 0
    #[unity::class_method(166)] pub fn get_tech(&self) -> i32; // Offset: 0x1A2BFD0 Flags: 0
    #[unity::class_method(167)] pub fn get_quick(&self) -> i32; // Offset: 0x1A2C150 Flags: 0
    #[unity::class_method(168)] pub fn get_luck(&self) -> i32; // Offset: 0x1A2C2D0 Flags: 0
    #[unity::class_method(169)] pub fn get_def(&self) -> i32; // Offset: 0x1A2C450 Flags: 0
    #[unity::class_method(170)] pub fn get_magic(&self) -> i32; // Offset: 0x1A2C5D0 Flags: 0
    #[unity::class_method(171)] pub fn get_mdef(&self) -> i32; // Offset: 0x1A2C760 Flags: 0
    #[unity::class_method(172)] pub fn get_phys(&self) -> i32; // Offset: 0x1A2C8F0 Flags: 0
    #[unity::class_method(173)] pub fn get_sight(&self) -> i32; // Offset: 0x1A2CA80 Flags: 0
    #[unity::class_method(174)] pub fn get_move_power(&self) -> i32; // Offset: 0x1A2CCC0 Flags: 0
    #[unity::class_method(175)] pub fn get_move_power_without_removing(&self) -> i32; // Offset: 0x1A2D040 Flags: 0

     */
    // #[unity::class_method(176)] pub fn find_skill(&self, flags: SkillDataFlags) -> &'static SkillData; // Offset: 0x1A2D7C0 Flags: 0
    // #[unity::class_method(177)] pub fn find_skill2(&self, states: SkillDataStates) -> &'static SkillData; // Offset: 0x1A2D910 Flags: 0
    // #[unity::class_method(178)] pub fn get_removable_skill(&self) -> &'static SkillData; // Offset: 0x1A2DA60 Flags: 0
    // #[unity::class_method(179)] pub fn get_removable_power(&self) -> i32; // Offset: 0x1A2DBE0 Flags: 0
    #[unity::class_method(180)] pub fn get_capability(&self, index: i32, calc_enhance: bool) -> i32; // Offset: 0x1A2DD80 Flags: 0
    // #[unity::class_method(181)] pub fn get_capability2(&self, type: CapabilityDefinitionType, calc_enhance: bool) -> i32; // Offset: 0x1A2EE50 Flags: 0
    #[unity::class_method(182)] pub fn get_capability_grow(&self, index: i32, is_auto_grow: bool) -> i32; // Offset: 0x1A2FF20 Flags: 0
    // #[unity::class_method(183)] pub fn get_capability_grow2(&self, type: CapabilityDefinitionType, is_auto_grow: bool) -> i32; // Offset: 0x1A304E0 Flags: 0
    // #[unity::class_method(184)] pub fn get_capability_limit(&self, type: CapabilityDefinitionType) -> i32; // Offset: 0x1A30A90 Flags: 0
    #[unity::class_method(185)] pub fn get_capability_limit(&self, index: i32) -> i32; // Offset: 0x1A30B60 Flags: 0
    // #[unity::class_method(186)] pub fn can_capability_grow(&self, type: CapabilityDefinitionType) -> bool; // Offset: 0x1A30C30 Flags: 0
    // #[unity::class_method(187)] pub fn can_capability_grow2(&self, index: i32) -> bool; // Offset: 0x1A31AA0 Flags: 0
    // #[unity::class_method(188)] pub fn get_strength_capability_index(&self, rand: &Random) -> CapabilityDefinitionType; // Offset: 0x1A31B80 Flags: 0
    /*
    #[unity::class_method(189)] pub fn get_no_enhance_max_hp(&self) -> i32; // Offset: 0x1A32A40 Flags: 0
    #[unity::class_method(190)] pub fn get_no_enhance_str(&self) -> i32; // Offset: 0x1A32BA0 Flags: 0
    #[unity::class_method(191)] pub fn get_no_enhance_tech(&self) -> i32; // Offset: 0x1A32D00 Flags: 0
    #[unity::class_method(192)] pub fn get_no_enhance_quick(&self) -> i32; // Offset: 0x1A32E60 Flags: 0
    #[unity::class_method(193)] pub fn get_no_enhance_luck(&self) -> i32; // Offset: 0x1A32FC0 Flags: 0
    #[unity::class_method(194)] pub fn get_no_enhance_def(&self) -> i32; // Offset: 0x1A33120 Flags: 0
    #[unity::class_method(195)] pub fn get_no_enhance_magic(&self) -> i32; // Offset: 0x1A33280 Flags: 0
    #[unity::class_method(196)] pub fn get_no_enhance_mdef(&self) -> i32; // Offset: 0x1A333F0 Flags: 0
    #[unity::class_method(197)] pub fn get_no_enhance_phys(&self) -> i32; // Offset: 0x1A33560 Flags: 0
    #[unity::class_method(198)] pub fn get_no_enhance_sight(&self) -> i32; // Offset: 0x1A336D0 Flags: 0
    #[unity::class_method(199)] pub fn get_no_enhance_move_power(&self) -> i32; // Offset: 0x1A338F0 Flags: 0

     */
    // #[unity::class_method(200)] pub fn get_no_enhance_capability(&self, index: i32) -> i32; // Offset: 0x1A2A920 Flags: 0
    // #[unity::class_method(201)] pub fn get_no_enhance_capability2(&self, type: CapabilityDefinitionType) -> i32; // Offset: 0x1A30D10 Flags: 0
    #[unity::class_method(202)] pub fn get_enhanced_level(&self) -> i32; // Offset: 0x1A33AA0 Flags: 0
    #[unity::class_method(203)] pub fn add_hp(&self, v: i32); // Offset: 0x1A33CC0 Flags: 0
    #[unity::class_method(204)] pub fn is_dont_attack(&self, unit: &Unit) -> bool; // Offset: 0x1A33E80 Flags: 0
    #[unity::class_method(205)] pub fn set_dont_attack(&self, person: &PersonData); // Offset: 0x1A33ED0 Flags: 0
    #[unity::class_method(206)] pub fn set_dont_attack_force_mask(&self, force_mask: u32); // Offset: 0x1A33EE0 Flags: 0
    // #[unity::class_method(207)] pub fn get_entrust_for_ai(&self) -> UnitEntrustType; // Offset: 0x1A33EF0 Flags: 0
    #[unity::class_method(208)] pub fn can_talk(&self) -> bool; // Offset: 0x1A34170 Flags: 0
    #[unity::class_method(209)] pub fn can_dance(&self, target: &Unit) -> bool; // Offset: 0x1A343F0 Flags: 0
    #[unity::class_method(210)] pub fn can_contract(&self, target: &Unit) -> bool; // Offset: 0x1A34690 Flags: 0
    #[unity::class_method(211)] pub fn can_again(&self) -> bool; // Offset: 0x1A34880 Flags: 0
    #[unity::class_method(212)] pub fn can_chain(&self, parent: &Unit) -> bool; // Offset: 0x1A349E0 Flags: 0
    #[unity::class_method(213)] pub fn can_chain_attack(&self) -> bool; // Offset: 0x1A34C60 Flags: 0
    #[unity::class_method(214)] pub fn is_terrain_invalid(&self, terrain: &TerrainData) -> bool; // Offset: 0x1A34C90 Flags: 0
    #[unity::class_method(215)] pub fn can_chain_guard(&self) -> bool; // Offset: 0x1A34D10 Flags: 0
    // #[unity::class_method(216)] pub fn get_guard_type(&self) -> UnitGuardType; // Offset: 0x1A34F50 Flags: 0
    // #[unity::class_method(217)] pub fn can_ccogitation(&self) -> bool; // Offset: 0x1A351A0 Flags: 0
    #[unity::class_method(218)] pub fn can_trade(&self) -> bool; // Offset: 0x1A351C0 Flags: 0
    #[unity::class_method(219)] pub fn can_gain_reliance(&self) -> bool; // Offset: 0x1A35480 Flags: 0
    #[unity::class_method(220)] pub fn has_skill(&self, skill: &SkillData) -> bool; // Offset: 0x1A35520 Flags: 0
    #[unity::class_method(221)] pub fn has_sid(&self, sid: &Il2CppString) -> bool; // Offset: 0x1A35540 Flags: 0
    // #[unity::class_method(222)] pub fn has_skill3(&self, flags: SkillDataFlags) -> bool; // Offset: 0x1A355F0 Flags: 0
    // #[unity::class_method(223)] pub fn has_skill4(&self, states: SkillDataStates) -> bool; // Offset: 0x1A35610 Flags: 0
    #[unity::class_method(224)] pub fn has_dance_skill(&self) -> bool; // Offset: 0x1A35630 Flags: 0
    #[unity::class_method(225)] pub fn has_contract_skill(&self) -> bool; // Offset: 0x1A35730 Flags: 0
    #[unity::class_method(226)] pub fn is_standing_die(&self) -> bool; // Offset: 0x1A35740 Flags: 0
    #[unity::class_method(227)] pub fn is_immortal(&self) -> bool; // Offset: 0x1A35930 Flags: 0
    #[unity::class_method(228)] pub fn is_last_boss(&self) -> bool; // Offset: 0x1A35A50 Flags: 0
    #[unity::class_method(229)] pub fn get_equip_skill(&self, index: i32) -> &'static SkillData; // Offset: 0x1A35CA0 Flags: 0
    #[unity::class_method(230)] pub fn has_sid_skill(&self, sid: &Il2CppString) -> bool; // Offset: 0x1A35DF0 Flags: 0
    #[unity::class_method(231)] pub fn has_skill_equip(&self, skill: &SkillData) -> bool; // Offset: 0x1A35EC0 Flags: 0
    #[unity::class_method(232)] pub fn add_equip_sid(&self, sid: &Il2CppString) -> bool; // Offset: 0x1A35EF0 Flags: 0
    #[unity::class_method(233)] pub fn add_equip_skill(&self, skill: &SkillData) -> bool; // Offset: 0x1A35F80 Flags: 0
    #[unity::class_method(234)] pub fn remove_equip_sid(&self, sid: &Il2CppString); // Offset: 0x1A36E80 Flags: 0
    #[unity::class_method(235)] pub fn remove_equip_skill(&self, skill: &SkillData); // Offset: 0x1A36F10 Flags: 0
    // #[unity::class_method(236)] pub fn replace_equip_skill(&self, index: i32, sid: &Il2CppString); // Offset: 0x1A37230 Flags: 0
    // #[unity::class_method(237)] pub fn replace_equip_skill2(&self, index: i32, skill: &SkillData); // Offset: 0x1A372D0 Flags: 0
    #[unity::class_method(238)] pub fn move_equip_skill(&self, old_index: i32, new_index: i32); // Offset: 0x1A37610 Flags: 0
    #[unity::class_method(239)] pub fn has_sid_private(&self, sid: &Il2CppString) -> bool; // Offset: 0x1A378B0 Flags: 0
    #[unity::class_method(240)] pub fn has_skill_private(&self, skill: &SkillData) -> bool; // Offset: 0x1A37970 Flags: 0
    #[unity::class_method(241)] pub fn add_private_sid(&self, sid: &Il2CppString) -> bool; // Offset: 0x1A0C630 Flags: 0
    #[unity::class_method(242)] pub fn add_private_skill(&self, skill: &SkillData) -> bool; // Offset: 0x1A37990 Flags: 0
    #[unity::class_method(243)] pub fn add_private_skills(&self, skills: &SkillArray) -> bool; // Offset: 0x1A37F20 Flags: 0
    #[unity::class_method(244)] pub fn remove_private_sid(&self, sid: &Il2CppString) -> bool; // Offset: 0x1A38090 Flags: 0
    #[unity::class_method(245)] pub fn remove_private_skill(&self, skill: &SkillData) -> bool; // Offset: 0x1A38120 Flags: 0
    #[unity::class_method(246)] pub fn remove_private_skills(&self, skills: &SkillArray) -> bool; // Offset: 0x1A38450 Flags: 0
    #[unity::class_method(247)] pub fn get_record(&self, kind: UnitRecordKinds) -> i32; // Offset: 0x1A385C0 Flags: 0
    #[unity::class_method(248)] pub fn set_record(&self, kind: UnitRecordKinds, value: i32); // Offset: 0x1A385D0 Flags: 0
    #[unity::class_method(249)] pub fn add_record(&self, kind: UnitRecordKinds, value: i32); // Offset: 0x1A38630 Flags: 0
    #[unity::class_method(250)] pub fn inc_record(&self, kind: UnitRecordKinds); // Offset: 0x1A386C0 Flags: 0
    #[unity::class_method(251)] pub fn dec_record(&self, kind: UnitRecordKinds); // Offset: 0x1A38750 Flags: 0
    #[unity::class_method(252)] pub fn add_record_with_history(&self, kind: UnitRecordKinds, value: i32); // Offset: 0x1A387E0 Flags: 0
    #[unity::class_method(253)] pub fn get_from_equip_skill_pool(&self, index: i32) -> &'static SkillData; // Offset: 0x1A388D0 Flags: 0
    #[unity::class_method(254)] pub fn add_to_equip_skill_pool_by_sid(&self, sid: &Il2CppString); // Offset: 0x1A38A20 Flags: 0
    #[unity::class_method(255)] pub fn add_to_equip_skill_pool(&self, skill: &SkillData); // Offset: 0x1A36560 Flags: 0
    #[unity::class_method(256)] pub fn remove_from_equip_skill_pool_by_sid(&self, sid: &Il2CppString); // Offset: 0x1A38AB0 Flags: 0
    #[unity::class_method(257)] pub fn remove_from_equip_skill_pool(&self, skill: &SkillData); // Offset: 0x1A38B40 Flags: 0
    #[unity::class_method(258)] pub fn is_exist_in_equip_skill_pool_by_sid(&self, sid: &Il2CppString) -> bool; // Offset: 0x1A39110 Flags: 0
    #[unity::class_method(259)] pub fn is_exist_in_equip_skill_pool(&self, skill: &SkillData) -> bool; // Offset: 0x1A391E0 Flags: 0
    #[unity::class_method(260)] pub fn is_inheritance_enable(&self, skill: &SkillData) -> bool; // Offset: 0x1A39210 Flags: 0
    #[unity::class_method(261)] pub fn is_poison(&self) -> bool; // Offset: 0x1A39390 Flags: 0
    #[unity::class_method(262)] pub fn is_stun(&self) -> bool; // Offset: 0x1A393B0 Flags: 0
    #[unity::class_method(263)] pub fn is_sleep(&self) -> bool; // Offset: 0x1A393C0 Flags: 0
    #[unity::class_method(264)] pub fn is_silence(&self) -> bool; // Offset: 0x1A393D0 Flags: 0
    #[unity::class_method(265)] pub fn is_charm(&self) -> bool; // Offset: 0x1A393E0 Flags: 0
    #[unity::class_method(266)] pub fn is_confusion(&self) -> bool; // Offset: 0x1A393F0 Flags: 0
    #[unity::class_method(267)] pub fn is_charm_or_confusion(&self) -> bool; // Offset: 0x1A39400 Flags: 0
    #[unity::class_method(268)] pub fn is_freeze(&self) -> bool; // Offset: 0x1A39420 Flags: 0
    #[unity::class_method(269)] pub fn is_weakness(&self) -> bool; // Offset: 0x1A39430 Flags: 0
    #[unity::class_method(270)] pub fn is_not_enhance(&self) -> bool; // Offset: 0x1A39440 Flags: 0
    #[unity::class_method(271)] pub fn is_disorder(&self) -> bool; // Offset: 0x1A39450 Flags: 0
    // #[unity::class_method(272)] pub fn clear_disorder(&self, state: SkillDataStates) -> bool; // Offset: 0x1A39470 Flags: 0
    #[unity::class_method(273)] pub fn is_vision(&self) -> bool; // Offset: 0x1A39620 Flags: 0
    #[unity::class_method(274)] pub fn is_vision2(&self, owner: &Unit) -> bool; // Offset: 0x1A39680 Flags: 0
    #[unity::class_method(275)] pub fn can_gain_item(&self) -> bool; // Offset: 0x1A39720 Flags: 0
    #[unity::class_method(276)] pub fn get_vision_owner(&self) -> &'static Unit; // Offset: 0x1A39790 Flags: 0
    #[unity::class_method(277)] pub fn is_summon(&self) -> bool; // Offset: 0x1A39850 Flags: 0
    #[unity::class_method(278)] pub fn is_summon2(&self, owner: &Unit) -> bool; // Offset: 0x1A398B0 Flags: 0
    #[unity::class_method(279)] pub fn is_summon_god(&self) -> bool; // Offset: 0x1A39950 Flags: 0
    #[unity::class_method(280)] pub fn get_summon_owner(&self) -> Option<&'static Unit>; // Offset: 0x1A399E0 Flags: 0
    // #[unity::class_method(281)] pub fn is_lockon(&self) -> bool; // Offset: 0x1A39B10 Flags: 0
    // #[unity::class_method(282)] pub fn try_get_lock_target(&self, x: i32, z: i32) -> bool; // Offset: 0x1A39B70 Flags: 0
    // #[unity::class_method(283)] pub fn set_lock_target(&self, x: i32, z: i32); // Offset: 0x1A39C10 Flags: 0
    // #[unity::class_method(284)] pub fn reset_lock_target(&self); // Offset: 0x1A39C90 Flags: 0
    #[unity::class_method(285)] pub fn is_on_map(&self) -> bool; // Offset: 0x1A39D00 Flags: 0
    #[unity::class_method(286)] pub fn is_decoy(&self) -> bool; // Offset: 0x1A39D30 Flags: 0
    #[unity::class_method(287)] pub fn add_exp(&self, exp: i32); // Offset: 0x1A39D40 Flags: 0
    #[unity::class_method(288)] pub fn add_sp(&self, sp: i32); // Offset: 0x1A39DB0 Flags: 0
    #[unity::class_method(289)] pub fn can_grow(&self) -> bool; // Offset: 0x1A39DF0 Flags: 0
    // #[unity::class_method(290)] pub fn normalize_exp(&self, exp: i32) -> i32; // Offset: 0x1A39F60 Flags: 0
    // #[unity::class_method(291)] pub fn exp_to_skill_point(&self, exp: i32) -> i32; // Offset: 0x1A39FE0 Flags: 0
    // #[unity::class_method(292)] pub fn nomralize_skill_point(&self, skill_point: i32) -> i32; // Offset: 0x1A3A010 Flags: 0
    #[unity::class_method(293)] pub fn level_up(&self, abort: i32); // Offset: 0x1A3A040 Flags: 0
    #[unity::class_method(294)] pub fn level_down(&self); // Offset: 0x1A3ABA0 Flags: 0
    #[unity::class_method(295)] pub fn try_learn_job_skill(&self) -> Option<&'static SkillData>; // Offset: 0x1A3C290 Flags: 0
    // #[unity::class_method(296)] pub fn learn_job_skill_for_chart(&self, level: i32) -> &'static SkillData; // Offset: 0x1A3C350 Flags: 0
    // #[unity::class_method(297)] pub fn learn_job_skill(&self, level: i32, job: &JobData) -> &'static SkillData; // Offset: 0x1A3C2F0 Flags: 0
    // #[unity::class_method(298)] pub fn learn_job_skill3(&self, job: &JobData) -> &'static SkillData; // Offset: 0x1A3C3B0 Flags: 0
    #[unity::class_method(299)] pub fn class_change_check_aptitude(&self, job: &JobData) -> bool; // Offset: 0x1A3C5F0 Flags: 0
    #[unity::class_method(300)] pub fn class_change_with_item(&self, job: &JobData, item: Option<&ItemData>); // Offset: 0x1A3C7B0 Flags: 0
    #[unity::class_method(301)] pub fn class_change_for_chart(&self, job: &JobData, item: &ItemData, level: i32); // Offset: 0x1A3CC30 Flags: 0
    #[unity::class_method(302)] pub fn update_difficulty(&self); // Offset: 0x1A3CC60 Flags: 0
    #[unity::class_method(303)] pub fn reset_weapon_mask(&self); // Offset: 0x1A0B9B0 Flags: 0
    #[unity::class_method(304)] pub fn set_selected_weapon(&self, selected_weapon_mask: &WeaponMask); // Offset: 0x1A3CD50 Flags: 0
    #[unity::class_method(305)] pub fn set_selected_weapon_from_original_aptitude(&self, aptitude: &WeaponMask); // Offset: 0x1A0BA00 Flags: 0
    #[unity::class_method(306)] pub fn set_weapon_mask_from_parson(&self); // Offset: 0x1A3CD70 Flags: 0
    #[unity::class_method(307)] pub fn set_optimal_weapon_for_class_change(&self, weapon_mask: &WeaponMask, is_bullet: bool) -> &'static UnitItem; // Offset: 0x1A3CDF0 Flags: 0
    // #[unity::class_method(308)] pub fn set_selected_weapon_from_items(&self, items: &UnitItemsForSelectedWeapon); // Offset: 0x1A3D9E0 Flags: 0
    // #[unity::class_method(309)] pub fn set_selected_weapon_for_chart(&self, chart_items: &Array<Item>); // Offset: 0x1A3DCB0 Flags: 0
    #[unity::class_method(310)] pub fn set_selected_weapon_from_dispos(&self, data: &DisposData); // Offset: 0x1A3DD20 Flags: 0
    #[unity::class_method(311)] pub fn inherit_apt(&self, god_unit: &GodUnit); // Offset: 0x1A3DD90 Flags: 0
    #[unity::class_method(312)] pub fn inherit_apt_from_bond(&self, god_bond: &GodBond); // Offset: 0x1A3DDF0 Flags: 0
    // #[unity::class_method(313)] pub fn add_aptitude_for_chart(&self); // Offset: 0x1A3DE40 Flags: 0
    #[unity::class_method(314)] pub fn set_aptitude_from_dispos(&self, data: &DisposData); // Offset: 0x1A3DE50 Flags: 0
    #[unity::class_method(315)] pub fn add_aptitude_from_weapon_mask(&self); // Offset: 0x1A0BF10 Flags: 0
    #[unity::class_method(316)] pub fn update_weapon_mask(&self); // Offset: 0x1A0BD90 Flags: 0
    #[unity::class_method(317)] pub fn ai_activate(&self, attacked: bool); // Offset: 0x1A3E400 Flags: 0
    #[unity::class_method(318)] pub fn ai_activate_cause_attacked(&self, longrange: bool); // Offset: 0x1A3E470 Flags: 0
    #[unity::class_method(319)] pub fn ai_set_engage_attack(&self); // Offset: 0x1A3E4F0 Flags: 0
    #[unity::class_method(320)] pub fn ai_clear_engage_attack(&self); // Offset: 0x1A3E580 Flags: 0
    #[unity::class_method(321)] pub fn has_item(&self, is_exclude_engage: bool) -> bool; // Offset: 0x1A3E610 Flags: 0
    #[unity::class_method(322)] pub fn has_rod(&self) -> bool; // Offset: 0x1A3EA00 Flags: 0
    #[unity::class_method(323)] pub fn has_heal_rod(&self) -> bool; // Offset: 0x1A3EA20 Flags: 0
    // #[unity::class_method(324)] pub fn has_heal_rod_for_oneself(&self, rod_type: ItemDataRodTypes) -> bool; // Offset: 0x1A3EA40 Flags: 0
    #[unity::class_method(325)] pub fn has_support_rod(&self) -> bool; // Offset: 0x1A3EA60 Flags: 0
    // #[unity::class_method(326)] pub fn has_support_rod_for_oneself(&self, rod_type: ItemDataRodTypes) -> bool; // Offset: 0x1A3EA80 Flags: 0
    #[unity::class_method(327)] pub fn has_interference_rod(&self) -> bool; // Offset: 0x1A3EAA0 Flags: 0
    #[unity::class_method(328)] pub fn has_critical_weapon(&self) -> bool; // Offset: 0x1A3EAC0 Flags: 0
    #[unity::class_method(329)] pub fn has_efficacy_weapon(&self, target: &Unit) -> bool; // Offset: 0x1A3EAE0 Flags: 0
    #[unity::class_method(330)] pub fn has_drop_item(&self) -> bool; // Offset: 0x1A3EB00 Flags: 0
    #[unity::class_method(331)] pub fn has_max_item(&self) -> bool; // Offset: 0x1A3EB10 Flags: 0
    #[unity::class_method(332)] pub fn get_item(&self, index: i32) -> &'static UnitItem; // Offset: 0x1A3EB40 Flags: 0
    // #[unity::class_method(333)] pub fn get_item_index(&self, iid: &Il2CppString) -> i32; // Offset: 0x1A3EB50 Flags: 0
    // #[unity::class_method(334)] pub fn get_item_index2(&self, item: &ItemData) -> i32; // Offset: 0x1A3ECF0 Flags: 0
    
    #[unity::class_method(337)] pub fn get_actual_item(&self, unit_item: &UnitItem) -> &'static UnitItem; // Offset: 0x1A3F100 Flags: 0
    #[unity::class_method(338)] pub fn is_item_equipped(&self) -> bool; // Offset: 0x1A3F3B0 Flags: 0
    #[unity::class_method(339)] pub fn get_item_equipped(&self) -> &'static UnitItem; // Offset: 0x1A3F3C0 Flags: 0
    #[unity::class_method(340)] pub fn get_item_index_equipped(&self) -> i32; // Offset: 0x1A3F3D0 Flags: 0
    #[unity::class_method(341)] pub fn get_item_hold(&self) -> &'static UnitItem; // Offset: 0x1A3F3E0 Flags: 0
    #[unity::class_method(342)] pub fn get_item_index_hold(&self) -> i32; // Offset: 0x1A3F3F0 Flags: 0
    #[unity::class_method(343)] pub fn add_item_iid(&self, iid: &Il2CppString) -> i32; // Offset: 0x1A0C990 Flags: 0
    #[unity::class_method(344)] pub fn add_item_iids(&self, iids: &'static Array<&'static Il2CppString>) -> bool; // Offset: 0x1A3F480 Flags: 0
    /// Add item to the unit's inventory without a notification Offset: 0x1A3F40, return inventory index
    #[unity::class_method(345)] pub fn add_item(&self, item: &ItemData) -> i32;
    
    #[unity::class_method(346)] pub fn add_unit_item(&self, unit_item: &UnitItem) -> i32; // Offset: 0x1A0C040 Flags: 0
    // #[unity::class_method(347)] pub fn item_add_on_dlc_evil(&self, iids: &Array<&Il2CppString>, chapter: &ChapterData, level: i32); // Offset: 0x1A3F520 Flags: 0
    #[unity::class_method(348)] pub fn add_equipable_item(&self, iids: &Array<&Il2CppString>) -> bool; // Offset: 0x1A408E0 Flags: 0
    #[unity::class_method(349)] pub fn item_move(&self, from: i32, to: i32); // Offset: 0x1A40FC0 Flags: 0
    #[unity::class_method(350)] pub fn item_close_up(&self); // Offset: 0x1A1F540 Flags: 0
    #[unity::class_method(351)] pub fn item_equip(&self) -> bool; // Offset: 0x1A21530 Flags: 0
    #[unity::class_method(352)] pub fn item_equip_reorder(&self, index: i32, reorder: bool) -> bool; // Offset: 0x1A282D0 Flags: 0
    #[unity::class_method(353)] pub fn item_reorder(&self); // Offset: 0x1A279E0 Flags: 0
    // #[unity::class_method(354)] pub fn item_equip3(&self, unit_item: &UnitItem) -> bool; // Offset: 0x1A40FD0 Flags: 0
    // #[unity::class_method(355)] pub fn item_equip4(&self, item_data: &ItemData) -> bool; // Offset: 0x1A412B0 Flags: 0
    // #[unity::class_method(356)] pub fn item_take_off(&self, index: i32); // Offset: 0x1A41400 Flags: 0
    // #[unity::class_method(357)] pub fn item_take_off2(&self, unit_item: &UnitItem); // Offset: 0x1A3D580 Flags: 0
    // #[unity::class_method(358)] pub fn item_put_off(&self, index: i32, closeup: bool); // Offset: 0x1A41450 Flags: 0
    // #[unity::class_method(359)] pub fn item_put_off2(&self, unit_item: &UnitItem, closeup: bool); // Offset: 0x1A414A0 Flags: 0
    /// Discards all inventory items
    #[unity::class_method(360)] pub fn put_off_all_item(&self); // Offset: 0x1A41770 Flags: 0
    // #[unity::class_method(361)] pub fn has_item2(&self, item: &ItemData) -> bool; // Offset: 0x1A417B0 Flags: 0
    #[unity::class_method(362)] pub fn get_item_count(&self) -> i32; // Offset: 0x1A417C0 Flags: 0
    #[unity::class_method(363)] pub fn get_item_selected(&self) -> &'static UnitItem; // Offset: 0x1A417D0 Flags: 0
    #[unity::class_method(364)] pub fn set_item_selected(&self, unit_item: &UnitItem); // Offset: 0x1A417E0 Flags: 0
    #[unity::class_method(365)] pub fn has_equipable_item(&self) -> bool; // Offset: 0x1A417F0 Flags: 0
    #[unity::class_method(366)] pub fn has_equipable_item_range(&self, range: i32) -> bool; // Offset: 0x1A41E30 Flags: 0
    #[unity::class_method(367)] pub fn has_equipable_item_kind(&self, kind: i32) -> bool; // Offset: 0x1A42510 Flags: 0
    // #[unity::class_method(368)] pub fn can_item_equip(&self, unit_item: &UnitItem, rod: bool, exp: bool) -> bool; // Offset: 0x1A42B50 Flags: 0
    #[unity::class_method(369)] pub fn can_equip_item(&self, item: &ItemData, rod: bool, weapon_level: bool) -> bool; // Offset: 0x1A43120 Flags: 0
    // #[unity::class_method(370)] pub fn can_item_equip3(&self, index: i32, rod: bool, exp: bool) -> bool; // Offset: 0x1A436B0 Flags: 0
    #[unity::class_method(371)] pub fn can_equip_item_kind(&self, kind: i32, rod: bool, weapon_mask: bool) -> bool; // Offset: 0x1A43C90 Flags: 0
    // #[unity::class_method(372)] pub fn can_use_cannon(&self, x: i32, z: i32) -> bool; // Offset: 0x1A43CE0 Flags: 0
    // #[unity::class_method(373)] pub fn can_use_cannon2(&self, terrain: &TerrainData) -> bool; // Offset: 0x1A44A40 Flags: 0
    // #[unity::class_method(374)] pub fn next_item_equip(&self, reverse: bool) -> bool; // Offset: 0x1A456A0 Flags: 0
    // #[unity::class_method(375)] pub fn is_item_sealed(&self, unit_item: &UnitItem) -> bool; // Offset: 0x1A45E50 Flags: 0
    // #[unity::class_method(376)] pub fn is_item_sealed2(&self, item: &ItemData) -> bool; // Offset: 0x1A436A0 Flags: 0
    // #[unity::class_method(377)] pub fn is_item_sealed3(&self, index: i32) -> bool; // Offset: 0x1A45ED0 Flags: 0
    // #[unity::class_method(378)] pub fn get_item_index_key_door(&self) -> i32; // Offset: 0x1A45F60 Flags: 0
    // #[unity::class_method(379)] pub fn can_unlock_door(&self, enable_item: bool) -> bool; // Offset: 0x1A461A0 Flags: 0
    // #[unity::class_method(380)] pub fn can_unlock_treasure_box(&self, enable_item: bool) -> bool; // Offset: 0x1A462A0 Flags: 0
    /*
    #[unity::class_method(381)] pub fn get_range_i(&self, index: i32, skill: &SkillData) -> i32; // Offset: 0x1A462B0 Flags: 0
    #[unity::class_method(382)] pub fn get_range_i2(&self, item: &ItemData, skill: &SkillData) -> i32; // Offset: 0x1A46370 Flags: 0
    #[unity::class_method(383)] pub fn get_range_i3(&self, unit_item: &UnitItem, skill: &SkillData) -> i32; // Offset: 0x1A46460 Flags: 0
    #[unity::class_method(384)] pub fn get_range_o(&self, index: i32, skill: &SkillData) -> i32; // Offset: 0x1A469E0 Flags: 0
    #[unity::class_method(385)] pub fn get_range_o2(&self, item: &ItemData, skill: &SkillData) -> i32; // Offset: 0x1A46B30 Flags: 0
    #[unity::class_method(386)] pub fn get_range_o3(&self, unit_item: &UnitItem, skill: &SkillData) -> i32; // Offset: 0x1A46A90 Flags: 0
    #[unity::class_method(387)] pub fn get_range_io(&self, index: i32, range_i: i32, range_o: i32, skill: &SkillData) -> bool; // Offset: 0x1A46C10 Flags: 0
    #[unity::class_method(388)] pub fn get_range_io2(&self, item: &ItemData, range_i: i32, range_o: i32, skill: &SkillData) -> bool; // Offset: 0x1A46CC0 Flags: 0
    #[unity::class_method(389)] pub fn get_range_io3(&self, unit_item: &UnitItem, range_i: i32, range_o: i32, skill: &SkillData) -> bool; // Offset: 0x1A46DB0 Flags: 0
    #[unity::class_method(390)] pub fn get_item_distance(&self, item: &ItemData) -> i32; // Offset: 0x1A46E60 Flags: 0
    #[unity::class_method(391)] pub fn get_rod_range_extend(&self, item: &ItemData) -> i32; // Offset: 0x1A47040 Flags: 0
    #[unity::class_method(392)] pub fn get_item_range(&self, min_range: i32, max_range: i32, item: &ItemData, skill: &SkillData) -> bool; // Offset: 0x1A47210 Flags: 0
    #[unity::class_method(393)] pub fn get_skill_range(&self, min_range: i32, max_range: i32, skill: &SkillData) -> bool; // Offset: 0x1A47370 Flags: 0
    #[unity::class_method(394)] pub fn get_item_range2(&self, min_range: i32, max_range: i32, equip_item: &UnitItem, skill: &SkillData) -> bool; // Offset: 0x1A473F0 Flags: 0
    #[unity::class_method(395)] pub fn get_item_range3(&self, min_range: i32, max_range: i32, item_mask: u32, skill: &SkillData) -> bool; // Offset: 0x1A47560 Flags: 0
    #[unity::class_method(396)] pub fn get_engage_range(&self, min_range: i32, max_range: i32) -> bool; // Offset: 0x1A476F0 Flags: 0
    #[unity::class_method(397)] pub fn get_attack_range(&self, min_range: i32, max_range: i32, skill: &SkillData) -> bool; // Offset: 0x1A48280 Flags: 0
    #[unity::class_method(398)] pub fn get_rod_range(&self, min_range: i32, max_range: i32, skill: &SkillData) -> bool; // Offset: 0x1A48A70 Flags: 0
    #[unity::class_method(399)] pub fn get_attack_range2(&self, min_range: i32, max_range: i32, equip_item: &UnitItem, skill: &SkillData) -> bool; // Offset: 0x1A49260 Flags: 0
    #[unity::class_method(400)] pub fn get_rod_range2(&self, min_range: i32, max_range: i32, equip_item: &UnitItem, skill: &SkillData) -> bool; // Offset: 0x1A49B90 Flags: 0
    #[unity::class_method(401)] pub fn get_revenge_weapon(&self, target: &Unit, target_item: &UnitItem, range: i32) -> &'static UnitItem; // Offset: 0x1A4A4C0 Flags: 0
    #[unity::class_method(402)] pub fn get_rod_heal_power(&self, unit_item: &UnitItem) -> i32; // Offset: 0x1A4B7C0 Flags: 0
    #[unity::class_method(403)] pub fn can_item_use(&self, item: &ItemData) -> bool; // Offset: 0x1A4BA20 Flags: 0
    #[unity::class_method(404)] pub fn can_item_use2(&self, item: &ItemData, target_unit: &Unit) -> bool; // Offset: 0x1A4C060 Flags: 0
    #[unity::class_method(405)] pub fn item_use_impl(&self, item: &ItemData, use_type: ItemDataUseTypes, power: i32, give_skills: &SkillArray); // Offset: 0x1A4C690 Flags: 0
    #[unity::class_method(406)] pub fn item_use(&self, item: &ItemData); // Offset: 0x1A4D990 Flags: 0
    #[unity::class_method(407)] pub fn item_enchant(&self, unit: &Unit, item: &ItemData); // Offset: 0x1A4D9A0 Flags: 0
    #[unity::class_method(408)] pub fn is_item_enhance_having(&self, item_data: &ItemData) -> bool; // Offset: 0x1A4DA10 Flags: 0
    #[unity::class_method(409)] pub fn is_draw_active_color(&self, unit_item: &UnitItem) -> bool; // Offset: 0x1A4DA20 Flags: 0

     */
    #[unity::class_method(410)] pub fn get_accessory_list(&self) -> &'static UnitAccessoryList; // Offset: 0x1A4DFF0 Flags: 0
    #[unity::class_method(411)] pub fn set_ring(&self, ring: &UnitRing); // Offset: 0x1A4E000 Flags: 0
    #[unity::class_method(412)] pub fn set_ring_impl(&self, ring: &UnitRing); // Offset: 0x1A4E0C0 Flags: 0
    #[unity::class_method(413)] pub fn clear_ring(&self); // Offset: 0x1A4E160 Flags: 0
    #[unity::class_method(414)] pub fn clear_ring_impl(&self); // Offset: 0x1A0EFE0 Flags: 0
    #[unity::class_method(415)] pub fn get_ring(&self) -> Option<&'static UnitRing>; // Offset: 0x1A4E1C0 Flags: 0
    #[unity::class_method(416)] pub fn get_power(&self) -> i32; // Offset: 0x1A4E1D0 Flags: 0
    #[unity::class_method(417)] pub fn set_god_unit(&self, god_unit: &GodUnit); // Offset: 0x1A4F180 Flags: 0
    #[unity::class_method(418)] pub fn set_god_unit_to_copy(&self, god_unit: &GodUnit); // Offset: 0x1A4F380 Flags: 0
    #[unity::class_method(419)] pub fn clear_god_unit(&self); // Offset: 0x1A4F4C0 Flags: 0
    #[unity::class_method(420)] pub fn clear_god_unit_from_copy(&self); // Offset: 0x1A4F5E0 Flags: 0
    #[unity::class_method(421)] pub fn can_god_equip(&self, god_unit: &GodUnit) -> bool; // Offset: 0x1A2B6E0 Flags: 0
    #[unity::class_method(422)] pub fn reset_engage_count(&self); // Offset: 0x1A1A740 Flags: 0
    #[unity::class_method(423)] pub fn play_engage_cancel(&self); // Offset: 0x1A19D00 Flags: 0
    #[unity::class_method(424)] pub fn reset_engaging(&self); // Offset: 0x1A1A590 Flags: 0
    /// Attemps to equip god unit to unit
    #[unity::class_method(425)] pub fn try_connect_god_unit(&self, god_unit: &GodUnit) -> Option<&'static GodUnit>; // Offset: 0x1A4F240 Flags: 0
    #[unity::class_method(426)] pub fn try_connect_god_unit_to_copy(&self, god_unit: &GodUnit) -> &'static GodUnit; // Offset: 0x1A4F3E0 Flags: 0
    #[unity::class_method(427)] pub fn try_disconnect_god_unit(&self) -> Option<&'static GodUnit>; // Offset: 0x1A0EA80 Flags: 0
    #[unity::class_method(428)] pub fn try_disconnect_ring(&self) -> Option<&'static UnitRing>; // Offset: 0x1A1A640 Flags: 0
    #[unity::class_method(429)] pub fn try_disconnect_god_unit_from_copy(&self) -> &'static GodUnit; // Offset: 0x1A4F720 Flags: 0
    #[unity::class_method(430)] pub fn try_disconnect_god_link_from_copy(&self) -> &'static GodUnit; // Offset: 0x1A4F7C0 Flags: 0
    #[unity::class_method(431)] pub fn sight_up(&self, unit: &Unit, item: &ItemData) -> bool; // Offset: 0x1A4D730 Flags: 0
    #[unity::class_method(432)] pub fn can_revive(&self) -> bool; // Offset: 0x1A4F860 Flags: 0
    // #[unity::class_method(433)] pub fn is_asphyxiation(&self) -> bool; // Offset: 0x1A4F880 Flags: 0
    #[unity::class_method(434)] pub fn revive(&self); // Offset: 0x1A4F8B0 Flags: 0
    // #[unity::class_method(435)] pub fn revive_for_rewind_v0(&self, hp_stock_count: u8); // Offset: 0x1A4FAA0 Flags: 0
    // #[unity::class_method(436)] pub fn revive_for_rewind_v1(&self, plain_hp_stock_count: u8, extra_hp_stock_count: u8, extra_hp_stock_count_max: u8); // Offset: 0x1A4FB90 Flags: 0
    // #[unity::class_method(437)] pub fn set_god_state(&self, index: i32, state: GodState); // Offset: 0x1A4FC90 Flags: 0
    // #[unity::class_method(438)] pub fn get_god_state(&self, index: i32) -> GodState; // Offset: 0x1A4FE70 Flags: 0
    // #[unity::class_method(439)] pub fn get_god_state_count(&self) -> i32; // Offset: 0x1A4FED0 Flags: 0
    // #[unity::class_method(440)] pub fn is_default_god_states(&self) -> bool; // Offset: 0x1A4FDD0 Flags: 0
    // #[unity::class_method(441)] pub fn get_current_god_state_index(&self) -> i32; // Offset: 0x1A4FE50 Flags: 0
    // #[unity::class_method(442)] pub fn get_current_god_state(&self) -> GodState; // Offset: 0x1A264B0 Flags: 0
    #[unity::class_method(443)] pub fn can_set_extra_hp_stock(&self) -> bool; // Offset: 0x1A4FEF0 Flags: 0
    // #[unity::class_method(444)] pub fn has_extra_hp_stock(&self) -> bool; // Offset: 0x1A4FF40 Flags: 0
    #[unity::class_method(445)] pub fn set_extra_hp_stock(&self); // Offset: 0x1A4FF50 Flags: 0
    // #[unity::class_method(446)] pub fn clear_extra_hp_stock(&self); // Offset: 0x1A1A670 Flags: 0
    // #[unity::class_method(447)] pub fn extra_hp_stock_for_rewind(&self, is_set: bool, extra_hp_stock_count: u8, extra_hp_stock_count_max: u8); // Offset: 0x1A4FFA0 Flags: 0
    // #[unity::class_method(448)] pub fn dbg_set_hp_stock_count(&self, count: i32); // Offset: 0x1A4FFC0 Flags: 0
    // #[unity::class_method(449)] pub fn update_god_unit_for_god_state(&self, new_god_state: GodState, old_god_state: GodState); // Offset: 0x1A4FA50 Flags: 0
    // #[unity::class_method(450)] pub fn serialize(&self, stream: &Stream); // Offset: 0x1A500E0 Flags: 0
    // #[unity::class_method(451)] pub fn deserialize(&self, stream: &Stream); // Offset: 0x1A506B0 Flags: 0
    // #[unity::class_method(452)] pub fn get_prev(&self) -> &'static Unit; // Offset: 0x1A52260 Flags: 0
    // #[unity::class_method(453)] pub fn set_prev(&self, value: &Unit); // Offset: 0x1A52270 Flags: 0
    // #[unity::class_method(454)] pub fn get_next(&self) -> &'static Unit; // Offset: 0x1A52280 Flags: 0
    // #[unity::class_method(455)] pub fn set_next(&self, value: &Unit); // Offset: 0x1A52290 Flags: 0
    #[unity::class_method(456)] pub fn get_ai(&self) -> &'static mut UnitAI; // Offset: 0x1A522A0 Flags: 0
    #[unity::class_method(457)] pub fn get_edit(&self) -> &'static mut UnitEdit; // Offset: 0x1A522B0 Flags: 0
    #[unity::class_method(458)] pub fn get_person(&self) -> &'static PersonData; // Offset: 0x1A522C0 Flags: 0
    #[unity::class_method(459)] pub fn set_person(&self, value: &PersonData); // Offset: 0x1A522D0 Flags: 0
    // #[unity::class_method(460)] pub fn get_pid(&self) -> &'static Il2CppString; // Offset: 0x1A35BF0 Flags: 0
    // #[unity::class_method(461)] pub fn get_prefixless_pid(&self) -> &'static Il2CppString; // Offset: 0x1A522E0 Flags: 0
    #[unity::class_method(462)] pub fn get_job(&self) -> &'static JobData; // Offset: 0x1A52390 Flags: 0
    #[unity::class_method(463)] pub fn set_job(&self, value: &JobData); // Offset: 0x1A523A0 Flags: 0
    #[unity::class_method(466)] pub fn get_force(&self) -> Option<&'static Force>; // Offset: 0x1A52510 Flags: 0
    #[unity::class_method(467)] pub fn set_force(&self, value: &Force); // Offset: 0x1A52520 Flags: 0
    #[unity::class_method(468)] pub fn get_force_type(&self) -> ForceType; // Offset: 0x1A52530 Flags: 0
    #[unity::class_method(469)] pub fn get_force_mask(&self) -> u32; // Offset: 0x1A52550 Flags: 0
    // #[unity::class_method(470)] pub fn get_base_capability(&self) -> &'static UnitBaseCapability; // Offset: 0x1A52570 Flags: 0
    // #[unity::class_method(471)] pub fn get_grow_capability(&self) -> &'static Capability; // Offset: 0x1A52580 Flags: 0
    // #[unity::class_method(472)] pub fn get_level_capability(&self) -> &'static UnitBaseCapability; // Offset: 0x1A52590 Flags: 0
    #[unity::class_method(473)] pub fn get_grow_seed(&self) -> u32; // Offset: 0x1A525A0 Flags: 0
    #[unity::class_method(474)] pub fn set_grow_seed(&self, value: u32); // Offset: 0x1A525B0 Flags: 0
    #[unity::class_method(475)] pub fn get_drop_seed(&self) -> u32; // Offset: 0x1A525C0 Flags: 0
    #[unity::class_method(476)] pub fn set_drop_seed(&self, value: u32); // Offset: 0x1A525D0 Flags: 0
    #[unity::class_method(477)] pub fn get_actor(&self) -> &'static UnitActor; // Offset: 0x1A525E0 Flags: 0
    #[unity::class_method(478)] pub fn set_actor(&self, value: &UnitActor); // Offset: 0x1A525F0 Flags: 0
    // #[unity::class_method(479)] pub fn get_info(&self) -> &'static MapInfoRoot; // Offset: 0x1A52600 Flags: 0
    // #[unity::class_method(480)] pub fn set_info(&self, value: &MapInfoRoot); // Offset: 0x1A52610 Flags: 0
    // #[unity::class_method(481)] pub fn get_unit_model(&self) -> &'static UnitModel; // Offset: 0x1A52620 Flags: 0
    // #[unity::class_method(482)] pub fn get_god_model(&self) -> &'static UnitModel; // Offset: 0x1A52640 Flags: 0
    // #[unity::class_method(483)] pub fn get_position(&self) -> Vector3<f32>; // Offset: 0x1A52660 Flags: 0
    // #[unity::class_method(484)] pub fn get_cell_center_position(&self) -> Vector3<f32>; // Offset: 0x1A53360 Flags: 0
    // #[unity::class_method(485)] pub fn get_direction(&self) -> Vector3; // Offset: 0x1A54AD0 Flags: 0
    #[unity::class_method(486)] pub fn get_index(&self) -> i32; // Offset: 0x1A54BD0 Flags: 0
    #[unity::class_method(487)] pub fn set_index(&self, value: i32); // Offset: 0x1A54BE0 Flags: 0
    #[unity::class_method(488)] pub fn get_bmap_size(&self) -> i32; // Offset: 0x1A54BF0 Flags: 0
    #[unity::class_method(489)] pub fn get_style(&self) -> BattleStyleTypes; // Offset: 0x1A54C10 Flags: 0
    #[unity::class_method(490)] pub fn get_x(&self) -> i32; // Offset: 0x1A54C20 Flags: 0
    #[unity::class_method(491)] pub fn set_x(&self, value: i32); // Offset: 0x1A22FE0 Flags: 0
    #[unity::class_method(492)] pub fn get_z(&self) -> i32; // Offset: 0x1A54C30 Flags: 0
    #[unity::class_method(493)] pub fn set_z(&self, value: i32); // Offset: 0x1A22FF0 Flags: 0
    #[unity::class_method(494)] pub fn get_dispos_x(&self) -> i32; // Offset: 0x1A54C40 Flags: 0
    #[unity::class_method(495)] pub fn set_dispos_x(&self, value: i32); // Offset: 0x1A54C50 Flags: 0
    #[unity::class_method(496)] pub fn get_dispos_z(&self) -> i32; // Offset: 0x1A54C60 Flags: 0
    #[unity::class_method(497)] pub fn set_dispos_z(&self, value: i32); // Offset: 0x1A54C70 Flags: 0
    // #[unity::class_method(498)] pub fn get_center_x(&self) -> i32; // Offset: 0x1A54C80 Flags: 0
    // #[unity::class_method(499)] pub fn get_center_z(&self) -> i32; // Offset: 0x1A54CA0 Flags: 0
    #[unity::class_method(500)] pub fn get_angle(&self) -> f32; // Offset: 0x1A54CC0 Flags: 0
    #[unity::class_method(501)] pub fn set_angle(&self, value: f32); // Offset: 0x1A54CD0 Flags: 0
    #[unity::class_method(502)] pub fn get_dispos_sound(&self) -> i32; // Offset: 0x1A54CE0 Flags: 0
    #[unity::class_method(503)] pub fn get_move_type(&self) -> JobDataMoveTypes; // Offset: 0x1A54D00 Flags: 0
    #[unity::class_method(504)] pub fn get_level(&self) -> i32; // Offset: 0x1A54D20 Flags: 0
    #[unity::class_method(505)] pub fn set_level(&self, value: i32); // Offset: 0x1A0E1D0 Flags: 0
    // #[unity::class_method(506)] pub fn get_limit_level(&self) -> i32; // Offset: 0x1A54D30 Flags: 0
    // #[unity::class_method(507)] pub fn get_exp(&self) -> i32; // Offset: 0x1A54D40 Flags: 0
    #[unity::class_method(508)] pub fn set_exp(&self, value: i32); // Offset: 0x1A39DA0 Flags: 0
    #[unity::class_method(509)] pub fn get_sp(&self) -> i32; // Offset: 0x1A54D50 Flags: 0
    #[unity::class_method(510)] pub fn set_sp(&self, value: i32); // Offset: 0x1A0E090 Flags: 0
    #[unity::class_method(511)] pub fn get_hp(&self) -> i32; // Offset: 0x1A54D60 Flags: 0
    #[unity::class_method(512)] pub fn set_hp(&self, value: i32); // Offset: 0x1A0E1E0 Flags: 0
    #[unity::class_method(513)] pub fn get_display_hp(&self) -> i32; // Offset: 0x1A54D70 Flags: 0
    // #[unity::class_method(514)] pub fn get_hp_stock_count(&self) -> i32; // Offset: 0x1A54D80 Flags: 0
    // #[unity::class_method(515)] pub fn get_hp_stock_count_max(&self) -> i32; // Offset: 0x1A54D90 Flags: 0
    // #[unity::class_method(516)] pub fn get_plain_hp_stock_count(&self) -> i32; // Offset: 0x1A54DA0 Flags: 0
    // #[unity::class_method(517)] pub fn set_plain_hp_stock_count(&self, value: i32); // Offset: 0x1A54DB0 Flags: 0
    // #[unity::class_method(518)] pub fn get_plain_hp_stock_count_max(&self) -> i32; // Offset: 0x1A54DC0 Flags: 0
    // #[unity::class_method(519)] pub fn get_extra_hp_stock_count(&self) -> i32; // Offset: 0x1A54DD0 Flags: 0
    // #[unity::class_method(520)] pub fn get_extra_hp_stock_count_max(&self) -> i32; // Offset: 0x1A54DE0 Flags: 0
    // #[unity::class_method(521)] pub fn get_base_move_power(&self) -> i32; // Offset: 0x1A54DF0 Flags: 0
    #[unity::class_method(522)] pub fn get_item_list(&self) -> &'static UnitItemList; // Offset: 0x1A54E50 Flags: 0
    #[unity::class_method(523)] pub fn get_god_unit(&self) -> Option<&'static GodUnit>; // Offset: 0x1A54E60 Flags: 0
    #[unity::class_method(524)] pub fn get_actual_god_unit(&self) -> Option<&'static GodUnit>; // Offset: 0x1A54E80 Flags: 0
    #[unity::class_method(525)] pub fn get_extra_sight(&self) -> i32; // Offset: 0x1A54E90 Flags: 0
    #[unity::class_method(526)] pub fn set_extra_sight(&self, value: i32); // Offset: 0x1A54EA0 Flags: 0
    #[unity::class_method(527)] pub fn get_move_distance(&self) -> i32; // Offset: 0x1A54EB0 Flags: 0
    // #[unity::class_method(528)] pub fn get_mask_skill_lock(&self) -> Object; // Offset: 0x1A54EC0 Flags: 0
    #[unity::class_method(529)] pub fn get_mask_skills(&self) -> &'static SkillArray; // Offset: 0x1A54ED0 Flags: 0
    #[unity::class_method(530)] pub fn get_equip_skills(&self) -> &'static SkillArray; // Offset: 0x1A54EE0 Flags: 0
    #[unity::class_method(531)] pub fn get_private_skills(&self) -> &'static SkillArray; // Offset: 0x1A54EF0 Flags: 0
    #[unity::class_method(532)] pub fn get_supported_skills(&self) -> &'static SkillArray; // Offset: 0x1A54F00 Flags: 0
    #[unity::class_method(533)] pub fn get_receive_skills(&self) -> &'static SkillArray; // Offset: 0x1A54F10 Flags: 0
    #[unity::class_method(534)] pub fn get_equip_skill_pool(&self) -> &'static SkillArray; // Offset: 0x1A54F20 Flags: 0
    #[unity::class_method(535)] pub fn get_learned_job_skill(&self) -> Option<&'static SkillData>; // Offset: 0x1A54F30 Flags: 0
    #[unity::class_method(536)] pub fn set_learned_job_skill(&self, value: Option<&SkillData>); // Offset: 0x1A54F40 Flags: 0
    #[unity::class_method(537)] pub fn get_original_aptitude(&self) -> &'static WeaponMask; // Offset: 0x1A54F50 Flags: 0
    #[unity::class_method(538)] pub fn get_aptitude(&self) -> &'static WeaponMask; // Offset: 0x1A54F60 Flags: 0
    #[unity::class_method(539)] pub fn get_weapon_mask(&self) -> &'static mut WeaponMask; // Offset: 0x1A54F70 Flags: 0
    #[unity::class_method(540)] pub fn get_selected_weapon_mask(&self) -> &'static WeaponMask; // Offset: 0x1A54F80 Flags: 0
    #[unity::class_method(541)] pub fn get_enhance_factors(&self) -> Option<&'static UnitEnhanceFactors>; // Offset: 0x1A54F90 Flags: 0
    #[unity::class_method(542)] pub fn get_internal_level(&self) -> i32; // Offset: 0x1A54FA0 Flags: 0
    #[unity::class_method(543)] pub fn set_internal_level(&self, value: i32); // Offset: 0x1A0C740 Flags: 0
    // #[unity::class_method(544)] pub fn get_last_pick_voice(&self) -> i8; // Offset: 0x1A54FB0 Flags: 0
    // #[unity::class_method(545)] pub fn set_last_pick_voice(&self, value: i8); // Offset: 0x1A54FC0 Flags: 0
    #[unity::class_method(546)] pub fn get_max_stun(&self) -> i32; // Offset: 0x1A54FD0 Flags: 0
    #[unity::class_method(547)] pub fn get_stun(&self) -> i32; // Offset: 0x1A54FE0 Flags: 0
    #[unity::class_method(548)] pub fn get_engage_count(&self) -> i32; // Offset: 0x1A550F0 Flags: 0
    #[unity::class_method(549)] pub fn set_engage_count(&self, value: i32); // Offset: 0x1A264A0 Flags: 0
    #[unity::class_method(550)] pub fn play_set_engage_count(&self, engage_count: i32, target_unit: &Unit); // Offset: 0x1A55100 Flags: 0
    // #[unity::class_method(551)] pub fn play_set_engage_count2(&self, start: Vector3, goal: Vector3, count: i32); // Offset: 0x1A57900 Flags: 0
    #[unity::class_method(552)] pub fn get_engage_count_view(&self) -> i32; // Offset: 0x1A57AD0 Flags: 0
    #[unity::class_method(553)] pub fn set_engage_count_view(&self, count: i32); // Offset: 0x1A57B70 Flags: 0
    #[unity::class_method(554)] pub fn get_engage_count_limit(&self) -> i32; // Offset: 0x1A57B90 Flags: 0
    #[unity::class_method(555)] pub fn get_engage_turn(&self) -> i32; // Offset: 0x1A57C30 Flags: 0
    #[unity::class_method(556)] pub fn set_engage_turn(&self, value: i32); // Offset: 0x1A4F850 Flags: 0
    #[unity::class_method(557)] pub fn get_remaining_engage_turn(&self) -> i32; // Offset: 0x1A57C40 Flags: 0
    #[unity::class_method(558)] pub fn get_engage_turn_limit(&self) -> i32; // Offset: 0x1A57D60 Flags: 0
    #[unity::class_method(559)] pub fn get_alpha(&self) -> f32; // Offset: 0x1A57E80 Flags: 0
    //#[unity::class_method(560)] pub fn get_ident(&self) -> i32; // Offset: 0x1A57F90 Flags: 0
    #[unity::class_method(561)] pub fn set_ident(&self, value: i32); // Offset: 0x1A57FA0 Flags: 0
    // #[unity::class_method(562)] pub fn get_record2(&self) -> &'static UnitRecord; // Offset: 0x1A57FB0 Flags: 0
    // #[unity::class_method(563)] pub fn get_map_history_index(&self) -> i32; // Offset: 0x1A57FC0 Flags: 0
    // #[unity::class_method(564)] pub fn set_map_history_index(&self, value: i32); // Offset: 0x1A57FD0 Flags: 0
    // #[unity::class_method(565)] pub fn get_relay_player_index(&self) -> i32; // Offset: 0x1A57FE0 Flags: 0
    // #[unity::class_method(566)] pub fn set_relay_player_index(&self, value: i32); // Offset: 0x1A57FF0 Flags: 0
    #[unity::class_method(567)] pub fn create_impl1(&self, person: &PersonData, job: &JobData, level: i32, random: &Random); // Offset: 0x1A08740 Flags: 0
    #[unity::class_method(568)] pub fn try_copy_edit(&self); // Offset: 0x1A58000 Flags: 0
    #[unity::class_method(569)] pub fn create_impl2(&self); // Offset: 0x1A08B60 Flags: 0
    #[unity::class_method(570)] pub fn set_dispos(&self, data: &DisposData); // Offset: 0x1A08EC0 Flags: 0
    #[unity::class_method(571)] pub fn set_dispos_weapon_mask(&self, data: &DisposData); // Offset: 0x1A58060 Flags: 0
    #[unity::class_method(572)] pub fn set_dispos_skill(&self, data: &DisposData); // Offset: 0x1A58130 Flags: 0
    #[unity::class_method(573)] pub fn set_dispos_god(&self, data: &DisposData); // Offset: 0x1A581E0 Flags: 0
    #[unity::class_method(574)] pub fn add_person_item(&self, person: &PersonData) -> bool; // Offset: 0x1A0B8F0 Flags: 0
    #[unity::class_method(575)] pub fn add_dispos_item(&self, data: &DisposData) -> bool; // Offset: 0x1A0B990 Flags: 0
    #[unity::class_method(576)] pub fn set_dispos_belong(&self, data: &DisposData); // Offset: 0x1A0C0D0 Flags: 0
    #[unity::class_method(577)] pub fn set_dispos_ai(&self, data: &DisposData); // Offset: 0x1A0C0E0 Flags: 0
    // #[unity::class_method(578)] pub fn create_god_states(&self, data: &DisposData) -> &'static Array<GodState>; // Offset: 0x1A582D0 Flags: 0
    // #[unity::class_method(579)] pub fn get_god_state2(&self, data: &DisposData, index: i32, prev_state: GodState) -> GodState; // Offset: 0x1A58450 Flags: 0
    // #[unity::class_method(580)] pub fn dispos_state2_god_state(&self, dispos_state: DisposDataState) -> GodState; // Offset: 0x1A58440 Flags: 0
    #[unity::class_method(581)] pub fn setup_for_vision(&self, original: &Unit); // Offset: 0x1A0D450 Flags: 0
    // #[unity::class_method(582)] pub fn create_for_summon_impl1(&self, person: &PersonData, original: &Unit, rank: PersonDataRanks); // Offset: 0x1A0D650 Flags: 0
    #[unity::class_method(583)] pub fn setup_for_summon(&self, owner: &Unit, rank: i32); // Offset: 0x1A0DB80 Flags: 0
    #[unity::class_method(584)] pub fn create_impl2_exclude_internal_level(&self); // Offset: 0x1A0D0F0 Flags: 0
    #[unity::class_method(585)] pub fn setup_capability_for_vision(&self, original: &Unit); // Offset: 0x1A584C0 Flags: 0
    // #[unity::class_method(586)] pub fn get_safe_person(&self) -> &'static PersonData; // Offset: 0x1A5A540 Flags: 0
    // #[unity::class_method(587)] pub fn get_safe_job(&self) -> &'static JobData; // Offset: 0x1A5A5D0 Flags: 0
    // #[unity::class_method(588)] pub fn calculate_auto_grow_capability(&self, percents: &CapabilityInt); // Offset: 0x1A0E0A0 Flags: 0
    // #[unity::class_method(589)] pub fn get_max_hp_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5A660 Flags: 0
    // #[unity::class_method(590)] pub fn get_str_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5A7F0 Flags: 0
    // #[unity::class_method(591)] pub fn get_tech_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5A970 Flags: 0
    // #[unity::class_method(592)] pub fn get_quick_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5AAF0 Flags: 0
    // #[unity::class_method(593)] pub fn get_luck_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5AC70 Flags: 0
    // #[unity::class_method(594)] pub fn get_def_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5ADF0 Flags: 0
    // #[unity::class_method(595)] pub fn get_magic_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5AF80 Flags: 0
    // #[unity::class_method(596)] pub fn get_mdef_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5B110 Flags: 0
    // #[unity::class_method(597)] pub fn get_phys_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5B2A0 Flags: 0
    // #[unity::class_method(598)] pub fn get_sight_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5B430 Flags: 0
    // #[unity::class_method(599)] pub fn get_move_power_impl(&self, calc_enhance: bool) -> i32; // Offset: 0x1A5B690 Flags: 0
    // #[unity::class_method(600)] pub fn get_capability_impl(&self, capability_type: CapabilityDefinitionType, calc_enhance: bool) -> i32; // Offset: 0x1A5BA20 Flags: 0
    #[unity::class_method(601)] pub fn get_total_level(&self) -> i32; // Offset: 0x1A5BBC0 Flags: 0
    // #[unity::class_method(602)] pub fn get_order(&self) -> i32; // Offset: 0x1A5BBD0 Flags: 0
    // #[unity::class_method(603)] pub fn calc_item_range(&self, item: &ItemData, range_i: i32, range_o: i32, skill: &SkillData) -> bool; // Offset: 0x1A46500 Flags: 0
    // #[unity::class_method(604)] pub fn get_item_grow_capability(&self, item: &ItemData) -> &'static CapabilitySbyte; // Offset: 0x1A4D6D0 Flags: 0
    // #[unity::class_method(605)] pub fn get_attack_image(&self) -> &'static MapDeployAttackImage; // Offset: 0x1A5BC10 Flags: 0
    // #[unity::class_method(606)] pub fn get_rod_image(&self) -> &'static MapDeployRodImage; // Offset: 0x1A5BC20 Flags: 0
    // #[unity::class_method(607)] pub fn get_heal_image(&self) -> &'static MapDeployHealImage; // Offset: 0x1A5BC30 Flags: 0
    // #[unity::class_method(608)] pub fn get_support_image(&self) -> &'static MapDeploySupportImage; // Offset: 0x1A5BC40 Flags: 0
    // #[unity::class_method(609)] pub fn get_interference_image(&self) -> &'static MapDeployInterferenceImage; // Offset: 0x1A5BC50 Flags: 0
    // #[unity::class_method(610)] pub fn get_total_order(&self) -> i32; // Offset: 0x1A5BC60 Flags: 0
    // #[unity::class_method(611)] pub fn set_total_order(&self, value: i32); // Offset: 0x1A5BC70 Flags: 0
    // #[unity::class_method(612)] pub fn get_total_action(&self) -> i32; // Offset: 0x1A5BC80 Flags: 0
    // #[unity::class_method(613)] pub fn set_total_action(&self, value: i32); // Offset: 0x1A5BC90 Flags: 0
    // #[unity::class_method(614)] pub fn get_total_attack(&self) -> i32; // Offset: 0x1A5BCA0 Flags: 0
    // #[unity::class_method(615)] pub fn set_total_attack(&self, value: i32); // Offset: 0x1A5BCB0 Flags: 0
    // #[unity::class_method(616)] pub fn get_total_damage(&self) -> i32; // Offset: 0x1A5BCC0 Flags: 0
    // #[unity::class_method(617)] pub fn set_total_damage(&self, value: i32); // Offset: 0x1A5BCD0 Flags: 0
    // #[unity::class_method(618)] pub fn get_total_result(&self) -> BattleSceneResult; // Offset: 0x1A5BCE0 Flags: 0
    // #[unity::class_method(619)] pub fn set_total_result(&self, value: BattleSceneResult); // Offset: 0x1A5BCF0 Flags: 0
    // #[unity::class_method(620)] pub fn get_side_type(&self) -> BattleSideType; // Offset: 0x1A5BD00 Flags: 0
    // #[unity::class_method(621)] pub fn set_side_type(&self, value: BattleSideType); // Offset: 0x1A5BD10 Flags: 0
    // #[unity::class_method(622)] pub fn update_image(&self, deploy: &MapDnagerDeploy); // Offset: 0x1A5BD20 Flags: 0
    // #[unity::class_method(623)] pub fn clear_image(&self); // Offset: 0x1A0EB20 Flags: 0
    // #[unity::class_method(624)] pub fn try_get_engage_mind(&self, mind: MapMindType) -> bool; // Offset: 0x1A5BFD0 Flags: 0
    #[unity::class_method(625)] pub fn get_engage_attack(&self) -> Option<&'static SkillData>; // Offset: 0x1A21460 Flags: 0
    #[unity::class_method(626)] pub fn get_engage_skills(&self) -> &'static SkillArray; // Offset: 0x1A21440 Flags: 0
    #[unity::class_method(627)] pub fn get_engage_skill(&self, index: i32) -> &'static SkillData; // Offset: 0x1A5C120 Flags: 0
    #[unity::class_method(628)] pub fn get_support_skill(&self) -> &'static SkillData; // Offset: 0x1A5C150 Flags: 0
    // #[unity::class_method(629)] pub fn add_cannon_skill(&self, inspector: &CannonInspector); // Offset: 0x1A5C380 Flags: 0
    // #[unity::class_method(630)] pub fn remove_cannon_skill(&self, inspector: &CannonInspector); // Offset: 0x1A5C4C0 Flags: 0
    #[unity::class_method(631)] pub fn get_command_give_skills(skill: &SkillData) -> &'static SkillArray; // Offset: 0x1A5C560 Flags: 0
    #[unity::class_method(632)] pub fn add_command_skill(&self, skill: &SkillData); // Offset: 0x1A5C600 Flags: 0
    #[unity::class_method(633)] pub fn remove_command_skill(&self, skill: &SkillData); // Offset: 0x1A5CE80 Flags: 0
    #[unity::class_method(634)] pub fn add_give_skill(&self, give: &SkillData); // Offset: 0x1A5D430 Flags: 0
    #[unity::class_method(635)] pub fn add_give_skills(&self, gives: &SkillArray); // Offset: 0x1A5D4E0 Flags: 0
    #[unity::class_method(636)] pub fn get_battle_temporary(&self) -> i32; // Offset: 0x1A5D6D0 Flags: 0
    #[unity::class_method(637)] pub fn set_battle_temporary(&self, value: i32); // Offset: 0x1A5D6E0 Flags: 0
    #[unity::class_method(638)] pub fn is_exist_die(&self) -> bool; // Offset: 0x1A22540 Flags: 0
    // #[unity::class_method(639)] pub fn update_move_distance(&self); // Offset: 0x1A5D6F0 Flags: 0
    // #[unity::class_method(640)] pub fn update_move_distance2(&self, x: i32, z: i32); // Offset: 0x1A5D700 Flags: 0
    // #[unity::class_method(641)] pub fn update_move_distance3(&self, route: &MapRoute, x: i32, z: i32); // Offset: 0x1A5D840 Flags: 0
    // #[unity::class_method(642)] pub fn clear_move_distance(&self); // Offset: 0x1A1E8B0 Flags: 0
    // #[unity::class_method(643)] pub fn can_grow_up(&self, type: CapabilityDefinitionType) -> bool; // Offset: 0x1A5D980 Flags: 0
    // #[unity::class_method(644)] pub fn get_simple_power(&self) -> i32; // Offset: 0x1A5DAE0 Flags: 0
    // #[unity::class_method(645)] pub fn get_simple_physical_power(&self) -> i32; // Offset: 0x1A5DB10 Flags: 0
    // #[unity::class_method(646)] pub fn get_simple_magic_power(&self) -> i32; // Offset: 0x1A5DB40 Flags: 0
    // #[unity::class_method(647)] pub fn get_simple_physical_defense(&self) -> i32; // Offset: 0x1A5DB70 Flags: 0
    // #[unity::class_method(648)] pub fn get_simple_magic_defense(&self) -> i32; // Offset: 0x1A5DBA0 Flags: 0
    // #[unity::class_method(649)] pub fn get_simple_hit(&self) -> i32; // Offset: 0x1A5DBD0 Flags: 0
    // #[unity::class_method(650)] pub fn get_simple_avoid(&self) -> i32; // Offset: 0x1A5DC00 Flags: 0
    // #[unity::class_method(651)] pub fn get_simple_critical(&self) -> i32; // Offset: 0x1A5DC30 Flags: 0
    // #[unity::class_method(652)] pub fn get_simple_secure(&self) -> i32; // Offset: 0x1A5DC60 Flags: 0
    // #[unity::class_method(653)] pub fn get_simple_continuous(&self) -> i32; // Offset: 0x1A5DC90 Flags: 0
    // #[unity::class_method(654)] pub fn reset_param(&self); // Offset: 0x1A5DCC0 Flags: 0
    // #[unity::class_method(655)] pub fn get_fortune_seed(&self) -> u32; // Offset: 0x1A5DEF0 Flags: 0
    // #[unity::class_method(656)] pub fn set_fortune_seed(&self, value: u32); // Offset: 0x1A5DF00 Flags: 0
    // #[unity::class_method(657)] pub fn get_fortune_target(&self) -> &'static PersonData; // Offset: 0x1A5DF10 Flags: 0
    // #[unity::class_method(658)] pub fn set_fortune_target(&self, value: &PersonData); // Offset: 0x1A5DF20 Flags: 0
    // #[unity::class_method(659)] pub fn get_fortune_random_value(&self, update: bool) -> i32; // Offset: 0x1A5DF30 Flags: 0
    // #[unity::class_method(660)] pub fn get_weapon_level(&self, kind: i32, calc_enhance: bool) -> i32; // Offset: 0x1A0C050 Flags: 0
    // #[unity::class_method(661)] pub fn get_skill_equip(&self, skill: &SkillData, index: i32) -> &'static UnitItem; // Offset: 0x1A29830 Flags: 0
    // #[unity::class_method(662)] pub fn get_engage_equip(&self, skill: &SkillData, target: &Unit) -> &'static UnitItem; // Offset: 0x1A478C0 Flags: 0
    // #[unity::class_method(663)] pub fn get_effect_position(&self) -> Vector3; // Offset: 0x1A56C80 Flags: 0
    // #[unity::class_method(664)] pub fn can_stay_without_god(&self) -> bool; // Offset: 0x1A5DFD0 Flags: 0
}