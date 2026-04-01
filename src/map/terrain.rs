use unity::prelude::*;
use unity::il2cpp::object::Array;
use crate::util::try_get_instance;

#[unity::class("App", "MapTerrain")]
 pub struct MapTerrain {
     _super: u64,
     pub x: i32,
     pub z: i32,
     pub width: i32,
     pub height: i32,
     layers: u64,
     overlaps: u64,
     pub terrains: &'static Array<&'static Il2CppString>, 
 }
impl MapTerrain {
    pub fn get_tid(&self, x: i32, z: i32) -> Option<&'static Il2CppString> { unsafe { get_map_terrain_tid(self, x, z, None) }}
    pub fn get_instance() -> Option<&'static MapTerrain> { unsafe {  get_map_terrain(None) }}
    pub fn set_tid(&self, x: i32, z:i32, tid: &Il2CppString) { unsafe { set_map_terrain_tid(self, x, z, tid, None); }}
    pub fn update_image(&self) { unsafe { update_image_map_terrain(self, None) } }
}

#[unity::class("App", "MapTerrainInfo")]
pub struct MapTerrainInfo {}

impl MapTerrainInfo {
    pub fn get_instance() -> Option<&'static mut Self> { try_get_instance::<Self>() }
    #[unity::class_method(2)] pub fn show_all(&self); // Offset: 0x201C940 Flags: 0
    #[unity::class_method(3)] pub fn hide_all(&self); // Offset: 0x201C9A0 Flags: 0
    #[unity::class_method(4)] pub fn event_show_all(&self); // Offset: 0x201CA00 Flags: 0
    #[unity::class_method(5)] pub fn event_hide_all(&self); // Offset: 0x201CA60 Flags: 0
    #[unity::class_method(6)] pub fn is_show_any(&self) -> bool; // Offset: 0x201CAC0 Flags: 0
    #[unity::class_method(7)] pub fn get_current_unit(x: i32, z: i32) -> Option<&'static crate::unit::Unit>; // Offset: 0x201CB30 Flags: 0
    #[unity::class_method(10)] pub fn create_objects(&self); // Offset: 0x201CEC0 Flags: 0
    #[unity::class_method(11)] pub fn delete_objects(&self); // Offset: 0x201CE40 Flags: 0
    /*
    #[unity::class_method(12)] pub fn get_left(&self) -> &'static MapTerrainInfoMapTerrainInfoSingle; // Offset: 0x201D000 Flags: 0
    #[unity::class_method(13)] pub fn get_right(&self) -> &'static MapTerrainInfoMapTerrainInfoSingle; // Offset: 0x201D030 Flags: 0
    #[unity::class_method(14)] pub fn get_edit(&self) -> &'static MapTerrainInfoMapTerrainInfoSingle; // Offset: 0x201D060 Flags: 0
    #[unity::class_method(16)] pub fn create_edit(&self); // Offset: 0x201D080 Flags: 0
    #[unity::class_method(17)] pub fn delete_edit(&self); // Offset: 0x201D1A0 Flags: 0
     */
}

#[unity::from_offset("App", "MapTerrain", "GetTid")]
fn get_map_terrain_tid(this: &MapTerrain, x: i32, z: i32, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("App", "MapTerrain", "SetTid")]
fn set_map_terrain_tid(this: &MapTerrain, x: i32, z: i32, tid: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "MapSetting", "get_MapTerrain")]
fn get_map_terrain(method_info: OptionalMethod) -> Option<&'static MapTerrain>;

#[unity::from_offset("App", "MapTerrain", "UpdateMapImage")]
fn update_image_map_terrain(this: &MapTerrain, method_info: OptionalMethod);
