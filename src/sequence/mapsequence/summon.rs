use crate::battle::{BattleCalculator, BattleInfo};
use crate::gamedata::PersonData;
use super::*;
#[unity::class("App", "MapSequenceEngageSummon")]
pub struct MapSequenceEngageSummon {
    pub proc: SingletonProcInstFields,
    pub info: &'static mut BattleInfo,
    pub sim_info: &'static mut BattleInfo,
    pub calculator: &'static mut BattleCalculator,
    pub sim_calculator: &'static mut BattleCalculator,
    reliance: u64,
    pub person_data: Option<&'static PersonData>,
    pub rank: i32,
}
impl Bindable for MapSequenceEngageSummon {}

impl MapSequenceEngageSummon {
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
    #[unity::class_method(0)] pub fn get_person(&self) -> &'static PersonData; // Offset: 0x23CF350 Flags: 0
    #[unity::class_method(1)] pub fn set_person(&self, value: &PersonData); // Offset: 0x23CF360 Flags: 0
    #[unity::class_method(2)] pub fn get_rank(&self) -> i32; // PersonDataRanks; // Offset: 0x23CF370 Flags: 0
    #[unity::class_method(3)] pub fn set_rank(&self, value: i32); // PersonDataRanks); // Offset: 0x23CF380 Flags: 0
    #[unity::class_method(5)] pub fn mind_start(&self); // Offset: 0x23CF3F0 Flags: 0
    #[unity::class_method(6)] pub fn mind_end(&self); // Offset: 0x23CF4A0 Flags: 0
    #[unity::class_method(7)] pub fn bgm_start(&self); // Offset: 0x23CF550 Flags: 0
    #[unity::class_method(8)] pub fn bgm_end(&self); // Offset: 0x23CF5B0 Flags: 0
    #[unity::class_method(9)] pub fn calculate(&self); // Offset: 0x23CF640 Flags: 0
    #[unity::class_method(10)] pub fn branch(&self); // Offset: 0x23CF850 Flags: 0
    #[unity::class_method(11)] pub fn simple_summon(&self); // Offset: 0x23CF940 Flags: 0
    #[unity::class_method(12)] pub fn combat_summon(&self); // Offset: 0x23CFA40 Flags: 0
    #[unity::class_method(13)] pub fn commit(&self); // Offset: 0x23CFA50 Flags: 0
    #[unity::class_method(14)] pub fn grow(&self); // Offset: 0x23CFBD0 Flags: 0
    #[unity::class_method(15)] pub fn god_exp(&self); // Offset: 0x23CFBE0 Flags: 0
    #[unity::class_method(16)] pub fn try_skip(&self); // Offset: 0x23CFC60 Flags: 0
    #[unity::class_method(17)] pub fn create_telop(&self); // Offset: 0x23CFD30 Flags: 0
}