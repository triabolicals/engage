use unity::engine::MonoBehaviorFields;
use unity::prelude::*;
use unity::system::List;

use crate::gamedata::{
    animal::AnimalData,
    *,
};
use crate::gamedata::hub::HubDisposData;
use crate::sequence::hub::HubSequence;
use crate::unityengine::{GameObject, Transform, UnityComponent, UnityObject};

#[unity::class("App", "HubAccess")]
pub struct HubAccess {
    parent: MonoBehaviorFields,
    pub aid: &'static Il2CppString, // Offset 0x18, Attr: 6
    player_transform: &'static Transform, // Offset 0x20, Attr: 1
    target_transform: &'static Transform, // Offset 0x28, Attr: 1
    junk: [u8; 0x30],
    pub access_data: Option<&'static mut HubAccessData>, // Offset 0x60, Attr: 1
    pub item_effect: Option<&'static GameObject>,
    /*
    help_offset: Vector3, // Offset 0x30, Attr: 1
    is_wall: bool, // Offset 0x3C, Attr: 1
    orig_position: Vector3, // Offset 0x40, Attr: 1
    orig_rotation: Quaternion, // Offset 0x4C, Attr: 1
    m_access_data: &HubAccessData, // Offset 0x60, Attr: 1
    item_effect: &GameObject, // Offset 0x68, Attr: 1
    access_cursor_object: &GameObject, // Offset 0x70, Attr: 1
    access_cursor: &HubAccessCursor, // Offset 0x78, Attr: 1
     */
}
impl UnityComponent for HubAccess {}
impl UnityObject for HubAccess {}

#[unity::class("App", "HubAccessData")]
pub struct HubAccessData {
    pub aid: Option<&'static Il2CppString>,
    pub dispos_data: &'static HubDisposData,
    pub is_story: bool,
    pub is_reliance: bool,
    pub is_god: bool,
    pub is_unit: bool,
    pub is_animal: bool,
    pub is_person: bool,
    pub talk_index: i32,
    pub is_hero_birthday: bool,
    pub talk_item: Option<&'static Il2CppString>,
    pub item_count: i32,
}

#[unity::class("App", "HubAccessManager")]
pub struct HubAccessManager {
    pub scene_name: &'static Il2CppString,
    pub access_list: &'static List<HubAccessData>,
    pub dispos_list: &'static List<HubDisposData>,
    pub dispos_item_list: &'static List<HubDisposData>,
    talk_limit: * const u8,
    pub animal_list: &'static List<AnimalData>, 
}

impl HubAccess {
    #[unity::class_method(42)] pub fn locate(&self, locator: &Il2CppString); // Offset: 0x2169360 Flags: 0
    #[unity::class_method(43)] pub fn clear(&self); // Offset: 0x216A0B0 Flags: 0
    #[unity::class_method(44)] pub fn done_access(&self); // Offset: 0x216A0F0 Flags: 0
    #[unity::class_method(45)] pub fn execute(&self, hub_sequence: &HubSequence) -> bool; // Offset: 0x216A2B0 Flags: 0
    #[unity::class_method(46)] pub fn refresh(&self); // Offset: 0x216A820 Flags: 0
    #[unity::class_method(55)] pub fn create_item_effect(&self); // Offset: 0x2169CE0 Flags: 0
}


impl HubAccessManager {
    #[unity::class_method(12)] pub fn is_item_type(dispos: &HubDisposData) -> bool; // Offset: 0x216FA20 Flags: 0
    #[unity::class_method(14)] pub fn confirm_content(&self); // Offset: 0x21708F0 Flags: 0
    #[unity::class_method(15)] pub fn reset(&self); // Offset: 0x2170380 Flags: 0
    #[unity::class_method(16)] pub fn refresh(&self); // Offset: 0x2170EE0 Flags: 0
    #[unity::class_method(17)] pub fn is_used_locator(&self, locator_name: &Il2CppString) -> bool; // Offset: 0x21706A0 Flags: 0
    #[unity::class_method(19)] pub fn try_remove_access_object(&self, data: &HubDisposData) -> bool; // Offset: 0x2170850 Flags: 0
    #[unity::class_method(20)] pub fn add_new_locator(&self, locator: &Il2CppString) -> Option<&'static HubAccessData>; // Offset: 0x2171AD0 Flags: 0
    #[unity::class_method(21)] pub fn clear_locator(&self, locator: &Il2CppString); // Offset: 0x2171C40 Flags: 0
    #[unity::class_method(22)] pub fn find_locator(&self, locator: &Il2CppString) -> Option<&'static HubAccessData>; // Offset: 0x2169BF0 Flags: 0
    #[unity::class_method(23)] pub fn find_pid(&self, pid: &Il2CppString) -> Option<&'static HubAccessData>; // Offset: 0x2171CC0 Flags: 0
    #[unity::class_method(24)] pub fn is_already_located(&self, pid: &Il2CppString) -> bool; // Offset: 0x2171E40 Flags: 0
    #[unity::class_method(25)] pub fn is_available_pid(&self, pid: &Il2CppString, disabled_talk: bool) -> bool; // Offset: 0x2171E60 Flags: 0
    #[unity::class_method(31)] pub fn entry_talk_limit(&self, talk_type: &Il2CppString) -> bool; // Offset: 0x216EE20 Flags: 0
    #[unity::class_method(32)] pub fn get_not_take_piece_of_bond(&self) -> i32; // Offset: 0x21733A0 Flags: 0
}

impl HubAccessData {
    // marks the access point as interacted
    #[unity::class_method(15)] pub fn get_is_done(&self) -> bool; // Offset: 0x21681C0 Flags: 0
    #[unity::class_method(16)] pub fn get_is_accessed(&self) -> bool; // Offset: 0x2167FB0 Flags: 0
    #[unity::class_method(32)] pub fn done_access(&self) -> bool; // Offset: 0x216A220 Flags: 0
    #[unity::class_method(30)] pub fn update_access_count(&self); // Offset: 0x21687C0 Flags: 0
    #[unity::class_method(31)] pub fn pre_locate(&self); // Offset: 0x216CC00 Flags: 0
    #[unity::class_method(39)] pub fn try_get_pid(&self) -> Option<&'static Il2CppString>; // Offset: 0x216BBD0 Flags: 0
        // Checks if access point is interacted
}