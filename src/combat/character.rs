use unity::engine::{Color, Vector3};
use unity::il2cpp::object::Array;
use unity::macro_context::{Il2CppClass, MethodInfo};
use unity::prelude::{Il2CppClassData, Il2CppString, OptionalMethod};
use unity::system::{Dictionary, List};
use unity::system::action::Action;
use crate::combat::{AnimAsset, Character, CharacterGameStatus};
use crate::gamedata::assettable::{AssetTableResult, AssetTableSound};
use crate::god::GodUnit;
use crate::unit::Unit;
use crate::unityengine::{GameObject, SkinnedMeshRenderer, Transform, UnityComponent, UnityObject};

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum AssetType {
    BodyModel = 0, // Attr: 17
    DressModel = 1, // Attr: 17
    HeadModel = 2, // Attr: 17
    HairModel = 3, // Attr: 17
    RideModel = 4, // Attr: 17
    RideDressModel = 5, // Attr: 17
    LeftHandModel = 6, // Attr: 17
    RightHandModel = 7, // Attr: 17
    PrivateEffectCatalog = 8, // Attr: 17
    MagicPrefab = 9, // Attr: 17
    BodyAnim = 10, // Attr: 17
    RideAnim = 11, // Attr: 17
    Acc0Model = 12, // Attr: 17
    Acc1Model = 13, // Attr: 17
    Acc2Model = 14, // Attr: 17
    Acc3Model = 15, // Attr: 17
    Acc4Model = 16, // Attr: 17
    Acc5Model = 17, // Attr: 17
    Acc6Model = 18, // Attr: 17
    Acc7Model = 19, // Attr: 17
    Max = 20, // Attr: 17
}
#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum WeaponStyle {
    DoNothing = 0, // Attr: 17
    Hit = 1, // Attr: 17
    Shoot = 2, // Attr: 17
    Magic = 3, // Attr: 17
    Special = 4, // Attr: 17
}

#[unity::class("Combat", "DressUtility")]
pub struct DressUtility { pub body_cache: &'static mut HierarchyCache, }

impl DressUtility {
    pub fn get_method_class() -> &'static Il2CppClass { DressUtility::class() }
    pub fn new(body_root: &Transform) -> &'static mut Self {
        let ut = DressUtility::instantiate().unwrap();
        ut.ctor(body_root);
        ut
    }
    #[unity::class_method(0)] pub fn get_cache(&self) -> &'static HierarchyCache; // Offset: 0x22D2AC0 Flags: 0
    #[unity::class_method(1)] pub fn get_item(&self, name: &Il2CppString) -> Option<&'static Transform>; // Offset: 0x22D2AD0 Flags: 0
    #[unity::class_method(2)] pub fn ctor(&self, body_root: &Transform); // Offset: 0x22D2AE0 Flags: 0
    #[unity::class_method(3)] pub fn transplant_dress_only_bones(&self, dress: &Transform); // Offset: 0x22D2B60 Flags: 0
    #[unity::class_method(4)] pub fn transplant_skinned_mesh(&self, dress_go: &GameObject, dress_meshes: &Array<&'static SkinnedMeshRenderer>, parent_name: &Il2CppString); // Offset: 0x22D2E20 Flags: 0
    #[unity::class_method(4)] pub fn transplant_skinned_mesh_mut(&self, dress_go: &GameObject, dress_meshes: &Array<&'static mut SkinnedMeshRenderer>, parent_name: &Il2CppString); // Offset: 0x22D2E20 Flags: 0
    #[unity::class_method(5)] pub fn transplant_mesh(&self, acc_go: &GameObject, parent_name: &Il2CppString); // Offset: 0x22D3100 Flags: 0
}
#[unity::class("Combat", "HierarchyCache")]
pub struct HierarchyCache {
    pub dic: &'static mut Dictionary<'static, &'static Il2CppString, &'static Transform>,
}

