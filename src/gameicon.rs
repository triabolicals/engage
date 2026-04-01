use unity::prelude::*;
use unity::engine::Sprite;
use crate::spriteatlasmanager::SpriteAtlasManager;

#[unity::class("App", "GameIcon")]
#[static_fields(GameIconStaticFields)]
pub struct GameIcon { }

impl GameIcon {
    pub fn try_get_system<'a>(icon_name: impl Into<&'a Il2CppString>) -> Option<&'static mut Sprite> {
        unsafe { gameicon_trygetsystem(icon_name.into(), None) }
    }
    pub fn try_get_unit_icon_index<'a>(icon_name: impl Into<&'a Il2CppString>) -> Option<&'static mut Sprite> {
        unsafe { gameicon_tryget_unit_icon(icon_name.into(), None) }
    }
    pub fn try_get_skill<'a>(icon_name: impl Into<&'a Il2CppString>) -> Option<&'static mut Sprite> {
        unsafe { gameicon_trygetskill(icon_name.into(), None) }
    }
    pub fn try_get_accessory_kind(kind: i32) -> Option<&'static mut Sprite> {
        unsafe { gameicon_tryget_accessory_kind(kind, None) }
    }
    pub fn try_get_efficacy<'a>(label: impl Into<&'a Il2CppString>, outline: bool) -> Option<&'static mut Sprite> {
        unsafe { gameicon_try_get_effiacacy(label.into(), outline, None) }
    }
    pub fn try_get_item<'a>(icon_name: impl Into<&'a Il2CppString>) -> Option<&'static mut Sprite> {
        unsafe { gameicon_try_get_item_by_icon_name(icon_name.into(), None) }
    }
}

pub struct GameIconStaticFields {
    pub skill: &'static mut SpriteAtlasManager,
    pub item: &'static mut SpriteAtlasManager,

    pub efficacy: &'static SpriteAtlasManager,
    pub efficacy_outline: &'static SpriteAtlasManager,

    pub item_kinds: &'static SpriteAtlasManager,
    pub item_kinds_outline: &'static SpriteAtlasManager,

    pub god_symbol: &'static SpriteAtlasManager,
    pub god_ring: &'static mut SpriteAtlasManager,

    pub system: &'static SpriteAtlasManager,
    pub unit_icon_index: &'static SpriteAtlasManager,
    pub unit_icon_pallete: &'static SpriteAtlasManager,
}



#[unity::from_offset("App", "GameIcon", "TyrGetUnitIconIndex")]
extern "C" fn gameicon_tryget_unit_icon(icon_name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut Sprite>;

#[skyline::from_offset(0x227cd50)]
fn gameicon_try_get_item_by_icon_name(icon_name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut Sprite>;

#[unity::from_offset("App", "GameIcon", "TryGetSystem")]
extern "C" fn gameicon_trygetsystem(icon_name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut Sprite>;

#[unity::from_offset("App", "GameIcon", "TryGetSkill")]
extern "C" fn gameicon_trygetskill(icon_name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut Sprite>;

#[unity::from_offset("App", "GameIcon", "TryGetAccessoryKinds")]
extern "C" fn gameicon_tryget_accessory_kind(kind: i32, method_info: OptionalMethod) -> Option<&'static mut Sprite>;
#[unity::from_offset("App", "GameIcon", "TryGetEfficacy")]
fn gameicon_try_get_effiacacy(label: &Il2CppString, outline: bool, method_info: OptionalMethod) -> Option<&'static mut Sprite>;