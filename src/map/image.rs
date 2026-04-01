use unity::prelude::*;
use unity::il2cpp::object::Array;
use crate::unit::Unit;

#[unity::class("App", "MapPos")]
pub struct MapPos {
  pub x: i32,
  pub y: i32,
}
#[unity::class("App", "MapImage")]
pub struct MapImage {
  junk: [u8;0x10],
  name:  &'static Il2CppString,
  pub unit: &'static MapImageUnit,
  pub terrain: &'static MapImageTerrain,
  cost: &'static(),
  danger: &'static(),
  talk: &'static(),
  range: &'static(),
  history: &'static(),
  backup_terrains: &'static(),
  w: i32,
  h: i32,
  pub playarea_x: i32,
  pub playarea_z: i32,
  pub playarea_w: i32,
  pub playarea_h: i32,
  x1: i32,
  z1: i32,
  x2: i32,
  z2: i32,
  pub playarea_x1: i32,
  pub playarea_z1: i32,
  pub playarea_x2: i32,
  pub playarea_z2: i32,
}




impl MapImage {
  pub fn get_target_unit(&self, x: i32, y: i32) -> Option<&Unit> {
    unsafe { mapimage_get_target_unit(self, x, y, None) }
  }
}
#[unity::class("App", "MapImageUnit")]
pub struct MapImageUnit{
  parent: [u8; 8],
  pub cells: &'static Array<MapPosFields>,
}
impl MapImageUnit {
  pub fn get_unit(&self, x: i32, z: i32) -> Option<&Unit> {
    unsafe { mapimage_unit_get_unit(self, x, z, None) }
  }
}

#[unity::class("App", "MapImageTerrain")]
pub struct MapImageTerrain {
  m_original: &'static MapImageCoreByte,
  m_base: &'static MapImageCoreByte,
  pub m_result: &'static MapImageCoreByte,
  m_minimap_infos: &'static (),
  m_minimap_buffers: &'static (),
}

#[unity::class("App", "MapImageCoreByte")]
pub struct MapImageCoreByte { }

#[repr(C)]
#[unity::class("App", "MapImageCore`1")]
pub struct MapImageCore { }

#[unity::from_offset("App", "MapImage", "GetTargetUnit")]
extern "C" fn mapimage_get_target_unit(this: &MapImage, x: i32, y: i32, method_info: OptionalMethod) -> Option<&Unit>;

#[unity::from_offset("App", "MapImageUnit", "GetUnit")]
fn mapimage_unit_get_unit(this: &MapImageUnit, x: i32, z: i32, method_info: OptionalMethod) -> Option<&Unit>;
