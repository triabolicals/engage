use unity::prelude::{Il2CppString, Il2CppClassData};
use crate::combat::CharacterAppearance;
use crate::gamedata::accessory::AccessoryData;
use crate::gamedata::animal::AnimalData;
use crate::gamedata::ItemData;
use crate::unit::Unit;

#[unity::class("App", "HubUtil")]
pub struct HubUtil { }

impl HubUtil {
    #[unity::class_method(19)] pub fn get_character_appearance(pid: &Il2CppString, accessory: &Il2CppString) -> &'static CharacterAppearance; // Offset: 0x2A5ED90 Flags: 0
    #[unity::class_method(20)] pub fn get_player_appearance(pid: &Il2CppString) -> &'static CharacterAppearance; // Offset: 0x2A5F290 Flags: 0
    #[unity::class_method(22)] pub fn is_main() -> bool; // Offset: 0x2A5F410 Flags: 0
    #[unity::class_method(23)] pub fn is_hub_sequence() -> bool; // Offset: 0x2A5F4D0 Flags: 0
    #[unity::class_method(24)] pub fn is_player_female() -> bool; // Offset: 0x2A5AD80 Flags: 0
    #[unity::class_method(25)] pub fn is_complete(cid: &Il2CppString) -> bool; // Offset: 0x2A5F590 Flags: 0
    #[unity::class_method(26)] pub fn pid_to_gid(pid: &Il2CppString) -> &'static Il2CppString; // Offset: 0x2A5F710 Flags: 0
    #[unity::class_method(27)] pub fn gid_to_pid(gid: &Il2CppString) -> &'static Il2CppString; // Offset: 0x2A5F780 Flags: 0
    #[unity::class_method(28)] pub fn try_get_sortie_unit(pid: &Il2CppString) -> Option<&'static Unit>; // Offset: 0x2A5F7F0 Flags: 0
    #[unity::class_method(29)] pub fn is_sortie_unit(pid: &Il2CppString) -> bool; // Offset: 0x2A5F850 Flags: 0
    #[unity::class_method(30)]pub fn is_best_reliance(pid: &Il2CppString) -> bool; // Offset: 0x2A5D260 Flags: 0
    #[unity::class_method(45)] pub fn continuous_sortie_count(pid: &Il2CppString) -> i32; // Offset: 0x2A61750 Flags: 0
    #[unity::class_method(46)] pub fn is_exists_mid(label: &Il2CppString) -> bool; // Offset: 0x2A61790 Flags: 0
    #[unity::class_method(47)] pub fn has_story_talk(pid: &Il2CppString) -> bool; // Offset: 0x2A61870 Flags: 0
    #[unity::class_method(48)] pub fn has_swimsuit(pid: &Il2CppString) -> bool; // Offset: 0x2A61B80 Flags: 0
    #[unity::class_method(49)] pub fn get_swimsuit() -> Option<&'static AccessoryData>; // Offset: 0x2A61C60 Flags: 0
    #[unity::class_method(50)] pub fn get_animal(locator: &Il2CppString) -> Option<&'static AnimalData>; // Offset: 0x2A61E40 Flags: 0
    #[unity::class_method(51)] pub fn is_capture_animal(animal: &AnimalData) -> bool; // Offset: 0x2A622C0 Flags: 0
    #[unity::class_method(52)] pub fn get_animal_capture_num(animal: &AnimalData) -> i32; // Offset: 0x2A62370 Flags: 0
    #[unity::class_method(53)] pub fn set_animal_capture_num(animal: &AnimalData, num: i32); // Offset: 0x2A62460 Flags: 0
    #[unity::class_method(54)] pub fn inc_animal_capture_num(animal: &AnimalData, num: i32); // Offset: 0x2A62640 Flags: 0
    #[unity::class_method(55)] pub fn get_encount_material_base() -> i32; // Offset: 0x2A626C0 Flags: 0
    #[unity::class_method(56)] pub fn get_encount_material_item_count() -> i32; // Offset: 0x2A62800 Flags: 0
    #[unity::class_method(57)] pub fn get_item_count_with_bonus(item: &ItemData, base_count: i32) -> i32; // Offset: 0x2A629C0 Flags: 0
    #[unity::class_method(58)] pub fn update_locate(pid: &Il2CppString); // Offset: 0x2A62B80 Flags: 0
    #[unity::class_method(107)]
    pub fn get_current_cooking_pid() -> Option<&'static Il2CppString>; // Offset: 0x2A64450 Flags: 0
    #[unity::class_method(108)]
    pub fn set_current_cooking_pid(value: &Il2CppString); // Offset: 0x2A644C0 Flags: 0

    #[unity::class_method(133)] pub fn get_dragon_ride_play_rank_num() -> i32; // Offset: 0x2A64F60 Flags: 0
    #[unity::class_method(134)] pub fn set_dragon_ride_play_rank_num(value: i32); // Offset: 0x2A64FD0 Flags: 0
}