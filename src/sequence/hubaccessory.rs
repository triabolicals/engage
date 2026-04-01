use super::*;
use unity::prelude::*;
use crate::menu::menus::accessory::AccessoryShopUnitSelectRoot;
use crate::menu::menus::accessory::change::AccessoryShopChangeRoot;
use crate::unit::Unit;

pub mod room;

#[unity::class("App", "HubAccessoryShopSequence")]
pub struct HubAccessoryShopSequence {
    pub proc: ProcInstFields,
    pub shop_menu_result: i32,
    pub shop_unit_select_menu_result: i32,
    pub unit_select_root: &'static mut AccessoryShopUnitSelectRoot,
    accessory_buy_root: *const u8,
    pub change_root: &'static mut AccessoryShopChangeRoot,
    pub unit: Option<&'static mut Unit>,
    pub select_menu_scroll_index: i32,
    pub changed: bool,
}
impl HubAccessoryShopSequence {
    #[unity::class_method(5)] pub fn start_sequence(&self); // Offset: 0x2D75860 Flags: 0
    #[unity::class_method(6)] pub fn create_accessory_shop_top_menu(&self); // Offset: 0x2D75940 Flags: 0
    #[unity::class_method(7)] pub fn create_shop_unit_select_menu(&self); // Offset: 0x2D75B90 Flags: 0
    #[unity::class_method(8)] pub fn destroy_shop_unit_select_menu(&self); // Offset: 0x2D75EE0 Flags: 0
    #[unity::class_method(9)] pub fn create_accessory_shop_buy_menu(&self); // Offset: 0x2D75F20 Flags: 0
    #[unity::class_method(10)] pub fn destroy_accessory_shop_buy_menu(&self); // Offset: 0x2D760F0 Flags: 0
    #[unity::class_method(11)] pub fn create_accessory_shop_change_menu(&self); // Offset: 0x2D76130 Flags: 0
    #[unity::class_method(12)] pub fn destroy_accessory_shop_change_menu(&self); // Offset: 0x2D76300 Flags: 0
}
impl Bindable for HubAccessoryShopSequence {}
impl AsRef<ProcInstFields> for HubAccessoryShopSequence {
    fn as_ref(&self) -> &ProcInstFields { &self.proc }
}
impl AsMut<ProcInstFields> for HubAccessoryShopSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields { &mut self.proc }
}