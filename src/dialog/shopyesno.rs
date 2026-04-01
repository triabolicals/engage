use crate::gamedata::item::ItemEvolveData;
use crate::gamedata::ItemData;
use crate::gamedata::skill::SkillData;
use crate::impl_action_handlers;
use crate::menu::BasicMenuMethods;
use crate::proc::Bindable;
use crate::unit::UnitItem;
use unity::prelude::*;
use crate::dialog::exchangeyesno::ExchangeYesNoDialog;
use crate::gamedata::accessory::AccessoryData;

#[unity::class("App", "ShopBuyYesNoDialog")]
pub struct ShopBuyYesNoDialog { }

impl ShopBuyYesNoDialog {
    #[unity::class_method(0)]
    pub fn create_bind_item<B>(
        proc: &B,
        item_data: &ItemData,
        num: i32,
        yes_event_handler: &ShopBuyYesNoDialogYesHandler
    ) -> &'static mut ExchangeYesNoDialog
    where B: Bindable; // Offset: 0x21AB9A0 Flags: 0
    
    #[unity::class_method(1)]
    pub fn create_bind_item2<B>(
        proc: &B,
        item_data: &ItemData,
        num: i32,
        enabled_price_down: bool,
        yes_event_handler: &ShopBuyYesNoDialogYesHandler
    ) -> &'static mut ExchangeYesNoDialog
    where B: Bindable; // Offset: 0x21ABCF0 Flags: 0
    
    #[unity::class_method(2)]
    pub fn create_bind_accessory<B>(
        proc: &B,
        accessory_data: &AccessoryData,
        yes_event_handler: &ShopBuyYesNoDialogYesHandler
    ) -> &'static mut ExchangeYesNoDialog
    where B: Bindable;
    // Offset: 0x21AC0A0 Flags: 0
    
    #[unity::class_method(3)]
    pub fn create_bind_refine_item<B>(
        proc: &B,
        base_unit_item: &UnitItem,
        refined_unit_item: &UnitItem,
        iron_num: i32,
        steel_num: i32,
        silver_num: i32,
        price: i32,
        yes_event_handler: &ShopBuyYesNoDialogYesHandler
    ) -> &'static mut ExchangeYesNoDialog; // Offset: 0x21AC600 Flags: 0
    
    #[unity::class_method(4)]
    pub fn create_bind_evolve_item<B>(
        proc: &B,
        base_unit_item: &UnitItem,
        evolved_unit_item: &UnitItem,
        item_evolve_data: &ItemEvolveData,
        yes_event_handler: &ShopBuyYesNoDialogYesHandler
    ) -> &'static mut ExchangeYesNoDialog; // Offset: 0x21AD100 Flags: 0
    #[unity::class_method(5)]
    pub fn create_bind_skill_inherit<B>(proc: &B, skill_data: &SkillData, cost: i32, yes_event_handler: &ShopBuyYesNoDialogYesHandler) -> &'static ExchangeYesNoDialog; // Offset: 0x21ADE70 Flags: 0
    // #[unity::class_method(6)]pub fn ctor(&self, menu_item_list: List<&BasicMenuItem>, menu_content: &BasicDialogContent); // Offset: 0x21AE1A0 Flags: 0
}
impl_action_handlers!([ShopBuyYesNoDialog, "YesEventHandler", ShopBuyYesNoDialogYesHandler],);


impl BasicMenuMethods for ShopBuyYesNoDialog {}