impl HierarchyCache {
    #[unity::class_method(1)] pub fn get_item(&self, name: &Il2CppString) -> Option<&'static Transform>; // Offset: 0x2166370 Flags: 0
    #[unity::class_method(2)] pub fn ctor(&self, root: &Transform); // Offset: 0x21663F0 Flags: 0
    #[unity::class_method(3)] pub fn add_range(&self, root: &Transform); // Offset: 0x21664A0 Flags: 0
}

#[unity::class("Combat", "CharacterAppearance")]
pub struct CharacterAppearance {
    pub assets: &'static mut Array<&'static mut CharacterAssetT>,
    pub animset_names: &'static List<Il2CppString>,
    pub acc_target: &'static mut Array<Option<&'static Il2CppString>>,
    pub mask_color_100: Color,
    pub mask_color_075: Color,
    pub mask_color_050: Color,
    pub mask_color_025: Color,
    pub skin_color: Color,
    pub grad_color: Color,
    pub hair_color: Color,
    pub toon_shadow_color: Color,
    pub sound: AssetTableSound,
    pub proportion: &'static mut ProportionParameters,
    pub animset: &'static mut CharacterAnimset,
    pub weapon_style: i32,
}
#[unity::class("Combat", "CharacterAnimset")]
pub struct CharacterAnimset {
    preload_anim: i64,
    body_daoc: u64,
    ride_daoc: u64,
    pub attack_1: Option<&'static mut AnimAsset>,
    pub attack_1r: Option<&'static mut AnimAsset>,
    pub attack_2: Option<&'static mut AnimAsset>,
    pub attack_2r: Option<&'static mut AnimAsset>,
    pub attack_3: Option<&'static mut AnimAsset>,
    pub attack_3r: Option<&'static mut AnimAsset>,
    pub attack_4: Option<&'static mut AnimAsset>,
    pub attack_4r: Option<&'static mut AnimAsset>,
    pub attack_5: Option<&'static mut AnimAsset>,
    pub attack_5r: Option<&'static mut AnimAsset>,
    pub attack_c: Option<&'static mut AnimAsset>,
    pub attack_cr: Option<&'static mut AnimAsset>,
    pub attack_t: Option<&'static mut AnimAsset>,
    pub attack_tr: Option<&'static mut AnimAsset>,
}

#[unity::class("Combat", "ProportionParameters")]
pub struct ProportionParameters {
    pub scale_all: f32, // Offset: 0x10
    pub scale_head: f32, // Offset: 0x14
    pub scale_neck: f32, // Offset: 0x18
    pub scale_torso: f32, // Offset: 0x1C
    pub scale_shoulders: f32, // Offset: 0x20
    pub scale_arms: f32, // Offset: 0x24
    pub scale_hands: f32, // Offset: 0x28
    pub scale_legs: f32, // Offset: 0x2C
    pub scale_feet: f32, // Offset: 0x30
    pub volume_arms: f32, // Offset: 0x34
    pub volume_legs: f32, // Offset: 0x38
    pub volume_bust: f32, // Offset: 0x3C
    pub volume_abdomen: f32, // Offset: 0x40
    pub volume_torso: f32, // Offset: 0x44
    pub hip_joint_height: f32, // Offset: 0x48
    pub ankle_height: f32, // Offset: 0x4C
    pub target_nodes: &'static mut Array<Option<&'static Transform>>,
    pub proportion_scale: &'static mut Array<Vector3<f32>>,
    pub write_count: i32,
}
impl ProportionParameters {
    #[unity::class_method(0)] pub fn import_leg_scale_params_from_model(&self, root: &Transform); // Offset: 0x2C08280 Flags: 0
    #[unity::class_method(1)] pub fn ctor(&self); // Offset: 0x2C083A0 Flags: 0
    #[unity::class_method(3)] pub fn reset_to_one(&self); // Offset: 0x2C08490 Flags: 0
    #[unity::class_method(5)] pub fn random(&self); // Offset: 0x2C087A0 Flags: 0
    #[unity::class_method(6)] pub fn get_is_valid(&self) -> bool; // Offset: 0x2C088F0 Flags: 0
    #[unity::class_method(7)] pub fn copy_from(&self, rhs: &ProportionParameters); // Offset: 0x2C08A10 Flags: 0
    #[unity::class_method(8)] pub fn import_from_asset_result(&self, r: &AssetTableResult); // Offset: 0x2C08AA0 Flags: 0
    #[unity::class_method(11)] pub fn calculate(&self, j: &CharacterJoint); // Offset: 0x2C091E0 Flags: 0
    #[unity::class_method(12)] pub fn flush(&self); // Offset: 0x2C09D10 Flags: 0
    // crate::build_method_fn!(<_calculate>g_sxyz|28_0, (), 13, self, t: &Transform, value: f32); // Offset: 0x2C09820 Flags: 0
    // crate::build_method_fn!(<_calculate>g_s_yz|28_1, (), 14, self, t: &Transform, value: f32); // Offset: 0x2C09950 Flags: 0
    // crate::build_method_fn!(<_calculate>g_s_y_|28_2, (), 15, self, t: &Transform, value: f32); // Offset: 0x2C09A90 Flags: 0
    // crate::build_method_fn!(<_calculate>g_s_z|28_3, (), 16, self, t: &Transform, value: f32); // Offset: 0x2C09BD0 Flags: 0
}

