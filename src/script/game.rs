use unity::{prelude::*, il2cpp::object::Array};
use super::{DynValue, EventScript};

#[unity::class("App", "ScriptGame")]
pub struct ScriptGame { }

impl ScriptGame {
    #[unity::class_method(0)] pub fn item_gain(args: &Array<&DynValue>); // Offset: 0x1EC9A80 Flags: 0
    #[unity::class_method(1)] pub fn gold_gain(args: &Array<&DynValue>); // Offset: 0x1EC9BD0 Flags: 0
    #[unity::class_method(2)] pub fn item_put_off_all(args: &Array<&DynValue>); // Offset: 0x1EC9D00 Flags: 0
    #[unity::class_method(3)] pub fn difficulty_get(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EC9E20 Flags: 0
    #[unity::class_method(4)] pub fn game_mode_get(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EC9FE0 Flags: 0
    #[unity::class_method(5)] pub fn gold_get(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA0A0 Flags: 0
    #[unity::class_method(6)] pub fn gold_set(args: &Array<&DynValue>); // Offset: 0x1ECA160 Flags: 0
    #[unity::class_method(7)] pub fn person_get_index(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA230 Flags: 0
    #[unity::class_method(8)] pub fn job_get_index(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA330 Flags: 0
    #[unity::class_method(9)] pub fn item_get_index(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA430 Flags: 0
    #[unity::class_method(10)] pub fn skill_get_index(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA530 Flags: 0
    #[unity::class_method(12)] pub fn person_get_id(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA630 Flags: 0
    #[unity::class_method(13)] pub fn job_get_id(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA700 Flags: 0
    #[unity::class_method(14)] pub fn item_get_id(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA7D0 Flags: 0
    #[unity::class_method(15)] pub fn skill_get_id(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECA8A0 Flags: 0
    #[unity::class_method(16)] pub fn config_set_battle_scene(args: &Array<&DynValue>); // Offset: 0x1ECA970 Flags: 0
    #[unity::class_method(17)] pub fn config_set_support_scene(args: &Array<&DynValue>); // Offset: 0x1ECAA70 Flags: 0
    #[unity::class_method(18)] pub fn config_get_battle_scene(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECAB70 Flags: 0
    #[unity::class_method(19)] pub fn config_get_support_scene(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ECAC80 Flags: 0
    #[unity::class_method(20)] pub fn play_chapter_title(args: &Array<&DynValue>); // Offset: 0x1ECAD90 Flags: 0
    #[unity::class_method(21)] pub fn regist(script: &EventScript); // Offset: 0x1ECAEC0 Flags: 0
    #[unity::class_method(22)] pub fn ctor(&self); // Offset: 0x1ECB490 Flags: 0
}
