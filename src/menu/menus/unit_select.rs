use crate::menu::BasicMenuMethods;
use unity::prelude::Il2CppClassData;
use crate::unit::Unit;
use crate::unityengine::{MonoBehaviorFields, GameObject};

#[unity::class("App", "UnitSelectRoot")]
pub struct UnitSelectRoot {
    parent: MonoBehaviorFields,
    pub unit_list_root: &'static GameObject, // Offset 0x18, Attr: 6
    pub god_image_object: &'static GameObject, // Offset 0x20, Attr: 6
    pub unit: Option<&'static Unit>, // Offset 0x28, Attr: 1
}

#[unity::class("App", "UnitSelectMenu")]
pub struct UnitSelectMenu { }

impl UnitSelectMenu {
    #[unity::class_method(4)] pub fn set_select_index_from_unit(&self, unit: &Unit); // Offset: 0x1C5E030 Flags: 0
}
impl BasicMenuMethods for UnitSelectMenu {}
