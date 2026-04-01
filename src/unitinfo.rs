use unity::engine::{ui::*, *};
use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::action::Action;
use unity::system::List;
use crate::combat::Character;
use crate::gamedata::item::ItemData;
use crate::god::GodUnit;
use crate::proc::ProcInstFields;
use crate::unit::Unit;
use crate::unityengine::{Animator, GameObject, RectTransform, RenderTexture, Transform, UnityComponent};
use crate::util::{get_instance, get_singleton_proc_instance};

#[unity::class("App", "UnitStatusSetter")]
pub struct UnitStatusSetter {}
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitInfoSide {
    Left = 0,
    Right = 1,
}
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnitInfoMode {
    Simple = 0,
    Hide = 1,
    ModelOnly = 2,
}
#[unity::class("App", "UnitInfo")]
pub struct UnitInfo {
    proc: ProcInstFields,
    padding: [u8; 0x10],
    pub windows: &'static mut Array<&'static mut UnitInfoWindow>,
}

#[unity::class("", "Window")]
#[nested_from_type(UnitInfo)]
pub struct UnitInfoWindow {
    stack: u64,
    pub unit: &'static Unit,
    pub god_unit: Option<&'static GodUnit>,
    pub unit_x: i32,
    pub unit_z: i32,
    pub window_simple: u64,
    pub unit_info_window_chara_model: &'static mut UnitInfoWindowCharaModel,
    pub is_visible: bool,
}
// Namespace: App, Token: 0x20010D3
#[unity::class("App", "UnitStatus")]
pub struct UnitStatus {}

impl UnitInfo {
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
    #[unity::class_method(15)] pub fn set_unit(side: UnitInfoSide, unit: Option<&Unit>, b_relax: bool, b_reverse_rotation: bool, is_delay_load: bool, on_setup_done_callback: Option<&Action>);

    #[unity::class_method(24)] pub fn play_chara_voice(side: UnitInfoSide, person_switch_name: &Il2CppString, engage_switch_name: &Il2CppString, event_name: &Il2CppString); // Offset: 0x1F87460 Flags: 0
    #[unity::class_method(25)] pub fn reserve_chara_voice(side: UnitInfoSide, person_switch_name: &Il2CppString, engage_switch_name: &Il2CppString, event_name: &Il2CppString); // Offset: 0x1F87570 Flags: 0
    #[unity::class_method(37)] pub fn update_current_unit(side: UnitInfoSide); // Offset: 0x1F88330 Flags: 0
    #[unity::class_method(40)] pub fn get_render_texture(side: UnitInfoSide) -> &'static RenderTexture; // Offset: 0x1F885E0 Flags: 0

