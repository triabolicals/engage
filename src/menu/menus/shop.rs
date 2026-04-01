use unity::prelude::*;
use crate::gamedata::accessory::AccessoryData;
use crate::gamedata::{GodData, ItemData};
use crate::unit::{Unit, UnitItem};

pub mod weapon_buy;
pub mod shopunitselect;
pub mod item_buy;

#[unity::class("App", "ShopCore")]
pub struct ShopCore { }

impl ShopCore {
    #[unity::class_method(0)] pub fn buy_on_weapon_shop(unit: Option<&Unit>, item_data: &ItemData) -> bool; // Offset: 0x21AE700 Flags: 0
    #[unity::class_method(1)] pub fn buy_on_item_shop(unit: Option<&Unit>, item_data: &ItemData) -> bool; // Offset: 0x21AEC90 Flags: 0
    #[unity::class_method(2)] pub fn buy_on_flea_market(unit: Option<&Unit>, item_data: &ItemData) -> bool; // Offset: 0x21AEE00 Flags: 0
    #[unity::class_method(3)] pub fn buy_on_accessory_shop(accessory_data: &AccessoryData); // Offset: 0x21AEEF0 Flags: 0
    #[unity::class_method(4)] pub fn sell(unit: Option<&Unit>, item_index: i32, closeup: bool); // Offset: 0x21AF020 Flags: 0
    #[unity::class_method(5)] pub fn refine(unit: Option<&Unit>, item_index: i32, refine_level: i32) -> &'static UnitItem; // Offset: 0x21AF310 Flags: 0
    #[unity::class_method(6)] pub fn get_needed_iron_to_refine(unit_item: &UnitItem, new_refine_level: i32) -> i32; // Offset: 0x21AF6E0 Flags: 0
    #[unity::class_method(7)] pub fn get_needed_steel_to_refine(unit_item: &UnitItem, new_refine_level: i32) -> i32; // Offset: 0x21AF800 Flags: 0
    #[unity::class_method(8)] pub fn get_needed_silver_to_refine(unit_item: &UnitItem, new_refine_level: i32) -> i32; // Offset: 0x21AF920 Flags: 0
    #[unity::class_method(9)] pub fn get_needed_money_to_refine(unit_item: &UnitItem, new_refine_level: i32) -> i32; // Offset: 0x21AFA40 Flags: 0
    #[unity::class_method(10)] pub fn evolve(unit: Option<&Unit>, item_index: i32, evolve_data_index: i32) -> &'static UnitItem; // Offset: 0x21AFD50 Flags: 0
    #[unity::class_method(11)] pub fn engrave(unit: Option<&Unit>, item_index: i32, god_data: &GodData) -> &'static UnitItem; // Offset: 0x21B0080 Flags: 0
    #[unity::class_method(12)] pub fn get_engrave_cost(item_data: &ItemData) -> i32; // Offset: 0x21B0310 Flags: 0
    // #[unity::class_method(13)] pub fn exchange(source_material_data: &ItemRefineExchangeData, target_material_data: &ItemRefineExchangeData, source_material_count: i32) -> i32; // Offset: 0x21B0390 Flags: 0
    #[unity::class_method(14)] pub fn get_unit_item_empty_count(unit: &Unit) -> i32; // Offset: 0x21B0A00 Flags: 0
    #[unity::class_method(15)] pub fn get_unit_item(unit: &Unit, item_index: i32) -> &'static UnitItem; // Offset: 0x21AF620 Flags: 0
    #[unity::class_method(16)] pub fn is_price_down() -> bool; // Offset: 0x21B0A30 Flags: 0
    #[unity::class_method(17)] pub fn get_price(item_data: &ItemData, is_discountable_shop: bool) -> i32; // Offset: 0x21AC000 Flags: 0
    #[unity::class_method(18)] pub fn can_add(unit: &Unit, item_data: &ItemData) -> bool; // Offset: 0x21B0B30 Flags: 0
    #[unity::class_method(19)] pub fn calc_add(unit: &Unit, item_data: &ItemData) -> ShopCoreResult; // Offset: 0x21B0B50 Flags: 0
    #[unity::class_method(20)] pub fn is_inventory_max(item_data: &ItemData) -> bool; // Offset: 0x21B0C40 Flags: 0
    #[unity::class_method(21)] pub fn add_item(unit: &Unit, item_data: &ItemData) -> ShopCoreResult; // Offset: 0x21AE870 Flags: 0
    // #[unity::class_method(22)] pub fn get_refine_material_count(material_data: &ItemRefineExchangeData) -> i32; // Offset: 0x21B0790 Flags: 0
    // #[unity::class_method(23)] pub fn get_refine_material_max(material_data: &ItemRefineExchangeData) -> i32; // Offset: 0x21B0920 Flags: 0
    #[unity::class_method(24)] pub fn add_achievement_on_buy(item_data: &ItemData, count: i32); // Offset: 0x21AEA30 Flags: 0
    #[unity::class_method(25)] pub fn add_achievement_on_sell(item_data: &ItemData, count: i32); // Offset: 0x21AF230 Flags: 0
    #[unity::class_method(26)] pub fn add_achievement_on_refine(item_data: &ItemData, count: i32); // Offset: 0x21AFB60 Flags: 0
    #[unity::class_method(27)] pub fn add_achievement_on_buy_accessory(); // Offset: 0x21B0CB0 Flags: 0
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ShopCoreResult {
    Failed = 0, // Attr: 17
    Unit = 1, // Attr: 17
    Transporter = 2, // Attr: 17
    Inventory = 3, // Attr: 17
}