use unity::engine::Color;
use unity::engine::ui::Image;
use unity::prelude::*;
use crate::gamedata::accessory::AccessoryData;
use crate::impl_action_handlers;
use crate::menu::BasicMenu;
use crate::menu::menu_item::{BasicMenuItemContentFields, MenuItem, MenuItemContent};
use crate::tmpro::TextMeshProUGUI;
use crate::unityengine::GameObject;

#[unity::class("App", "AccessoryMenuItem")]
pub struct AccessoryMenuItem {
    pub menu: &'static mut BasicMenu<AccessoryMenuItem>,
    pub menu_item_content: Option<&'static mut AccessoryMenuItemContent>,
    pub name: &'static Il2CppString,
    pub index: i32,
    pub full_index: i32,
    pub attribute: i32,
    pub cursor_color: Color,
    pub active_active: Color,
    pub inactive_active: Color,
    pub padding: i32,
    pub accessory_data: Option<&'static AccessoryData>,
    pub kind: i32,
    pub decided: bool,
    pub female: i32,
    pub always_active: bool,
    pub select_event_handler: Option<&'static mut AccessoryMenuItemSelectHandler>,
    pub decide_event_handler: Option<&'static mut AccessoryMenuItemDecideHandler>,
}
impl AccessoryMenuItem {
    #[unity::class_method(8)]
    pub fn ctor(
        &self, data: Option<&AccessoryData>,
        accessory_kind: i32,
        decided: bool,
        lways_active: bool,
        female: i32,
        select_event_handler:  Option<&AccessoryMenuItemSelectHandler>,
        decided_handler: Option<&AccessoryMenuItemDecideHandler>
    );
    #[unity::class_method(12)] pub fn on_build_menu_item_content_(&self); // Offset: 0x27B7C50 Flags: 0
    #[unity::class_method(16)] pub fn set_decide(&self); // Offset: 0x27B7FB0 Flags: 0
    #[unity::class_method(17)] pub fn unset_decide(&self); // Offset: 0x27B8170 Flags: 0

}
impl MenuItem for AccessoryMenuItem {}

#[unity::class("App", "AccessoryEmptyMenuItem")]
pub struct AccessoryEmptyMenuItem {
    pub menu: &'static mut BasicMenu<AccessoryEmptyMenuItem>,
    pub menu_item_content: Option<&'static mut AccessoryMenuItemContent>,
    pub name: &'static Il2CppString,
    pub index: i32,
    pub full_index: i32,
    pub attribute: i32,
    pub cursor_color: Color,
    pub active_active: Color,
    pub inactive_active: Color,
    pub kind: i32,
    pub decide_event: Option<&'static mut AccessoryMenuItemDecideHandler>,
}
impl MenuItem for AccessoryEmptyMenuItem {}

#[unity::class("App", "AccessoryMenuItemContent")]
pub struct AccessoryMenuItemContent {
    pub parent: BasicMenuItemContentFields,
    pub fixed_cursor_object: &'static GameObject,
    fixed_cursor_image: &'static Image,
    frame_cursor_image: &'static Image,
    pub kind_icon: &'static GameObject,
    pub kind_icon_image: &'static mut Image,
    pub name_object: &'static GameObject,
    pub name_text: &'static mut TextMeshProUGUI,
}
impl AccessoryMenuItemContent {
    #[unity::class_method(1)] pub fn build_text_(&self); // Offset: 0x27B8510 Flags: 0
}
impl MenuItemContent<AccessoryMenuItem> for AccessoryMenuItemContent {}
impl MenuItemContent<AccessoryEmptyMenuItem> for AccessoryMenuItemContent {}

impl_action_handlers!([AccessoryMenuItem, "SelectEventHandler", AccessoryMenuItemSelectHandler], data: &AccessoryData);
impl_action_handlers!([AccessoryMenuItem, "DecideEventHandler", AccessoryMenuItemDecideHandler], data: &AccessoryData);
impl_action_handlers!([AccessoryMenuItem, "RequestCloseEventHandler", AccessoryMenuItemRequestCloseHandler],);