impl CharacterAppearance {
    #[unity::class_method(1)] pub fn get_anim_set(&self) -> &'static CharacterAnimset; // Offset: 0x2B0AC00 Flags: 0
    #[unity::class_method(2)] pub fn set_anim_set(&self, value: &CharacterAnimset); // Offset: 0x2B0AC10 Flags: 0
    #[unity::class_method(3)] pub fn get_weapon_style(&self) -> WeaponStyle; // Offset: 0x2B0AC20 Flags: 0
    #[unity::class_method(4)] pub fn set_weapon_style(&self, value: WeaponStyle); // Offset: 0x2B0AC30 Flags: 0
    #[unity::class_method(7)] pub fn get_acc_targets(&self) -> &'static Array<String>; // Offset: 0x2B0AC80 Flags: 0
    #[unity::class_method(9)] pub fn get_is_ride(&self) -> bool; // Offset: 0x2B04340 Flags: 0
    #[unity::class_method(10)] pub fn get_is_flying(&self) -> bool; // Offset: 0x2B0AC90 Flags: 0
    #[unity::class_method(11)] pub fn get_is_corrupt_animal(&self) -> bool; // Offset: 0x2B0AD00 Flags: 0
    #[unity::class_method(12)] pub fn get_is_big_dragon(&self) -> bool; // Offset: 0x2B006E0 Flags: 0
    #[unity::class_method(13)] pub fn get_is_wyrm(&self) -> bool; // Offset: 0x2B00920 Flags: 0
    #[unity::class_method(14)] pub fn get_is_brawl(&self) -> bool; // Offset: 0x2B0AD80 Flags: 0
    #[unity::class_method(15)] pub fn get_has_rod(&self) -> bool; // Offset: 0x2B0AD90 Flags: 0
    #[unity::class_method(17)] pub fn get_is_high_class(&self) -> bool; // Offset: 0x2B0AF80 Flags: 0
    #[unity::class_method(19)] pub fn get_is_last_boss(&self) -> bool; // Offset: 0x2B00660 Flags: 0
    #[unity::class_method(20)] pub fn get_backward_cancel_position(&self) -> f32; // Offset: 0x2AFFAF0 Flags: 0
    #[unity::class_method(21)] pub fn ctor(&self); // Offset: 0x2B0AFF0 Flags: 0
    #[unity::class_method(22)] pub fn ctor2(&self, rhs: &CharacterAppearance, weapon_style: WeaponStyle); // Offset: 0x2B0B570 Flags: 0
    #[unity::class_method(23)] pub fn dispose(&self); // Offset: 0x2B0BD60 Flags: 0
    #[unity::class_method(25)] pub fn set_default_name(&self); // Offset: 0x2B0C0B0 Flags: 0

