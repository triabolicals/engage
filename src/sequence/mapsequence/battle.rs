pub use unity::prelude::*;
use super::*;
use crate::battle::{BattleCalculator, BattleInfo};
use crate::util::get_singleton_proc_instance;

#[unity::class("App", "MapSequenceBattle")]
pub struct MapSequenceBattle {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub is_loaded: bool,
    pub info: &'static BattleInfo,
    pub sum_info: &'static BattleInfo,
    pub calculator: &'static BattleCalculator,
    pub sim_calculator: &'static BattleCalculator,
    reliance: *const u8,

}

impl Bindable for MapSequenceBattle {}

impl MapSequenceBattle {
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<MapSequenceBattle>() }
    #[unity::class_method(1)] pub fn mind_start(&self); // Offset: 0x23B3D40 Flags: 0
    #[unity::class_method(2)] pub fn mind_end(&self); // Offset: 0x23B3F00 Flags: 0
    #[unity::class_method(3)] pub fn calc_battle(&self); // Offset: 0x23B3FB0 Flags: 0
    #[unity::class_method(5)] pub fn start_branch(&self); // Offset: 0x23B4870 Flags: 0
    #[unity::class_method(6)] pub fn calc_simple(&self, is_warmup: bool); // Offset: 0x23B43C0 Flags: 0
    #[unity::class_method(7)] pub fn calc_simple2(&self, info: &BattleInfo, is_warmup: bool, is_simulation: bool); // Offset: 0x23B4AB0 Flags: 0
    #[unity::class_method(74)] pub fn to_pre_bgm(&self); // Offset: 0x23BC8B0 Flags: 0
    /*
    #[unity::class_method(4)] pub fn engage_attack_telop(&self); // Offset: 0x23B4550 Flags: 0
    #[unity::class_method(8)] pub fn multi_battle_warmup(&self); // Offset: 0x23B4D60 Flags: 0
    #[unity::class_method(10)] pub fn multi_battle_combat(&self); // Offset: 0x23B4E20 Flags: 0
    #[unity::class_method(11)] pub fn multi_battle_branch(&self); // Offset: 0x23B4F10 Flags: 0
    #[unity::class_method(12)] pub fn multi_battle_end(&self); // Offset: 0x23B5090 Flags: 0
    #[unity::class_method(13)] pub fn multi_battle_next(&self); // Offset: 0x23B50A0 Flags: 0
    #[unity::class_method(14)] pub fn multi_battle_result(&self); // Offset: 0x23B50B0 Flags: 0
    #[unity::class_method(15)] pub fn multi_battle_grow(&self); // Offset: 0x23B50F0 Flags: 0
    #[unity::class_method(16)] pub fn try_full_bullet_expend(&self) -> bool; // Offset: 0x23B5250 Flags: 0
    #[unity::class_method(17)] pub fn mult_battle_expend(&self); // Offset: 0x23B5960 Flags: 0
    #[unity::class_method(18)] pub fn battle_branch(&self); // Offset: 0x23B5C30 Flags: 0
    #[unity::class_method(19)] pub fn combat_rotation(&self); // Offset: 0x23B5D00 Flags: 0
    #[unity::class_method(20)] pub fn combat_battle(&self); // Offset: 0x23B4ED0 Flags: 0
    #[unity::class_method(21)] pub fn combat_branch(&self); // Offset: 0x23B5F10 Flags: 0
    #[unity::class_method(22)] pub fn simple_battle(&self); // Offset: 0x23B5FE0 Flags: 0
    #[unity::class_method(23)] pub fn multi_battle_once(&self); // Offset: 0x23B78A0 Flags: 0
    #[unity::class_method(24)] pub fn battle_update(&self); // Offset: 0x23B7990 Flags: 0
    #[unity::class_method(25)] pub fn commit_battle(&self); // Offset: 0x23B7A20 Flags: 0
    #[unity::class_method(26)] pub fn is_show_skill(skill: &SkillData) -> bool; // Offset: 0x23B7A90 Flags: 0
    #[unity::class_method(34)] pub fn try_warp_impl(&self, rod_unit: &Unit, target: &Unit, x: i32, z: i32) -> bool; // Offset: 0x23B7EF0 Flags: 0
    #[unity::class_method(35)] pub fn try_warp(&self, rod_unit: &Unit, target: &Unit, x: i32, z: i32) -> bool; // Offset: 0x23B8070 Flags: 0
    #[unity::class_method(36)] pub fn range_warp(&self, rod_unit: &Unit, unit: &Unit, range: i32, warp_x: i32, warp_z: i32, can_self: bool); // Offset: 0x23B8110 Flags: 0
    #[unity::class_method(37)] pub fn range_rescue(&self, rod_unit: &Unit, unit: &Unit, range: i32); // Offset: 0x23B8420 Flags: 0
    #[unity::class_method(38)] pub fn range_torch(&self, unit: &Unit, range: i32, item: &ItemData); // Offset: 0x23B8760 Flags: 0
    #[unity::class_method(39)] pub fn get_action_callback(&self); // Offset: 0x23B5DC0 Flags: 0
    #[unity::class_method(40)] pub fn use_rod(&self); // Offset: 0x23B9160 Flags: 0
    #[unity::class_method(41)] pub fn use_rest(&self, scene: &BattleScene); // Offset: 0x23BA010 Flags: 0
    #[unity::class_method(42)] pub fn use_move(&self); // Offset: 0x23BA320 Flags: 0
    #[unity::class_method(43)] pub fn use_focus(&self); // Offset: 0x23BA240 Flags: 0
    #[unity::class_method(44)] pub fn play_engage_turn_recovery_effect(unit: &Unit); // Offset: 0x23BA510 Flags: 0
    #[unity::class_method(45)] pub fn engage_turn_recovery_unit(unit: &Unit); // Offset: 0x23BA5A0 Flags: 0
    #[unity::class_method(48)] pub fn destroy_action(&self); // Offset: 0x23BA730 Flags: 0
    #[unity::class_method(49)] pub fn destroy_after(&self); // Offset: 0x23BA740 Flags: 0
    #[unity::class_method(50)] pub fn update_terrain(&self); // Offset: 0x23BA7B0 Flags: 0
    #[unity::class_method(51)] pub fn try_update_info(&self, side: BattleSideType); // Offset: 0x23BAB20 Flags: 0
    #[unity::class_method(52)] pub fn try_update_info2(&self, unit: &Unit); // Offset: 0x23BAC00 Flags: 0
    #[unity::class_method(53)] pub fn update_map_info_unit(&self); // Offset: 0x23BAC90 Flags: 0
    #[unity::class_method(54)] pub fn terrain_set(&self, side: &BattleInfoSide) -> bool; // Offset: 0x23BAA00 Flags: 0
    #[unity::class_method(55)] pub fn can_gain_situation(&self) -> bool; // Offset: 0x23BAE40 Flags: 0
    #[unity::class_method(56)] pub fn get_winner(&self, dead: &Unit) -> &'static Unit; // Offset: 0x23BAF00 Flags: 0
    #[unity::class_method(58)] pub fn pickup_item(&self); // Offset: 0x23BB1D0 Flags: 0
    #[unity::class_method(59)] pub fn gain_gold(&self); // Offset: 0x23BB3F0 Flags: 0
    #[unity::class_method(60)] pub fn item_put_off(&self, unit: &Unit, unit_item: &UnitItem, mid: &Il2CppString); // Offset: 0x23B5470 Flags: 0
    #[unity::class_method(61)] pub fn item_expend(&self); // Offset: 0x23B5A70 Flags: 0
    #[unity::class_method(62)] pub fn blow(&self); // Offset: 0x23BB5A0 Flags: 0
    #[unity::class_method(63)] pub fn decrease_shell(&self); // Offset: 0x23BB7D0 Flags: 0
    #[unity::class_method(64)] pub fn try_add_dead(&self, unit: &Unit); // Offset: 0x23BB920 Flags: 0
    #[unity::class_method(66)] pub fn grow(&self); // Offset: 0x23BB9C0 Flags: 0
    #[unity::class_method(67)] pub fn battle_before_event(&self); // Offset: 0x23BB9E0 Flags: 0
    #[unity::class_method(68)] pub fn battle_after_event(&self); // Offset: 0x23BBA90 Flags: 0
    #[unity::class_method(69)] pub fn is_command_skill(&self, skill: &SkillData, before: bool) -> bool; // Offset: 0x23BBB40 Flags: 0
    #[unity::class_method(70)] pub fn command_skill_commit(&self, side: &BattleInfoSide, result: MapSkillResult); // Offset: 0x23BBB70 Flags: 0
    #[unity::class_method(71)] pub fn command_skill_before(&self); // Offset: 0x23BBB90 Flags: 0
    #[unity::class_method(72)] pub fn command_skill_after(&self); // Offset: 0x23BC2A0 Flags: 0
    #[unity::class_method(73)] pub fn gain_gurad_unit(&self); // Offset: 0x23BC6B0 Flags: 0
    #[unity::class_method(75)] pub fn to_main_bgm(&self); // Offset: 0x23BC930 Flags: 0
    #[unity::class_method(76)] pub fn return_bgm(&self); // Offset: 0x23BC9B0 Flags: 0
    #[unity::class_method(77)] pub fn sound_after_battle(&self); // Offset: 0x23BCA30 Flags: 0
    #[unity::class_method(78)] pub fn try_combat_after_die(&self); // Offset: 0x23BCC00 Flags: 0
    #[unity::class_method(79)] pub fn try_combat_after_grow(&self); // Offset: 0x23BCCC0 Flags: 0
    #[unity::class_method(80)] pub fn process_dead_unit(&self); // Offset: 0x23BCD60 Flags: 0
    #[unity::class_method(81)] pub fn focus_mind(&self); // Offset: 0x23BD350 Flags: 0
    */
}


