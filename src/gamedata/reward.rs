use crate::random::Random;
use super::*;

#[unity::class("App", "RewardData")]
pub struct RewardData {
    pub parent: StructBaseFields,
    pub iid: &'static Il2CppString, // Offset 0x28, Attr: 1
    pub ratio: f32, // Offset 0x30, Attr: 1
    pub factor: f32, // Offset 0x34, Attr: 1
    pub min: f32, // Offset 0x38, Attr: 1
    pub max: f32, // Offset 0x3C, Attr: 1
    pub is_show: bool, // Offset 0x40, Attr: 1
    pub condition: &'static Il2CppString, // Offset 0x48, Attr: 1
}

impl RewardData {
    #[unity::class_method(15)] pub fn get_item(&self) -> &'static ItemData; // Offset: 0x2018B80 Flags: 0
    #[unity::class_method(17)] pub fn is_condition(condition: &Il2CppString) -> bool; // Offset: 0x1FFB940 Flags: 0
    #[unity::class_method(18)] pub fn get_percent(&self, level: i32) -> f32; // Offset: 0x2018C20 Flags: 0
    #[unity::class_method(19)] pub fn get_exp_form_challenge(sortie_count: i32, difficulty_diff: i32) -> i32; // Offset: 0x2018C80 Flags: 0
    #[unity::class_method(21)] pub fn calc_rewards(name: &Il2CppString, random: &Random, level: i32, is_dump: bool) -> Option<&'static mut List<&'static mut ItemData>>; // Offset: 0x2019160 Flags: 0
    #[unity::class_method(22)] pub fn calc_rewards2(name: &Il2CppString) -> Option<&'static mut List<&'static mut ItemData>>; // Offset: 0x20193E0 Flags: 0
    #[unity::class_method(23)] pub fn calc_game_clear_rewards(difficulty: i32) -> &'static mut List<&'static ItemData>; // Offset: 0x2019640 Flags: 0
    #[unity::class_method(24)] pub fn calc_evil_clear_rewards() -> List<&'static ItemData>; // Offset: 0x20196D0 Flags: 0
    #[unity::class_method(25)] pub fn ctor(&self); // Offset: 0x2019720 Flags: 0
}