    #[unity::class_method(27)] pub fn load_async(&self); // Offset: 0x2B0C320 Flags: 0
    #[unity::class_method(28)] pub fn load_anim_set_async(&self); // Offset: 0x2B0C450 Flags: 0
    #[unity::class_method(29)] pub fn is_loading(&self) -> bool; // Offset: 0x2B0C4A0 Flags: 0

    #[unity::class_method(33)] pub fn get_tall(&self) -> f32; // Offset: 0x2B0CBE0 Flags: 0
    #[unity::class_method(34)] pub fn get_body_size(&self) -> f32; // Offset: 0x2B0CEE0 Flags: 0
    #[unity::class_method(35)] pub fn modify_colors(&self, go: &GameObject); // Offset: 0x2B0CF90 Flags: 0
    // #[unity::class_method(36)] pub fn has_material_to_modify(&self, mats: &Array<Material>) -> bool; // Offset: 0x2B0D8B0 Flags: 0
    #[unity::class_method(37)] pub fn destroy_instanced_materials(&self); // Offset: 0x2B0BE60 Flags: 0
    #[unity::class_method(41)] pub fn create_from_game_status(game_status: &CharacterGameStatus, map_distance: i32, conditions: Option<&Array<&Il2CppString>>) -> &'static mut CharacterAppearance; // Offset: 0x2B0EA10 Flags: 0
    #[unity::class_method(42)] pub fn create_for_sound(unit: &Unit) -> &'static mut CharacterAppearance; // Offset: 0x2B0F340 Flags: 0
    #[unity::class_method(43)] pub fn create_from_god_unit(g_unit: &GodUnit, conditions: &Array<&Il2CppString>) -> &'static mut CharacterAppearance; // Offset: 0x2B0F460 Flags: 0
    #[unity::class_method(44)] pub fn create_from_preset(name: &Il2CppString) -> &'static mut CharacterAppearance; // Offset: 0x2B0F4F0 Flags: 0
    #[unity::class_method(45)] pub fn get_constions(conditions: Option<&Array<&Il2CppString>>) -> &'static Array<&'static Il2CppString>; // Offset: 0x2B0EB20 Flags: 0
    #[unity::class_method(46)] pub fn create_from_result(result: &AssetTableResult, map_distance_1or2: i32) -> &'static mut CharacterAppearance; // Offset: 0x2B0ED80 Flags: 0
    #[unity::class_method(47)] pub fn calc_weapon_style(&self, map_distance: i32) -> WeaponStyle; // Offset: 0x2B0CA30 Flags: 0
    #[unity::class_method(48)] pub fn is_same_character(l: &CharacterAppearance, r: &CharacterAppearance) -> bool; // Offset: 0x2B0F570 Flags: 0
}

#[unity::class("Combat", "CharacterBuilder")]
pub struct CharacterBuilder {
    monobehaviour_fields: u64,
    pub appearance: &'static mut CharacterAppearance,
    pub viewer_preset_index: i32, // Offset: 0x24
    pub viewer_person_index: i32, // Offset: 0x28
    pub viewer_job_index: i32, // Offset: 0x2C
    pub viewer_weapon_index: i32, // Offset: 0x30
    pub initial_invisibility: bool, // Offset: 0x34
    pub is_done: bool, // Offset: 0x35
    pub is_building_watch_from_character_asset_form: bool, // Offset: 0x36
    pub body: &'static GameObject,
    pub dress: &'static GameObject,
    pub head: &'static GameObject,
    pub hair: &'static GameObject,
    pub left: &'static GameObject,
    pub right: &'static GameObject,
    pub ride: &'static GameObject,
    pub rideress: &'static GameObject,
    body_aoc: u64,
    ride_aoc: u64,
    magic: u64,
    private_effect: u64,
    pub dress_ut: Option<&'static mut DressUtility>,
}

