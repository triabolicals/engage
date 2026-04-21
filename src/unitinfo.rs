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
use crate::unityengine::{Camera, GameObject, RectTransform, RenderTexture, UnityComponent};
use crate::util::{get_singleton_proc_instance};

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
    #[unity::class_method(41)] pub fn get_face_camera_component(side: UnitInfoSide) -> &'static Camera;
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

/*
#[skyline::from_offset(0x1fac4a0)] fn unit_info_window_chara_model_create_chara_model(this: &UnitInfoWindowCharaModel, character: &Character, optional_method: OptionalMethod) -> &'static mut Character;
#[skyline::from_offset(0x01fa7fb0)] fn destroy_char_model(this: &UnitInfoWindowCharaModel, char: &Character, optional_method: OptionalMethod);
#[skyline::from_offset(0x027dde40)] fn character_factory_create_for_unit_info(unit: &Unit, locator: &GameObject, optional_method: OptionalMethod) -> &'static mut Character;
#[skyline::from_offset(0x1f89290)] fn unit_info_set_visible(is_visible: bool, optional_method: OptionalMethod);
#[skyline::from_offset(0x1f86a50)] fn unit_info_set_unit(side: i32, unit: Option<&Unit>, b_relax: bool, r_reverse: bool, delay_load: bool, call_back: u64, optional_method: OptionalMethod);
#[unity::from_offset("App", "UnitInfo", "CharaOnlyOn")] fn unit_info_only_chara_only_on(is_change_chara_model_anim: bool, method_info: OptionalMethod);
#[unity::from_offset("App", "UnitInfo", "CharaOnlyOff")] fn unit_info_only_chara_off(method_info: OptionalMethod);
 */