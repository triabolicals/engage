use unity::engine::MonoBehaviorFields;
use crate::unit::Unit;
use crate::unityengine::GameObject;

#[unity::class("App", "UnitSelectRoot")]
pub struct UnitSelectRoot {
    parent: MonoBehaviorFields,
    pub unit_list_root: &'static GameObject, // Offset 0x18, Attr: 6
    pub god_image_object: &'static GameObject, // Offset 0x20, Attr: 6
    pub unit: Option<&'static Unit>, // Offset 0x28, Attr: 1
}