impl CharacterBuilder  {
    #[unity::class_method(24)] pub fn on_destroy(&self); // Offset: 0x27D5840 Flags: 0
    #[unity::class_method(25)] pub fn get_main_weapon(&self) -> &'static GameObject; // Offset: 0x27D5850 Flags: 0
    #[unity::class_method(26)] pub fn get_main_hand(&self) -> &'static Transform; // Offset: 0x27D58E0 Flags: 0
    #[unity::class_method(29)] pub fn get_weapon_name(&self) -> &'static Il2CppString; // Offset: 0x27D59B0 Flags: 0
    #[unity::class_method(30)] pub fn set_weapon_name(&self, value: &Il2CppString); // Offset: 0x27D5AD0 Flags: 0
    #[unity::class_method(31)] pub fn get_is_brawl(&self) -> bool; // Offset: 0x27D5CF0 Flags: 0
    #[unity::class_method(32)] pub fn get_is_flying(&self) -> bool; // Offset: 0x27D5D00 Flags: 0
    #[unity::class_method(33)] pub fn get_tall(&self) -> f32; // Offset: 0x27D5D10 Flags: 0
    #[unity::class_method(34)] pub fn get_is_visible(&self) -> bool; // Offset: 0x27D5D20 Flags: 0
    #[unity::class_method(35)] pub fn set_is_visible(&self, value: bool); // Offset: 0x27D4770 Flags: 0
    #[unity::class_method(37)] pub fn build_hierarchy(&self); // Offset: 0x27D6260 Flags: 0
    #[unity::class_method(38)] pub fn build_base_hierarchy(&self); // Offset: 0x27D64C0 Flags: 0
    #[unity::class_method(39)] pub fn setup_animator_with_daoc(&self); // Offset: 0x27D7970 Flags: 0
    #[unity::class_method(40)] pub fn setup_animator_with_aoc(&self); // Offset: 0x27D7580 Flags: 0
    #[unity::class_method(41)] pub fn attach_head_hair_and_weapons(&self); // Offset: 0x27D6800 Flags: 0
    #[unity::class_method(42)] pub fn unload_d(&self, chr: &Transform); // Offset: 0x27D7F90 Flags: 0
    #[unity::class_method(43)] pub fn others(&self); // Offset: 0x27D7280 Flags: 0
    #[unity::class_method(44)] pub fn get_go(&self, asset: &CharacterAssetT) -> &'static GameObject; // Offset: 0x27D7460 Flags: 0
    #[unity::class_method(45)] pub fn repair_aoc(&self); // Offset: 0x27D64B0 Flags: 0
    #[unity::class_method(46)] pub fn attach_ridress(&self); // Offset: 0x27D6E20 Flags: 0
    #[unity::class_method(47)] pub fn attach_dress(&self); // Offset: 0x27D7000 Flags: 0
    #[unity::class_method(48)] pub fn build_proportion(&self); // Offset: 0x27D82B0 Flags: 0
    #[unity::class_method(49)] pub fn attach_accessory(&self, i: i32); // Offset: 0x27D7FD0 Flags: 0
    #[unity::class_method(51)] pub fn replace_goname(go: &GameObject, src: &Il2CppString, dst: &Il2CppString); // Offset: 0x27D7E60 Flags: 0
    #[unity::class_method(54)] pub fn set_visible_forced(&self, value: bool); // Offset: 0x27D5D30 Flags: 0
    #[unity::class_method(55)] pub fn make_cached_renderers_list(&self); // Offset: 0x27D9700 Flags: 0
    #[unity::class_method(56)] pub fn invalidate_cached_renderers_list(&self); // Offset: 0x27D6450 Flags: 0
    #[unity::class_method(57)] pub fn ctor(&self); // Offset: 0x27D9840 Flags: 0
    #[unity::class_method(3, vtable)] pub fn to_string(&self) -> &'static Il2CppString; // Offset: 0x32EFDE0 Flags: 0
    #[unity::class_method(6, vtable)] pub fn set_via_table_result(&self, r: &AssetTableResult); // Offset: 0x27D3D10 Flags: 0
    #[unity::class_method(7, vtable)] pub fn unload_d2(&self, chr: &Transform); // Offset: 0x27D7F90 Flags: 0
}
impl CharacterAssetFormMethods for CharacterBuilder {}
impl UnityComponent for CharacterBuilder {}

