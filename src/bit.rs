use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::SystemType;
#[repr(C)]
pub struct BitStruct{ pub bits: &'static mut Array<u8>, }

impl BitStruct {
    pub fn set(&mut self, index: i32, enable: bool) {
        let i = (index >> 3) as usize;
        if i < self.bits.len() {
            if enable { self.bits[i] |= 1 << (index & 7); }
            else { self.bits[i] &= !(1 << (index & 7)); }
        }
    }
    pub fn get(&self, index: i32) -> bool {
        let i = (index >> 3) as usize;
        if i < self.bits.len() { (self.bits[i] >> (index & 7)) & 1 != 0 }
        else { false }
    }
}

#[unity::class("App", "BitField32")]
pub struct BitField32 {
    pub value: i32, // Offset 0x10, Attr: 4
}
pub trait BitField32Methods: Il2CppClassData {
    #[unity::class_method(3, BitField32)] fn set(&self, f: i32); // Offset: 0x2987440 Flags: 0
    #[unity::class_method(5, BitField32)] fn clear(&self, f: i32); // Offset: 0x2987470 Flags: 0
    #[unity::class_method(7, BitField32)] fn change(&self, f: i32); // Offset: 0x29874A0 Flags: 0
    #[unity::class_method(9, BitField32)] fn copy(&self, f: i32); // Offset: 0x29874D0 Flags: 0
    #[unity::class_method(12, BitField32)] fn exclusive(&self, n: i32, m: i32) -> bool; // Offset: 0x2987510 Flags: 0
    #[unity::class_method(15, BitField32)] fn test(&self, f: i32) -> bool; // Offset: 0x2987550 Flags: 0
    #[unity::class_method(17, BitField32)] fn not(&self, f: i32) -> bool; // Offset: 0x2987580 Flags: 0
    #[unity::class_method(19, BitField32)] fn reset(&self); // Offset: 0x29875B0 Flags: 0
    #[unity::class_method(23, BitField32)] fn get_value_type(&self) -> &'static SystemType; // Offset: 0x2987620 Flags: 0
    #[unity::class_method(24, BitField32)] fn get_value(&self) -> i32; // Offset: 0x2987690 Flags: 0
    #[unity::class_method(25, BitField32)] fn set_value(&self, value: i32); // Offset: 0x29876A0 Flags: 0
}
#[unity::class("App", "BitField64")]
pub struct BitField64 {
    pub value: i32, // Offset 0x10, Attr: 4
}
pub trait BitField64Methods: Il2CppClassData {
    #[unity::class_method(3, BitField64)] fn set(&self, f: i32); // Offset: 0x2987440 Flags: 0
    #[unity::class_method(5, BitField64)] fn clear(&self, f: i32); // Offset: 0x2987470 Flags: 0
    #[unity::class_method(7, BitField64)] fn change(&self, f: i32); // Offset: 0x29874A0 Flags: 0
    #[unity::class_method(9, BitField64)] fn copy(&self, f: i32); // Offset: 0x29874D0 Flags: 0
    #[unity::class_method(12, BitField64)] fn exclusive(&self, n: i32, m: i32) -> bool; // Offset: 0x2987510 Flags: 0
    #[unity::class_method(15, BitField64)] fn test(&self, f: i32) -> bool; // Offset: 0x2987550 Flags: 0
    #[unity::class_method(17, BitField64)] fn not(&self, f: i32) -> bool; // Offset: 0x2987580 Flags: 0
    #[unity::class_method(19, BitField64)] fn reset(&self); // Offset: 0x29875B0 Flags: 0
    #[unity::class_method(23, BitField64)] fn get_value_type(&self) -> &'static SystemType; // Offset: 0x2987620 Flags: 0
    #[unity::class_method(24, BitField64)] fn get_value(&self) -> i32; // Offset: 0x2987690 Flags: 0
    #[unity::class_method(25, BitField64)] fn set_value(&self, value: i32); // Offset: 0x29876A0 Flags: 0
}