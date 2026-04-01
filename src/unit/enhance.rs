use unity::il2cpp::object::Array;
use unity::prelude::OptionalMethod;
use crate::unit::Unit;

#[unity::class("App", "UnitEnhanceValues")]
pub struct UnitEnhanceValues {
    pub values: &'static mut Array<i32>,
}

#[unity::class("App", "UnitEnhanceCalculator")]
pub struct UnitEnhanceCalculator {
    pub values: Option<&'static UnitEnhanceValues>,
    pub temp:   Option<&'static UnitEnhanceValues>,
}

#[unity::class("App", "UnitEnhanceFactors")]
pub struct UnitEnhanceFactors {
    pub hub_values:  Option<&'static UnitEnhanceValues>,
    pub food_values: Option<&'static UnitEnhanceValues>,
    pub item_values: Option<&'static UnitEnhanceValues>,
}

impl UnitEnhanceValues {
    pub fn get_item(&self, index: i32) -> i32 { unsafe { return unit_enhance_values_get_item(self, index, None); } }
}

impl UnitEnhanceFactors {
    pub fn get_food_values(&self) -> Option<&'static mut UnitEnhanceValues> { unsafe { return unit_enhance_factors_get_food_values(self, None); } }
}

#[skyline::from_offset(0x01a54f90)]
pub fn unit_get_enhance_factors(this: &Unit, method_info: OptionalMethod) -> Option<&'static mut UnitEnhanceFactors>;

#[skyline::from_offset(0x01f781b0)]
pub fn unit_enhance_values_get_item(this: &UnitEnhanceValues, index: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01f7aff0)]
pub fn unit_enhance_factors_get_food_values(this: &UnitEnhanceFactors, _method_info : OptionalMethod) -> Option<&'static mut UnitEnhanceValues>;