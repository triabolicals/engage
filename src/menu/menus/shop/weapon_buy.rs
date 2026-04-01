use unity::engine::{Color, MonoBehaviorFields};
use unity::engine::ui::Image;
use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::List;
use crate::dialog::exchangeyesno::ExchangeYesNoDialog;
use crate::gamedata::{ItemData, JobData};
use crate::impl_action_handlers;
use crate::menu::{BasicMenu, BasicMenuSelect};
use crate::menu::content::BasicMenuContent;
use crate::menu::content::common::ShopContent;
use crate::menu::menu_item::{BasicMenuItem, BasicMenuItemContentFields, MenuItem, MenuItemContent};
use crate::menu::menus::unit_item::{UnitItemMenu, UnitItemMenuContent};
use crate::proc::{ProcInst, ProcInstFields};
use crate::tmpro::TextMeshProUGUI;
use crate::unit::{Unit, UnitItem};
use crate::unityengine::{GameObject, UnityComponent};

#[unity::class("App", "WeaponShopBuyRoot")]
pub struct WeaponShopBuyRoot {
    parent: MonoBehaviorFields,
    root_canvas_group: u64, // Offset 0x18, Attr: 6
    pub menu_object: &'static GameObject, // Offset 0x20, Attr: 6
    pub unit_item_menu_content: &'static UnitItemMenuContent, // Offset 0x28, Attr: 6
    pub holding_info_window_object: &'static GameObject, // Offset 0x30, Attr: 6
    pub item_help_object: &'static GameObject, // Offset 0x38, Attr: 6
    pub unit_image_object: &'static GameObject, // Offset 0x40, Attr: 6
    pub price_down_object: &'static GameObject, // Offset 0x48, Attr: 6
    pub price_down_text: &'static TextMeshProUGUI, // Offset 0x50, Attr: 6
    pub equipable_weapons_caption_text: &'static TextMeshProUGUI, // Offset 0x58, Attr: 6
    pub equipable_weapon_info: &'static Array<&'static WeaponShopBuyRootEquipableWeaponInfo>, // Offset 0x60, Attr: 6
    pub unit: Option<&'static Unit>, // Offset 0x68, Attr: 1
    pub weapon_shop_buy_menu: &'static mut WeaponShopBuyMenu, // Offset 0x70, Attr: 1
    pub unit_item_menu: Option<&'static UnitItemMenu>, // Offset 0x78, Attr: 1
    weapon_holding_info_window: u64, // &WeaponHoldingInfoWindow, // Offset 0x80, Attr: 1
    item_menu_detail_setter: u64, //&ItemMenuDetailSetter, // Offset 0x88, Attr: 1
    return_event_handler: u64, //&WeaponShopBuyRootReturnEventHandler, // Offset 0x90, Attr: 1
    pub yes_no_dialog: Option<&'static ExchangeYesNoDialog>, // &ExchangeYesNoDialog, // Offset 0x98, Attr: 1
    pub send_item_menu: Option<&'static ProcInst>, // Offset 0xA0, Attr: 1
    pub discard_item_menu: Option<&'static ProcInst>, // Offset 0xA8, Attr: 1
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
impl UnityComponent for WeaponShopBuyRoot {}

#[unity::class("App", "WeaponShopBuyMenu")]
pub struct WeaponShopBuyMenu {
    pub proc: ProcInstFields,
    pub menu_content: &'static mut BasicMenuContent,
    pub menu_item_list: &'static mut List<WeaponShopBuyMenuItem>,
    pub full_menu_item_list: &'static mut List<WeaponShopBuyMenuItem>,
    status_field: *const u8,
    pub result: i32,
    scroll_preced_input_a: bool,
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
    pub request_close_event_handler: Option<&'static mut WeaponShopBuyRequestCloseHandler>, // Offset 0xE0, Attr: 1
    pub unit: Option<&'static Unit>, // Offset 0xE8, Attr: 1
    pub select_event_handler: Option<&'static mut WeaponShopBuySelectHandler>, // Offset 0xF0, Attr: 1
    pub decide_event_handler: Option<&'static mut WeaponShopBuyDecideHandler>, // Offset 0xF8, Attr: 1
    pub change_unit_to_prev_event_handler: Option<&'static mut WeaponShopBuyChangeUnitPrevHandler>, // Offset 0x100, Attr: 1
    pub change_unit_to_next_event_handler: Option<&'static mut WeaponShopBuyChangeUnitNextHandler>, // Offset 0x108, Attr: 1
    pub switch_detail_display_way_event_handler: Option<&'static mut WeaponShopBuySwitchDisplayHandler>, // Offset 0x110, Attr: 1
}
impl WeaponShopBuyMenu {
    #[unity::class_method(3)] pub fn rebuild_menu(&self, keep_select: bool, setup_shopdata: bool); // Offset: 0x21CBF60 Flags: 0
}
crate::impl_menu_class!(WeaponShopBuyMenu);
impl_action_handlers!([WeaponShopBuyMenu, "RequestCloseEventHandler", WeaponShopBuyRequestCloseHandler],);
impl_action_handlers!([WeaponShopBuyMenu, "SelectEventHandler", WeaponShopBuySelectHandler], unit_item: &UnitItem);
impl_action_handlers!([WeaponShopBuyMenu, "ChangeUnitToNextEventHandler", WeaponShopBuyChangeUnitNextHandler],unit_item: &UnitItem);
impl_action_handlers!([WeaponShopBuyMenu, "ChangeUnitToNextEventHandler", WeaponShopBuyChangeUnitPrevHandler],unit_item: &UnitItem);
impl_action_handlers!([WeaponShopBuyMenu, "DecideEventHandler", WeaponShopBuyDecideHandler], item: &ItemData);
impl_action_handlers!([WeaponShopBuyMenu, "SwitchDetailDisplaywayEventHandler", WeaponShopBuySwitchDisplayHandler], );

#[unity::class("", "EquipableWeaponInfo")]
#[nested_from_type(WeaponShopBuyRoot)]
pub struct WeaponShopBuyRootEquipableWeaponInfo {
    pub root: &'static GameObject, // Offset 0x10, Attr: 6
    pub icon_image: &'static mut Image, // Offset 0x18, Attr: 6
    pub level_text: &'static mut TextMeshProUGUI, // Offset 0x20, Attr: 6
}

impl WeaponShopBuyRootEquipableWeaponInfo {
    #[unity::class_method(0)] pub fn set(&self, item_kinds: i32, weapon_level: i32, job_data: &JobData); // Offset: 0x1B2BC70 Flags: 0
}



#[unity::class("App", "WeaponShopBuyMenuItem")]
pub struct WeaponShopBuyMenuItem {
    pub menu: &'static mut BasicMenu<WeaponShopBuyMenuItem>,
    pub menu_item_content: Option<&'static mut WeaponShopBuyMenuItemContent>,
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
    pub select_event: Option<&'static mut WeaponShopBuySelectHandler>,
    pub decide_event: Option<&'static mut WeaponShopBuyDecideHandler>,
    pub change_unit: Option<&'static mut WeaponShopBuyChangeUnitPrevHandler>,
    pub change_unit2: Option<&'static mut WeaponShopBuyChangeUnitNextHandler>,
}
impl WeaponShopBuyMenuItem {
    #[unity::class_method(8)]
    pub fn ctor(
        &self,
        shop_content: &ShopContent,
        unit: Option<&Unit>,
        select_event_handler: Option<&WeaponShopBuySelectHandler>,
        decide_event_handler: Option<&WeaponShopBuyDecideHandler>,
        change_unit_to_prev_event_handler: Option<&WeaponShopBuyChangeUnitPrevHandler>,
        change_unit_to_next_event_handler: Option<&WeaponShopBuyChangeUnitNextHandler>
    );
    #[unity::class_method(0)] pub fn get_item_data(&self) -> Option<&'static ItemData>; // Offset: 0x21CD680 Flags: 0
    #[unity::class_method(1)] pub fn set_item_data(&self, value: &ItemData); // Offset: 0x21CD690 Flags: 0
    #[unity::class_method(13)] pub fn set_initial_color(&self); // Offset: 0x21CD7D0 Flags: 0
}
impl MenuItem for WeaponShopBuyMenuItem {}

#[unity::class("App", "WeaponShopBuyMenuItemContent")]
pub struct WeaponShopBuyMenuItemContent {
    pub parent: BasicMenuItemContentFields,
    pub kind_frame_object: &'static GameObject, // Offset 0x48, Attr: 6
    pub kind_icon_object: &'static GameObject, // Offset 0x50, Attr: 6
    pub name_object: &'static GameObject, // Offset 0x58, Attr: 6
    pub stock_value_object: &'static GameObject, // Offset 0x60, Attr: 6
    pub price_value_object: &'static GameObject, // Offset 0x68, Attr: 6
    pub price_gobject: &'static GameObject, // Offset 0x70, Attr: 6
    pub new_icon_object: &'static GameObject, // Offset 0x78, Attr: 6
    pub text_base_color2: Color, // Offset 0x80, Attr: 1
}
impl MenuItemContent<WeaponShopBuyMenuItem> for WeaponShopBuyMenuItemContent {}
