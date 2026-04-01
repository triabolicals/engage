use unity::prelude::*;
use crate::force::ForceType;
use crate::gamedata::ChapterData;
use crate::unit::Unit;
use crate::util::get_instance;
// Fog of War 

#[unity::class("App", "MapSight")]
pub struct MapSight {
   junk: [u8; 0x28],
   pub usable: bool,   // true = Fog of War
}

impl MapSight {
    pub fn get_instance() -> &'static mut MapSight { get_instance::<Self>() }
    #[unity::class_method(3)] pub fn setup(&self, chapter: &ChapterData); // Offset: 0x1F47190 Flags: 0
    #[unity::class_method(4)] pub fn update(&self, unit: &Unit); // Offset: 0x1F471A0 Flags: 0
    #[unity::class_method(5)] pub fn update2(&self, force_type: ForceType); // Offset: 0x1F2F650 Flags: 0
    #[unity::class_method(6)] pub fn update_all(&self); // Offset: 0x1F35220 Flags: 0
    #[unity::class_method(7)] pub fn update_projection(&self); // Offset: 0x1F47470 Flags: 0
    #[unity::class_method(8)] pub fn update_unit(&self); // Offset: 0x1F47270 Flags: 0
}