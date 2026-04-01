use unity::engine::{Color, MonoBehaviorFields};
use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::List;
use crate::dialog::exchangeyesno::ExchangeYesNoDialog;
use crate::gamedata::ItemData;
use crate::impl_action_handlers;
use crate::menu::{BasicMenu, BasicMenuContent, BasicMenuMethods, BasicMenuSelect, BasicMenuStatus};
use crate::menu::content::common::ShopContent;
use crate::menu::menu_item::{BasicMenuItemContent, BasicMenuItemContentFields, MenuItem, MenuItemContent};
use crate::menu::menus::unit_item::{UnitItemMenu, UnitItemMenuContent};
use crate::proc::{Bindable, ProcInst, ProcInstFields};
use crate::tmpro::TextMeshProUGUI;
use crate::unit::{Unit, UnitItem};
use crate::unityengine::{GameObject, UnityComponent};

#[unity::class("App", "ItemShopBuyRoot")]
pub struct ItemShopBuyRoot {
    parent: MonoBehaviorFields,
    pub menu_object: &'static GameObject, // Offset 0x18, Attr: 6
    pub unit_item_menu_content: &'static UnitItemMenuContent, // Offset 0x20, Attr: 6
    pub holding_info_window_object: &'static GameObject, // Offset 0x28, Attr: 6
    pub detail_info_window_object: &'static GameObject, // Offset 0x30, Attr: 6
    pub unit_image_object: &'static GameObject, // Offset 0x38, Attr: 6
    pub price_down_object: &'static GameObject, // Offset 0x40, Attr: 6
    pub price_down_text: &'static TextMeshProUGUI, // Offset 0x48, Attr: 6
    pub unit: Option<&'static Unit>,
    pub item_shop_buy_menu: &'static mut ItemShopBuyMenu, // Offset 0x58, Attr: 1
    pub unit_item_menu: Option<&'static mut UnitItemMenu>, // Offset 0x60, Attr: 1
    pub item_holding_info_window: u64, //&ItemHoldingInfoWindow, // Offset 0x68, Attr: 1
    pub item_menu_detail_setter: u64, //&ItemMenuDetailSetter, // Offset 0x70, Attr: 1
    pub return_event_handler: u64, //&ItemShopBuyRootReturnEventHandler, // Offset 0x78, Attr: 1
    pub yes_no_dialog: Option<&'static mut ExchangeYesNoDialog>, //&ExchangeYesNoDialog, // Offset 0x80, Attr: 1
    pub send_item_menu: Option<&'static ProcInst>, // Offset 0x88, Attr: 1
    pub discard_item_menu: Option<&'static ProcInst>, // Offset 0x90, Attr: 1
    pub unit_item: Option<&'static UnitItem>, // Offset 0xB0, Attr: 1
    pub item_data: Option<&'static ItemData>, // Offset 0xB8, Attr: 1
    pub is_sending_item_to_transporter: bool, // Offset 0xC0, Attr: 1
    pub is_sending_unit_item: bool, // Offset 0xC1, Attr: 1
    pub sending_unit_item_index: i32, // Offset 0xC4, Attr: 1
    pub is_discarding_transporter_item: bool, // Offset 0xC8, Attr: 1
    pub discarding_transporter_item_index: i32, // Offset 0xCC, Attr: 1
    pub existing_owner: bool, // Offset 0xD0, Attr: 1
    pub item_detail_display_with_unit: bool, // Offset 0xD1, Attr: 1
}
impl ItemShopBuyRoot {
    #[unity::class_method(0)] pub fn load_prefab_async(); // Offset: 0x204E9C0
    #[unity::class_method(1)] pub fn is_loading_prefab() -> bool; // Offset: 0x204EA60 Flags: 0
    #[unity::class_method(2)] pub fn unload_prefab(); // Offset: 0x204EAE0 Flags: 0
    #[unity::class_method(5)] pub fn destroy(root: &ItemShopBuyRoot); // Offset: 0x204F340 Flags: 0
    #[unity::class_method(9)] pub fn update_item_detail(&self); // Offset: 0x204F530 Flags: 0
    #[unity::class_method(10)] pub fn on_change_unit_to_prev(&self, unit_item: &UnitItem); // Offset: 0x204F660 Flags: 0
    #[unity::class_method(11)] pub fn on_change_unit_to_next(&self, unit_item: &UnitItem); // Offset: 0x204F820 Flags: 0
    #[unity::class_method(12)] pub fn on_switch_detail_display_way(&self); // Offset: 0x204F9F0 Flags: 0
    #[unity::class_method(13)] pub fn on_select_menu_item(&self, unit_item: &UnitItem); // Offset: 0x204FA00 Flags: 0
    #[unity::class_method(14)] pub fn on_decide_menu_item(&self, item_data: &ItemData) -> bool; // Offset: 0x204FB50 Flags: 0
    #[unity::class_method(15)] pub fn on_yes_to_buy(&self); // Offset: 0x204FCB0 Flags: 0
}
impl UnityComponent for ItemShopBuyRoot {}

