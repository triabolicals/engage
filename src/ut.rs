use unity::engine::{Material, Texture2D};
use unity::il2cpp::object::Array;
use unity::prelude::*;
use crate::unityengine::{GameObject, Renderer, Transform, UnityComponent};

#[unity::class("App", "Ut")]
pub struct Ut { }

impl Ut {
    pub fn hash_fnv_string<'a>(str: impl Into<&'a Il2CppString>) -> i32 { Self::hash_fnv_1_string(str.into()) }
    #[unity::class_method(9)] pub fn msec_to_frame(msec: f32) -> f32; // Offset: 0x1C7D420 Flags: 0
    #[unity::class_method(10)] pub fn frame_to_msec(frame: f32) -> f32; // Offset: 0x1C7D440 Flags: 0
    #[unity::class_method(11)] pub fn get_frame_step() -> f32; // Offset: 0x1C7D460 Flags: 0
    #[unity::class_method(13)] pub fn get_fast_input_scale() -> i32; // Offset: 0x1C7D4A0 Flags: 0
    #[unity::class_method(14)] pub fn change_key_value(value: i32, min: i32, max: i32, step: i32) -> i32; // Offset: 0x1C7D6A0 Flags: 0
    #[unity::class_method(15)] pub fn change_key_value2(value: f32, min: f32, max: f32, step: f32) -> f32; // Offset: 0x1C7E030 Flags: 0
    #[unity::class_method(16)] pub fn change_key_value3(value: bool) -> bool; // Offset: 0x1C7E0C0 Flags: 0
    #[unity::class_method(17)] pub fn change_key_value_impl(value: f64, min: f64, max: f64, step: f64) -> f64; // Offset: 0x1C7D750 Flags: 0
    #[unity::class_method(19)] pub fn find_game_object(name: &Il2CppString, warning: bool) -> &'static GameObject; // Offset: 0x1C7E840 Flags: 0
    #[unity::class_method(20)] pub fn try_create_game_object(name: &Il2CppString, parent: &GameObject) -> &'static GameObject; // Offset: 0x1C7E8F0 Flags: 0
    #[unity::class_method(21)] pub fn create_game_object(name: &Il2CppString, parent: &Il2CppString) -> &'static GameObject; // Offset: 0x1C7EAF0 Flags: 0
    #[unity::class_method(22)] pub fn find_root_object(name: &Il2CppString) -> &'static GameObject; // Offset: 0x1C7EBF0 Flags: 0
    #[unity::class_method(23)] pub fn try_create_root_object(name: &Il2CppString) -> &'static GameObject; // Offset: 0x1C7ED20 Flags: 0
    #[unity::class_method(28)] pub fn create_game_object_by_asset(original: &GameObject, parent: &GameObject) -> &'static GameObject; // Offset: 0x1C7EE00 Flags: 0
    #[unity::class_method(29)] pub fn destroy_game_object(game_object: &GameObject); // Offset: 0x1C7EF50 Flags: 0
    #[unity::class_method(30, generic)] pub fn destroy_game_object2<T>(component: &T) where T: Il2CppClassData; // Offset: -1 Flags: 1
    #[unity::class_method(31, generic)] pub fn destroy_game_object3<T>(component: &T) where T: Il2CppClassData; // Offset: -1 Flags: 1
    #[unity::class_method(32)] pub fn destroy_child_object(game_object: &GameObject); // Offset: 0x1C7F010 Flags: 0
    #[unity::class_method(33)] pub fn set_layer_recursively(transform: &Transform, layer: i32) -> bool; // Offset: 0x1C7F160 Flags: 0
    #[unity::class_method(34)] pub fn set_layer_recursively2(game_object: &GameObject, layer: i32) -> bool; // Offset: 0x1C7F280 Flags: 0
    #[unity::class_method(35)] pub fn set_layer_recursively3(game_object: &GameObject, name: &Il2CppString) -> bool; // Offset: 0x1C7F310 Flags: 0
    #[unity::class_method(36)] pub fn create_debug_text_object(parent: &Transform, name: &Il2CppString, size: i32) -> &'static GameObject; // Offset: 0x1C7F410 Flags: 0
    #[unity::class_method(37)] pub fn get_object_full_path(game_object: &GameObject) -> &'static Il2CppString; // Offset: 0x1C7F750 Flags: 0
    #[unity::class_method(40)] pub fn find_child_game_object2(game_object: &GameObject, child_name: &Il2CppString) -> &'static GameObject; // Offset: 0x1C5F9A0 Flags: 0
    #[unity::class_method(41)] pub fn find_child_transform(parent: &Transform, child_name: &Il2CppString) -> &'static Transform; // Offset: 0x1C7F8E0 Flags: 0
    #[unity::class_method(42)] pub fn get_top_parent_object(game_object: &GameObject) -> &'static GameObject; // Offset: 0x1C7FA60 Flags: 0
    #[unity::class_method(43)] pub fn get_unique_object_name(name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1C7FB50 Flags: 0
    #[unity::class_method(44)] pub fn get_current_scene_path() -> &'static Il2CppString; // Offset: 0x1C7FC80 Flags: 0
    #[unity::class_method(45)] pub fn get_current_scene_dir() -> &'static Il2CppString; // Offset: 0x1C7FD40 Flags: 0
    #[unity::class_method(46)] pub fn get_current_scene_name() -> &'static Il2CppString; // Offset: 0x1C7FDF0 Flags: 0
    #[unity::class_method(47)] pub fn get_scene_name(game_object: &GameObject) -> &'static Il2CppString; // Offset: 0x1C7FEA0 Flags: 0
    #[unity::class_method(48)] pub fn get_scene_name2<C: UnityComponent>(component: &C) -> &'static Il2CppString; // Offset: 0x1C7FF40 Flags: 0
    #[unity::class_method(49)] pub fn get_prefixless(path: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1C80080 Flags: 0
    #[unity::class_method(50, generic)]
    pub fn find_child_component<T>(game_object: &GameObject) -> &'static T where T: UnityComponent; // Offset: -1 Flags: 1
    #[unity::class_method(51, generic)]
    pub fn find_child_component2<T>(game_object: &GameObject, child_name: &Il2CppString) -> &'static T where T: UnityComponent; // Offset: -1 Flags: 1
    #[unity::class_method(52, generic)]
    pub fn find_child_components<T>(game_object: &GameObject) -> &'static Array<&'static T> where T: UnityComponent; // Offset: -1 Flags: 1
    #[unity::class_method(56)] pub fn round_kb(size: i64) -> i64; // Offset: 0x1C80270 Flags: 0
    #[unity::class_method(57, generic)] pub fn set_component_enable<T>(game_object: &GameObject, enable: bool) where T: UnityComponent; // Offset: -1 Flags: 1
    #[unity::class_method(58)] pub fn set_component_enable2(game_object: &GameObject, enable: bool); // Offset: 0x1C80280 Flags: 0
    #[unity::class_method(59)] pub fn dump_transform(transform: &Transform, depth: i32); // Offset: 0x1C80310 Flags: 0
    #[unity::class_method(62)] pub fn get_streaming_assets_path() -> &'static Il2CppString; // Offset: 0x1C805D0 Flags: 0
    #[unity::class_method(63)] pub fn get_streaming_assets_path2(path: &Il2CppString, platform: bool) -> &'static Il2CppString; // Offset: 0x1C80660 Flags: 0
    #[unity::class_method(69)] pub fn get_instance_materials(render: &Renderer) -> &'static Array<&'static Material>; // Offset: 0x1C808F0 Flags: 0
    #[unity::class_method(70)] pub fn hash_fnv_1_u8(bytes: &Array<u8>) -> i32; // Offset: 0x1C80990 Flags: 0
    #[unity::class_method(71)] pub fn hash_fnv_1_u8_2(bytes: &Array<u8>, length: i32) -> i32; // Offset: 0x1C80AE0 Flags: 0
    #[unity::class_method(72)] pub fn hash_fnv_1_u32(values: &Array<u32>) -> i32; // Offset: 0x1C80BE0 Flags: 0
    #[unity::class_method(73)] pub fn hash_fnv_1_u32_2(values: &Array<u32>, length: i32) -> i32; // Offset: 0x1C80D30 Flags: 0
    #[unity::class_method(74)] pub fn hash_fnv_1_string(name: &Il2CppString) -> i32; // Offset: 0x1C80E30 Flags: 0
    #[unity::class_method(77)] pub fn get_all_asset_paths(root: &Il2CppString, extensions: &Array<&Il2CppString>) -> &'static Array<&'static Il2CppString>; // Offset: 0x1C81230 Flags: 0
    #[unity::class_method(78)] pub fn asset_path(path: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1C81240 Flags: 0
    #[unity::class_method(79)] pub fn system_path(path: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1C81320 Flags: 0
    #[unity::class_method(85)] pub fn value_to_string(value: i32) -> &'static Il2CppString; // Offset: 0x1C817B0 Flags: 0
    #[unity::class_method(87)] pub fn string_with_comma(value: i32) -> &'static Il2CppString; // Offset: 0x1C81AD0 Flags: 0
    #[unity::class_method(88)] pub fn get_language_label(label: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1C81BC0 Flags: 0
    #[unity::class_method(89)] pub fn get_short_name(name: &Il2CppString, length: i32) -> &'static Il2CppString; // Offset: 0x1C81CE0 Flags: 0
    #[unity::class_method(90)] pub fn get_size_name(size: i64) -> &'static Il2CppString; // Offset: 0x1C81DA0 Flags: 0
    #[unity::class_method(91)] pub fn capture_texture(w: i32, h: i32) -> &'static Texture2D; // Offset: 0x1C81E60 Flags: 0
    #[unity::class_method(92)] pub fn get_time_stamp() -> &'static Il2CppString; // Offset: 0x1C82040 Flags: 0
    // #[unity::class_method(93)] pub fn force_rebuild_layout(component: &Component); // Offset: 0x1C823A0 Flags: 0
}