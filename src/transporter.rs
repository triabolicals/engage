use unity::prelude::*;
use unity::il2cpp::object::Array;
use crate::gamedata::item::ItemData;
use crate::unit::UnitItem;

#[unity::class("App", "Transporter")] pub struct Transporter {}

#[unity::class("", "Data")]
#[nested_from_type(Transporter)]
pub struct TransporterData {
    pub item: &'static mut UnitItem,
}
#[repr(C)]
pub struct TransporterStaticFields {
    pub data: &'static mut Array<&'static mut TransporterData>,
}

impl Transporter {
    #[unity::class_method(0)] pub fn initialize(); // Offset: 0x22A1060 Flags: 0
    #[unity::class_method(1)] pub fn reset(); // Offset: 0x22A1180 Flags: 0
    #[unity::class_method(2)] pub fn is_available() -> bool; // Offset: 0x22A12D0 Flags: 0
    #[unity::class_method(3)] pub fn get(index: i32) -> Option<&'static TransporterData>; // Offset: 0x22A13D0 Flags: 0
    #[unity::class_method(4)] pub fn get_index(unit_item: &UnitItem) -> i32; // Offset: 0x22A1450 Flags: 0
    #[unity::class_method(5)] pub fn can_add() -> bool; // Offset: 0x22A1570 Flags: 0
    #[unity::class_method(6)] pub fn try_get_lowest_item_index(lowest_index: i32, lowest_worth: u64) -> bool; // Offset: 0x22A1610 Flags: 0
    #[unity::class_method(7)] pub fn get_empty_index() -> i32; // Offset: 0x22A1990 Flags: 0
    #[unity::class_method(8)] pub fn discard(unit_item: &UnitItem); // Offset: 0x22A1A30 Flags: 0
    #[unity::class_method(9)] pub fn add_unit_item(unit_item: &UnitItem) -> i32; // Offset: 0x22A1B70 Flags: 0
    #[unity::class_method(10)] pub fn add_item(item_data: &ItemData) -> i32; // Offset: 0x22A2040 Flags: 0
    #[unity::class_method(11)] pub fn sub(index: i32); // Offset: 0x22A20C0 Flags: 0
    #[unity::class_method(12)] pub fn delete(index: i32); // Offset: 0x22A2260 Flags: 0
    #[unity::class_method(13)] pub fn clear(); // Offset: 0x22A2390 Flags: 0
    #[unity::class_method(14)] pub fn get_count() -> i32; // Offset: 0x22A24E0 Flags: 0
    #[unity::class_method(15)] pub fn delete_item(data: &ItemData); // Offset: 0x22A2570 Flags: 0
    #[unity::class_method(16)] pub fn get_item_count(data: &ItemData) -> i32; // Offset: 0x22A2630 Flags: 0
    // #[unity::class_method(17)] pub fn get_type_item_count(type: ItemDataUseTypes) -> i32; // Offset: 0x22A26D0 Flags: 0
    #[unity::class_method(20)] pub fn is_rare(unit_item: &UnitItem) -> bool; // Offset: 0x22A1840 Flags: 0
    #[unity::class_method(21)] pub fn calc_worth(unit_item: &UnitItem) -> u64; // Offset: 0x22A18B0 Flags: 0
}