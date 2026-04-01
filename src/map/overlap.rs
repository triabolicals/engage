use unity::prelude::*;
use crate::force::ForceType;
use crate::gamedata::terrain::TerrainData;
use crate::unit::Unit;

/// Contains tiles created/placed over the map terrains
#[unity::class("App", "MapOverlap")]
pub struct MapOverlap {}

impl MapOverlap {
    #[unity::class_method(8)] pub fn get_terrain_by_index(i: i32) -> Option<&'static TerrainData>;
    #[unity::class_method(9)] pub fn get_terrain(x: i32, z: i32) -> Option<&'static TerrainData>;
    #[unity::class_method(17)] pub fn set(x: i32, z: i32, tid: &Il2CppString, turn: i32, phase: ForceType) -> bool;
    #[unity::class_method(18)] pub fn set_by_terrain(x: i32, z: i32, terrain: &TerrainData, turn: i32, phase: ForceType) -> bool; // Offset: 0x1DFDEE0 Flags: 0
    #[unity::class_method(26)] pub fn can_create(attacker: Option<&Unit>, x: i32, z: i32, data: &TerrainData) -> bool; // Offset: 0x1DFE300 Flags: 0
}