    #[unity::class_method(53)] pub fn set_visible(is_visible: bool); // Offset: 0x1F89290 Flags: 0
    #[unity::class_method(54)] pub fn set_visible_side(side: UnitInfoSide, is_visible: bool); // Offset: 0x1F89400 Flags: 0
    #[unity::class_method(63)] pub fn chara_only_on(is_change_chara_model_anim: bool); // Offset: 0x1F89BD0 Flags: 0
    #[unity::class_method(64)] pub fn chara_only_off(); // Offset: 0x1F89CE0 Flags: 0
    /*
    #[unity::class_method(0)] pub fn ctor(&self); // Offset: 0x1F85680 Flags: 0
    #[unity::class_method(1)] pub fn construct(&self, left_unit: &Unit, right_unit: &Unit); // Offset: 0x1F85710 Flags: 0
    #[unity::class_method(2)] pub fn on_dispose(&self); // Offset: 0x1F85900 Flags: 0
    #[unity::class_method(3)] pub fn prepare_chara_model(&self); // Offset: 0x1F85A60 Flags: 0
    #[unity::class_method(4)] pub fn is_preparing_chara_model(&self) -> bool; // Offset: 0x1F85AD0 Flags: 0
    #[unity::class_method(5)] pub fn prepare_window(&self); // Offset: 0x1F85B60 Flags: 0
    #[unity::class_method(6)] pub fn is_preparing_window(&self) -> bool; // Offset: 0x1F85BD0 Flags: 0
    #[unity::class_method(7)] pub fn postprepare(&self); // Offset: 0x1F85C60 Flags: 0
    #[unity::class_method(8)] pub fn tick(&self); // Offset: 0x1F85CD0 Flags: 0
    #[unity::class_method(9)] pub fn create_async(super: &ProcInst); // Offset: 0x1F861F0 Flags: 0
    #[unity::class_method(10)] pub fn create_bind_async(super: &ProcInst); // Offset: 0x1F863D0 Flags: 0
    #[unity::class_method(11)] pub fn create_impl(super: &ProcInst, p: &UnitInfo); // Offset: 0x1F86340 Flags: 0
    #[unity::class_method(12)] pub fn create_bind_impl(super: &ProcInst, p: &UnitInfo); // Offset: 0x1F86520 Flags: 0
    #[unity::class_method(13)] pub fn create_descs(p: &UnitInfo) -> &'static Array<ProcDesc>; // Offset: 0x1F865B0 Flags: 0
    #[unity::class_method(14)] pub fn get_current_window_object() -> &'static GameObject; // Offset: 0x1F86980 Flags: 0
 // Offset: 0x1F86A50 Flags: 0
    #[unity::class_method(16)] pub fn set_unit_battle_info(side: UnitInfoSide, unit: &Unit, unit_item: &UnitItem); // Offset: 0x1F86B80 Flags: 0
    #[unity::class_method(17)] pub fn set_override_create_reserve_unit_item(side: UnitInfoSide, unit_item: &UnitItem); // Offset: 0x1F86CD0 Flags: 0
    #[unity::class_method(18)] pub fn set_unit_hub(side: UnitInfoSide, unit: &Unit, b_relax: bool, b_reverse_rotation: bool, is_delay_load: bool); // Offset: 0x1F86DD0 Flags: 0
    #[unity::class_method(19)] pub fn set_unit_relay(side: UnitInfoSide, person: &PersonData, job: &JobData, edit: &UnitEdit); // Offset: 0x1F86EF0 Flags: 0
    #[unity::class_method(20)] pub fn reset_unit(side: UnitInfoSide, unit_item: &UnitItem, b_relax: bool, b_reverse_rotation: bool, is_delay_load: bool); // Offset: 0x1F87000 Flags: 0
    #[unity::class_method(21)] pub fn reset_unit2(); // Offset: 0x1F87150 Flags: 0
    #[unity::class_method(22)] pub fn add_chara_rot(side: UnitInfoSide, quaternion: Quaternion); // Offset: 0x1F87270 Flags: 0
    #[unity::class_method(23)] pub fn set_left_camera_adjust_y(); // Offset: 0x1F87390 Flags: 0

    #[unity::class_method(26)] pub fn play_reserved_chara_voice(side: UnitInfoSide); // Offset: 0x1F87680 Flags: 0
    #[unity::class_method(27)] pub fn set_weapon_shop_chara(side: UnitInfoSide); // Offset: 0x1F87770 Flags: 0
    #[unity::class_method(28)] pub fn set_summon_chara(); // Offset: 0x1F87860 Flags: 0
    #[unity::class_method(29)] pub fn set_god(side: UnitInfoSide, god: &GodUnit, b_relax: bool, b_reverse_rotation: bool, on_setup_done_callback: &Action); // Offset: 0x1F87940 Flags: 0
    // #[unity::class_method(30)] pub fn set_right_for_ring_cleaning(god: &GodUnit, type: RingCleaningSequenceGodType, b_relax: bool); // Offset: 0x1F87A60 Flags: 0
    #[unity::class_method(31)] pub fn set_efficacy_attack(side: UnitInfoSide, is_efficacy: bool); // Offset: 0x1F87D70 Flags: 0
    #[unity::class_method(32)] pub fn set_relax_anime(side: UnitInfoSide, transition_duration: f32); // Offset: 0x1F87E60 Flags: 0
    #[unity::class_method(33)] pub fn set_status_anime(side: UnitInfoSide, transition_duration: f32); // Offset: 0x1F87F60 Flags: 0
    #[unity::class_method(34)] pub fn set_fortune_telling_good_anime(side: UnitInfoSide, is_allow_unit_null: bool); // Offset: 0x1F88060 Flags: 0
    #[unity::class_method(35)] pub fn set_fortune_telling_bad_anime(side: UnitInfoSide, is_allow_unit_null: bool); // Offset: 0x1F88150 Flags: 0
    #[unity::class_method(36)] pub fn set_select_god_normal_face(side: UnitInfoSide); // Offset: 0x1F88240 Flags: 0

    #[unity::class_method(38)] pub fn get_unit(side: UnitInfoSide) -> &'static Unit; // Offset: 0x1F88420 Flags: 0
    #[unity::class_method(39)] pub fn get_god(side: UnitInfoSide) -> &'static GodUnit; // Offset: 0x1F88500 Flags: 0

    #[unity::class_method(41)] pub fn get_face_camera_component(side: UnitInfoSide) -> &'static Camera; // Offset: 0x1F886D0 Flags: 0
    #[unity::class_method(42)] pub fn is_hide() -> bool; // Offset: 0x1F887C0 Flags: 0
    #[unity::class_method(43)] pub fn push_mode(mode: UnitInfoMode); // Offset: 0x1F888B0 Flags: 0
    #[unity::class_method(44)] pub fn push_mode_simple(); // Offset: 0x1F889F0 Flags: 0
    #[unity::class_method(45)] pub fn push_mode_hide(); // Offset: 0x1F88A60 Flags: 0
    #[unity::class_method(46)] pub fn push_mode_model_only(); // Offset: 0x1F88AD0 Flags: 0
    #[unity::class_method(47)] pub fn pop_mode(); // Offset: 0x1F88B40 Flags: 0
    #[unity::class_method(48)] pub fn set_mode(mode: UnitInfoMode); // Offset: 0x1F88C80 Flags: 0
    #[unity::class_method(49)] pub fn set_mode_if_hide(mode: UnitInfoMode); // Offset: 0x1F88E70 Flags: 0
    #[unity::class_method(50)] pub fn set_current_mode_to_game_user_data(); // Offset: 0x1F89020 Flags: 0
    #[unity::class_method(51)] pub fn push_mode_from_game_user_data(); // Offset: 0x1F89130 Flags: 0
    #[unity::class_method(52)] pub fn set_mode_from_game_user_data_if_hide(); // Offset: 0x1F891E0 Flags: 0

    #[unity::class_method(55)] pub fn reset_alpha(); // Offset: 0x1F89540 Flags: 0
    #[unity::class_method(56)] pub fn toggle_visible(); // Offset: 0x1F89730 Flags: 0
    #[unity::class_method(57)] pub fn toggle_visible2(side: UnitInfoSide); // Offset: 0x1F89830 Flags: 0
    #[unity::class_method(58)] pub fn to_visible(); // Offset: 0x1F89920 Flags: 0
    #[unity::class_method(59)] pub fn to_visible2(side: UnitInfoSide); // Offset: 0x1F89990 Flags: 0
    #[unity::class_method(60)] pub fn to_invisible(); // Offset: 0x1F89A00 Flags: 0
    #[unity::class_method(61)] pub fn to_invisible2(side: UnitInfoSide); // Offset: 0x1F89A70 Flags: 0
    #[unity::class_method(62)] pub fn is_visible(side: UnitInfoSide) -> bool; // Offset: 0x1F89AE0 Flags: 0

    #[unity::class_method(65)] pub fn is_chara_only_transition() -> bool; // Offset: 0x1F89DE0 Flags: 0
    #[unity::class_method(66)] pub fn init_stand_by_anime(); // Offset: 0x1F89F10 Flags: 0
    // #[unity::class_method(67)] pub fn update_offense_stand_by_anime(unit_item: &UnitItem, is_weapon_shop: bool); // Offset: 0x1F8A020 Flags: 0
    // #[unity::class_method(68)] pub fn update_defense_stand_by_anime(unit_item: &UnitItem, is_weapon_shop: bool); // Offset: 0x1F8A110 Flags: 0
    #[unity::class_method(69)] pub fn hide_weapon(); // Offset: 0x1F8A200 Flags: 0
    #[unity::class_method(70)] pub fn transparent_on(side: UnitInfoSide); // Offset: 0x1F8A300 Flags: 0
    #[unity::class_method(71)] pub fn transparent_off(side: UnitInfoSide); // Offset: 0x1F8A3F0 Flags: 0
    #[unity::class_method(72)] pub fn get_head_locator(window: &UnitInfoWindow) -> &'static Transform; // Offset: 0x1F8A4E0 Flags: 0
    #[unity::class_method(73)] pub fn set_sight(); // Offset: 0x1F85D90 Flags: 0
    #[unity::class_method(74)] pub fn set_head_locator(side: UnitInfoSide, transform: &Transform, weight: f32, is_weight_interpolated: bool); // Offset: 0x1F8A500 Flags: 0
    #[unity::class_method(75)] pub fn set_look_at(side: UnitInfoSide, transform: &Transform); // Offset: 0x1F8A600 Flags: 0
    #[unity::class_method(76)] pub fn set_look_at_camera(side: UnitInfoSide); // Offset: 0x1F8A6D0 Flags: 0
    #[unity::class_method(77)] pub fn set_eyes_weight(side: UnitInfoSide, eyes_weight: f32); // Offset: 0x1F8A7A0 Flags: 0
    #[unity::class_method(78)] pub fn is_chara_visible(side: UnitInfoSide) -> bool; // Offset: 0x1F8A880 Flags: 0
    #[unity::class_method(79)] pub fn set_visible_of_status(side: UnitInfoSide, mode: UnitInfoMode, is_visible: bool); // Offset: 0x1F8A970 Flags: 0
    #[unity::class_method(80)] pub fn is_visible_status(side: UnitInfoSide, mode: UnitInfoMode) -> bool; // Offset: 0x1F8AA80 Flags: 0
    #[unity::class_method(81)] pub fn hide_chara_image(side: UnitInfoSide); // Offset: 0x1F8AB80 Flags: 0
    #[unity::class_method(82)] pub fn show_chara_image(side: UnitInfoSide); // Offset: 0x1F8AC70 Flags: 0
    #[unity::class_method(83)] pub fn dump_mode(); // Offset: 0x1F8AD60 Flags: 0
    #[unity::class_method(84)] pub fn cctor(); // Offset: 0x1F8B020 Flags: 0

     */
}

