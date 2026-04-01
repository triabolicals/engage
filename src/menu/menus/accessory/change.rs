use unity::il2cpp::object::Array;
use unity::system::List;
use crate::gamedata::accessory::AccessoryData;
use crate::impl_action_handlers;
use crate::menu::{
    *, content::accessory::*,
    menu_item::{
        MenuItemContent,
        accessory::{AccessoryMenuItem, AccessoryMenuItemDecideHandler, AccessoryMenuItemSelectHandler}
    },
};
use crate::proc::ProcInstFields;
use crate::tmpro::TextMeshProUGUI;
use crate::unit::Unit;
use crate::unityengine::{GameObject, UnityComponent};

#[unity::class("App", "AccessoryShopChangeMenu")]
pub struct AccessoryShopChangeMenu {
    pub proc: ProcInstFields,
    pub menu_content: &'static mut AccessoryShopChangeMenuContent,
    pub menu_item_list: &'static mut List<AccessoryMenuItem>,
    pub full_menu_item_list: &'static mut List<AccessoryMenuItem>,
    pub status: &'static mut BasicMenuStatus,
    pub result: i32,
    pub scroll_precede_input_a: bool,
    pub row_num: i32,
    pub show_row_num: i32,
    pub select_index: i32,
    pub select_index_old: i32,
    pub scroll_index: i32,
    pub scroll_index_old: i32,
    pub reserved_select_index: i32,
    pub reserved_scroll_index: i32,
    pub reserved_show_row_num: i32,
    pub memory_display_index: i32,
    pub suspend: i32,
    pub kind: i32,
    pub unit: &'static Unit,
    pub selects: &'static mut Array<&'static mut BasicMenuSelect>,
    pub accessory_select: Option<&'static mut AccessoryMenuItemSelectHandler>,
    pub accessory_decide: Option<&'static mut AccessoryMenuItemDecideHandler>,
    pub request_close: Option<&'static mut AccessoryShopChangeMenuRequestCloseHandler>,
    pub change_kind: Option<&'static mut AccessoryShopChangeMenuChangeKindHandler>,
}
crate::impl_menu_class!(AccessoryShopChangeMenu);
impl_action_handlers!([AccessoryShopChangeMenu, "RequestCloseEventHandler", AccessoryShopChangeMenuRequestCloseHandler],);
impl_action_handlers!([AccessoryShopChangeMenu, "ChangeKindEventHandler", AccessoryShopChangeMenuChangeKindHandler], data: &AccessoryData);

#[unity::class("App", "AccessoryShopChangeRoot")]
pub struct AccessoryShopChangeRoot {
    parent: u64,
    pub menu_object: &'static GameObject,
    pub unit_name_object: &'static GameObject,
    pub unit_name: &'static mut TextMeshProUGUI,
    pub equipment_info_window: &'static GameObject,
    pub detail_info_window: &'static GameObject,
    pub key_help: &'static GameObject,
    key_animator: u64,
    key_help_controller: u64,
    pub change_root_proc: &'static mut AccessoryShopChangeRootProc,
    pub unit: &'static Unit,    // 0x50
    pub change_menu: &'static mut AccessoryShopChangeMenu,  //x58
    pub equipment_menu: &'static mut AccessoryEquipmentInfo,    //0x60
    pub detail_info: &'static mut AccessoryDetailInfoWindow,
    pub return_event_handler: Option<&'static mut AccessoryShopChangeRootReturnHandler>,
    pub accessory_data: Option<&'static AccessoryData>,
    changed: bool,
}
impl AccessoryShopChangeRoot {
    #[unity::class_method(0)] pub fn load_prefab_async(); // Offset: 0x27C15E0 Flags: 0
    #[unity::class_method(1)] pub fn is_loading_prefab() -> bool; // Offset: 0x27C1680 Flags: 0
    #[unity::class_method(2)] pub fn unload_prefab(); // Offset: 0x27C1700 Flags: 0
    #[unity::class_method(5)] pub fn destroy(root: &AccessoryShopChangeRoot); // Offset: 0x27C21A0 Flags: 0
    #[unity::class_method(6)] pub fn ctor(&self); // Offset: 0x27C2220 Flags: 0
    #[unity::class_method(7)] pub fn on_select_menu_item(&self, accessory_data: Option<&AccessoryData>); // Offset: 0x27C2230 Flags: 0
    #[unity::class_method(8)] pub fn on_decide_menu_item(&self, accessory_data: Option<&AccessoryData>); // Offset: 0x27C25E0 Flags: 0
    #[unity::class_method(9)] pub fn on_change_unit_to_prev(&self, watching: bool); // Offset: 0x27C2820 Flags: 0
    #[unity::class_method(10)] pub fn on_change_unit_to_next(&self, watching: bool); // Offset: 0x27C2D80 Flags: 0
    #[unity::class_method(11)] pub fn on_change_kind(&self, accessory_data: Option<&AccessoryData>); // Offset: 0x27C32E0 Flags: 0
    #[unity::class_method(12)] pub fn on_start_watching(&self) -> bool; // Offset: 0x27C3380 Flags: 0
    #[unity::class_method(13)] pub fn on_end_watching(&self); // Offset: 0x27C35A0 Flags: 0
    #[unity::class_method(14)] pub fn on_show_ui(&self); // Offset: 0x27C38E0 Flags: 0
    #[unity::class_method(15)] pub fn on_hide_ui(&self); // Offset: 0x27C3A50 Flags: 0
    #[unity::class_method(16)] pub fn on_request_close_menu(&self); // Offset: 0x27C3BC0 Flags: 0
}
impl UnityComponent for AccessoryShopChangeRoot {}

