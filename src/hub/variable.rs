use unity::prelude::{Il2CppString, Il2CppClassData};
use crate::gamevariable::GameVariable;

#[unity::class("App", "HubVariable")] pub struct HubVariable {}

impl HubVariable {
    #[unity::class_method(0)]
    pub fn get_variable() -> &'static GameVariable; // Offset: 0x2A68E60 Flags: 0
    #[unity::class_method(1)]
    pub fn get_random_key_name() -> &'static Il2CppString; // Offset: 0x2A68EE0 Flags: 0
    #[unity::class_method(2)]
    pub fn get_random_item_key_name() -> &'static Il2CppString; // Offset: 0x2A68F50 Flags: 0
    #[unity::class_method(3)]
    pub fn get_timezone_key_name() -> &'static Il2CppString; // Offset: 0x2A68FC0 Flags: 0
    #[unity::class_method(4)]
    pub fn get_scene_key() -> &'static Il2CppString; // Offset: 0x2A69030 Flags: 0
    #[unity::class_method(5)]
    pub fn get_start_key() -> &'static Il2CppString; // Offset: 0x2A690A0 Flags: 0
    #[unity::class_method(6)]
    pub fn get_enter_count_key_name() -> &'static Il2CppString; // Offset: 0x2A69110 Flags: 0
    #[unity::class_method(7)]
    pub fn get_enter_hub_key_name() -> &'static Il2CppString; // Offset: 0x2A69180 Flags: 0
    #[unity::class_method(8)]
    pub fn get_mascot_point_name() -> &'static Il2CppString; // Offset: 0x2A691F0 Flags: 0
    #[unity::class_method(9)]
    pub fn get_mascot_limit_point_name() -> &'static Il2CppString; // Offset: 0x2A69260 Flags: 0
    #[unity::class_method(10)]
    pub fn get_mascot_equip_head_acc() -> &'static Il2CppString; // Offset: 0x2A692D0 Flags: 0
    #[unity::class_method(11)]
    pub fn get_mascot_equip_tail_acc() -> &'static Il2CppString; // Offset: 0x2A69340 Flags: 0
    #[unity::class_method(12)]
    pub fn get_mascot_color() -> &'static Il2CppString; // Offset: 0x2A693B0 Flags: 0
    #[unity::class_method(13)]
    pub fn get_mascot_first_flag_name() -> &'static Il2CppString; // Offset: 0x2A69420 Flags: 0
    #[unity::class_method(14)]
    pub fn get_mascot_done_strok() -> &'static Il2CppString; // Offset: 0x2A69490 Flags: 0
    #[unity::class_method(15)]
    pub fn get_mascot_done_eat_food() -> &'static Il2CppString; // Offset: 0x2A69500 Flags: 0
    #[unity::class_method(16)]
    pub fn get_mascot_done_change_equip() -> &'static Il2CppString; // Offset: 0x2A69570 Flags: 0
    #[unity::class_method(17)]
    pub fn get_dragon_king_class_changed() -> &'static Il2CppString; // Offset: 0x2A695E0 Flags: 0
    #[unity::class_method(18)]
    pub fn get_placed_rare_animal_flag_name() -> &'static Il2CppString; // Offset: 0x2A69650 Flags: 0
    #[unity::class_method(19)]
    pub fn get_statue_condition_flag_name() -> &'static Il2CppString; // Offset: 0x2A696C0 Flags: 0
    #[unity::class_method(20)]
    pub fn get_promise_ring_flag_name() -> &'static Il2CppString; // Offset: 0x2A69730 Flags: 0
    #[unity::class_method(21)]
    pub fn get_has_promise_ring() -> bool; // Offset: 0x2A5FEA0 Flags: 0
    #[unity::class_method(22)]
    pub fn set_has_promise_ring(value: bool); // Offset: 0x2A697A0 Flags: 0
    #[unity::class_method(23)]
    pub fn get_enter_count() -> i32; // Offset: 0x2A69B30 Flags: 0
    #[unity::class_method(24)]
    pub fn set_enter_count(value: i32); // Offset: 0x2A69CA0 Flags: 0
    #[unity::class_method(25)]
    pub fn get_enter_hub() -> bool; // Offset: 0x2A69F00 Flags: 0
    #[unity::class_method(26)]
    pub fn set_enter_hub(value: bool); // Offset: 0x2A6A080 Flags: 0
    #[unity::class_method(27)]
    pub fn get_animal_flag_name(index: i32) -> &'static Il2CppString; // Offset: 0x2A62230 Flags: 0
    #[unity::class_method(28)]
    pub fn get_animal_item_flag_name(index: i32) -> &'static Il2CppString; // Offset: 0x2A6A410 Flags: 0
    #[unity::class_method(29)]
    pub fn create_global_flags(); // Offset: 0x2A6A4A0 Flags: 0
    #[unity::class_method(30)]
    pub fn completed_chapter(); // Offset: 0x2A6C690 Flags: 0
    #[unity::class_method(31)]
    pub fn setup_scene(); // Offset: 0x2A6FBC0 Flags: 0
    #[unity::class_method(33)]
    pub fn set_current_scene(scene_name: &Il2CppString, start: &Il2CppString); // Offset: 0x2A70A30 Flags: 0
    #[unity::class_method(34)]
    pub fn set_current_start_name(start: &Il2CppString); // Offset: 0x2A70EB0 Flags: 0
    #[unity::class_method(35)]
    pub fn clear_current_start_name(); // Offset: 0x2A71110 Flags: 0
    #[unity::class_method(36)]
    pub fn set_random_seed(seed: i32); // Offset: 0x2A71370 Flags: 0
    #[unity::class_method(37)]
    pub fn get_random_seed() -> i32; // Offset: 0x2A715D0 Flags: 0
    #[unity::class_method(38)]
    pub fn get_statue_condition() -> i32; // Offset: 0x2A5AB00 Flags: 0
    #[unity::class_method(39)]
    pub fn set_statue_condition(value: i32); // Offset: 0x2A6F960 Flags: 0
    #[unity::class_method(41)]
    pub fn get_current_scene_name() -> &'static Il2CppString; // Offset: 0x2A71860 Flags: 0
    #[unity::class_method(42)]
    pub fn get_current_start_name() -> &'static Il2CppString; // Offset: 0x2A719D0 Flags: 0
    #[unity::class_method(43)]
    pub fn get_hub_solanel_bgm() -> &'static Il2CppString; // Offset: 0x2A71B40 Flags: 0
    #[unity::class_method(44)]
    pub fn set_hub_solanel_bgm(event_name: &Il2CppString); // Offset: 0x2A71C70 Flags: 0
    #[unity::class_method(45)]
    pub fn is_appeared_rare_animal(prefixless_cid: &Il2CppString) -> bool; // Offset: 0x2A71E90 Flags: 0
    #[unity::class_method(46)]
    pub fn set_chapter_for_debug(); // Offset: 0x2A72020 Flags: 0
    #[unity::class_method(47)]
    pub fn setup_debug_option(is_done_first_event: bool); // Offset: 0x2A738F0 Flags: 0
    #[unity::class_method(48)]
    pub fn setup_debug_mini_game(); // Offset: 0x2A73900 Flags: 0
}

