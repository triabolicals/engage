use unity::engine::ui::Image;
use unity::prelude::Il2CppString;
use crate::tmpro::TextMeshProUGUI;
use crate::unityengine::{MonoBehaviorFields, GameObject};

#[unity::class("App", "UnitMenuItemSetter")]
pub struct UnitMenuItemSetter {
    parent: MonoBehaviorFields,
    pub select_bg: &'static mut Image,
    pub sub_frame: &'static mut Image,
    pub unit_name: &'static mut TextMeshProUGUI,
    pub face: &'static mut Image,
    pub icon: &'static mut Image,
    pub icon_check: &'static GameObject,
    pub message: &'static mut TextMeshProUGUI,
}

#[unity::class("App", "ShopContent")]
pub struct ShopContent {
    pub iid: &'static Il2CppString,
    pub stock: i32,
    pub new_arrival: bool,
}