// Namespace: App, Token: 0x2000D5A
#[unity::class("App", "ItemShopBuyMenu")]
pub struct ItemShopBuyMenu {
    pub proc: ProcInstFields,
    pub menu_content: &'static mut BasicMenuContent,
    pub menu_item_list: &'static mut List<ItemShopBuyMenuItem>,
    pub full_menu_item_list: &'static mut List<ItemShopBuyMenuItem>,
    status_field: &'static mut BasicMenuStatus,
    pub result: i32,
    scroll_precede_input_a: bool,
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
    pub shop_content_array: &'static Array<&'static mut ShopContent>, // Offset 0xC8, Attr: 1
    pub item_kind: i32, // ItemDataKinds, // Offset 0xD0, Attr: 1
    pub selects: &'static Array<&'static mut BasicMenuSelect>, // Offset 0xD8, Attr: 1
    pub request_close_event_handler: Option<&'static mut ItemShopBuyRequestCloseHandler>, // Offset 0xE0, Attr: 1
    pub unit: Option<&'static Unit>, // Offset 0xE8, Attr: 1
    pub select_event_handler: Option<&'static mut ItemShopBuySelectHandler>, // Offset 0xF0, Attr: 1
    pub decide_event_handler: Option<&'static mut ItemShopBuyDecideHandler>, // Offset 0xF8, Attr: 1
    pub change_unit_to_prev_event_handler: Option<&'static mut ItemShopBuyChangeUnitPrevHandler>, // Offset 0x100, Attr: 1
    pub change_unit_to_next_event_handler: Option<&'static mut ItemShopBuyChangeUnitNextHandler>, // Offset 0x108, Attr: 1
}
impl Bindable for ItemShopBuyMenu {}
impl BasicMenuMethods for ItemShopBuyMenu {}

impl ItemShopBuyMenu {
    #[unity::class_method(6)] pub fn rebuild_menu(&self, keep_select: bool, setup_shopdata: bool); // Offset: 0x204AF30 Flags: 0
}

impl_action_handlers!([ItemShopBuyMenu, "DecideEventHandler", ItemShopBuyDecideHandler], item: &ItemData);
impl_action_handlers!([ItemShopBuyMenu, "SelectEventHandler", ItemShopBuySelectHandler], unit_item: &UnitItem);
impl_action_handlers!([ItemShopBuyMenu, "RequestCloseEventHandler", ItemShopBuyRequestCloseHandler],);
impl_action_handlers!([ItemShopBuyMenu, "ChangeUnitToNextEventHandler", ItemShopBuyChangeUnitNextHandler],unit_item: &UnitItem);
impl_action_handlers!([ItemShopBuyMenu, "ChangeUnitToNextEventHandler", ItemShopBuyChangeUnitPrevHandler],unit_item: &UnitItem);


#[unity::class("App", "ItemShopBuyMenuItem")]
pub struct ItemShopBuyMenuItem {
    pub menu: &'static mut BasicMenu<ItemShopBuyMenuItem>,
    pub menu_item_content: Option<&'static mut BasicMenuItemContent>,
    pub name: &'static Il2CppString,
    pub index: i32,
    pub full_index: i32,
    pub attribute: i32,
    pub cursor_color: Color,
    pub active_text_color: Color,
    pub inactive_text_color: Color,
    pub padding: i32,
    pub iid: &'static mut Il2CppString,
    pub item_data: Option<&'static ItemData>,
    pub unit_item: &'static mut UnitItem,
    pub stock: i32,
    pub new_arrival: bool,
    pub unit: Option<&'static Unit>,
    pub active_text_color2: Color,
    pub inactive_text_color2: Color,
    pub select_event: Option<&'static mut ItemShopBuySelectHandler>,
    pub decide_event: Option<&'static mut ItemShopBuyDecideHandler>,
    pub change_unit_prev: Option<&'static mut ItemShopBuyChangeUnitPrevHandler>,
    pub change_unit_next: Option<&'static mut ItemShopBuyChangeUnitNextHandler>,
    pub for_flea_market: bool, // Offset 0xD0, Attr: 1
}
impl MenuItem for ItemShopBuyMenuItem {}
impl ItemShopBuyMenuItem {
    #[unity::class_method(8)]
    pub fn ctor(&self,
        shop_content: &ShopContent,
        unit: Option<&Unit>,
        select: Option<&ItemShopBuySelectHandler>,
        decide_event_handler: Option<&ItemShopBuyDecideHandler>,
        change_prev: Option<&ItemShopBuyChangeUnitPrevHandler>,
        change_next: Option<&ItemShopBuyChangeUnitNextHandler>,
        flea_market: bool,
    );
    #[unity::class_method(13)] pub fn set_initial_color(&self); // Offset: 0x204BB50 Flags: 0

}
#[unity::class("App", "ItemShopBuyMenuItemContent")]
pub struct ItemShopBuyMenuItemContent {
    pub parent: BasicMenuItemContentFields,
    pub kind_frame_object: &'static GameObject, // Offset 0x48, Attr: 6
    pub kind_icon_object: &'static GameObject, // Offset 0x50, Attr: 6
    pub name_object: &'static GameObject, // Offset 0x58, Attr: 6
    pub count_value_object: &'static GameObject, // Offset 0x60, Attr: 6
    pub stock_value_object: &'static GameObject, // Offset 0x68, Attr: 6
    pub price_value_object: &'static GameObject, // Offset 0x70, Attr: 6
    pub price_gobject: &'static GameObject, // Offset 0x78, Attr: 6
    pub new_icon_object: &'static GameObject, // Offset 0x80, Attr: 6
    pub text_base_color2: Color, // Offset 0x88, Attr: 1
}
impl MenuItemContent<ItemShopBuyMenuItem> for ItemShopBuyMenuItemContent {}