impl UnitStatus {
    #[unity::class_method(4)] pub fn is_creating(&self) -> bool; // Offset: 0x1C63DD0 Flags: 0
    #[unity::class_method(5)] pub fn prepare(&self); // Offset: 0x1C63E00 Flags: 0
    #[unity::class_method(6)] pub fn tick(&self); // Offset: 0x1C63F30 Flags: 0
    #[unity::class_method(7)] pub fn start_job_intro(unit: &Unit); // Offset: 0x1C640A0 Flags: 0
    #[unity::class_method(8)] pub fn start_unit_select(unit: &Unit, is_sortie: bool); // Offset: 0x1C64250 Flags: 0
    #[unity::class_method(9)] pub fn close(); // Offset: 0x1C64410 Flags: 0
    #[unity::class_method(10)] pub fn set_unit(unit: &Unit); // Offset: 0x1C61C50 Flags: 0
    #[unity::class_method(11)] pub fn set_front(); // Offset: 0x1C64BD0 Flags: 0
    #[unity::class_method(12)] pub fn get_game_object() -> &'static GameObject; // Offset: 0x1C64C30 Flags: 0
}


#[unity::class("App", "UnitInfoWindowCharaModel")]
pub struct UnitInfoWindowCharaModel {
    pub fortune_anim_face_hash: i32,
    pub padding: i32,
    reserved_char_voice: u64,
    prefab_handle: u64,
    pub game_object: &'static GameObject,
    pub camera_object: &'static GameObject,
    pub render_texture: &'static RenderTexture,
    offscreen_camera: u64,
    pub updater: &'static mut UnitInfoWindowCharaUpdater,
    padding3: [u8; 0x40],
    pub char: &'static mut Character,
}