#[unity::class("Combat", "CharacterAssetForm")]
pub struct CharacterAssetForm {
    pub appearance: &'static mut CharacterAppearance, // Offset 0x18, Attr: 6
    pub m_b_self_appearance: bool, // Offset 0x20, Attr: 1
    pub viewer_preset_index: i32, // Offset 0x24, Attr: 6
    pub viewer_person_index: i32, // Offset 0x28, Attr: 6
    pub viewer_job_index: i32, // Offset 0x2C, Attr: 6
    pub viewer_weapon_index: i32, // Offset 0x30, Attr: 6
    pub initial_invisibility: bool, // Offset 0x34, Attr: 1
    pub is_done: bool, // Offset 0x35, Attr: 1
    pub is_building_watch_from_character_asset_form: bool, // Offset 0x36, Attr: 4
}
impl CharacterAssetForm {
    #[unity::class_method(15,CharacterAssetForm)] pub fn attach(me: &GameObject, parent: &Transform); // Offset: 0x27D3F30 Flags: 0
}

pub trait CharacterAssetFormMethods: Il2CppClassData {
    #[unity::class_method(0, CharacterAssetForm)] fn get_item(&self, index: i32) -> Option<&'static CharacterAssetT>; // Offset: 0x27D3A40 Flags: 0
    #[unity::class_method(1, CharacterAssetForm)] fn get_item2(&self, asset_type: AssetType) -> Option<&'static CharacterAssetT>; // Offset: 0x27D3A50 Flags: 0
    #[unity::class_method(2, CharacterAssetForm)] fn get_initial_invisibility(&self) -> bool; // Offset: 0x27D3A60 Flags: 0
    #[unity::class_method(3, CharacterAssetForm)] fn set_initial_invisibility(&self, value: bool); // Offset: 0x27D3A70 Flags: 0
    #[unity::class_method(4, CharacterAssetForm)] fn get_is_done(&self) -> bool; // Offset: 0x27D3A80 Flags: 0
    #[unity::class_method(5, CharacterAssetForm)] fn set_is_done(&self, value: bool); // Offset: 0x27D3A90 Flags: 0
    #[unity::class_method(6, CharacterAssetForm)] fn build(&self, appearance: &CharacterAppearance, invisible: bool); // Offset: 0x27D3AA0 Flags: 0
    #[unity::class_method(11,CharacterAssetForm)] fn begin_contents_change(&self); // Offset: 0x27D3CF0 Flags: 0
    #[unity::class_method(12,CharacterAssetForm)] fn end_contents_change(&self); // Offset: 0x27D3D00 Flags: 0
    #[unity::class_method(13,CharacterAssetForm)] fn set_via_table_result(&self, r: &AssetTableResult); // Offset: 0x27D3D10 Flags: 0
    #[unity::class_method(14,CharacterAssetForm)] fn unload_d(&self, chr: &Transform); // Offset: 0x27D3DB0 Flags: 0

    #[unity::class_method(16,CharacterAssetForm)] fn attach2(&self, me: &GameObject, parent: &Il2CppString); // Offset: 0x27D4020 Flags: 0
    #[unity::class_method(17,CharacterAssetForm)] fn find_in_children(&self, parent: &Il2CppString) -> &'static Transform; // Offset: 0x27D4190 Flags: 0
    #[unity::class_method(19,CharacterAssetForm)] fn on_destroy(&self); // Offset: 0x27D42A0 Flags: 0
    #[unity::class_method(21,CharacterAssetForm)] fn ctor(&self); // Offset: 0x27D42C0 Flags: 0
}
impl CharacterAssetFormMethods for CharacterAssetForm {}

