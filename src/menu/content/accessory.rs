use unity::engine::ui::Image;
use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::List;
use crate::gamedata::accessory::{AccessoryData, AccessoryDataKinds};
use crate::menu::menu_item::accessory::AccessoryMenuItem;
use crate::tmpro::TextMeshProUGUI;
use crate::unit::Unit;
use super::*;
use crate::unityengine::{MonoBehaviorFields, GameObject, UnityComponent};

#[unity::class("App", "AccessoryShopChangeMenuContent")]
#[static_fields(AccessoryShopChangeMenuContentStaticFields)]
pub struct AccessoryShopChangeMenuContent {
    pub parent: BasicMenuContentFields,
    pub kind_icon: &'static Array<&'static AccessoryShopChangeMenuContentKindIcon>, // Offset 0xE8, Attr: 6
    pub content_object: &'static GameObject, // Offset 0xF0, Attr: 6
}

#[repr(C)]
pub struct AccessoryShopChangeMenuContentStaticFields {
    prefab_path: &'static Il2CppString, // Offset 0x0, Attr: 49
}
impl AccessoryShopChangeMenuContent {
    #[unity::class_method(0)] pub fn get_menu_item_content_max(&self) -> i32; // Offset: 0x27C13E0 Flags: 0
    #[unity::class_method(1)] pub fn calc_cursor_moved_pos_y(&self, menu_item_index: i32) -> f32; // Offset: 0x27C1480 Flags: 0
    #[unity::class_method(2)] pub fn set_kind(&self, kind: AccessoryDataKinds); // Offset: 0x27C00B0 Flags: 0
    #[unity::class_method(3)] pub fn set_to_prev_kind(&self, kind: AccessoryDataKinds) -> AccessoryDataKinds; // Offset: 0x27C0DE0 Flags: 0
    #[unity::class_method(4)] pub fn set_to_next_kind(&self, kind: AccessoryDataKinds) -> AccessoryDataKinds; // Offset: 0x27C1210 Flags: 0
    #[unity::class_method(5)] pub fn is_first_kind(&self, kind: AccessoryDataKinds) -> bool; // Offset: 0x27C0DA0 Flags: 0
    #[unity::class_method(6)] pub fn is_last_kind(&self, kind: AccessoryDataKinds) -> bool; // Offset: 0x27C11C0 Flags: 0
    #[unity::class_method(7)] pub fn get_kind_count(&self) -> i32; // Offset: 0x27C0230 Flags: 0
    #[unity::class_method(8)] pub fn ctor(&self); // Offset: 0x27C1570 Flags: 0
}

impl MenuContent for AccessoryShopChangeMenuContent {}
impl UnityComponent for AccessoryShopChangeMenuContent {}

#[unity::class("", "KindIcon")]
#[nested_from_type(AccessoryShopChangeMenuContent)]
pub struct AccessoryShopChangeMenuContentKindIcon {
    pub image: &'static mut Image, // Offset 0x10, Attr: 6
    pub kind: AccessoryDataKinds, // Offset 0x18, Attr: 6
}

#[unity::class("App", "AccessoryDetailInfoWindow")]
pub struct AccessoryDetailInfoWindow {
    parent: MonoBehaviorFields,
    pub accessory_name: &'static mut TextMeshProUGUI,
    pub message: &'static mut TextMeshProUGUI,
    pub body_parts: &'static mut Array<&'static mut BodyParts>,
}

#[unity::class("", "BodyParts")]
#[nested_from_type(AccessoryDetailInfoWindow)]
pub struct BodyParts {
    pub object: &'static GameObject,
    pub image: &'static mut Image,
    pub text: &'static mut TextMeshProUGUI,
}

#[unity::class("App", "AccessoryEquipmentInfo")]
pub struct AccessoryEquipmentInfo {
    parent: MonoBehaviorFields,
    pub content_object: &'static GameObject,
    pub cursor_object: &'static GameObject,
    pub menu_list: &'static mut List<AccessoryMenuItem>,
}
impl UnityComponent for AccessoryEquipmentInfo {}
impl AccessoryEquipmentInfo {
    #[unity::class_method(0)] pub fn create(); // Offset: 0x27B6310 Flags: 0
    #[unity::class_method(1)] pub fn ctor(&self); // Offset: 0x27B6320 Flags: 0
    #[unity::class_method(2)] pub fn open(&self); // Offset: 0x27B6330 Flags: 0
    #[unity::class_method(3)] pub fn close(&self); // Offset: 0x27B6430 Flags: 0
    #[unity::class_method(4)] pub fn build(&self, default_unit: &Unit); // Offset: 0x27B6530 Flags: 0
    #[unity::class_method(5)] pub fn set_data(&self, unit: &Unit); // Offset: 0x27B6AB0 Flags: 0
    #[unity::class_method(6)] pub fn get_item_content_max(&self) -> i32; // Offset: 0x27B68E0 Flags: 0
    #[unity::class_method(7)] pub fn show_cursor(&self); // Offset: 0x27B73A0 Flags: 0
    #[unity::class_method(8)] pub fn show_cursor2(&self, accessory_data: &AccessoryData); // Offset: 0x27B7450 Flags: 0
    #[unity::class_method(9)] pub fn show_cursor3(&self, kind: i32); // Offset: 0x27B7810 Flags: 0
    #[unity::class_method(10)] pub fn hide_cursor(&self); // Offset: 0x27B7AE0 Flags: 0
}
impl UnityComponent for AccessoryDetailInfoWindow {}

impl AccessoryDetailInfoWindow {
    #[unity::class_method(0)] pub fn ctor(&self); // Offset: 0x27B5A90 Flags: 0
    #[unity::class_method(1)] pub fn hide(&self); // Offset: 0x27B5AA0 Flags: 0
    #[unity::class_method(2)] pub fn show(&self); // Offset: 0x27B5AC0 Flags: 0
    #[unity::class_method(3)] pub fn close(&self); // Offset: 0x27B5AE0 Flags: 0
    #[unity::class_method(4)] pub fn set_data(&self, accessory_data: &AccessoryData, female: i32); // Offset: 0x27B5BE0 Flags: 0
}