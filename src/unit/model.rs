use unity::engine::{Color, Vector3};
use unity::il2cpp::object::Array;
use unity::system::List;
use crate::combat::{CharacterJoint, ProportionParameters};
use crate::gamedata::assettable::{AssetTableResult, AssetTableSound};
use crate::gamedata::item::ItemData;
use crate::resourcemanager::TResourceHandle;
use super::*;
use crate::unityengine::{Animator, GameObject, SkinnedMeshRenderer, Transform, UnityComponent, UnityObject};

#[unity::class("App", "UnitActor")]
pub struct UnitActor {
    parent: unity::engine::MonoBehaviorFields,
    pub obj: &'static GameObject,
    pub unit_model: &'static UnitModel,
    pub god_model: Option<&'static UnitModel>,
}

#[unity::class("App", "UnitModel")]
pub struct UnitModel {
    parent: unity::engine::MonoBehaviorFields,
    unit: &'static Unit, // Offset 0x18, Attr: 1
    god_unit: &'static GodUnit, // Offset 0x20, Attr: 1
    m_handle_a: u64, // Offset 0x28, Attr: 1
    m_handle_b: u64, // Offset 0x30, Attr: 1
    pub handle: Option<&'static mut UnitModelResourceHandle>, // Offset 0x38, Attr: 1
    m_materials: &'static List<()>, // Offset 0x40, Attr: 1
    m_renderers: &'static List<()>, // Offset 0x48, Attr: 1
    pub m_hair_color: Color, // Offset 0x50, Attr: 6
    pub m_skin_color: Color, // Offset 0x60, Attr: 6
    pub m_mask_color100: Color, // Offset 0x70, Attr: 6
    pub m_mask_color075: Color, // Offset 0x80, Attr: 6
    pub m_mask_color050: Color, // Offset 0x90, Attr: 6
    pub m_mask_color025: Color, // Offset 0xA0, Attr: 6
    pub root: &'static GameObject, // Offset 0xB0, Attr: 1
    m_root_scale: Vector3<f32>, // Offset 0xB8, Attr: 1
    m_wing_scale: Vector3<f32>, // Offset 0xC4, Attr: 1
    m_load_mode: i32, //UnitModelLoadMode, // Offset 0xD0, Attr: 1
    pub joint: &'static CharacterJoint, // Offset 0xD8, Attr: 1
    pub proportion_parameters: &'static mut ProportionParameters, // Offset 0xE0, Attr: 1
    m_left_hand_object: &'static GameObject, // Offset 0xE8, Attr: 1
    m_right_hand_object: &'static GameObject, // Offset 0xF0, Attr: 1
    m_skin_quality: i32, //SkinQuality, // Offset 0xF8, Attr: 1
    m_skinned_mesh_renderers: &'static Array<&'static SkinnedMeshRenderer>, // Offset 0x100, Attr: 1
    m_animators: &'static Array<&'static Animator>, // Offset 0x108, Attr: 1
    m_play_anim: i32, // UnitAnimTypes, // Offset 0x110, Attr: 1
    m_idle_anim: i32, // UnitAnimTypes, // Offset 0x114, Attr: 1
    m_play_time: i32, // UnitAnimTimes, // Offset 0x118, Attr: 1
    m_equip_item: Option<&'static ItemData>, // Offset 0x120, Attr: 1
    m_force_item: Option<&'static ItemData>, // Offset 0x128, Attr: 1
    m_model_hash: i32, // Offset 0x130, Attr: 1
    m_equip_hash: i32, // Offset 0x134, Attr: 1
    m_speed: f32, // Offset 0x138, Attr: 1
    m_alpha: f32, // Offset 0x13C, Attr: 1
    m_map_alpha: f32, // Offset 0x140, Attr: 1
    m_is_visible: bool, // Offset 0x144, Attr: 1
    m_color_flags: i32, // UnitModelColorFlags, // Offset 0x148, Attr: 1
    m_dirty_flags: i32, // UnitModelDirtyFlags, // Offset 0x14C, Attr: 1
    m_interpolator_shine: u64, // &InterpolatorFloat, // Offset 0x150, Attr: 1
    m_interpolator_fader: u64, // &InterpolatorFloat, // Offset 0x158, Attr: 1
    m_interpolator_goder: u64, // &InterpolatorFloat, // Offset 0x160, Attr: 1
    m_head: &'static GameObject, // Offset 0x168, Attr: 1
    pub body: &'static GameObject, // Offset 0x170, Attr: 1
    m_ride: &'static GameObject, // Offset 0x178, Attr: 1
    m_wing_l: &'static Transform, // Offset 0x180, Attr: 1
    m_wing_r: &'static Transform, // Offset 0x188, Attr: 1
    m_trans: &'static Transform, // Offset 0x190, Attr: 1
    m_sound: AssetTableSound, // Offset 0x198, Attr: 1
}

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum UnitAnimTypes {
    None = 0, // Attr: 17
    StandBy = 1, // Attr: 17
    IdleRelax = 2, // Attr: 17
    IdleNormal = 3, // Attr: 17
    RunLoop = 4, // Attr: 17
    Start = 5, // Attr: 17
    Attack = 6, // Attr: 17
    Shoot = 7, // Attr: 17
    Special = 8, // Attr: 17
    Rod = 9, // Attr: 17
    Dance = 10, // Attr: 17
    MagicWeapon = 11, // Attr: 17
    Event1 = 12, // Attr: 17
    Event2 = 13, // Attr: 17
    Event3 = 14, // Attr: 17
    Event4 = 15, // Attr: 17
    Num = 16, // Attr: 17
}