#[unity::class("Combat", "CharacterAsset")]
pub struct CharacterAssetT {
    pub asset_type: i32,
    pub name: Option<&'static Il2CppString>,
    pub addr_path: Option<&'static Il2CppString>,
}
impl CharacterAssetT {
    fn get_method(&self, method_name: &str, arg_count: usize) -> Option<&MethodInfo> {
        if let Some(method) = self.klass.get_method_from_name(method_name, arg_count).ok()
            .or_else(|| self.klass._1.parent.get_method_from_name(method_name, arg_count).ok())
        {
            Some(method)
        }
        else {
            println!("Failed to get {} in {}", method_name, self.klass.get_name());
            None
        }
    }
    pub fn load_async(&self, call_back: Option<&Action>) {
        if let Some(method) = self.get_method("LoadAsync", 1) {
            let load_async = unsafe { std::mem::transmute::<_, fn(&Self, Option<&Action>, &MethodInfo)>(method.method_ptr) };
            load_async(&self, call_back, method);
        }
    }
    pub fn set_name(&self, name: &Il2CppString) {
        if let Some(method) = self.get_method("set_Name", 1){
            let function = unsafe { std::mem::transmute::<_, fn(&Self, &Il2CppString, &MethodInfo)>(method.method_ptr) };
            function(&self, name, method);
        }
    }
    pub fn get_asset(&self) -> Option<&'static GameObject> {
        if let Some(method) = self.get_method("get_Asset", 0){
            let function = unsafe { std::mem::transmute::<_, fn(&Self, &MethodInfo) -> Option<&'static GameObject>>(method.method_ptr) };
            function(&self, method)
        }
        else { None }
    }
    pub fn dispose(&self) {
        if let Some(method) = self.get_method("Dispose", 0){
            let function = unsafe { std::mem::transmute::<_, fn(&Self, &MethodInfo)>(method.method_ptr) };
            function(&self, method)
        }
    }
    pub fn is_ready(&self) -> bool {
        if let Some(method) = self.get_method("IsReady", 0){
            let function = unsafe { std::mem::transmute::<_, fn(&Self, &MethodInfo) -> bool >(method.method_ptr) };
            function(&self, method)
        }
        else { false }
    }
    pub fn instantiate_object(&self, target: &Transform) -> Option<&'static GameObject> {
        if let Some(method) = self.get_method("Instantiate", 1){
            let function = unsafe { std::mem::transmute::<_, fn(&Self, &Transform, &MethodInfo) -> Option<&'static GameObject>>(method.method_ptr) };
            function(&self, target, method)
        }
        else { None }
    }
}

#[unity::class("Combat", "CharacterProportion")]
pub struct CharacterProportion {
    parent: u64,
    pub proportion_parameters: &'static mut ProportionParameters,
    pub shadow_scale_all: f32,
    pub shadow_scale_legs: f32,
    pub shadow_scale_feet: f32,
    pad: i32,
    pub chr_jnt: &'static mut CharacterJoint,
    pub cp: &'static mut Character,
}
impl UnityComponent for CharacterProportion {}
impl CharacterProportion {
    #[unity::class_method(0)] pub fn is_shadow_scale_one(&self) -> bool; // Offset: 0x25E7DA0 Flags: 0
    #[unity::class_method(1)] pub fn get_character_joint(&self) -> &'static CharacterJoint; // Offset: 0x25E7DE0 Flags: 0
    #[unity::class_method(2)] pub fn get_cp(&self) -> &'static Character; // Offset: 0x25E7EA0 Flags: 0
    #[unity::class_method(3)] pub fn start(&self); // Offset: 0x25E7F60 Flags: 0
    #[unity::class_method(4)] pub fn reset(&self); // Offset: 0x25E80E0 Flags: 0
    #[unity::class_method(5)] pub fn swap(&self); // Offset: 0x25E81A0 Flags: 0
    #[unity::class_method(6)] pub fn late_update(&self); // Offset: 0x25E8270 Flags: 0
    #[unity::class_method(7)] pub fn commit_changes(&self); // Offset: 0x25E8310 Flags: 0
}

