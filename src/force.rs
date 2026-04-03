use unity::prelude::*;
use crate::unit::Unit;

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum ForceType {
    Player = 0,
    Enemy = 1,
    Ally = 2,
    Absent = 3,
    Dead = 4,
    Lost = 5,
    Temporary = 6,
    Empty = 7,
}

#[unity::class("App", "Force")]
pub struct Force {
    pub head: Option<&'static Unit>,
    pub tail: Option<&'static Unit>,
    pub force_type: i32,
    // ...
}

impl Force {
    pub fn iter(&self) -> ForceIterator {
        ForceIterator(self.head)
    }
    #[unity::class_method(0)] pub fn get(ty: ForceType) -> Option<&'static mut Force>; // Offset: 0x2616200 Flags: 0
    #[unity::class_method(9)] pub fn transfer(&self, ty: ForceType, is_last: bool); // Offset: 0x26166A0 Flags: 0
    #[unity::class_method(12)] pub fn get_count(&self) -> i32; // Offset: 0x26167F0 Flags: 0
    #[unity::class_method(14)] pub fn get_hero_unit(&self) -> &'static mut Unit; // Offset: 0x2616860 Flags: 0
}

pub struct ForceIterator(Option<&'static Unit>);

impl Iterator for ForceIterator {
    type Item = &'static Unit;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Some(unit) => {
                let res = Some(unit);
                self.0 = unit.next;
                res
            },
            None => None,
        }
    }
}