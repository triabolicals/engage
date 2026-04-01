use super::*;
use crate::{
    proc::{ProcInstFields, Bindable},
    gamedata::{item::ItemData, job::JobData, WeaponMask},
    unit::Unit,
};
use crate::battle::BattleInfoSide;
use crate::menu::menus::class_change::ClassChangeJobData;

#[unity::class("App", "UnitGrowSequence")]
pub struct UnitGrowSequence {
    pub proc: ProcInstFields,
    pub camera_mode: i32,
    pub unit: &'static Unit,
    pub exp: i32,
    pub old_level: i32,
    pub is_talk: bool,
    pub skill_point: i32,
    pub class_change_job: Option<&'static JobData>,
    pub class_change_item: Option<&'static ItemData>,
    pub class_change_weapon_mask: Option<&'static WeaponMask>,
    pub class_change_weapon: Option<&'static ItemData>,
}
impl Bindable for UnitGrowSequence {}
impl AsMut<ProcInstFields> for UnitGrowSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}
impl UnitGrowSequence {
    pub const HASH: i32 = -813168385;
    #[unity::class_method(2)] pub fn set_unit_grow_data(&self, unit: &Unit, exp: i32, skill_point: i32, is_talk: bool);
    #[unity::class_method(0)] pub fn set_unit_grow_data2(&self, side: &BattleInfoSide, is_talk: bool); // Offset: 0x1F7E970 Flags: 0
    #[unity::class_method(1)] pub fn set_unit_grow_data3(&self, unit: &Unit, exp: i32); // Offset: 0x1F7EA90 Flags: 0
    #[unity::class_method(3)] pub fn set_unit_class_change(&self, unit: &Unit, job: &JobData, item: &ItemData); // Offset: 0x1F7EAF0 Flags: 0
    #[unity::class_method(4)] pub fn set_unit_class_change2(&self, unit: &Unit, data: &ClassChangeJobData); // Offset: 0x1F7EB60 Flags: 0
    #[unity::class_method(5)] pub fn create_bind<B>(proc: &B) -> &'static UnitGrowSequence where B: Bindable; // Offset: 0x1F7EBC0 Flags: 0
    #[unity::class_method(6)] pub fn prepare(&self); // Offset: 0x1F7F210 Flags: 0
    #[unity::class_method(7)] pub fn gain_exp(&self); // Offset: 0x1F7F360 Flags: 0
    #[unity::class_method(8)] pub fn check_level_up(&self); // Offset: 0x1F7F3C0 Flags: 0
    #[unity::class_method(9)] pub fn level_up(&self); // Offset: 0x1F7F450 Flags: 0
    #[unity::class_method(10)] pub fn check_class_change(&self); // Offset: 0x1F7F4F0 Flags: 0
    #[unity::class_method(11)] pub fn class_change(&self); // Offset: 0x1F7F580 Flags: 0
    #[unity::class_method(12)] pub fn set_weapon(&self); // Offset: 0x1F7FB70 Flags: 0
}