#[unity::class("", "Mascot")]
#[nested_from_type(HubVariable)]
pub struct HubVariableMascot { }

impl HubVariableMascot {
    #[unity::class_method(0)] pub fn get_can_follow() -> bool; // Offset: 0x2AEE140 Flags: 0
    #[unity::class_method(1)] pub fn is_found() -> bool; // Offset: 0x2AEE390 Flags: 0
    #[unity::class_method(2)] pub fn found(); // Offset: 0x2AEE4B0 Flags: 0
    #[unity::class_method(3)] pub fn get_point() -> i32; // Offset: 0x2AEE270 Flags: 0
    #[unity::class_method(4)] pub fn set_point(point: i32); // Offset: 0x2AEE6B0 Flags: 0
    #[unity::class_method(5)] pub fn get_ignore_point() -> i32; // Offset: 0x2AEE8F0 Flags: 0
    #[unity::class_method(6)] pub fn get_turn_limit_point() -> i32; // Offset: 0x2AEE990 Flags: 0
    #[unity::class_method(7)] pub fn get_limit_point() -> i32; // Offset: 0x2AEE9E0 Flags: 0
    #[unity::class_method(8)] pub fn set_limit_point(point: i32); // Offset: 0x2AEEB00 Flags: 0
    #[unity::class_method(9)] pub fn add_point(point: i32); // Offset: 0x2AEED20 Flags: 0
    #[unity::class_method(10)] pub fn dec_point(point: i32); // Offset: 0x2AEED60 Flags: 0
    #[unity::class_method(11)] pub fn set_head_acc_name(acc_name: &Il2CppString); // Offset: 0x2AEED90 Flags: 0
    #[unity::class_method(12)] pub fn get_head_acc_name() -> &'static Il2CppString; // Offset: 0x2AEEF90 Flags: 0
    #[unity::class_method(13)] pub fn set_tail_acc_name(acc_name: &Il2CppString); // Offset: 0x2AEF0B0 Flags: 0
    #[unity::class_method(14)] pub fn get_tail_acc_name() -> &'static Il2CppString; // Offset: 0x2AEF2B0 Flags: 0
    #[unity::class_method(15)] pub fn set_color_index(color_index: i32); // Offset: 0x2AEF3D0 Flags: 0
    #[unity::class_method(16)] pub fn get_color_index() -> i32; // Offset: 0x2AEF5D0 Flags: 0
    #[unity::class_method(17)] pub fn done_strok(); // Offset: 0x2AEF6F0 Flags: 0
    #[unity::class_method(18)] pub fn is_done_strok() -> bool; // Offset: 0x2AEF8F0 Flags: 0
    #[unity::class_method(19)] pub fn done_eat_food(); // Offset: 0x2AEFA10 Flags: 0
    #[unity::class_method(20)] pub fn is_done_eat_food() -> bool; // Offset: 0x2AEFC10 Flags: 0
    #[unity::class_method(21)] pub fn done_change_equip(); // Offset: 0x2AEFD30 Flags: 0
    #[unity::class_method(22)] pub fn is_done_change_equip() -> bool; // Offset: 0x2AEFF30 Flags: 0
}