#[unity::class("App", "MapSequenceBattleAction")]
pub struct MapSequenceBattleAction {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub is_loaded: bool,
    pub calculator: &'static BattleCalculator,
    pub sim_calculator: &'static BattleCalculator,
    signal: *const u8,
    pub scene_index: i32,
    pub battle_count: i32,
    pub attack_count: i32,
    pub info_wait: f32,
    pub attack_side: i32,
}
impl Bindable for MapSequenceBattleAction {}
impl MapSequenceBattleAction {
    pub fn get_instance() -> Option<&'static mut Self> {
        get_singleton_proc_instance::<Self>()
    }
    #[unity::class_method(56)] pub fn to_pre_bgm(&self); // Offset: 0x23C41E0 Flags: 0
    #[unity::class_method(57)] pub fn to_main_bgm(&self); // Offset: 0x23C4270 Flags: 0
    #[unity::class_method(58)] pub fn return_bgm(&self); // Offset: 0x23C4300 Flags: 0
    #[unity::class_method(59)] pub fn play_skill(&self); // Offset: 0x23C4390 Flags: 0
    /*
    #[unity::class_method(0)] pub fn get_can_wait_skip(&self) -> bool; // Offset: 0x23C0530 Flags: 0
    #[unity::class_method(1)] pub fn ctor(&self, calculator: &BattleCalculator, sim_calculator: &BattleCalculator, battle_count: i32); // Offset: 0x23C0540 Flags: 0
    #[unity::class_method(2)] pub fn on_create(&self); // Offset: 0x23C0610 Flags: 0
    #[unity::class_method(3)] pub fn on_dispose(&self); // Offset: 0x23C0770 Flags: 0
    #[unity::class_method(4)] pub fn get_battle_count(&self) -> i32; // Offset: 0x23C0820 Flags: 0
    #[unity::class_method(5)] pub fn get_scene_index(&self) -> i32; // Offset: 0x23C0830 Flags: 0
    // #[unity::class_method(6)] pub fn get_scene_list(&self) -> &'static BattleSceneList; // Offset: 0x23C0840 Flags: 0
    // #[unity::class_method(7)] pub fn get_current_scene(&self) -> &'static BattleScene; // Offset: 0x23C0850 Flags: 0
    #[unity::class_method(8)] pub fn next_scene(&self); // Offset: 0x23C08C0 Flags: 0
    #[unity::class_method(9)] pub fn is_show_info(&self) -> bool; // Offset: 0x23C08D0 Flags: 0
    #[unity::class_method(10)] pub fn show_info(&self); // Offset: 0x23C0910 Flags: 0
    #[unity::class_method(11)] pub fn out_info(&self); // Offset: 0x23C09B0 Flags: 0
    #[unity::class_method(12)] pub fn hide_info(&self); // Offset: 0x23C0A50 Flags: 0
    #[unity::class_method(13)] pub fn wait_info(&self); // Offset: 0x23C0AF0 Flags: 0
    #[unity::class_method(14)] pub fn is_multi_battle(&self) -> bool; // Offset: 0x23C0900 Flags: 0
    #[unity::class_method(15)] pub fn is_cannon_battle(&self) -> bool; // Offset: 0x23C0C80 Flags: 0
    #[unity::class_method(16)] pub fn is_full_bullet(&self) -> bool; // Offset: 0x23C0C90 Flags: 0
    #[unity::class_method(17)] pub fn is_rod_battle(&self) -> bool; // Offset: 0x23C0CA0 Flags: 0
    #[unity::class_method(18)] pub fn is_talk(&self) -> bool; // Offset: 0x23C0CB0 Flags: 0
    #[unity::class_method(19)] pub fn battle_talk(&self); // Offset: 0x23C0D00 Flags: 0
    #[unity::class_method(20)] pub fn play_action(&self, current: &BattleInfoSide, action: UnitSequenceAction); // Offset: 0x23C0D60 Flags: 0
    #[unity::class_method(21)] pub fn play_animation(&self, current: &BattleInfoSide, ty: UnitAnimTypes); // Offset: 0x23C0E20 Flags: 0
    #[unity::class_method(22)] pub fn play_rotation(&self, current: &BattleInfoSide, target: &BattleInfoSide); // Offset: 0x23C0ED0 Flags: 0
    #[unity::class_method(23)] pub fn play_rotation_and_action(&self, current: &BattleInfoSide, target: &BattleInfoSide, action: UnitSequenceAction); // Offset: 0x23C1030 Flags: 0
    #[unity::class_method(24)] pub fn play_cannon_rotation(&self, current: &BattleInfoSide, target: &BattleInfoSide); // Offset: 0x23C10F0 Flags: 0
    #[unity::class_method(25)] pub fn play_cannon_shoot(&self, current: &BattleInfoSide); // Offset: 0x23C1280 Flags: 0
    #[unity::class_method(26)] pub fn stop_cannon_shoot(&self, current: &BattleInfoSide); // Offset: 0x23C1380 Flags: 0
    #[unity::class_method(27)] pub fn play_engage_attack_skill(&self); // Offset: 0x23C1460 Flags: 0
    #[unity::class_method(28)] pub fn play_engage_shoot_skill(&self, current: &BattleInfoSide); // Offset: 0x23C1650 Flags: 0
    #[unity::class_method(29)] pub fn get_side(&self, type: BattleSideType) -> &'static BattleInfoSide; // Offset: 0x23C1850 Flags: 0
    #[unity::class_method(30)] pub fn get_unit(&self, type: BattleSideType) -> &'static Unit; // Offset: 0x23C15C0 Flags: 0
    #[unity::class_method(31)] pub fn update_scene(&self, index: i32); // Offset: 0x23C18E0 Flags: 0
    #[unity::class_method(32)] pub fn skip_scene(&self); // Offset: 0x23C19A0 Flags: 0
    #[unity::class_method(33)] pub fn get_attack_motion(&self, unit_item: &UnitItem) -> UnitAnimTypes; // Offset: 0x23C1A70 Flags: 0
    #[unity::class_method(34)] pub fn is_engage_attack_action(&self, scene: &BattleScene) -> bool; // Offset: 0x23C1B10 Flags: 0
    #[unity::class_method(35)] pub fn get_attack_motion2(&self, scene: &BattleScene) -> UnitAnimTypes; // Offset: 0x23C1BB0 Flags: 0
    #[unity::class_method(36)] pub fn wait_skip_signal(&self, signal: &UnitSignal); // Offset: 0x23C1E10 Flags: 0
    #[unity::class_method(37)] pub fn shoot_signal(&self, signal: &UnitSignal); // Offset: 0x23C1E20 Flags: 0
    #[unity::class_method(38)] pub fn clear_signal(&self); // Offset: 0x23C06C0 Flags: 0
    #[unity::class_method(39)] pub fn try_focus_attack(&self); // Offset: 0x23C1E60 Flags: 0
    #[unity::class_method(40)] pub fn try_focus_target(&self); // Offset: 0x23C1F70 Flags: 0
    #[unity::class_method(41)] pub fn landing(&self) -> &'static IEnumerator; // Offset: 0x23C21C0 Flags: 0
    #[unity::class_method(42)] pub fn get_rotation(current: &BattleInfoSide, target: &BattleInfoSide) -> Quaternion; // Offset: 0x23C1230 Flags: 0
    #[unity::class_method(43)] pub fn get_hit_effect(&self, scene: &BattleScene) -> &'static EffectData; // Offset: 0x23C2240 Flags: 0
    #[unity::class_method(44)] pub fn get_rod_effect(&self, scene: &BattleScene) -> &'static EffectData; // Offset: 0x23C25D0 Flags: 0
    #[unity::class_method(45)] pub fn get_offense(&self, scene: &BattleScene) -> &'static BattleInfoSide; // Offset: 0x23C26F0 Flags: 0
    #[unity::class_method(46)] pub fn get_defense(&self, scene: &BattleScene) -> &'static BattleInfoSide; // Offset: 0x23C2780 Flags: 0
    #[unity::class_method(47)] pub fn play_hit_effect(&self, scene: &BattleScene); // Offset: 0x23C2870 Flags: 0
    #[unity::class_method(48)] pub fn play_hit_sound(&self, scene: &BattleScene); // Offset: 0x23C2D00 Flags: 0
    #[unity::class_method(49)] pub fn play_popup(&self, scene: &BattleScene); // Offset: 0x23C2F50 Flags: 0
    #[unity::class_method(50)] pub fn play_damage(&self, scene: &BattleScene); // Offset: 0x23C3280 Flags: 0
    #[unity::class_method(51)] pub fn wait_loading(&self) -> &'static IEnumerator; // Offset: 0x23C34B0 Flags: 0
    #[unity::class_method(52)] pub fn on_persistent(&self); // Offset: 0x23C3530 Flags: 0
    #[unity::class_method(53)] pub fn set_model_item(&self, side: &BattleInfoSide, unit_item: &UnitItem); // Offset: 0x23C36A0 Flags: 0
    #[unity::class_method(54)] pub fn begin(&self); // Offset: 0x23C37E0 Flags: 0
    #[unity::class_method(55)] pub fn end(&self); // Offset: 0x23C3BF0 Flags: 0
    #[unity::class_method(56)] pub fn to_pre_bgm(&self); // Offset: 0x23C41E0 Flags: 0
    #[unity::class_method(57)] pub fn to_main_bgm(&self); // Offset: 0x23C4270 Flags: 0
    #[unity::class_method(58)] pub fn return_bgm(&self); // Offset: 0x23C4300 Flags: 0
    #[unity::class_method(59)] pub fn play_skill(&self); // Offset: 0x23C4390 Flags: 0
    #[unity::class_method(60)] pub fn focus_attack(&self); // Offset: 0x23C45A0 Flags: 0
    #[unity::class_method(61)] pub fn play_attack(&self); // Offset: 0x23C46A0 Flags: 0
    #[unity::class_method(62)] pub fn stop_attack(&self); // Offset: 0x23C48E0 Flags: 0
    #[unity::class_method(63)] pub fn start_signal(&self); // Offset: 0x23C49F0 Flags: 0
    #[unity::class_method(64)] pub fn impact(&self); // Offset: 0x23C4CD0 Flags: 0
    #[unity::class_method(65)] pub fn commit(&self); // Offset: 0x23C5380 Flags: 0
    #[unity::class_method(66)] pub fn branch(&self); // Offset: 0x23C5390 Flags: 0
    #[unity::class_method(67)] pub fn next_wait(&self); // Offset: 0x23C55C0 Flags: 0
    #[unity::class_method(68)] pub fn wait(&self); // Offset: 0x23C56F0 Flags: 0
    #[unity::class_method(69)] pub fn wait_camera(&self); // Offset: 0x23C5740 Flags: 0
    #[unity::class_method(70)] pub fn try_white_out(&self); // Offset: 0x23C5800 Flags: 0
    #[unity::class_method(71)] pub fn has_actor(side: &BattleInfoSide) -> bool; // Offset: 0x23C58A0 Flags: 0
    #[unity::class_method(72)] pub fn try_trans_on(&self) -> &'static IEnumerator; // Offset: 0x23C5930 Flags: 0
    #[unity::class_method(73)] pub fn try_trans_off(&self) -> &'static IEnumerator; // Offset: 0x23C59B0 Flags: 0
     */
}