impl_action_handlers!([AccessoryShopChangeRoot, "ReturnEventHandler", AccessoryShopChangeRootReturnHandler], unit: &Unit, changed: bool);

#[unity::class("App", "AccessoryShopChangeRootProc")]
pub struct AccessoryShopChangeRootProc {
    pub proc: ProcInstFields,
    pub key_help_all: &'static GameObject,
    key_help_animator: &'static GameObject,
    pub change_unit_previous: Option<&'static mut AccessoryShopChangeRootProcChangeUnitPrevHandler>,
    pub change_unit_next: Option<&'static mut AccessoryShopChangeRootProcChangeUnitNextHandler>,
    pub start_watching: Option<&'static mut AccessoryShopChangeRootProcStartWatchingHandler>,
    pub end_watching: Option<&'static mut AccessoryShopChangeRootProcEndWatchingHandler>,
    pub show_ui: Option<&'static mut AccessoryShopChangeRootProcShowUIHandler>,
    pub hide_ui: Option<&'static mut AccessoryShopChangeRootProcHideUIHandler>,
    pub watching: bool,
    pub visible_ui: bool,
}
impl Bindable for AccessoryShopChangeRootProc {}
impl AccessoryShopChangeRootProc {
    #[unity::class_method(7)] pub fn on_select_menu_item(&self, accessory_data: &AccessoryData); // Offset: 0x27C2230 Flags: 0
    #[unity::class_method(8)] pub fn on_decide_menu_item(&self, accessory_data: &AccessoryData); // Offset: 0x27C25E0 Flags: 0
    #[unity::class_method(9)] pub fn on_change_unit_to_prev(&self, watching: bool); // Offset: 0x27C2820 Flags: 0
    #[unity::class_method(10)] pub fn on_change_unit_to_next(&self, watching: bool); // Offset: 0x27C2D80 Flags: 0
    #[unity::class_method(11)] pub fn on_change_kind(&self, accessory_data: &AccessoryData); // Offset: 0x27C32E0 Flags: 0
    #[unity::class_method(12)] pub fn on_start_watching(&self) -> bool; // Offset: 0x27C3380 Flags: 0
    #[unity::class_method(13)] pub fn on_end_watching(&self); // Offset: 0x27C35A0 Flags: 0
    #[unity::class_method(14)] pub fn on_show_ui(&self); // Offset: 0x27C38E0 Flags: 0
    #[unity::class_method(15)] pub fn on_hide_ui(&self); // Offset: 0x27C3A50 Flags: 0
    #[unity::class_method(16)] pub fn on_request_close_menu(&self); // Offset: 0x27C3BC0 Flags: 0
}
impl_action_handlers!([AccessoryShopChangeRootProc, "HideUIEventHandler", AccessoryShopChangeRootProcHideUIHandler],);
impl_action_handlers!([AccessoryShopChangeRootProc, "ShowUIEventHandler", AccessoryShopChangeRootProcShowUIHandler],);
impl_action_handlers!([AccessoryShopChangeRootProc, "StartWatchingEventHandler", AccessoryShopChangeRootProcStartWatchingHandler],);
impl_action_handlers!([AccessoryShopChangeRootProc, "EndWatchingEventHandler", AccessoryShopChangeRootProcEndWatchingHandler],);
impl_action_handlers!([AccessoryShopChangeRootProc, "ChangeUnitToNextEventHandler", AccessoryShopChangeRootProcChangeUnitNextHandler],);
impl_action_handlers!([AccessoryShopChangeRootProc, "ChangeUnitToPrevEventHandler", AccessoryShopChangeRootProcChangeUnitPrevHandler],);




