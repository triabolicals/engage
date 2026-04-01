use unity::prelude::*;
use crate::{
    battle::BattleInfo,
    gamedata::GodData,
    god::GodUnit,
    proc::ProcInst,
    transporter::TransporterData,
    unit::{Unit, UnitItem},
};
use crate::force::ForceType;
use crate::gamedata::ai::AIValueOrder;
use crate::gamedata::item::ItemDataKinds;
use crate::map::inspectors::CannonInspector;
use crate::map::mind::MapMindType;
use crate::stream::Stream;
use crate::unit::UnitRecordKinds;

#[unity::class("App", "MapHistory")] pub struct MapHistory {}

impl MapHistory {
    #[unity::class_method(0)] pub fn rewind_create(); // Offset: 0x1DD6200 Flags: 0
    #[unity::class_method(1)] pub fn replay_create(); // Offset: 0x1DD6320 Flags: 0
    #[unity::class_method(2)] pub fn rewind_is_created() -> bool; // Offset: 0x1DD63F0 Flags: 0
    #[unity::class_method(3)] pub fn replay_is_created() -> bool; // Offset: 0x1DD64A0 Flags: 0
    #[unity::class_method(4)] pub fn delete(); // Offset: 0x1DD6550 Flags: 0
    #[unity::class_method(5)] pub fn serialize(stream: &Stream); // Offset: 0x1DD6650 Flags: 0
    #[unity::class_method(6)] pub fn overwrite(stream: &Stream, pos: i32, val: i32); // Offset: 0x1DD69B0 Flags: 0
    #[unity::class_method(7)] pub fn deserialize(stream: &Stream); // Offset: 0x1DD6A20 Flags: 0
    #[unity::class_method(8)] pub fn begin(proc: &ProcInst); // Offset: 0x1DD6DE0 Flags: 0
    #[unity::class_method(9)] pub fn end(); // Offset: 0x1DD6F20 Flags: 0
    #[unity::class_method(10)] pub fn phase_begin(); // Offset: 0x1DD7050 Flags: 0
    #[unity::class_method(11)] pub fn phase_next(); // Offset: 0x1DD7180 Flags: 0
    #[unity::class_method(12)] pub fn pick_up(unit: &Unit); // Offset: 0x1DD72B0 Flags: 0
    #[unity::class_method(13)] pub fn cancel_pick_up(); // Offset: 0x1DD7380 Flags: 0
    #[unity::class_method(14)] pub fn cancel_unit_command(); // Offset: 0x1DD7440 Flags: 0
    #[unity::class_method(15)] pub fn mind(); // Offset: 0x1DD7500 Flags: 0
    #[unity::class_method(16)] pub fn talk(from_unit: &Unit, to_unit: &Unit); // Offset: 0x1DD7630 Flags: 0
    #[unity::class_method(17)] pub fn pretrade(from_unit: &Unit, to_unit: &Unit); // Offset: 0x1DD7710 Flags: 0
    #[unity::class_method(18)] pub fn pretrade_undone(unit: &Unit); // Offset: 0x1DD77F0 Flags: 0
    #[unity::class_method(19)] pub fn posttrade(from_unit: &Unit, to_unit: &Unit); // Offset: 0x1DD78C0 Flags: 0
    #[unity::class_method(20)] pub fn posttrade_undone(unit: &Unit); // Offset: 0x1DD79A0 Flags: 0
    #[unity::class_method(21)] pub fn transporter(unit: &Unit); // Offset: 0x1DD7A70 Flags: 0
    #[unity::class_method(22)] pub fn battle_calc(info: &BattleInfo); // Offset: 0x1DD7B40 Flags: 0
    #[unity::class_method(23)] pub fn gain_item(unit: &Unit, unit_item: &UnitItem); // Offset: 0x1DD7C10 Flags: 0
    #[unity::class_method(24)] pub fn preequip_item(unit: &Unit); // Offset: 0x1DD7CF0 Flags: 0
    #[unity::class_method(25)] pub fn postequip_item(unit: &Unit); // Offset: 0x1DD7DC0 Flags: 0
    #[unity::class_method(26)] pub fn pretake_off_item(unit: &Unit); // Offset: 0x1DD7E90 Flags: 0
    #[unity::class_method(27)] pub fn posttake_off_item(unit: &Unit); // Offset: 0x1DD7F60 Flags: 0
    #[unity::class_method(28)] pub fn presort_item(unit: &Unit); // Offset: 0x1DD8030 Flags: 0
    #[unity::class_method(29)] pub fn postsort_item(unit: &Unit); // Offset: 0x1DD8100 Flags: 0
    #[unity::class_method(30)] pub fn preput_off_item(unit: &Unit, from_menu: bool); // Offset: 0x1DD81D0 Flags: 0
    #[unity::class_method(31)] pub fn postput_off_item(unit: &Unit, from_menu: bool); // Offset: 0x1DD82B0 Flags: 0
    #[unity::class_method(32)] pub fn event_battle(); // Offset: 0x1DD8390 Flags: 0
    #[unity::class_method(33)] pub fn mind_done(); // Offset: 0x1DD8450 Flags: 0
    #[unity::class_method(34)] pub fn engage_cancel(unit: &Unit); // Offset: 0x1DD8510 Flags: 0
    #[unity::class_method(35)] pub fn god_change_cancel(unit: &Unit); // Offset: 0x1DD8650 Flags: 0
    #[unity::class_method(36)] pub fn after_command_stack_cancel(unit: &Unit); // Offset: 0x1DD8790 Flags: 0
    #[unity::class_method(37)] pub fn status(unit: &Unit); // Offset: 0x1DD8860 Flags: 0
    #[unity::class_method(38)] pub fn status2(unit: &Unit, status: i64); // Offset: 0x1DD8930 Flags: 0
    #[unity::class_method(39)] pub fn hp(unit: &Unit); // Offset: 0x1DD8A10 Flags: 0
    #[unity::class_method(40)] pub fn hp2(unit: &Unit, hp: i32); // Offset: 0x1DD8AE0 Flags: 0
    #[unity::class_method(41)] pub fn base_capability(unit: &Unit, index: i32); // Offset: 0x1DD8BC0 Flags: 0
    #[unity::class_method(42)] pub fn engage_count(unit: &Unit); // Offset: 0x1DD8CA0 Flags: 0
    #[unity::class_method(43)] pub fn engage_count2(unit: &Unit, engage_count: i32); // Offset: 0x1DD8D70 Flags: 0
    #[unity::class_method(44)] pub fn extra_sight(unit: &Unit); // Offset: 0x1DD8E50 Flags: 0
    #[unity::class_method(45)] pub fn exp(unit: &Unit); // Offset: 0x1DD8F20 Flags: 0
    #[unity::class_method(46)] pub fn level_up(unit: &Unit); // Offset: 0x1DD8FF0 Flags: 0
    #[unity::class_method(47)] pub fn class_change(unit: &Unit); // Offset: 0x1DD90C0 Flags: 0
    #[unity::class_method(48)] pub fn position(unit: &Unit); // Offset: 0x1DD9190 Flags: 0
    #[unity::class_method(49)] pub fn position2(unit: &Unit, new_x: i32, new_z: i32); // Offset: 0x1DD9260 Flags: 0
    #[unity::class_method(50)] pub fn angle_once(unit: &Unit); // Offset: 0x1DD9350 Flags: 0
    #[unity::class_method(51)] pub fn private_skill(unit: &Unit); // Offset: 0x1DD9420 Flags: 0
    #[unity::class_method(52)] pub fn enhance_factor_item(unit: &Unit); // Offset: 0x1DD94F0 Flags: 0
    #[unity::class_method(53)] pub fn aiactive(unit: &Unit); // Offset: 0x1DD95C0 Flags: 0
    #[unity::class_method(54)] pub fn aiband(unit: &Unit); // Offset: 0x1DD9690 Flags: 0
    #[unity::class_method(55)] pub fn aipriority(unit: &Unit); // Offset: 0x1DD9760 Flags: 0
    #[unity::class_method(56)] pub fn aisequence(unit: &Unit, order: AIValueOrder); // Offset: 0x1DD9830 Flags: 0
    #[unity::class_method(57)] pub fn aivalue(unit: &Unit, order: AIValueOrder, index: i32); // Offset: 0x1DD9910 Flags: 0
    #[unity::class_method(58)] pub fn aiprohibit_engage_attack(unit: &Unit); // Offset: 0x1DD9A00 Flags: 0
    #[unity::class_method(59)] pub fn aiprohibit_rod(unit: &Unit); // Offset: 0x1DD9AD0 Flags: 0
    #[unity::class_method(60)] pub fn aiprohibit_overlap(unit: &Unit); // Offset: 0x1DD9BA0 Flags: 0
    #[unity::class_method(61)] pub fn aiengage_attack_once_done(unit: &Unit); // Offset: 0x1DD9C70 Flags: 0
    #[unity::class_method(62)] pub fn airerewarp(unit: &Unit); // Offset: 0x1DD9D40 Flags: 0
    #[unity::class_method(63)] pub fn airerewarp_count(unit: &Unit); // Offset: 0x1DD9E10 Flags: 0
    #[unity::class_method(64)] pub fn engage(unit: &Unit, link_unit: &Unit, is_event: bool); // Offset: 0x1DD9EE0 Flags: 0
    #[unity::class_method(65)] pub fn engage_for_decide_target_select(unit: &Unit, is_engaging: bool); // Offset: 0x1DDA050 Flags: 0
    #[unity::class_method(66)] pub fn engage_off_for_command_skill_after(unit: &Unit); // Offset: 0x1DDA190 Flags: 0
    #[unity::class_method(67)] pub fn dead(unit: &Unit); // Offset: 0x1DDA270 Flags: 0
    #[unity::class_method(68)] pub fn transfer(unit: &Unit, next_force_type: ForceType); // Offset: 0x1DDA3B0 Flags: 0
    #[unity::class_method(69)] pub fn revive(unit: &Unit); // Offset: 0x1DDA490 Flags: 0
    #[unity::class_method(70)] pub fn unit_phase_begin_all_begin(); // Offset: 0x1DDA560 Flags: 0
    #[unity::class_method(71)] pub fn unit_phase_begin_one_begin(unit: &Unit); // Offset: 0x1DDA620 Flags: 0
    #[unity::class_method(72)] pub fn unit_phase_begin_status(unit: &Unit); // Offset: 0x1DDA6F0 Flags: 0
    #[unity::class_method(73)] pub fn unit_phase_begin_private_skill(unit: &Unit); // Offset: 0x1DDA7C0 Flags: 0
    #[unity::class_method(74)] pub fn unit_phase_begin_extra_sight(unit: &Unit); // Offset: 0x1DDA890 Flags: 0
    #[unity::class_method(75)] pub fn unit_phase_begin_engage_turn(unit: &Unit); // Offset: 0x1DDA960 Flags: 0
    #[unity::class_method(76)] pub fn unit_phase_begin_engage(unit: &Unit); // Offset: 0x1DDAA30 Flags: 0
    #[unity::class_method(77)] pub fn unit_phase_begin_engage_count(unit: &Unit); // Offset: 0x1DDAB00 Flags: 0
    #[unity::class_method(78)] pub fn unit_phase_begin_aiprohibit_engage_attack(unit: &Unit); // Offset: 0x1DDABD0 Flags: 0
    #[unity::class_method(79)] pub fn unit_phase_begin_aiprohibit_rod(unit: &Unit); // Offset: 0x1DDACA0 Flags: 0
    #[unity::class_method(80)] pub fn unit_phase_begin_aiprohibit_overlap(unit: &Unit); // Offset: 0x1DDAD70 Flags: 0
    #[unity::class_method(81)] pub fn unit_phase_begin_multi_change_god(unit: &Unit, god_data: &GodData); // Offset: 0x1DDAE40 Flags: 0
    #[unity::class_method(82)] pub fn unit_phase_begin_position(unit: &Unit); // Offset: 0x1DDAF20 Flags: 0
    #[unity::class_method(83)] pub fn unit_phase_begin_one_end(unit: &Unit); // Offset: 0x1DDAFF0 Flags: 0
    #[unity::class_method(84)] pub fn unit_phase_begin_all_end(); // Offset: 0x1DDB0C0 Flags: 0
    #[unity::class_method(85)] pub fn unit_phase_end_all_begin(); // Offset: 0x1DDB180 Flags: 0
    #[unity::class_method(86)] pub fn unit_phase_end_one(unit: &Unit); // Offset: 0x1DDB240 Flags: 0
    #[unity::class_method(87)] pub fn unit_phase_end_all_end(); // Offset: 0x1DDB310 Flags: 0
    #[unity::class_method(88)] pub fn unit_item(unit: &Unit, index: i32); // Offset: 0x1DDB3D0 Flags: 0
    #[unity::class_method(89)] pub fn unit_item_list(unit: &Unit); // Offset: 0x1DDB4B0 Flags: 0
    #[unity::class_method(90)] pub fn dispos(unit: &Unit); // Offset: 0x1DDB580 Flags: 0
    #[unity::class_method(91)] pub fn god_create(god_data: &GodData); // Offset: 0x1DDB6C0 Flags: 0
    #[unity::class_method(92)] pub fn god_delete(god_unit: &GodUnit); // Offset: 0x1DDB790 Flags: 0
    #[unity::class_method(93)] pub fn god_connect(unit: &Unit); // Offset: 0x1DDB860 Flags: 0
    #[unity::class_method(94)] pub fn god_disconnect(unit: &Unit); // Offset: 0x1DDB930 Flags: 0
    #[unity::class_method(95)] pub fn god_change(unit: &Unit); // Offset: 0x1DDBA00 Flags: 0
    #[unity::class_method(96)] pub fn god_exp(god_unit: &GodUnit, unit: &Unit); // Offset: 0x1DDBB40 Flags: 0
    #[unity::class_method(97)] pub fn god_level_up(god_unit: &GodUnit, unit: &Unit); // Offset: 0x1DDBC20 Flags: 0
    #[unity::class_method(98)] pub fn god_darkness(god_unit: &GodUnit); // Offset: 0x1DDBD00 Flags: 0
    #[unity::class_method(99)] pub fn god_notify_level_cap_talk(god_unit: &GodUnit, unit: &Unit); // Offset: 0x1DDBDD0 Flags: 0
    #[unity::class_method(100)] pub fn god_state(unit: &Unit, index: i32); // Offset: 0x1DDBEB0 Flags: 0
    #[unity::class_method(101)] pub fn reliance_score(unit_a: &Unit, unit_b: &Unit); // Offset: 0x1DDBF90 Flags: 0
    #[unity::class_method(102)] pub fn transporter_data(index: i32, data: &TransporterData); // Offset: 0x1DDC070 Flags: 0
    #[unity::class_method(103)] pub fn cannon_shells(cannon_inspector: &CannonInspector); // Offset: 0x1DDC150 Flags: 0
    #[unity::class_method(104)] pub fn terrain_open(x: i32, z: i32); // Offset: 0x1DDC220 Flags: 0
    #[unity::class_method(105)] pub fn terrain_broken(x: i32, z: i32); // Offset: 0x1DDC300 Flags: 0
    #[unity::class_method(106)] pub fn terrain_action(x: i32, z: i32, action: i32); // Offset: 0x1DDC3E0 Flags: 0
    #[unity::class_method(107)] pub fn terrain_set_begin(); // Offset: 0x1DDC4D0 Flags: 0
    #[unity::class_method(108)] pub fn terrain_set(x: i32, z: i32); // Offset: 0x1DDC590 Flags: 0
    #[unity::class_method(109)] pub fn terrain_set_end(); // Offset: 0x1DDC670 Flags: 0
    #[unity::class_method(110)] pub fn terrain_set_one(x: i32, z: i32); // Offset: 0x1DDC730 Flags: 0
    #[unity::class_method(111)] pub fn overlap_begin(); // Offset: 0x1DDC8C0 Flags: 0
    #[unity::class_method(112)] pub fn overlap(x: i32, z: i32, tid: &Il2CppString); // Offset: 0x1DDC980 Flags: 0
    #[unity::class_method(113)] pub fn overlap_end(); // Offset: 0x1DDCA70 Flags: 0
    #[unity::class_method(114)] pub fn overlap_one(x: i32, z: i32, tid: &Il2CppString); // Offset: 0x1DDCB30 Flags: 0
    #[unity::class_method(115)] pub fn gold(gold: i32); // Offset: 0x1DDCCD0 Flags: 0
    #[unity::class_method(116)] pub fn material(kind: ItemDataKinds); // Offset: 0x1DDCDA0 Flags: 0
    #[unity::class_method(117)] pub fn piece_of_bond(); // Offset: 0x1DDCE70 Flags: 0
    #[unity::class_method(118)] pub fn variable(key: &Il2CppString); // Offset: 0x1DDCF30 Flags: 0
    #[unity::class_method(119)] pub fn win_rule(); // Offset: 0x1DDD000 Flags: 0
    #[unity::class_method(120)] pub fn win_rule_enemy_num(); // Offset: 0x1DDD0C0 Flags: 0
    #[unity::class_method(121)] pub fn win_rule_limit_turn(); // Offset: 0x1DDD180 Flags: 0
    #[unity::class_method(122)] pub fn win_rule_mid(); // Offset: 0x1DDD240 Flags: 0
    #[unity::class_method(123)] pub fn field_bgm_phase_bgm(player_phase_bgm: &Il2CppString, enemy_phase_bgm: &Il2CppString, ally_phase_bgm: &Il2CppString); // Offset: 0x1DDD300 Flags: 0
    #[unity::class_method(124)] pub fn field_bgm_war_situation(war_situation_state_name: &Il2CppString); // Offset: 0x1DDD3F0 Flags: 0
    #[unity::class_method(125)] pub fn engage_break(unit: &Unit); // Offset: 0x1DDD4C0 Flags: 0
    #[unity::class_method(126)] pub fn range_begin(); // Offset: 0x1DDD590 Flags: 0
    #[unity::class_method(127)] pub fn range(x: i32, z: i32); // Offset: 0x1DDD650 Flags: 0
    #[unity::class_method(128)] pub fn range_end(); // Offset: 0x1DDD730 Flags: 0
    #[unity::class_method(129)] pub fn range_clear(); // Offset: 0x1DDD7F0 Flags: 0
    #[unity::class_method(130)] pub fn god_escaping(god_unit: &GodUnit); // Offset: 0x1DDD8B0 Flags: 0
    #[unity::class_method(131)] pub fn rewind_danger_showing(unit: &Unit); // Offset: 0x1DDD980 Flags: 0
    #[unity::class_method(132)] pub fn map_kill_bonus(x: i32, z: i32, kind: i32); // Offset: 0x1DDDA50 Flags: 0
    #[unity::class_method(133)] pub fn god_dirty(god_unit: &GodUnit); // Offset: 0x1DDDB40 Flags: 0
    // #[unity::class_method(134)] pub fn effect_create(name: &Il2CppString, position: Vector3, rotation: Quaternion); // Offset: 0x1DBE110 Flags: 0
    #[unity::class_method(135)] pub fn effect_delete_begin(); // Offset: 0x1DBE250 Flags: 0
    // #[unity::class_method(136)] pub fn effect_delete(name: &Il2CppString, position: Vector3, rotation: Quaternion); // Offset: 0x1DBE310 Flags: 0
    #[unity::class_method(137)] pub fn effect_delete_end(); // Offset: 0x1DBE450 Flags: 0
    #[unity::class_method(138)] pub fn material_float_begin(name: &Il2CppString, material: &Il2CppString, property: &Il2CppString); // Offset: 0x1DDDC10 Flags: 0
    #[unity::class_method(139)] pub fn material_float(val: f32); // Offset: 0x1DDDD00 Flags: 0
    #[unity::class_method(140)] pub fn material_float_end(); // Offset: 0x1DDDDD0 Flags: 0
    #[unity::class_method(141)] pub fn material_color_begin(name: &Il2CppString, material: &Il2CppString, property: &Il2CppString); // Offset: 0x1DDDE90 Flags: 0
    // #[unity::class_method(142)] pub fn material_color(color: Color); // Offset: 0x1DDDF80 Flags: 0
    #[unity::class_method(143)] pub fn material_color_end(); // Offset: 0x1DDE070 Flags: 0
    #[unity::class_method(144)] pub fn field_bgm_special_turn(turn: i32); // Offset: 0x1DDE130 Flags: 0
    #[unity::class_method(145)] pub fn post_change_bgm_event(event_name: &Il2CppString); // Offset: 0x1DDE200 Flags: 0
    #[unity::class_method(146)] pub fn terrain_endurance(x: i32, z: i32, hp: i32, max_hp: i32); // Offset: 0x1DDE2D0 Flags: 0
    #[unity::class_method(147)] pub fn terrain_state(x: i32, z: i32, state: i32); // Offset: 0x1DDE3C0 Flags: 0
    #[unity::class_method(148)] pub fn lose_rule_mid(); // Offset: 0x1DDE4B0 Flags: 0
    #[unity::class_method(149)] pub fn battle_start(unit: &Unit, mind: MapMindType); // Offset: 0x1DDE570 Flags: 0
    #[unity::class_method(150)] pub fn phase_begin_after(); // Offset: 0x1DDE650 Flags: 0
    #[unity::class_method(151)] pub fn clear_ring(unit: &Unit); // Offset: 0x1DDE710 Flags: 0
    #[unity::class_method(152)] pub fn vision_delete(unit: &Unit); // Offset: 0x1DDE7E0 Flags: 0
    #[unity::class_method(153)] pub fn map_kill_bonus_count(x: i32, z: i32, kind: i32); // Offset: 0x1DDE8B0 Flags: 0
    #[unity::class_method(154)] pub fn unit_record(unit: &Unit, kind: UnitRecordKinds); // Offset: 0x1DDE9A0 Flags: 0
    #[unity::class_method(155)] pub fn skill_charge(); // Offset: 0x1DDEA80 Flags: 0
    #[unity::class_method(156)] pub fn surrender(); // Offset: 0x1DDEB40 Flags: 0
    #[unity::class_method(157)] pub fn set_extra_hp_stock(unit: &Unit); // Offset: 0x1DDEC00 Flags: 0
    #[unity::class_method(158)] pub fn clear_extra_hp_stock(unit: &Unit); // Offset: 0x1DDECD0 Flags: 0
    #[unity::class_method(159)] pub fn engage_turn(unit: &Unit); // Offset: 0x1DDEDA0 Flags: 0
    #[unity::class_method(160)] pub fn summon_delete(unit: &Unit); // Offset: 0x1DDEE70 Flags: 0
    #[unity::class_method(161)] pub fn map_sight_usable(); // Offset: 0x1DDEF40 Flags: 0
    #[unity::class_method(162)] pub fn plain_hp_stock(unit: &Unit); // Offset: 0x1DDF000 Flags: 0
    #[unity::class_method(163)] pub fn reset_lock_target(unit: &Unit); // Offset: 0x1DDF0D0 Flags: 0
    #[unity::class_method(164)] pub fn enchant_weapon(); // Offset: 0x1DDF1A0 Flags: 0
    #[unity::class_method(165)] pub fn aibullet_pattern(unit: &Unit); // Offset: 0x1DDF260 Flags: 0
    #[unity::class_method(166)] pub fn position_list_begin(); // Offset: 0x1DDF330 Flags: 0
    #[unity::class_method(167)] pub fn position_list(unit: &Unit); // Offset: 0x1DDF3F0 Flags: 0
    #[unity::class_method(168)] pub fn position_list_end(); // Offset: 0x1DDF4C0 Flags: 0
    #[unity::class_method(169)] pub fn aimove_limit(unit: &Unit); // Offset: 0x1DDF580 Flags: 0
    // #[unity::class_method(170)] pub fn terrain_action_move(x: i32, z: i32, moved_x: i32, moved_z: i32, action: MapObjectActions, state: i32); // Offset: 0x1DDF650 Flags: 0
    #[unity::class_method(171)] pub fn aimagic_shield_once_done(unit: &Unit); // Offset: 0x1DDF760 Flags: 0
    #[unity::class_method(172)] pub fn random_game(); // Offset: 0x1DDF830 Flags: 0
    #[unity::class_method(173)] pub fn full_bullet_attack(); // Offset: 0x1DDF8F0 Flags: 0
    #[unity::class_method(174)] pub fn lock_target(unit: &Unit); // Offset: 0x1DDF9B0 Flags: 0
    #[unity::class_method(175)] pub fn aienchant_weapon_done(unit: &Unit); // Offset: 0x1DDFA80 Flags: 0
    #[unity::class_method(176)] pub fn rewind_is_enable() -> bool; // Offset: 0x1DDFB50 Flags: 0
    #[unity::class_method(177)] pub fn rewind_enable(); // Offset: 0x1DDFC50 Flags: 0
    #[unity::class_method(178)] pub fn rewind_disable(); // Offset: 0x1DDFD10 Flags: 0
    #[unity::class_method(179)] pub fn rewind_dbg_set_use_count(count: i32); // Offset: 0x1DDFDD0 Flags: 0
    #[unity::class_method(180)] pub fn rewind_get_use_count() -> i32; // Offset: 0x1DDFDE0 Flags: 0
    #[unity::class_method(181)] pub fn rewind_get_max_use_count() -> i32; // Offset: 0x1DDFEE0 Flags: 0
    #[unity::class_method(182)] pub fn rewind_reset(); // Offset: 0x1DDFFE0 Flags: 0
    #[unity::class_method(183)] pub fn rewind_get_last_split_index() -> i32; // Offset: 0x1DE00A0 Flags: 0
    #[unity::class_method(184)] pub fn rewind_get_next_split_index(index: i32) -> i32; // Offset: 0x1DE01B0 Flags: 0
    #[unity::class_method(185)] pub fn rewind_get_prev_split_index(index: i32) -> i32; // Offset: 0x1DE02D0 Flags: 0
    // #[unity::class_method(186)] pub fn rewind_create_log(index: i32, result: &MapHistoryRewindLog) -> bool; // Offset: 0x1DE03F0 Flags: 0
    #[unity::class_method(187)] pub fn rewind_check_log_exists() -> bool; // Offset: 0x1DE0500 Flags: 0
    #[unity::class_method(188)] pub fn rewind_get_cursor_pos(index: i32, x: i32, z: i32) -> bool; // Offset: 0x1DE0600 Flags: 0
    #[unity::class_method(189)] pub fn rewind_is_phase_begin(index: i32) -> bool; // Offset: 0x1DE0730 Flags: 0
    #[unity::class_method(190)] pub fn rewind_preview_setup(); // Offset: 0x1DE0840 Flags: 0
    #[unity::class_method(191)] pub fn rewind_preview_apply(index: i32) -> bool; // Offset: 0x1DE0900 Flags: 0
    #[unity::class_method(192)] pub fn rewind_preview_decide(); // Offset: 0x1DE0A10 Flags: 0
    #[unity::class_method(193)] pub fn rewind_preview_cancel(); // Offset: 0x1DE0AD0 Flags: 0
    #[unity::class_method(194)] pub fn rewind_is_previewing() -> bool; // Offset: 0x1DBD130 Flags: 0
    #[unity::class_method(195)] pub fn rewind_preview_get_unit(map_history_index: i32) -> &'static Unit; // Offset: 0x1DE0B90 Flags: 0
    #[unity::class_method(196)] pub fn rewind_dbg_dump(); // Offset: 0x1DE0CA0 Flags: 0
    #[unity::class_method(197)] pub fn rewind_dbg_create_snapshot(); // Offset: 0x1DE0CB0 Flags: 0
    #[unity::class_method(198)] pub fn rewind_dbg_has_snapshot(index: i32) -> bool; // Offset: 0x1DE0CC0 Flags: 0
    #[unity::class_method(199)] pub fn rewind_dbg_delete_snapshot(index: i32); // Offset: 0x1DE0CD0 Flags: 0
}