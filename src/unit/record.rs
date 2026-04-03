use num_derive::FromPrimitive;
use unity::il2cpp::object::Array;
use unity::prelude::*;
use crate::gamedata::chapter::ChapterData;

#[unity::class("App", "UnitRecord")]
pub struct UnitRecord {
    pub values: &'static mut Array<i32>,
}
impl UnitRecord {
    #[unity::class_method(5)] pub fn get(&self, kind: UnitRecordKinds) -> i32; // Offset: 0x1C56D60 Flags: 0
    #[unity::class_method(6)] pub fn set(&self, kind: UnitRecordKinds, value: i32); // Offset: 0x1C56DA0 Flags: 0
    #[unity::class_method(7)] pub fn add(&self, kind: UnitRecordKinds, value: i32); // Offset: 0x1C56DE0 Flags: 0
    #[unity::class_method(16)] pub fn get_dead_chapter(&self) -> Option<&'static ChapterData>; // Offset: 0x1C57F30 Flags: 0
    #[unity::class_method(18)] pub fn get_dead_flag(&self) -> UnitRecordDeadFlags; // Offset: 0x1C58030 Flags: 0
    /*
    #[unity::class_method(0)] pub fn initialize(); // Offset: 0x1C567F0 Flags: 0
    #[unity::class_method(1)] pub fn get_key(kind: UnitRecordKinds) -> i32; // Offset: 0x1C56B90 Flags: 0
    #[unity::class_method(2)] pub fn get_kind(key: i32) -> UnitRecordKinds; // Offset: 0x1C56C40 Flags: 0
    #[unity::class_method(3)] pub fn get_name(kind: UnitRecordKinds) -> &'static Il2CppString; // Offset: 0x1C56B00 Flags: 0


    #[unity::class_method(8)] pub fn reset_map_begin(&self, unit: &Unit); // Offset: 0x1C56E20 Flags: 0
    #[unity::class_method(9)] pub fn reset_map_end(&self, unit: &Unit); // Offset: 0x1C57180 Flags: 0
    #[unity::class_method(10)] pub fn clear(&self); // Offset: 0x1C571D0 Flags: 0
    #[unity::class_method(11)] pub fn copy(&self, src: &UnitRecord); // Offset: 0x1C57220 Flags: 0
    #[unity::class_method(14)] pub fn get_hero_reliance(unit: &Unit) -> i32; // Offset: 0x1C577F0 Flags: 0
    #[unity::class_method(15)] pub fn get_mvp_unit() -> &'static Unit; // Offset: 0x1C57980 Flags: 0

    #[unity::class_method(17)] pub fn set_dead_chapter(&self, chapter: &ChapterData); // Offset: 0x1C57FE0 Flags: 0

    #[unity::class_method(19)] pub fn set_dead_flag(&self, flags: i32); // Offset: 0x1C58060 Flags: 0
     */
}
#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq)]
pub enum UnitRecordDeadFlags {
    Encount = 1, // Attr: 17
    ExistDead = 2, // Attr: 17
}
#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum UnitRecordKinds {
    None = 0, // Attr: 17
    SortieCount = 1, // Attr: 17
    NoSortieCount = 2, // Attr: 17
    ContinuousSortieCount = 3, // Attr: 17
    ContinuousNoSortieCount = 4, // Attr: 17
    ChallengeSortieCount = 5, // Attr: 17
    MvpCount = 6, // Attr: 17
    BattleCount = 7, // Attr: 17
    KillCount = 8, // Attr: 17
    DeadChapter = 9, // Attr: 17
    DeadFlag = 10, // Attr: 17
    MapKillCount = 11, // Attr: 17
    MapCriticalCount = 12, // Attr: 17
    MapHealCount = 13, // Attr: 17
    MapBreakCount = 14, // Attr: 17
    MapDamage = 15, // Attr: 17
    MapReciveHealCount = 16, // Attr: 17
    MapReciveDamage = 17, // Attr: 17
    MapLevelUpCount = 18, // Attr: 17
    MapEngageCount = 19, // Attr: 17
    MapDeadCount = 20, // Attr: 17
    MapBattleExpGiveCount = 21, // Attr: 17
    MapDestroyExpCount = 22, // Attr: 17
    MapRodExpCount = 23, // Attr: 17
    MapInterferenceExpCount = 24, // Attr: 17
    MapDanceExpCount = 25, // Attr: 17
    MapReceiveAttackCount = 26, // Attr: 17
    MapLastTarget = 27, // Attr: 17
    MapCompletedHP = 28, // Attr: 17
    MapGuardCount = 29, // Attr: 17
    MapEfficacyAttackCount = 30, // Attr: 17
    MapPositionAttackCount = 31, // Attr: 17
    MapSmashAttackCount = 32, // Attr: 17
    MapSkillCount = 33, // Attr: 17
    MapReceiveSkillCount = 34, // Attr: 17
    MapDirectAttackCount = 35, // Attr: 17
    MapIndirectAttackCount = 36, // Attr: 17
    MapEngageAttackCount = 37, // Attr: 17
    MapChainAttackCount = 38, // Attr: 17
    MapChainGuardCount = 39, // Attr: 17
    MapUseItemCount = 40, // Attr: 17
    MapMoveDistance = 41, // Attr: 17
}
