use unity::engine::Color;
use unity::engine::ui::Image;
use unity::prelude::Il2CppString;
use unity::system::List;
use crate::{impl_action_handlers, impl_menu_class};
use crate::menu::*;
use crate::menu::content::BasicMenuContent;
use crate::menu::content::common::UnitMenuItemSetter;
use crate::menu::menu_item::{MenuItem, MenuItemContent};
use crate::proc::ProcInstFields;
use crate::tmpro::TextMeshProUGUI;
use crate::unit::Unit;
use crate::unityengine::GameObject;

#[unity::class("App", "ShopUnitSelectMenu")]
pub struct ShopUnitSelectMenu {
    pub proc: ProcInstFields,
    pub menu_content: &'static mut BasicMenuContent,
    pub menu_item_list: &'static mut List<ShopUnitSelectMenuItem>,
    pub full_menu_item_list: &'static mut List<ShopUnitSelectMenuItem>,
    pub status: &'static BasicMenuStatus,
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
    pub help_handler: Option<&'static mut ShopUnitSelectMenuHelpHandler>,
    pub game_object: &'static GameObject,
    pub show_god_ring_icon: bool,
}
impl_menu_class!(ShopUnitSelectMenu);

impl_action_handlers!([ShopUnitSelectMenu, "SelectEventHandler", ShopUnitSelectMenuSelectHandler], unit: &Unit);
impl_action_handlers!([ShopUnitSelectMenu, "DecideEventHandler", ShopUnitSelectMenuDecideHandler], result: i32, unit: &Unit, scroll_index: i32);
impl_action_handlers!([ShopUnitSelectMenu, "HelpEventHandler", ShopUnitSelectMenuHelpHandler],);

#[unity::class("App", "ShopUnitSelectMenuItem")]
pub struct ShopUnitSelectMenuItem {
    pub menu: &'static mut BasicMenu<ShopUnitSelectMenuItem>,
    pub menu_item_content: Option<&'static mut ShopUnitSelectMenuItemContent>,
    pub name: &'static Il2CppString,
    pub index: i32,
    pub full_index: i32,
    pub attribute: i32,
    pub cursor_color: Color,
    pub active_active: Color,
    pub inactive_active: Color,
    pub unit: Option<&'static Unit>,
    pub select_handler: Option<&'static mut ShopUnitSelectMenuSelectHandler>,
    pub decided_handler: Option<&'static mut ShopUnitSelectMenuSelectHandler>,
}
impl MenuItem for ShopUnitSelectMenuItem {}



#[unity::class("App", "ShopUnitSelectMenuItemContent")]
pub struct ShopUnitSelectMenuItemContent {
    pub parent: u64,
    pub menu_item: &'static mut ShopUnitSelectMenuItem,
    pub text_base_color: Color,
    pub text_blend_color: Color,
    pub frm_content: &'static GameObject,
    pub setter: &'static mut UnitMenuItemSetter,
}

impl MenuItemContent<ShopUnitSelectMenuItem> for ShopUnitSelectMenuItemContent {}