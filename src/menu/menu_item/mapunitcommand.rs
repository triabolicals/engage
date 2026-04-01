use unity::prelude::*;
use crate::gamedata::skill::SkillData;
use crate::map::mind::MapMindType;
use crate::map::panel::MapPanelDeployMode;
use crate::menu::menu_item::MenuItem;
use crate::menu::menus::mapunitcommand::MapUnitCommandMenu;
use crate::unit::Unit;

#[unity::class("", "BaseMenuItem")]
#[nested_from_type(MapUnitCommandMenu)]
pub struct MapUnitCommandMenuBaseMenuItem {}

pub trait MapUnitCommandMenuItemMethods: MenuItem {
    #[unity::class_method(28, vtable)] fn get_help_text(&self) -> &'static Il2CppString; // Offset: 0x1F036B0 Flags: 0
    #[unity::class_method(30, vtable)] fn get_active_mind(&self) -> MapMindType; // Offset: 0x1E48B00 Flags: 0
    #[unity::class_method(31, vtable)] fn get_deploy_mode(&self) -> MapPanelDeployMode; // Offset: 0x1E48B10 Flags: 0
    #[unity::class_method(32, vtable)] fn get_command_skill(&self) -> Option<&'static SkillData>; // Offset: 0x1E48B20 Flags: 0
    #[unity::class_method(33, vtable)] fn get_is_forecast(&self) -> bool; // Offset: 0x1E48B30 Flags: 0
    #[unity::class_method(34, vtable)] fn get_command_help(&self) -> &'static Il2CppString; // Offset: 0x1E49400 Flags: 0
    #[unity::class_method(35, vtable)] fn check_buildable_with_unit3(&self, unit: &Unit) -> bool; // Offset: 0x1E484B0 Flags: 0
}

#[unity::class("", "TradeMenuItem")]
#[nested_from_type(MapUnitCommandMenu)]
pub struct TradeMenuItem {}

impl MenuItem for TradeMenuItem {}
impl MapUnitCommandMenuItemMethods for TradeMenuItem {}


#[unity::class("", "ItemMenuItem")]
#[nested_from_type(MapUnitCommandMenu)]
pub struct MapUnitCommandMenuItemMenuItem { }

impl MenuItem for MapUnitCommandMenuItemMenuItem {}
impl MapUnitCommandMenuItemMethods for MapUnitCommandMenuItemMenuItem {}