impl UnitInfoWindowCharaModel {
    pub fn destroy_chara_model(&self){ self.delete_chara_model2(self.char); }
    #[unity::class_method(16)] pub fn delete_chara_model2(&self, chara: &Character); // Offset: 0x1FA7FB0 Flags: 0
    #[unity::class_method(62)] pub fn create_chara_model(&self, chara: &Character) -> &'static Character; // Offset: 0x1FAC4A0 Flags: 0
    /*
    #[unity::class_method(0)] pub fn set_on_setup_done_callback(&self, callback: &Action); // Offset: 0x1FA71F0 Flags: 0
    #[unity::class_method(1)] pub fn ctor(&self, is_duplicate_render_texture: bool, is_reverse: bool); // Offset: 0x1FA7200 Flags: 0
    #[unity::class_method(2)] pub fn create_async(&self, is_duplicate_render_texture: bool, is_reverse: bool); // Offset: 0x1FA7330 Flags: 0
    #[unity::class_method(3)] pub fn is_creating(&self) -> bool; // Offset: 0x1FA74E0 Flags: 0
    #[unity::class_method(4)] pub fn destroy(&self); // Offset: 0x1FA7510 Flags: 0
    #[unity::class_method(5)] pub fn is_loading_model(&self) -> bool; // Offset: 0x1FA77D0 Flags: 0
    #[unity::class_method(6)] pub fn set_unit(&self, unit: &Unit, x: i32, z: i32, b_relax: bool, b_reverse_rotation: bool, is_delay_load: bool); // Offset: 0x1FA7880 Flags: 0
    #[unity::class_method(7)] pub fn forced_set_unit(&self, unit: &Unit, b_relax: bool, b_reverse_rotation: bool, is_delay_load: bool); // Offset: 0x1FA79F0 Flags: 0
    #[unity::class_method(8)] pub fn set_unit_hub(&self, unit: &Unit, b_relax: bool, b_reverse_rotation: bool, is_delay_load: bool); // Offset: 0x1FA7A20 Flags: 0
    #[unity::class_method(9)] pub fn set_unit_relay(&self, person: &PersonData, job: &JobData, edit: &UnitEdit); // Offset: 0x1FA7A50 Flags: 0
    #[unity::class_method(10)] pub fn set_unit_impl(&self, unit: &Unit, b_relax: bool, b_reverse_rotation: bool, is_delay_load: bool, is_changed: bool, is_hub: bool); // Offset: 0x1FA78C0 Flags: 0
    #[unity::class_method(11)] pub fn tick(&self); // Offset: 0x1FA8170 Flags: 0
    #[unity::class_method(12)] pub fn update_motion(&self); // Offset: 0x1FA8180 Flags: 0
    #[unity::class_method(13)] pub fn update_visible(&self); // Offset: 0x1FA82F0 Flags: 0
    #[unity::class_method(14)] pub fn create_impl(&self, is_duplicate_render_texture: bool, is_reverse: bool); // Offset: 0x1FA84D0 Flags: 0
    #[unity::class_method(15)] pub fn delete_chara_model(&self); // Offset: 0x1FA7640 Flags: 0

    #[unity::class_method(17)] pub fn create_chara_model(&self, b_relax: bool, is_hub: bool); // Offset: 0x1FA8070 Flags: 0
    #[unity::class_method(18)] pub fn update_character_animation(&self, chara: &Character, b_relax: bool, is_hub: bool, unit_item: &UnitItem); // Offset: 0x1FA8CF0 Flags: 0
    #[unity::class_method(19)] pub fn set_god(&self, god: &GodUnit, b_relax: bool, b_reverse_rotation: bool, is_delay_load: bool); // Offset: 0x1FA9D60 Flags: 0
    #[unity::class_method(20)] pub fn chara_only_on(&self); // Offset: 0x1FAA210 Flags: 0
    #[unity::class_method(21)] pub fn chara_only_off(&self); // Offset: 0x1FAA3B0 Flags: 0
    #[unity::class_method(22)] pub fn is_chara_only_transition(&self) -> bool; // Offset: 0x1FAA4A0 Flags: 0
    #[unity::class_method(23)] pub fn set_create_reserve_unit_item(&self, unit_item: &UnitItem, is_override: bool); // Offset: 0x1FAA590 Flags: 0
    #[unity::class_method(24)] pub fn update_stand_by_anime(&self, unit_item: &UnitItem, is_weapon_shop: bool); // Offset: 0x1FA9060 Flags: 0
    #[unity::class_method(25)] pub fn set_efficacy_attack(&self, is_efficacy: bool); // Offset: 0x1FAA760 Flags: 0
    #[unity::class_method(26)] pub fn set_relax_anime(&self, transition_duration: f32); // Offset: 0x1FAA860 Flags: 0
    #[unity::class_method(27)] pub fn set_status_anime(&self, transition_duration: f32); // Offset: 0x1FAA930 Flags: 0
    #[unity::class_method(28)] pub fn set_fortune_telling_good_anime(&self, is_allow_unit_null: bool); // Offset: 0x1FAAA00 Flags: 0
    #[unity::class_method(29)] pub fn set_fortune_telling_bad_anime(&self, is_allow_unit_null: bool); // Offset: 0x1FAAAD0 Flags: 0
    #[unity::class_method(30)] pub fn set_weapon_shop_chara(&self); // Offset: 0x1FAABA0 Flags: 0
    #[unity::class_method(31)] pub fn set_summon_chara(&self); // Offset: 0x1FAACF0 Flags: 0
    #[unity::class_method(32)] pub fn update_god_select_normal_face_anime(&self); // Offset: 0x1FAAE70 Flags: 0
    #[unity::class_method(33)] pub fn show_weapon(&self, item: &ItemData); // Offset: 0x1FAA630 Flags: 0
    #[unity::class_method(34)] pub fn hide_weapon(&self); // Offset: 0x1FA9C70 Flags: 0
    #[unity::class_method(35)] pub fn transparent_on(&self); // Offset: 0x1FAAF30 Flags: 0
    #[unity::class_method(36)] pub fn transparent_off(&self); // Offset: 0x1FAB020 Flags: 0
    #[unity::class_method(37)] pub fn activate(&self); // Offset: 0x1FAB110 Flags: 0
    #[unity::class_method(38)] pub fn deactivate(&self, b_clear_stand_by: bool); // Offset: 0x1FAB2B0 Flags: 0
    #[unity::class_method(39)] pub fn set_chara_image(&self, image_simple: &Image); // Offset: 0x1FAB4A0 Flags: 0
    #[unity::class_method(40)] pub fn get_render_texture(&self) -> &'static RenderTexture; // Offset: 0x1FAB550 Flags: 0
    #[unity::class_method(41)] pub fn get_face_camera_component(&self) -> &'static Camera; // Offset: 0x1FAB560 Flags: 0
    #[unity::class_method(42)] pub fn get_head_locator(&self) -> &'static Transform; // Offset: 0x1FAB5C0 Flags: 0
    #[unity::class_method(43)] pub fn set_head_locator(&self, pos: Vector3, default_weight: f32); // Offset: 0x1FAA660 Flags: 0
    #[unity::class_method(44)] pub fn set_head_locator2(&self, loc: &Transform, default_weight: f32, is_weight_interpolated: bool); // Offset: 0x1FAB6E0 Flags: 0
    #[unity::class_method(45)] pub fn set_look_at(&self, transform: &Transform); // Offset: 0x1FABA20 Flags: 0
    #[unity::class_method(46)] pub fn set_look_at_camera(&self); // Offset: 0x1FABA30 Flags: 0
    #[unity::class_method(47)] pub fn set_eyes_weight(&self, default_eyes_weight: f32); // Offset: 0x1FABA70 Flags: 0
    #[unity::class_method(48)] pub fn is_chara_visible(&self) -> bool; // Offset: 0x1FABBB0 Flags: 0
    #[unity::class_method(49)] pub fn play_chara_voice(&self, person_switch_name: &Il2CppString, engage_switch_name: &Il2CppString, event_name: &Il2CppString); // Offset: 0x1FABD30 Flags: 0
    #[unity::class_method(50)] pub fn reserve_chara_voice(&self, person_switch_name: &Il2CppString, engage_switch_name: &Il2CppString, event_name: &Il2CppString); // Offset: 0x1FABE70 Flags: 0
    #[unity::class_method(51)] pub fn play_reserved_chara_voice(&self); // Offset: 0x1FABF30 Flags: 0
    #[unity::class_method(52)] pub fn add_chara_rot(&self, quaternion: Quaternion); // Offset: 0x1FABF40 Flags: 0
    #[unity::class_method(53)] pub fn set_camera_adjust_y(&self); // Offset: 0x1FAC0A0 Flags: 0
    #[unity::class_method(54)] pub fn setup(&self, is_duplicate_render_texture: bool, is_reverse: bool) -> bool; // Offset: 0x1FA8650 Flags: 0
    #[unity::class_method(55)] pub fn get_game_object(&self, obj_name: &Il2CppString) -> &'static GameObject; // Offset: 0x1FAC150 Flags: 0
    #[unity::class_method(56)] pub fn create_unit_model(&self, unit: &Unit, callback: &CharacterFactoryAsynconLoad, is_hub: bool); // Offset: 0x1FA8A20 Flags: 0
    #[unity::class_method(57)] pub fn create_god_model(&self, god: &GodUnit, callback: &CharacterFactoryAsynconLoad); // Offset: 0x1FA9F50 Flags: 0
    #[unity::class_method(58)] pub fn create_unit_model_relay(&self, person: &PersonData, job: &JobData, edit: &UnitEdit, callback: &CharacterFactoryAsynconLoad); // Offset: 0x1FA7B10 Flags: 0
    #[unity::class_method(59)] pub fn wait_loading(&self) -> &'static IEnumerator; // Offset: 0x1FAC220 Flags: 0
    #[unity::class_method(60)] pub fn hide_chara_image(&self); // Offset: 0x1FA7DF0 Flags: 0
    #[unity::class_method(61)] pub fn on_setup_done(&self); // Offset: 0x1FAC2A0 Flags: 0

    #[unity::class_method(63)] pub fn is_pain(&self) -> bool; // Offset: 0x1FA9AA0 Flags: 0
    #[unity::class_method(64)] pub fn get_game_object2(&self) -> &'static GameObject; // Offset: 0x1FAC910 Flags: 0
    #[unity::class_method(65)] pub fn cctor(); // Offset: 0x1FAC920 Flags: 0
    */
}
#[unity::class("App", "UnitInfoWindowCharaUpdater")]
pub struct UnitInfoWindowCharaUpdater {
    parent: u64,
    pub is_request_to_play_body: bool, // Offset 0x18, Attr: 1
    pub is_request_to_play_face: bool, // Offset 0x19, Attr: 1
    pub body_anim_hash: i32, // Offset 0x1C, Attr: 1
    pub body_anim_transition_duration: f32, // Offset 0x20, Attr: 1
    pub face_anim_hash: i32, // Offset 0x24, Attr: 1
    pub is_request_to_set_param: bool, // Offset 0x28, Attr: 1
    pub body_param_hash: i32, // Offset 0x2C, Attr: 1
    pub body_param_value: bool, // Offset 0x30, Attr: 1
    pub is_request_to_weapon: bool, // Offset 0x31, Attr: 1
    pub request_weapon: Option<&'static crate::gamedata::item::ItemData>, // Offset 0x38, Attr: 1
    pub anime_change_wait_count: i32, // Offset 0x40, Attr: 1
    pub camera_object: &'static GameObject, // Offset 0x48, Attr: 1
    pub chara_image_simple: &'static Image, // Offset 0x50, Attr: 1
    pub default_zoom_camera_local_height: f32, // Offset 0x58, Attr: 1
    pub head_look_at_obj: &'static GameObject, // Offset 0x60, Attr: 6
    pub is_request_to_offset: bool, // Offset 0x68, Attr: 1
}

