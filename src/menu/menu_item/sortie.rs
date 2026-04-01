use unity::prelude::*;
use crate::menu::{BasicMenuItemAttribute, BasicMenuItemFields, BasicMenuResult};
use crate::unit::Unit;

#[unity::class("App", "SortieUnitSelect")]
pub struct SortieUnitSelect { }

#[unity::class("", "UnitMenuItem")]
#[nested_from_type(SortieUnitSelect)]
pub struct SortieUnitSelectUnitMenuItem {
    pub parent: BasicMenuItemFields,
    pub unit: Option<&'static mut Unit>, // Offset 0x68, Attr: 1
    pub can_sortie_for_relay: bool, // Offset 0x70, Attr: 1
}

impl SortieUnitSelectUnitMenuItem {
    #[unity::class_method(4)] pub fn build_attribute(&self) -> BasicMenuItemAttribute; // Offset: 0x1D7AE30 Flags: 0
    #[unity::class_method(5)] pub fn a_call(&self) -> BasicMenuResult; // Offset: 0x1D7B180 Flags: 0
    #[unity::class_method(6)] pub fn b_call(&self) -> BasicMenuResult; // Offset: 0x1D7BD90 Flags: 0
    #[unity::class_method(7)] pub fn x_call(&self) -> BasicMenuResult; // Offset: 0x1D7BEB0 Flags: 0
    #[unity::class_method(8)] pub fn plus_call(&self) -> BasicMenuResult; // Offset: 0x1D7BF40 Flags: 0
    #[unity::class_method(9)] pub fn l_call(&self) -> BasicMenuResult; // Offset: 0x1D7C070 Flags: 0
    #[unity::class_method(10)] pub fn r_call(&self) -> BasicMenuResult; // Offset: 0x1D7C2C0 Flags: 0
}
