use unity::prelude::*;

#[unity::class("App", "Random")]
pub struct Random {
    pub seed1: u32,
    pub seed2: u32,
    pub seed3: u32,
    pub seed4: u32,
}

impl Random {
    pub fn new(seed: u32) -> &'static mut Self {
        let random = Random::instantiate().unwrap();
        random.ctor(seed);
        random
    }

    // Game's default rngs
    #[unity::class_method(4)] pub fn get_system() -> &'static Random; // Offset: 0x23746E0 Flags: 0
    #[unity::class_method(5)] pub fn get_game() -> &'static Random; // Offset: 0x2374C70 Flags: 0
    #[unity::class_method(6)] pub fn get_spot() -> &'static Random; // Offset: 0x2374CE0 Flags: 0
    #[unity::class_method(7)] pub fn get_hub() -> &'static Random; // Offset: 0x2374D50 Flags: 0
    #[unity::class_method(8)] pub fn get_hub_item() -> &'static Random; // Offset: 0x2374DC0 Flags: 0
    #[unity::class_method(9)] pub fn get_kill_bonus() -> &'static Random; // Offset: 0x2374E30 Flags: 0
    #[unity::class_method(10)] pub fn get_combat() -> &'static Random; // Offset: 0x2374EA0 Flags: 0
    #[unity::class_method(12)] pub fn ctor(&self, seed: u32); // Offset: 0x2374F10 Flags: 0
    #[unity::class_method(14)] pub fn initialize(&self, v: u32); // Offset: 0x2374900 Flags: 0

    #[unity::class_method(17)] pub fn value(&self) -> i32; // Offset: 0x23748D0 Flags: 0
    #[unity::class_method(19)] pub fn get_value(&self, num: i32) -> i32; // Offset: 0x2375170 Flags: 0
    #[unity::class_method(20)] pub fn get_min_max(&self, min: i32, max: i32) -> i32; // Offset: 0x23751B0 Flags: 0
    #[unity::class_method(25)] pub fn probability_100(&self, percent: f32) -> bool; // Offset: 0x23754B0 Flags: 0
}