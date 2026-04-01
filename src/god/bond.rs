use unity::prelude::*;
use unity::system::Dictionary;
use crate::gamedata::god::{GodData, GodGrowthDataLevelData, Pool};
use crate::unit::Unit;

#[unity::class("App", "GodBondHolder")]
pub struct GodBondHolder {
    parent: u128,
    pub data: Option<&'static GodData>,
    reliance_s: u64,
    pub bonds: Option<&'static mut Dictionary<'static, &'static Il2CppString, GodBond>>,
    pub pool : &'static Pool<GodBond>,
}
impl GodBondHolder {
    #[unity::class_method(0)] pub fn build(&self, data: &GodData) -> &'static GodBondHolder; // Offset: 0x2B4F1A0 Flags: 0
    #[unity::class_method(1)] pub fn get(&self, unit: &Unit) -> &'static GodBond; // Offset: 0x2B4F3E0 Flags: 0
    #[unity::class_method(2)] pub fn create(&self, unit: &Unit); // Offset: 0x2B4F580 Flags: 0
    #[unity::class_method(5)] pub fn get_max_level(&self) -> i32; // Offset: 0x2B4F950 Flags: 0
    #[unity::class_method(7)] pub fn new_god_bond(&self, pid: &Il2CppString) -> &'static GodBond; // Offset: 0x2B4F510 Flags: 0
    #[unity::class_method(8)] pub fn clear(&self); // Offset: 0x2B4F2C0 Flags: 0
    #[unity::class_method(16)] pub fn get_data(&self) -> Option<&'static GodData>; // Offset: 0x2B50DD0 Flags: 0
    #[unity::class_method(17)] pub fn is_valid(&self) -> bool; // Offset: 0x2B50DE0 Flags: 0
    #[unity::class_method(18)] pub fn ctor(&self); // Offset: 0x2B50DF0 Flags: 0
}
#[unity::class("App", "GodBond")]
pub struct GodBond {
    pub god: &'static GodData,
    reliance_s: u64,
    pub pid: &'static Il2CppString,
    pub level: u16,
    pub exp: u16,
    __: i32,
    inherit: *const u8,
    talk: u64,
    pub level_data: &'static GodGrowthDataLevelData,
}

impl GodBond {
    #[unity::class_method(13)] pub fn level_up(&self); // Offset: 0x2B4DFF0 Flags: 0
    #[unity::class_method(14)] pub fn set_level(&self, level: i32); // Offset: 0x2B4D760 Flags: 0
    #[unity::class_method(37)] pub fn gain_ability(&self, level: i32); // Offset: 0x2B4D210 Flags: 0
    #[unity::class_method(41)] pub fn clear(&self); // Offset: 0x2B4D170 Flags: 0
}