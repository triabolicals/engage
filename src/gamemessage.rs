//! Utilities to display popup dialogues. Tied to the [`ProcInst`](crate::proc::ProcInst) system.

use unity::prelude::*;
use crate::gamedata::ItemData;
use crate::proc::Bindable;
use crate::unit::UnitItem;
#[unity::class("App", "GameMessage")]
pub struct GameMessage {}

impl GameMessage {
    pub fn create_key_wait<'a>(parent: &impl Bindable, message: impl Into<&'a Il2CppString>) {
        Self::create_key_wait_(parent, message.into());
    }
    pub fn create_key_wait_mess<'a>(parent: &impl Bindable, mess_label: impl Into<&'a Il2CppString>) {
        let message = crate::mess::Mess::get(mess_label);
        Self::create_key_wait_(parent, message);
    }
    #[unity::class_method(13)]
    pub fn create_common<B>(proc: &B, mess: &Il2CppString, is_bind: bool, status: i32) -> &'static GameMessage where B: Bindable; // Offset: 0x227EBD0 Flags: 0
    #[unity::class_method(14)]
    pub fn create_no_bind<B>(proc: &B, mess: &Il2CppString, status: i32) -> &'static GameMessage where B: Bindable; // Offset: 0x227EDA0 Flags: 0
    #[unity::class_method(15)]
    pub fn create_bind<B>(proc: &B, mess: &Il2CppString, status: i32) -> &'static GameMessage where B: Bindable; // Offset: 0x227EDB0 Flags: 0
    #[unity::class_method(16)]
    pub fn create_key_wait_<B>(proc: &B, mess: &Il2CppString) -> &'static GameMessage where B: Bindable; // Offset: 0x2270D00 Flags: 0
    #[unity::class_method(17)]
    pub fn create_system<B>(proc: &B, mess: &Il2CppString) -> &'static GameMessage where B: Bindable; // Offset: 0x227EDC0 Flags: 0
    #[unity::class_method(18)]
    pub fn create_warning<B>(proc: &B, mess: &Il2CppString) -> &'static GameMessage where B: Bindable; // Offset: 0x227EDD0 Flags: 0
    #[unity::class_method(19)]
    pub fn create_item_get_impl<B>(proc: &B, item: &ItemData, name: &Il2CppString, label: &Il2CppString, count: i32) -> &'static GameMessage where B: Bindable; // Offset: 0x227EE00 Flags: 0
    #[unity::class_method(20)]
    pub fn create_item_get<B>(proc: &B, item_data: &ItemData, label: &Il2CppString, count: i32) -> &'static GameMessage where B: Bindable; // Offset: 0x227F120 Flags: 0
    #[unity::class_method(21)]
    pub fn create_unit_item_get<B>(proc: &B, unit_item: &UnitItem, label: &Il2CppString, count: i32) -> &'static GameMessage where B: Bindable; // Offset: 0x227F170 Flags: 0
    #[unity::class_method(22)]
    pub fn create_gold_gain<B>(proc: &B, gold: i32, label: Option<&Il2CppString>) -> &'static GameMessage where B: Bindable; // Offset: 0x227F4C0 Flags: 0
    #[unity::class_method(23)]
    pub fn create_grow_up_item_use<B>(proc: &B, item: &ItemData) -> &'static GameMessage where B: Bindable; // Offset: 0x227F730 Flags: 0
    #[unity::class_method(24)]
    pub fn create_enhance_item_use<B>(proc: &B, item: &ItemData) -> &'static GameMessage where B: Bindable; // Offset: 0x227F9C0 Flags: 0

}