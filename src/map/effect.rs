use unity::prelude::*;
use crate::unit::{Unit, UnitModel};
use crate::unityengine::GameObject;

#[unity::class("App", "MapEffect")]
pub struct MapEffect {}

impl MapEffect {
    #[unity::class_method(0)] pub fn find_root() -> &'static GameObject; // Offset: 0x1DB8A00 Flags: 0
    #[unity::class_method(1)] pub fn try_create_root() -> Option<&'static GameObject>; // Offset: 0x1DB8A70 Flags: 0
    #[unity::class_method(2)] pub fn try_delete_root(); // Offset: 0x1DB8BC0 Flags: 0
    //#[unity::class_method(6)] pub fn play(name: &Il2CppString, position: Vector3) -> bool; // Offset: 0x1DB9600 Flags: 0
    #[unity::class_method(7)] pub fn play_at_position(name: &Il2CppString, x: i32, z: i32) -> bool; // Offset: 0x1DB98A0 Flags: 0
    // #[unity::class_method(8)] pub fn play3(data: &EffectData, x: i32, z: i32) -> bool; // Offset: 0x1DBA5F0 Flags: 0
    // #[unity::class_method(9)] pub fn play4(data: &EffectData, position: Vector3) -> bool; // Offset: 0x1DBB460 Flags: 0
    #[unity::class_method(10)] pub fn play_on_unit_model(name: &Il2CppString, model: &UnitModel) -> bool; // Offset: 0x1DBB530 Flags: 0
    #[unity::class_method(11)] pub fn play_on_unit(name: &Il2CppString, unit: &Unit) -> bool; // Offset: 0x1DBB6C0 Flags: 0
    // #[unity::class_method(12)] pub fn play7(name: &Il2CppString, position: Vector3, rotation: Quaternion) -> bool; // Offset: 0x1DB96D0 Flags: 0
    // #[unity::class_method(13)] pub fn can_playing() -> bool; // Offset: 0x1DBD070 Flags: 0
}