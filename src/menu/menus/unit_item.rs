use unity::engine::Color;
use unity::prelude::*;
use crate::impl_action_handlers;
use crate::menu::{BasicMenuFields, BasicMenuMethods};
use crate::menu::menu_item::{BasicMenuItemContent, MenuItem};
use crate::unit::Unit;

#[unity::class("App", "UnitItemMenu")]
pub struct UnitItemMenu {
    pub parent: BasicMenuFields<UnitItemMenuItem>,
    pub unit: Option<&'static Unit>,
    pub select_handler: Option<&'static mut UnitItemMenuSelectHandler>,
    pub decided_handler: Option<&'static mut UnitItemMenuDecideHandler>,
    pub request_close: Option<&'static mut UnitItemMenuRequestCloseHandler>,
}
impl BasicMenuMethods for UnitItemMenu {}
impl crate::proc::Bindable for UnitItemMenu {}
impl AsRef<crate::proc::inst::ProcInstFields> for UnitItemMenu {
    fn as_ref(&self) -> &crate::proc::inst::ProcInstFields { &self.parent.proc }
}
impl AsMut<crate::proc::inst::ProcInstFields> for UnitItemMenu {
    fn as_mut(&mut self) -> &mut crate::proc::inst::ProcInstFields { &mut self.parent.proc }
}

impl_action_handlers!([UnitItemMenu, "SelectEventHandler", UnitItemMenuSelectHandler],);
impl_action_handlers!([UnitItemMenu, "DecideEventHandler", UnitItemMenuDecideHandler],);
impl_action_handlers!([UnitItemMenu, "RequestCloseEventHandler", UnitItemMenuRequestCloseHandler],);

#[unity::class("App", "UnitItemMenuContent")]
pub struct UnitItemMenuContent {}

#[unity::class("App", "UnitItemMenuItem")]
pub struct UnitItemMenuItem {
    pub menu: &'static mut UnitItemMenu,
    pub menu_item_content: &'static mut BasicMenuItemContent,
    pub name: &'static Il2CppString,
    pub index: i32,
    pub full_index: i32,
    pub attribute: i32,
    pub cursor_color: Color,
    pub active_text_color: Color,
    pub inactive_text_color: Color,
    pub owner_item_index: i32,
    pub selectable_blank: bool,
    pub select_event_handler: Option<&'static UnitItemMenuSelectHandler>,
    pub decide_event_handler: Option<&'static UnitItemMenuDecideHandler>, // Offset 0x78, Attr: 4
    pub request_close_event_handler: Option<&'static UnitItemMenuRequestCloseHandler>, // Offset 0x80, Attr: 4
}
impl MenuItem for UnitItemMenuItem {}