#[unity::class("Combat", "CharacterJoint")]
pub struct CharacterJoint {
    parent: u64,
    pub cp: &'static Character, // Offset 0x18, Attr: 1
    pub is_late_update: bool, // Offset 0x20, Attr: 1
    pub use_position_cache: bool, // Offset 0x21, Attr: 1
    pub c_trans_position: Vector3<f32>, // Offset 0x24, Attr: 1
    pub c_trans_ride_position: Vector3<f32>, // Offset 0x30, Attr: 1
    pub c_hip_jnt_position: Vector3<f32>, // Offset 0x3C, Attr: 1
    pub c_head_loc_position: Vector3<f32>, // Offset 0x48, Attr: 1
    pub c_ride_loc_position: Vector3<f32>, // Offset 0x54, Attr: 1
    pub c_spine2_jnt_position: Vector3<f32>, // Offset 0x60, Attr: 1
    l_wpn1_loc_position: Vector3<f32>, // Offset 0x6C, Attr: 1
    r_wpn1_loc_position: Vector3<f32>, // Offset 0x78, Attr: 1
    hand01_jnt_position: Vector3<f32>, // Offset 0x84, Attr: 1
    l_leg_loc_position: Vector3<f32>, // Offset 0x90, Attr: 1
    r_leg_loc_position: Vector3<f32>, // Offset 0x9C, Attr: 1
    l_limb_f_loc_position: Vector3<f32>, // Offset 0xA8, Attr: 1
    l_limb_r_loc_position: Vector3<f32>, // Offset 0xB4, Attr: 1
    r_limb_f_loc_position: Vector3<f32>, // Offset 0xC0, Attr: 1
    r_limb_r_loc_position: Vector3<f32>, // Offset 0xCC, Attr: 1
}

impl CharacterJoint {
    #[unity::class_method(7)] pub fn update(&self); // Offset: 0x27E7FD0 Flags: 0
    #[unity::class_method(92)] pub fn import_cache_from_hierarchy_cache(&self, hc: &HierarchyCache); // Offset: 0x27D83D0 Flags: 0
    #[unity::class_method(93)] pub fn flush(&self); // Offset: 0x27EA9E0 Flags: 0
    #[unity::class_method(46)] pub fn get_c_trans(&self) -> Option<&'static Transform>; // Offset: 0x27E7D80 Flags: 0
    #[unity::class_method(49)] pub fn get_c_head_loc(&self) -> Option<&'static Transform>; // Offset: 0x27DE590 Flags: 0
}
impl UnityComponent for CharacterJoint {}

// Namespace: Combat, Token: 0x20003EB
#[unity::class("Combat", "CharacterFactoryAsync")]
pub struct CharacterFactoryAsync { }

impl CharacterFactoryAsync {
    #[unity::class_method(0)] pub fn create(appearance: &CharacterAppearance, parent: &Transform, invisible: bool) -> &'static mut Character; // Offset: 0x27DE810 Flags: 0
    #[unity::class_method(1)] pub fn create_for_talk(appearance: &CharacterAppearance, parent: &Transform, invisible: bool) -> &'static mut Character; // Offset: 0x27DE200 Flags: 0
    #[unity::class_method(2)] pub fn create_impl(asset_path: &Il2CppString, appearance: &CharacterAppearance, parent: &Transform, invisible: bool) -> &'static Character; // Offset: 0x27DE880 Flags: 0
}