impl UnitInfoWindowCharaUpdater {
    #[unity::class_method(0)] pub fn update(&self); // Offset: 0x1FACF60 Flags: 0
    #[unity::class_method(1)] pub fn try_update_offset(&self, chara: &Character) -> bool; // Offset: 0x1FAD020 Flags: 0
    #[unity::class_method(2)] pub fn late_update(&self); // Offset: 0x1FAD380 Flags: 0
    #[unity::class_method(3)] pub fn is_body_anim_end(&self) -> bool; // Offset: 0x1FA8310 Flags: 0
    #[unity::class_method(4)] pub fn try_same_body_anim_hash(&self, anim_hash: i32) -> bool; // Offset: 0x1FA8300 Flags: 0
    #[unity::class_method(5)] pub fn request_to_play_body(&self, body_anim_hash: i32, transition_duration: f32) -> bool; // Offset: 0x1FA84A0 Flags: 0
    #[unity::class_method(6)] pub fn play_body_forced(&self); // Offset: 0x1FA9050 Flags: 0
    #[unity::class_method(7)] pub fn request_to_play_face(&self, face_anim_hash: i32, is_forced: bool); // Offset: 0x1FA9A70 Flags: 0
    #[unity::class_method(8)] pub fn request_to_show_weapon(&self, item: &ItemData); // Offset: 0x1FAAF00 Flags: 0
    #[unity::class_method(9)] pub fn request_to_set_param(&self, body_param_hash: i32, value: bool); // Offset: 0x1FA9A50 Flags: 0
    #[unity::class_method(10)] pub fn set_camera_object(&self, camera_object: &GameObject); // Offset: 0x1FAC1D0 Flags: 0
    #[unity::class_method(11)] pub fn set_chara_image(&self, image_simple: &Image); // Offset: 0x1FAD680 Flags: 0
    #[unity::class_method(12)] pub fn try_set_camera_adjust_y(&self); // Offset: 0x1FAC140 Flags: 0
    #[unity::class_method(13)] pub fn reset_anime_request(&self); // Offset: 0x1FAD690 Flags: 0
    #[unity::class_method(14)] pub fn show_images(&self); // Offset: 0x1FA9CB0 Flags: 0
    #[unity::class_method(15)] pub fn hide_images(&self); // Offset: 0x1FAB400 Flag
}
#[unity::class("App", "UnitInfoCharaImageMaskOffset")]
pub struct UnitInfoCharaImageMaskOffset {
    pub parent: u64,
    pub kind: i32,
    pub offset_u: f32,
    pub text: Option<&'static mut Texture>,
    pub text_old:  Option<&'static mut Texture>,
    pub time: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    padd: i32,
    canvas_manager: u64,
    pub rect: &'static RectTransform,
    pub image: &'static Image,
    material_start: &'static Material,
    pub material: &'static Material,
    pub texture: &'static RenderTexture,
}

