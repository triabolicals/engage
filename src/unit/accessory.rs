use std::ops::{Deref, DerefMut};
use unity::prelude::*;
use crate::gamedata::accessory::AccessoryData;
use crate::stream::Stream;

#[unity::class("App", "UnitAccessoryList")]
pub struct UnitAccessoryList {
    pub unit_accessory_array: &'static mut Il2CppArray<&'static mut UnitAccessory>
}
impl Deref for UnitAccessoryListFields {
    type Target = [&'static mut UnitAccessory];
    fn deref(&self) -> &Self::Target { unsafe { std::slice::from_raw_parts(self.unit_accessory_array.m_items.as_ptr(), self.unit_accessory_array.max_length) } }
}
impl DerefMut for UnitAccessoryListFields {
    fn deref_mut(&mut self) -> &mut [&'static mut UnitAccessory] { unsafe { std::slice::from_raw_parts_mut(self.unit_accessory_array.m_items.as_mut_ptr(), self.unit_accessory_array.max_length) } }
}

#[unity::class("App", "UnitAccessory")] pub struct UnitAccessory { pub index: i32, }

impl UnitAccessory {
    pub fn serialize(&self, stream: &mut Stream) { unsafe { unitaccessory_serialize(self, stream, None) }; }
    pub fn deserialize(&mut self, stream: &Stream) { unsafe { unitaccessory_deserialize(self, stream, None) } }
}

impl UnitAccessoryList {
    pub fn get_count() -> i32 { Self::get_count_(None) }
    #[unity::class_method(0)] fn get_count_(this: Option<&UnitAccessoryList>) -> i32; // Offset: 0x1F61B10 Flags: 0
    #[unity::class_method(4)] pub fn clear(&self); // Offset: 0x1F61D80 Flags: 0
    #[unity::class_method(5)] pub fn is_exist(&self, accessory: &AccessoryData) -> bool; // Offset: 0x1F61E00 Flags: 0
}


#[unity::from_offset("App", "UnitAccessory", "Serialize")]
extern "C" fn unitaccessory_serialize(this: &UnitAccessory, stream: &mut Stream, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitAccessory", "Deserialize")]
extern "C" fn unitaccessory_deserialize(this: &mut UnitAccessory, stream: &Stream, method_info: OptionalMethod);