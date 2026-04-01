use unity::prelude::*;
use unity::il2cpp::object::Array;
use crate::force::Force;
use crate::gamedata::person::PersonData;
use crate::gamedata::ring::RingData;
use crate::unit::ring::UnitRing;
use crate::unit::Unit;

#[unity::class("App", "UnitPool")]
#[static_fields(UnitPoolStaticFields)]
pub struct UnitPool {}

#[repr(C)]
pub struct UnitPoolStaticFields {
    pub s_unit: &'static Array<&'static mut Unit>,
    pub forces: &'static Array<&'static mut Force>,
}

#[unity::class("App", "UnitFor")]
pub struct UnitFor {}

impl UnitFor {
    #[unity::class_method(8)] pub fn get_prev(unit: &Unit) -> Option<&'static mut Unit>; // Offset: 0x1F7E3F0 Flags: 0
    #[unity::class_method(9)] pub fn get_next(unit: &Unit) -> Option<&'static mut Unit>; // Offset: 0x1F7E360 Flags: 0
    #[unity::class_method(10)] pub fn get_prev_by_force(unit: &Unit, force_mask: i32) -> Option<&'static mut Unit>; // Offset: 0x1F7E660 Flags: 0
    #[unity::class_method(11)] pub fn get_next_by_force(unit: &Unit, force_mask: i32) -> Option<&'static mut Unit>; // Offset: 0x1F7E500 Flags: 0
}


impl UnitPool {
    #[unity::class_method(2)] pub fn get_count(force_mask: i32) -> i32; // Offset: 0x1C53BA0 Flags: 0
    #[unity::class_method(3)] pub fn get(index: i32) -> Option<&'static mut Unit>; // Offset: 0x1C53F80 Flags: 0
    #[unity::class_method(5)] pub fn get_first(force_mask: u32, start_force_index: i32) -> Option<&'static mut Unit>; // Offset: 0x1C54090 Flags: 0
    #[unity::class_method(6)] pub fn get_last(force_mask: u32, start_force_index: i32) -> Option<&'static mut Unit>; // Offset: 0x1C541D0 Flags: 0
    #[unity::class_method(7)] pub fn get_hero(consider_relay: bool) -> Option<&'static mut Unit>; // Offset: 0x1C54280 Flags: 0
    #[unity::class_method(8)] pub fn get_from_person(person: &PersonData, consider_relay: bool) -> Option<&'static mut Unit>; // Offset: 0x1C548D0 Flags: 0
    #[unity::class_method(10)] pub fn get_from_pid(pid: &Il2CppString, consider_relay: bool) -> Option<&'static mut Unit>; // Offset: 0x1C54EF0 Flags: 0
    #[unity::class_method(11)] pub fn get_from_person_force_mask(person: &PersonData, force_mask: i32) -> Option<&'static mut Unit>; // Offset: 0x1C55030 Flags: 0
    #[unity::class_method(15)] pub fn get_force(index: i32) -> &'static Force; // Offset: 0x1C54150 Flags: 0
}

#[unity::class("App", "UnitRingPool")]
pub struct UnitRingPool{}

impl UnitRingPool {
    #[unity::class_method(4)] pub fn get_all_stock_count(data: &RingData) -> i32; // Offset: 0x1C5CF40 Flags: 0
    #[unity::class_method(8)] pub fn add(rnid: &Il2CppString, owner: Option<&Unit>, stock_count: i32) -> &'static UnitRing; // Offset: 0x1C5D420 Flags: 0
    #[unity::class_method(6)] pub fn can_add(rnid: &Il2CppString, stock_count: i32) -> bool; // Offset: 0x1C5D310 Flags: 0
    #[unity::class_method(10)] pub fn sub_ring(rnid: &Il2CppString, unit: &Unit, stock_count: i32); // Offset: 0x1C5D5B0 Flags: 0
    #[unity::class_method(18)] pub fn delete_ring(ring: &UnitRing); // Offset: 0x1C5D6C0 Flags: 0
}