impl UnitInfoCharaImageMaskOffset {
    pub fn get_instance() -> &'static mut List<UnitInfoCharaImageMaskOffset> {
        let klass = get_generic_class!(SingletonMonoBehaviorList<UnitInfoCharaImageMaskOffset>).unwrap().get_method_from_name("get_Instance", 0).unwrap();
        let get_instance = unsafe { std::mem::transmute::<_, fn(&MethodInfo) -> &'static mut List<UnitInfoCharaImageMaskOffset>>(klass.method_ptr) };
        get_instance(klass)
    }
    #[unity::class_method(0)] pub fn start(&self); // Offset: 0x1F8B8E0 Flags: 0
    #[unity::class_method(1)] pub fn on_enable(&self); // Offset: 0x1F8BE20 Flags: 0
    #[unity::class_method(2)] pub fn on_disable(&self); // Offset: 0x1F8BE30 Flags: 0
    #[unity::class_method(3)] pub fn setup(&self); // Offset: 0x1F8B980 Flags: 0
    #[unity::class_method(4)] pub fn cleanup(&self); // Offset: 0x1F8BE40 Flags: 0
    #[unity::class_method(5)] pub fn reset_alpha(&self); // Offset: 0x1F89660 Flags: 0
    #[unity::class_method(6)] pub fn set_alpha_forced(&self, alpha: f32, is_show: bool); // Offset: 0x1F8C200 Flags: 0
    #[unity::class_method(7)] pub fn set_show_form_unit_info(&self, is_show: bool); // Offset: 0x1F893E0 Flags: 0
    #[unity::class_method(8)] pub fn show(&self); // Offset: 0x1F8C360 Flags: 0
    #[unity::class_method(9)] pub fn hide(&self); // Offset: 0x1F8C3D0 Flags: 0
    #[unity::class_method(10)] pub fn update_camera(&self, side: UnitInfoSide); // Offset: 0x1F8BFF0 Flags: 0
    #[unity::class_method(11)] pub fn set_custom_offscreen_camera_enabled(&self, side: UnitInfoSide, is_enabled: bool); // Offset: 0x1F8C0C0 Flags: 0
    #[unity::class_method(12)] pub fn update(&self); // Offset: 0x1F8C410 Flags: 0
    #[unity::class_method(13)] pub fn get_render_texture(&self) -> &'static RenderTexture; // Offset: 0x1F8C7A0 Flags: 0
    #[unity::class_method(14)] pub fn get_image(&self) -> &'static Image; // Offset: 0x1F8C7B0 Flags: 0
    #[unity::class_method(15)] pub fn is_visible(&self) -> bool; // Offset: 0x1F8C600 Flags: 0
    #[unity::class_method(16)] pub fn ctor(&self); // Offset: 0x1F8C7C0 Flags: 0
}
impl UnityComponent for UnitInfoCharaImageMaskOffset {}

