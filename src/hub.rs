use access::HubAccess;
use unity::{prelude::*, system::List};
use crate::unityengine::GameObject;

pub mod access;
pub mod util;
pub mod variable;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HubMiniMapMapMode {
    None = 0, // Attr: 17
    Small = 1, // Attr: 17
    Large = 2, // Attr: 17
}

#[unity::class("App", "HubMiniMap")]
pub struct HubMiniMap {}

#[unity::class("App", "HubPlayerController")]
pub struct HubPlayerController {}

#[unity::class("App", "HubLocatorGroup")]
pub struct HubLocatorGroup {
    active_group: u64,
    unit_list: u64,
    pub access_list: &'static mut List<HubAccess>,
    pub active: bool,
    pub system: bool,
    event_active: bool,
}

impl HubLocatorGroup {
    #[unity::class_method(9)] pub fn update_active(&self); // Offset: 0x28ADB50 Flags: 0
    #[unity::class_method(10)] pub fn is_valid(&self) -> bool; // Offset: 0x28ADFD0 Flags: 0
    #[unity::class_method(12)] pub fn add_new_locator(&self, locator: &GameObject, pid: &Il2CppString) -> &'static HubAccess; // Offset: 0x28AE300 Flags: 0
    #[unity::class_method(13)] pub fn clear_locator(&self, locator: &GameObject); // Offset: 0x28AE470 Flags: 0
    #[unity::class_method(14)] pub fn reload_locator(&self, access: &HubAccess); // Offset: 0x28AE590 Flags: 0
    #[unity::class_method(15)] pub fn release_characters(&self); // Offset: 0x28AEAE0 Flags: 0
    #[unity::class_method(17)] pub fn setup_after(&self); // Offset: 0x28AEB70 Flags: 0
    #[unity::class_method(18)] pub fn refresh_confirm_content(&self, pid: &Il2CppString); // Offset: 0x28AEC40 Flags: 0
    #[unity::class_method(19)] pub fn refresh_confirm_all(&self); // Offset: 0x28AED60 Flags: 0
    #[unity::class_method(20)] pub fn find_access(&self, pid: &Il2CppString) -> &'static HubAccess; // Offset: 0x28AEE80 Flags: 0
    #[unity::class_method(21)] pub fn update_locator(&self, locator: &Il2CppString); // Offset: 0x28AEFD0 Flags: 0
    #[unity::class_method(22)] pub fn relocate_access(&self); // Offset: 0x28AF120 Flags: 0
    #[unity::class_method(23)] pub fn is_character_loading(&self) -> bool; // Offset: 0x28AF4F0 Flags: 0
    #[unity::class_method(27)] pub fn save_accessory(&self); // Offset: 0x28AF8C0 Flags: 0
    #[unity::class_method(28)] pub fn restore_accessory(&self); // Offset: 0x28AFA60 Flags: 0
    #[unity::class_method(29)] pub fn reset_look_at(&self); // Offset: 0x28AFC30 Flags: 0
    #[unity::class_method(30)] pub fn reset_body(&self); // Offset: 0x28AFDB0 Flags: 0
    #[unity::class_method(31)] pub fn reload(&self, pid: &Il2CppString); // Offset: 0x28AFF30 Flags: 0
    // #[unity::class_method(33)] pub fn create_character(&self, pid: &Il2CppString, locator: &GameObject, access: &HubAccess, callback: Action<&HubUnitController>); // Offset: 0x28AE890 Flags: 0
}

impl HubMiniMap {
    #[unity::class_method(0)] pub fn next_change_mode(&self); // Offset: 0x28B9F60 Flags: 0
    #[unity::class_method(1)] pub fn show_system_menu(&self); // Offset: 0x28B7A10 Flags: 0
    #[unity::class_method(2)] pub fn hide_system_menu(&self); // Offset: 0x28B7D10 Flags: 0
    #[unity::class_method(3)] pub fn show(&self); // Offset: 0x28BA270 Flags: 0
    #[unity::class_method(4)] pub fn hide(&self); // Offset: 0x28BA290 Flags: 0
    #[unity::class_method(5)] pub fn push_layer(&self, layer: i32); // Offset: 0x28BA2B0 Flags: 0
    #[unity::class_method(6)] pub fn pop_layer(&self, layer: i32); // Offset: 0x28BA350 Flags: 0
    #[unity::class_method(25)] pub fn restore(&self); // Offset: 0x28BBB00 Flags: 0
    #[unity::class_method(27)] pub fn is_show(&self) -> bool; // Offset: 0x28BCF80 Flags: 0
    #[unity::class_method(28)] pub fn update(&self); // Offset: 0x28BD0A0 Flags: 0
    #[unity::class_method(29)] pub fn scroll(&self); // Offset: 0x28BD270 Flags: 0
    #[unity::class_method(30)] pub fn set_mode(&self, mode: HubMiniMapMapMode); // Offset: 0x28BA100 Flags: 0
}
impl HubPlayerController {
    #[unity::class_method(20)] pub fn hide(&self); // Offset: 0x23E3560 Flags: 0
    #[unity::class_method(21)] pub fn show(&self); // Offset: 0x23E3600 Flags: 0
    #[unity::class_method(22)] pub fn start(&self); // Offset: 0x23E36A0 Flags: 0
    #[unity::class_method(23)] pub fn load_character(&self); // Offset: 0x23E39A0 Flags: 0
    #[unity::class_method(24)] pub fn save_accessory(&self); // Offset: 0x23E3B70 Flags: 0
    #[unity::class_method(25)] pub fn restore_accessory(&self); // Offset: 0x23E3BB0 Flags: 0
    #[unity::class_method(26)] pub fn reload(&self); // Offset: 0x23E3C10 Flags: 0
    #[unity::class_method(30)] pub fn update_character_look_at(&self); // Offset: 0x23E4020 Flags: 0
    #[unity::class_method(37)] pub fn update_access(&self, force: bool); // Offset: 0x23E63F0 Flags: 0
    #[unity::class_method(38)] pub fn update_look_at(&self); // Offset: 0x23E4090 Flags: 0
    #[unity::class_method(45)] pub fn try_get_last_access(&self) -> Option<&'static HubAccess>; // Offset: 0x23E78E0 Flags: 0
    #[unity::class_method(46)] pub fn try_get_now_access(&self) -> Option<&'static HubAccess>; // Offset: 0x23E6D60 Flags: 0
}
