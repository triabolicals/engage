use std::ops::{Deref, DerefMut};
use num_derive::FromPrimitive;
use unity::il2cpp::object::Array;
use unity::prelude::*;

#[unity::class("App", "CapabilityDefinition")]
pub struct CapabilityDefinition {}
impl CapabilityDefinition {
    #[unity::class_method(0)] pub fn get_name(index: i32) -> &'static Il2CppString; // Offset: 0x25BCED0 Flags: 0
    #[unity::class_method(1)] pub fn get_help(index: i32) -> &'static Il2CppString; // Offset: 0x25BCFA0 Flags: 0
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum CapabilityDefinitionType {
    Hp = 0,
    Str = 1,
    Dex = 2,
    Speed = 3,
    Luck = 4,
    Def = 5, // Attr: 17
    Magic = 6, // Attr: 17
    Res = 7, // Attr: 17
    Build = 8, // Attr: 17
    Sight = 9, // Attr: 17
    Move = 10, // Attr: 17
    Num = 11, // Attr: 17
}

#[unity::class("App", "Capability")]
pub struct Capability {
    pub data: &'static mut Array<u8>,
}

#[unity::class("App", "CapabilitySbyte")]
pub struct CapabilitySbyte {
    pub data: &'static mut Array<i8>,
}

#[unity::class("App", "CapabilityShort")]
pub struct CapabilityShort {
    pub data: &'static mut Array<i16>,
}

#[unity::class("App", "CapabilityInt")]
pub struct CapabilityInt {
    pub data: &'static mut Array<i32>,
}
#[unity::class("App", "CapabilityFloat")]
pub struct CapabilityFloat {
    pub data: &'static mut Array<f32>,
}
#[unity::class("App", "UnitBaseCapability")]
pub struct UnitBaseCapability {
    pub data: &'static mut Array<i8>,
}

impl Deref for CapabilityFields {
    type Target = [u8];
    fn deref(&self) -> &Self::Target { unsafe { std::slice::from_raw_parts(self.data.m_items.as_ptr(), self.data.max_length) } }
}
impl DerefMut for CapabilityFields {
    fn deref_mut(&mut self) -> &mut [u8] { unsafe { std::slice::from_raw_parts_mut(self.data.m_items.as_mut_ptr(), self.data.max_length) } }
}


impl Deref for CapabilitySbyteFields {
    type Target = [i8];
    fn deref(&self) -> &Self::Target { unsafe { std::slice::from_raw_parts(self.data.m_items.as_ptr(), self.data.max_length) } }
}
impl DerefMut for CapabilitySbyteFields {
    fn deref_mut(&mut self) -> &mut [i8] { unsafe { std::slice::from_raw_parts_mut(self.data.m_items.as_mut_ptr(), self.data.max_length) } }
}

impl Deref for CapabilityShortFields {
    type Target = [i16];
    fn deref(&self) -> &Self::Target { unsafe { std::slice::from_raw_parts(self.data.m_items.as_ptr(), self.data.max_length) } }
}
impl DerefMut for CapabilityShortFields {
    fn deref_mut(&mut self) -> &mut [i16] { unsafe { std::slice::from_raw_parts_mut(self.data.m_items.as_mut_ptr(), self.data.max_length) } }
}

impl Deref for CapabilityIntFields {
    type Target = [i32];
    fn deref(&self) -> &Self::Target { unsafe { std::slice::from_raw_parts(self.data.m_items.as_ptr(), self.data.max_length) } }
}
impl DerefMut for CapabilityIntFields {
    fn deref_mut(&mut self) -> &mut [i32] { unsafe { std::slice::from_raw_parts_mut(self.data.m_items.as_mut_ptr(), self.data.max_length) } }
}

impl Deref for CapabilityFloatFields{
    type Target = [f32];
    fn deref(&self) -> &Self::Target { unsafe { std::slice::from_raw_parts(self.data.m_items.as_ptr(), self.data.max_length) } }
}
impl DerefMut for CapabilityFloatFields {
    fn deref_mut(&mut self) -> &mut [f32] { unsafe { std::slice::from_raw_parts_mut(self.data.m_items.as_mut_ptr(), self.data.max_length) } }
}

impl Deref for UnitBaseCapabilityFields{
    type Target = [i8];
    fn deref(&self) -> &Self::Target { unsafe { std::slice::from_raw_parts(self.data.m_items.as_ptr(), self.data.max_length) } }
}
impl DerefMut for UnitBaseCapabilityFields {
    fn deref_mut(&mut self) -> &mut [i8] { unsafe { std::slice::from_raw_parts_mut(self.data.m_items.as_mut_ptr(), self.data.max_length) } }
}

impl Capability {
    #[unity::class_method(1)] pub fn add(&self, i: i32, v: u8);
    #[unity::class_method(2)] pub fn is_zero(&self) -> bool;
}
impl CapabilityFloat {
    #[unity::class_method(1)] pub fn add(&self, i: i32, v: f32);
    #[unity::class_method(2)] pub fn is_zero(&self) -> bool;
}
impl CapabilityInt {
    #[unity::class_method(1)] pub fn add(&self, i: i32, v: i32);
    #[unity::class_method(2)] pub fn is_zero(&self) -> bool;
}
impl CapabilitySbyte {
    #[unity::class_method(1)] pub fn add(&self, i: i32, v: i8);
    #[unity::class_method(2)] pub fn is_zero(&self) -> bool;
}
impl CapabilityShort {
    #[unity::class_method(1)] pub fn add(&self, i: i32, v: i16);
    #[unity::class_method(2)] pub fn is_zero(&self) -> bool;
}