#[unity::class("App", "SingletonMonoBehaviourList`1")] pub struct SingletonMonoBehaviorList {}

#[skyline::from_offset(0x1fac4a0)] fn unit_info_window_chara_model_create_chara_model(this: &UnitInfoWindowCharaModel, character: &Character, optional_method: OptionalMethod) -> &'static mut Character;
#[skyline::from_offset(0x01fa7fb0)] fn destroy_char_model(this: &UnitInfoWindowCharaModel, char: &Character, optional_method: OptionalMethod);
#[skyline::from_offset(0x027dde40)] fn character_factory_create_for_unit_info(unit: &Unit, locator: &GameObject, optional_method: OptionalMethod) -> &'static mut Character;
#[skyline::from_offset(0x1f89290)] fn unit_info_set_visible(is_visible: bool, optional_method: OptionalMethod);
#[skyline::from_offset(0x1f86a50)] fn unit_info_set_unit(side: i32, unit: Option<&Unit>, b_relax: bool, r_reverse: bool, delay_load: bool, call_back: u64, optional_method: OptionalMethod);
#[unity::from_offset("App", "UnitInfo", "CharaOnlyOn")] fn unit_info_only_chara_only_on(is_change_chara_model_anim: bool, method_info: OptionalMethod);
#[unity::from_offset("App", "UnitInfo", "CharaOnlyOff")] fn unit_info_only_chara_off(method_info: OptionalMethod);