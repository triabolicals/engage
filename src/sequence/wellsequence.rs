use super::*;
use unity::system::List;
use crate::proc::desc::ProcDesc;
use crate::random::Random;
use crate::gamedata::item::ItemData;
use crate::unit::UnitItem;
use crate::proc::ProcInst;

#[unity::class("App", "WellSequence")]
pub struct WellSequence {
    proc: ProcInstFields,
}
impl Bindable for WellSequence {}

impl WellSequence {
    pub const HASH: i32 = -651818312;
    pub fn calc_item_exchange(level: i32, random: &Random) -> &'static mut List<ItemData> {
        Self::calc_item_exchange_(None, level, random)
    }
    #[unity::class_method(1)] pub fn calc_expected(unit_item_list: &List<&UnitItem>) -> i32; // Offset: 0x2938880 Flags: 0
    #[unity::class_method(3)] pub fn get_use_flag() -> WellSequenceUseFlags; // Offset: 0x2939950 Flags: 0
    #[unity::class_method(4)] pub fn set_use_flag(value: WellSequenceUseFlags); // Offset: 0x2939A80 Flags: 0
    #[unity::class_method(5)] pub fn get_exchange_level() -> i32; // Offset: 0x2939C90 Flags: 0
    #[unity::class_method(6)] pub fn set_exchange_level(value: i32); // Offset: 0x2939DC0 Flags: 0
    #[unity::class_method(7)] pub fn get_seed() -> i32; // Offset: 0x2939FD0 Flags: 0
    #[unity::class_method(8)] pub fn set_seed(value: i32); // Offset: 0x293A100 Flags: 0
    #[unity::class_method(9)] pub fn get_evil_weapon_event_state() -> i32; // Offset: 0x293A310 Flags: 0
    #[unity::class_method(10)] pub fn set_evil_weapon_event_state(value: i32); // Offset: 0x293A440 Flags: 0
    #[unity::class_method(11)] pub fn set_exchange(level: i32); // Offset: 0x29386B0 Flags: 0
    #[unity::class_method(12)] pub fn map_clear(); // Offset: 0x293A650 Flags: 0
    #[unity::class_method(13)] pub fn get_is_item_return() -> bool; // Offset: 0x293A700 Flags: 0
    #[unity::class_method(14)] pub fn get_can_item_in() -> bool; // Offset: 0x293A770 Flags: 0
    #[unity::class_method(15)] pub fn get_effect_name() -> &'static Il2CppString; // Offset: 0x293A7E0 Flags: 0
    #[unity::class_method(16)] pub fn try_create_item_effect(); // Offset: 0x293A910 Flags: 0
    #[unity::class_method(17)] pub fn try_fadeout_effect(); // Offset: 0x293AAB0 Flags: 0
    #[unity::class_method(18)] pub fn try_destroy_effect(); // Offset: 0x293AB60 Flags: 0
    #[unity::class_method(19)] pub fn create_bind<B>(parent: &B) where B: Bindable;  // Offset: 0x293AC80 Flags: 0
    #[unity::class_method(20)] pub fn load_prefab_async(&self); // Offset: 0x293C340 Flags: 0
    #[unity::class_method(21)] pub fn wait_load_prefab(&self); // Offset: 0x293C360 Flags: 0
    #[unity::class_method(22)] pub fn unload_prefab(&self); // Offset: 0x293C400 Flags: 0
    #[unity::class_method(23)] pub fn open_header(&self); // Offset: 0x293C420 Flags: 0
    #[unity::class_method(24)] pub fn exit(&self); // Offset: 0x293C560 Flags: 0
    #[unity::class_method(25)] pub fn create_top_menu(&self); // Offset: 0x293C570 Flags: 0
    #[unity::class_method(26)] fn calc_item_exchange_(proc_inst: Option<&ProcInst>, level: i32, random: &Random) -> &'static mut List<ItemData>; // Offset: 0x293C890 Flags: 0
    #[unity::class_method(27, WellSequence)] pub fn get_item<B>(proc: &B) where B: Bindable; // Offset: 0x293CB60 Flags: 0
    #[unity::class_method(29)] pub fn create_item_select_menu(&self); // Offset: 0x293CD40 Flags: 0
    #[unity::class_method(30)] pub fn check_item_select_result(&self); // Offset: 0x293CDB0 Flags: 0
    #[unity::class_method(31)] pub fn create_goto_evil_first_confirm_dialog(&self); // Offset: 0x293CF70 Flags: 0
    #[unity::class_method(32)] pub fn create_goto_evil_confirm_dialog(&self); // Offset: 0x293D1A0 Flags: 0
    #[unity::class_method(38)] pub fn branch(&self); // Offset: 0x293DAD0 Flags: 0
    #[unity::class_method(39)] pub fn get_evil_weapon1(&self); // Offset: 0x293DC40 Flags: 0
    #[unity::class_method(40)] pub fn get_evil_weapon2(&self); // Offset: 0x293DCF0 Flags: 0
    #[unity::class_method(43)] pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc>; // Offset: 0x293AD10 Flags: 0
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WellSequenceUseFlags {
    NotUse = 0, // Attr: 17
    Used = 1, // Attr: 17
    ItemReturn = 2, // Attr: 17
}