impl UnityComponent for UnitModel{}
impl UnityObject for UnitModel {}
impl UnityComponent for UnitActor{}
impl UnityObject for UnitActor {}

#[unity::class("", "ResourceHandle")]
#[nested_from_type(UnitModel)]
pub struct UnitModelResourceHandle {
    /*
    pub body_prefab: &ResourceGameObject, // Offset 0x10, Attr: 6
    pub head_prefab: &ResourceGameObject, // Offset 0x18, Attr: 6
    pub ride_prefab: &ResourceGameObject, // Offset 0x20, Attr: 6
    pub left_hand_prefab: &ResourceGameObject, // Offset 0x28, Attr: 6
    pub right_hand_prefab: &ResourceGameObject, // Offset 0x30, Attr: 6
    pub body_anim: &ResourceAnimatorController, // Offset 0x38, Attr: 6
    pub ride_anim: &ResourceAnimatorController, // Offset 0x40, Attr: 6


     */
    junk: [u8; 0x30],
    pub acc_prefabs: &'static mut List<TResourceHandle>, // Offset 0x48, Attr: 6
    pub acc_locators: &'static mut List<Il2CppString>, // Offset 0x50, Attr: 6
}

impl UnitModel {
    #[unity::class_method(31)] pub fn get_folder_name(name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1FBDD00 Flags: 0
    #[unity::class_method(32)] pub fn get_asset_path(root: &Il2CppString, name: &Il2CppString, subs: &Array<&Il2CppString>) -> &'static Il2CppString; // Offset: 0x1FBDEC0 Flags: 0
    #[unity::class_method(33)] pub fn get_footer(name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1FBE090 Flags: 0
    #[unity::class_method(34)] pub fn head_folder(name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1FBE110 Flags: 0
    #[unity::class_method(35)] pub fn body_folder(name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1FBE260 Flags: 0
    #[unity::class_method(36)] pub fn acc_folder(name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1FBE3B0 Flags: 0
    #[unity::class_method(38)] pub fn load_async(&self, result: &AssetTableResult) -> bool; // Offset: 0x1FBE510 Flags: 0

}