use crate::battle::BattleCalculator;
use crate::util::get_singleton_proc_instance;
use super::*;

#[unity::class("Combat", "CombatSequence")]
pub struct CombatSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub is_load: bool,
    pub calculator: &'static BattleCalculator,
    pub sim_calculator: &'static BattleCalculator,
    camera: *const u8,
}
impl Bindable for CombatSequence {}

impl CombatSequence {
    pub const HASH: i32 = -1353170183;
    pub fn get_instance() -> Option<&'static mut Self> {
        get_singleton_proc_instance::<Self>()
    }
    #[unity::class_method(0)] pub fn get_calculator(&self) -> &'static BattleCalculator; // Offset: 0x292BF50 Flags: 0
    #[unity::class_method(1)] pub fn set_calculator(&self, value: &BattleCalculator); // Offset: 0x292BF60 Flags: 0
    #[unity::class_method(2)] pub fn get_sim_calculator(&self) -> &'static BattleCalculator; // Offset: 0x292BF70 Flags: 0
    #[unity::class_method(3)] pub fn set_sim_calculator(&self, value: &BattleCalculator); // Offset: 0x292BF80 Flags: 0
    #[unity::class_method(23)] pub fn to_pre_bgm(&self); // Offset: 0x292D970 Flags: 0
    #[unity::class_method(24)] pub fn to_main_bgm(&self); // Offset: 0x292DA60 Flags: 0
    #[unity::class_method(25)] pub fn return_bgm(&self); // Offset: 0x292DAE0 Flags: 0
    /*
    // #[unity::class_method(4)] pub fn create_bind(super: &ProcInst, calculator: &BattleCalculator, sim_calculator: &BattleCalculator, callback: &ProcVoidMethod); // Offset: 0x292BF90 Flags: 0
    // #[unity::class_method(5)] pub fn ctor(&self, calculator: &BattleCalculator, sim_calculator: &BattleCalculator); // Offset: 0x292D050 Flags: 0
    #[unity::class_method(6)] pub fn timer_start(&self); // Offset: 0x292D100 Flags: 0
    #[unity::class_method(7)] pub fn timer_stop(&self); // Offset: 0x292D110 Flags: 0
    #[unity::class_method(8)] pub fn create_border(&self); // Offset: 0x292D120 Flags: 0
    #[unity::class_method(9)] pub fn delete_border(&self); // Offset: 0x292D1D0 Flags: 0
    #[unity::class_method(10)] pub fn load_voice(&self); // Offset: 0x292D270 Flags: 0
    #[unity::class_method(11)] pub fn load_voice_impl(&self, battle_side_type: BattleSideType); // Offset: 0x292D2A0 Flags: 0
    #[unity::class_method(12)] pub fn unload_voice(&self); // Offset: 0x292D3A0 Flags: 0
    #[unity::class_method(13)] pub fn unload_voice_impl(&self, battle_side_type: BattleSideType); // Offset: 0x292D3D0 Flags: 0
    #[unity::class_method(14)] pub fn load_scene(&self); // Offset: 0x292D4D0 Flags: 0
    #[unity::class_method(15)] pub fn wait_loading(&self) -> &'static IEnumerator; // Offset: 0x292D500 Flags: 0
    #[unity::class_method(16)] pub fn unload_scene(&self); // Offset: 0x292D580 Flags: 0
    #[unity::class_method(17)] pub fn tick(&self); // Offset: 0x292D700 Flags: 0
    #[unity::class_method(18)] pub fn on_create(&self); // Offset: 0x292D770 Flags: 0
    #[unity::class_method(19)] pub fn on_dispose(&self); // Offset: 0x292D7F0 Flags: 0
    #[unity::class_method(20)] pub fn on_persistent(&self); // Offset: 0x292D860 Flags: 0
    // #[unity::class_method(21)] pub fn bound_to_combat_camera(&self) -> &'static IEnumerator; // Offset: 0x292D870 Flags: 0
    // #[unity::class_method(22)] pub fn return_to_map_camera(&self) -> &'static IEnumerator; // Offset: 0x292D8F0 Flags: 0

    #[unity::class_method(26)] pub fn is_comeback_with_transition_camera(&self) -> bool; // Offset: 0x292DBD0 Flags: 0
    #[unity::class_method(27)] pub fn die_talk(&self); // Offset: 0x292DCD0 Flags: 0
    #[unity::class_method(28)] pub fn show_growth(&self); // Offset: 0x292DD20 Flags: 0
    #[unity::class_method(29)] pub fn move_cursor(&self); // Offset: 0x292E000 Flags: 0
    #[unity::class_method(30)] pub fn back_to_ground(&self); // Offset: 0x292E080 Flags: 0
    #[unity::class_method(31)] pub fn wait_for_back_to_ground(&self); // Offset: 0x292E850 Flags: 0
    #[unity::class_method(32)] pub fn back_to_ground_fade_in(&self); // Offset: 0x292E950 Flags: 0
     */
}