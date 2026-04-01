use unity::prelude::*;
use unity::engine::MonoBehaviorFields;
use crate::menu::content::AccessoryEquipmentInfo;
use crate::tmpro::TextMeshProUGUI;
use crate::unityengine::{GameObject, UnityComponent};
use crate::menu::menus::shop::shopunitselect::ShopUnitSelectMenuDecideHandler;

pub mod change;

#[unity::class("App", "AccessoryShopUnitSelectRoot")]
pub struct AccessoryShopUnitSelectRoot {
    parent: MonoBehaviorFields,
    pub menu_object: &'static GameObject,
    pub accessory_equip_info: &'static GameObject,
    pub unit_name: &'static mut TextMeshProUGUI,
    pub equipment: &'static mut AccessoryEquipmentInfo,
    pub decide_event_handler: Option<&'static mut ShopUnitSelectMenuDecideHandler>,
}
impl UnityComponent for AccessoryShopUnitSelectRoot {}

impl AccessoryShopUnitSelectRoot {
    #[unity::class_method(0)] pub fn load_prefab_async(); // Offset: 0x27C5120 Flags: 0
    #[unity::class_method(1)] pub fn is_loading_prefab() -> bool; // Offset: 0x27C5280 Flags: 0
    #[unity::class_method(2)] pub fn unload_prefab(); // Offset: 0x27C5300 Flags: 0
}
