use unity::{prelude::*, il2cpp::object::Array};
use crate::gamedata::dispos::DisposData;
use super::{DynValue, EventScript};
// Namespace: App, Token: 0x2000A52
#[unity::class("App", "ScriptMap")]
pub struct ScriptMap { }

impl ScriptMap {
    #[unity::class_method(0)] pub fn mind_get_force(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECC280 Flags: 0
    #[unity::class_method(1)] pub fn mind_get_unit(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECC3F0 Flags: 0
    #[unity::class_method(2)] pub fn mind_get_target_unit(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECC590 Flags: 0
    #[unity::class_method(3)] pub fn mind_get_event_unit(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECC730 Flags: 0
    #[unity::class_method(4)] pub fn cursor_get_x(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECC8D0 Flags: 0
    #[unity::class_method(5)] pub fn cursor_get_z(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECCAA0 Flags: 0
    #[unity::class_method(6)] pub fn cursor_set_pos(args: &Array<&DynValue>); // Offset: 0x1ECCC70 Flags: 0
    #[unity::class_method(7)] pub fn cursor_set_visible(args: &Array<&DynValue>); // Offset: 0x1ECCEE0 Flags: 0
    #[unity::class_method(8)] pub fn cursor_get_distance_mode(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECCF90 Flags: 0
    #[unity::class_method(9)] pub fn cursor_set_distance_mode(args: &Array<&DynValue>); // Offset: 0x1ECD030 Flags: 0
    #[unity::class_method(10)] pub fn cursor_set_distance_scale(args: &Array<&DynValue>); // Offset: 0x1ECD0E0 Flags: 0
    #[unity::class_method(11)] pub fn terrain_get(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECD1C0 Flags: 0
    #[unity::class_method(12)] pub fn terrain_set_begin(args: &Array<&DynValue>); // Offset: 0x1ECD380 Flags: 0
    #[unity::class_method(13)] pub fn terrain_set_end(args: &Array<&DynValue>); // Offset: 0x1ECD420 Flags: 0
    #[unity::class_method(14)] pub fn terrain_set(args: &Array<&DynValue>); // Offset: 0x1ECD4C0 Flags: 0
    #[unity::class_method(15)] pub fn terrain_set_one(args: &Array<&DynValue>); // Offset: 0x1ECD650 Flags: 0
    #[unity::class_method(16)] pub fn terrain_set_impl(args: &Array<&DynValue>, is_multi: bool); // Offset: 0x1ECD4D0 Flags: 0
    #[unity::class_method(17)] pub fn terrain_fill(args: &Array<&DynValue>); // Offset: 0x1ECD660 Flags: 0
    #[unity::class_method(18)] pub fn terrain_get_move_cost(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECD750 Flags: 0
    #[unity::class_method(19)] pub fn map_overlap_set_begin(args: &Array<&DynValue>); // Offset: 0x1ECD910 Flags: 0
    #[unity::class_method(20)] pub fn map_overlap_set_end(args: &Array<&DynValue>); // Offset: 0x1ECD9B0 Flags: 0
    #[unity::class_method(21)] pub fn map_overlap_set(args: &Array<&DynValue>); // Offset: 0x1ECDA50 Flags: 0
    #[unity::class_method(22)] pub fn map_overlap_set_one(args: &Array<&DynValue>); // Offset: 0x1ECDBC0 Flags: 0
    #[unity::class_method(23)] pub fn map_overlap_set_impl(args: &Array<&DynValue>, is_multi: bool); // Offset: 0x1ECDA60 Flags: 0
    #[unity::class_method(24)] pub fn map_overlap_get(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECDBD0 Flags: 0
    #[unity::class_method(25)] pub fn map_overlap_remove(args: &Array<&DynValue>); // Offset: 0x1ECDCB0 Flags: 0
    #[unity::class_method(26)] pub fn map_overlap_remove_impl(args: &Array<&DynValue>, is_multi: bool); // Offset: 0x1ECDCC0 Flags: 0
    #[unity::class_method(27)] pub fn map_range_add_begin(args: &Array<&DynValue>); // Offset: 0x1ECDE00 Flags: 0
    #[unity::class_method(28)] pub fn map_range_add_end(args: &Array<&DynValue>); // Offset: 0x1ECDE70 Flags: 0
    #[unity::class_method(29)] pub fn map_range_add(args: &Array<&DynValue>); // Offset: 0x1ECDF30 Flags: 0
    #[unity::class_method(30)] pub fn map_range_clear(args: &Array<&DynValue>); // Offset: 0x1ECE060 Flags: 0
    #[unity::class_method(31)] pub fn dispos(args: &Array<&DynValue>); // Offset: 0x1ECE110 Flags: 0
    #[unity::class_method(32)] pub fn dispos_get_group_count(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECE220 Flags: 0
    #[unity::class_method(33)] pub fn dispos_get_unit_x(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECE360 Flags: 0
    #[unity::class_method(34)] pub fn dispos_get_unit_z(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECE5C0 Flags: 0
    #[unity::class_method(35)] pub fn get_dispos_data(group: &Il2CppString, index: i32) -> Option<&'static DisposData>; // Offset: 0x1ECE4D0 Flags: 0
    #[unity::class_method(36)] pub fn map_damage_begin(args: &Array<&DynValue>); // Offset: 0x1ECE730 Flags: 0
    #[unity::class_method(37)] pub fn map_damage_add(args: &Array<&DynValue>); // Offset: 0x1ECE800 Flags: 0
    #[unity::class_method(38)] pub fn map_damage_end(args: &Array<&DynValue>); // Offset: 0x1ECE900 Flags: 0
    #[unity::class_method(39)] pub fn battle(args: &Array<&DynValue>); // Offset: 0x1ECEB60 Flags: 0
    #[unity::class_method(40)] pub fn battle_set_attack(args: &Array<&DynValue>); // Offset: 0x1ECED70 Flags: 0
    #[unity::class_method(41)] pub fn battle_add_target(args: &Array<&DynValue>); // Offset: 0x1ECEF70 Flags: 0
    #[unity::class_method(42)] pub fn battle_start(args: &Array<&DynValue>); // Offset: 0x1ECF060 Flags: 0
    #[unity::class_method(43)] pub fn map_get_turn(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECF2A0 Flags: 0
    #[unity::class_method(44)] pub fn map_get_phase(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECF360 Flags: 0
    #[unity::class_method(45)] pub fn map_get_average_level(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECF420 Flags: 0
    #[unity::class_method(46)] pub fn map_get_position(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECF4E0 Flags: 0
    #[unity::class_method(47)] pub fn map_get_height(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECF5F0 Flags: 0
    #[unity::class_method(48)] pub fn map_is_sight(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED0260 Flags: 0
    #[unity::class_method(49)] pub fn map_set_sight(args: &Array<&DynValue>); // Offset: 0x1ED03F0 Flags: 0
    #[unity::class_method(50)] pub fn map_is_recollection(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED05D0 Flags: 0
    #[unity::class_method(51)] pub fn win_rule_set(enable: bool, status: i32); // Offset: 0x1ED07B0 Flags: 0
    #[unity::class_method(52)] pub fn win_rule_set_breakdown(args: &Array<&DynValue>); // Offset: 0x1ED08D0 Flags: 0
    #[unity::class_method(53)] pub fn win_rule_set_destroy_boss(args: &Array<&DynValue>); // Offset: 0x1ED0950 Flags: 0
    #[unity::class_method(54)] pub fn win_rule_set_enemy_number_less_than_or_equal_to(args: &Array<&DynValue>); // Offset: 0x1ED09D0 Flags: 0
    #[unity::class_method(55)] pub fn win_rule_set_limit_turn(args: &Array<&DynValue>); // Offset: 0x1ED0AF0 Flags: 0
    #[unity::class_method(56)] pub fn win_rule_set_mid(args: &Array<&DynValue>); // Offset: 0x1ED0C10 Flags: 0
    #[unity::class_method(57)] pub fn lose_rule_set_mid(args: &Array<&DynValue>); // Offset: 0x1ED0D90 Flags: 0
    // #[unity::class_method(58)] pub fn try_get_effect_arg(name: &Il2CppString, position: Vector3, rotation: Quaternion, args: &Array<&DynValue>) -> bool; // Offset: 0x1ED0F10 Flags: 0
    #[unity::class_method(59)] pub fn effect_play(args: &Array<&DynValue>); // Offset: 0x1ED1C90 Flags: 0
    #[unity::class_method(60)] pub fn effect_is_playing(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED1D80 Flags: 0
    #[unity::class_method(61)] pub fn effect_create(args: &Array<&DynValue>); // Offset: 0x1ED1E20 Flags: 0
    #[unity::class_method(62)] pub fn effect_delete(args: &Array<&DynValue>); // Offset: 0x1ED1F10 Flags: 0
    #[unity::class_method(63)] pub fn event_entry_turn(args: &Array<&DynValue>); // Offset: 0x1ED1FD0 Flags: 0
    #[unity::class_method(64)] pub fn event_entry_turn_after(args: &Array<&DynValue>); // Offset: 0x1ED2070 Flags: 0
    #[unity::class_method(65)] pub fn event_entry_turn_end(args: &Array<&DynValue>); // Offset: 0x1ED2110 Flags: 0
    #[unity::class_method(66)] pub fn event_entry_area(args: &Array<&DynValue>); // Offset: 0x1ED21B0 Flags: 0
    #[unity::class_method(67)] pub fn event_entry_die(args: &Array<&DynValue>); // Offset: 0x1ED2250 Flags: 0
    #[unity::class_method(68)] pub fn event_entry_revive_before(args: &Array<&DynValue>); // Offset: 0x1ED22F0 Flags: 0
    #[unity::class_method(69)] pub fn event_entry_revive_after(args: &Array<&DynValue>); // Offset: 0x1ED2390 Flags: 0
    #[unity::class_method(70)] pub fn event_entry_fixed(args: &Array<&DynValue>); // Offset: 0x1ED2430 Flags: 0
    #[unity::class_method(71)] pub fn event_entry_talk(args: &Array<&DynValue>); // Offset: 0x1ED24D0 Flags: 0
    #[unity::class_method(72)] pub fn event_entry_battle_before(args: &Array<&DynValue>); // Offset: 0x1ED2570 Flags: 0
    #[unity::class_method(73)] pub fn event_entry_battle_talk(args: &Array<&DynValue>); // Offset: 0x1ED2610 Flags: 0
    #[unity::class_method(74)] pub fn event_entry_battle_after(args: &Array<&DynValue>); // Offset: 0x1ED26B0 Flags: 0
    #[unity::class_method(75)] pub fn event_entry_escape(args: &Array<&DynValue>); // Offset: 0x1ED2750 Flags: 0
    #[unity::class_method(76)] pub fn event_entry_breakdown(args: &Array<&DynValue>); // Offset: 0x1ED27F0 Flags: 0
    #[unity::class_method(77)] pub fn event_entry_breakdown_enemy(args: &Array<&DynValue>); // Offset: 0x1ED2890 Flags: 0
    #[unity::class_method(78)] pub fn event_entry_waypoint(args: &Array<&DynValue>); // Offset: 0x1ED2930 Flags: 0
    #[unity::class_method(79)] pub fn event_entry_command(args: &Array<&DynValue>); // Offset: 0x1ED29D0 Flags: 0
    #[unity::class_method(80)] pub fn event_entry_pickup(args: &Array<&DynValue>); // Offset: 0x1ED2A70 Flags: 0
    #[unity::class_method(81)] pub fn event_entry_target_select(args: &Array<&DynValue>); // Offset: 0x1ED2B10 Flags: 0
    #[unity::class_method(82)] pub fn event_entry_unit_command_prepare(args: &Array<&DynValue>); // Offset: 0x1ED2BB0 Flags: 0
    #[unity::class_method(83)] pub fn event_entry_unit_command_interrupt(args: &Array<&DynValue>); // Offset: 0x1ED2C50 Flags: 0
    #[unity::class_method(84)] pub fn event_entry_tbox(args: &Array<&DynValue>); // Offset: 0x1ED2CF0 Flags: 0
    #[unity::class_method(85)] pub fn event_entry_visit(args: &Array<&DynValue>); // Offset: 0x1ED2E30 Flags: 0
    #[unity::class_method(86)] pub fn event_entry_door(args: &Array<&DynValue>); // Offset: 0x1ED2F70 Flags: 0
    #[unity::class_method(87)] pub fn event_entry_destroy(args: &Array<&DynValue>); // Offset: 0x1ED30F0 Flags: 0
    #[unity::class_method(88)] pub fn event_open_object(args: &Array<&DynValue>); // Offset: 0x1ED3270 Flags: 0
    #[unity::class_method(89)] pub fn event_open_door(args: &Array<&DynValue>); // Offset: 0x1ED33A0 Flags: 0
    #[unity::class_method(90)] pub fn event_broken_object(args: &Array<&DynValue>); // Offset: 0x1ED3500 Flags: 0
    #[unity::class_method(91)] pub fn event_action_object(args: &Array<&DynValue>); // Offset: 0x1ED35E0 Flags: 0
    #[unity::class_method(92)] pub fn event_action_move_object(args: &Array<&DynValue>); // Offset: 0x1ED36D0 Flags: 0
    #[unity::class_method(93)] pub fn event_state_object(args: &Array<&DynValue>); // Offset: 0x1ED3800 Flags: 0
    #[unity::class_method(94)] pub fn event_is_playing_object(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED38F0 Flags: 0
    #[unity::class_method(95)] pub fn event_is_playing_sky_castle(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED3A00 Flags: 0
    #[unity::class_method(96)] pub fn event_engage_summon(args: &Array<&DynValue>); // Offset: 0x1ED3B50 Flags: 0
    #[unity::class_method(97)] pub fn event_entry_engage_before(args: &Array<&DynValue>); // Offset: 0x1ED3D60 Flags: 0
    #[unity::class_method(98)] pub fn event_entry_engage_after(args: &Array<&DynValue>); // Offset: 0x1ED3E00 Flags: 0
    #[unity::class_method(99)] pub fn map_camera_is_scroll(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED3EA0 Flags: 0
    #[unity::class_method(100)] pub fn turn_end(args: &Array<&DynValue>); // Offset: 0x1ED3F40 Flags: 0
    #[unity::class_method(101)] pub fn map_history_rewind_enable(args: &Array<&DynValue>); // Offset: 0x1ED3F50 Flags: 0
    #[unity::class_method(102)] pub fn map_history_rewind_disable(args: &Array<&DynValue>); // Offset: 0x1ED3FC0 Flags: 0
    #[unity::class_method(103)] pub fn map_history_rewind_reset(args: &Array<&DynValue>); // Offset: 0x1ED4030 Flags: 0
    #[unity::class_method(104)] pub fn map_history_mind_done(args: &Array<&DynValue>); // Offset: 0x1ED40A0 Flags: 0
    #[unity::class_method(105)] pub fn map_history_engage_break(args: &Array<&DynValue>); // Offset: 0x1ED4110 Flags: 0
    #[unity::class_method(106)] pub fn map_history_position_list_begin(args: &Array<&DynValue>); // Offset: 0x1ED41E0 Flags: 0
    #[unity::class_method(107)] pub fn map_history_position_list(args: &Array<&DynValue>); // Offset: 0x1ED4250 Flags: 0
    #[unity::class_method(108)] pub fn map_history_position_list_end(args: &Array<&DynValue>); // Offset: 0x1ED4320 Flags: 0
    #[unity::class_method(109)] pub fn map_material_set_float(args: &Array<&DynValue>); // Offset: 0x1ED4390 Flags: 0
    #[unity::class_method(110)] pub fn map_material_set_color(args: &Array<&DynValue>); // Offset: 0x1ED44B0 Flags: 0
    #[unity::class_method(111)] pub fn god_save_equip(args: &Array<&DynValue>); // Offset: 0x1ED4660 Flags: 0
    #[unity::class_method(112)] pub fn god_load_equip(args: &Array<&DynValue>); // Offset: 0x1ED4670 Flags: 0
    #[unity::class_method(113)] pub fn regist(script: &EventScript); // Offset: 0x1ED4680 Flags: 0
}

