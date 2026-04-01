use unity::engine::Color;
use unity::engine::ui::Image;
use unity::prelude::*;
use unity::system::action::Action1;
use crate::gamedata::skill::SkillData;
use crate::god::GodUnit;
use crate::menu::{BasicMenuResult, BasicMenu};
use crate::menu::menu_item::{BasicMenuItemContentFields, MenuItem, MenuItemContent};
use crate::tmpro::TextMeshProUGUI;
use crate::unit::Unit;

#[unity::class("App", "SkillInheritanceMenuItem")]
pub struct SkillInheritanceMenuItem {
    pub menu: &'static mut BasicMenu<SkillInheritanceMenuItem>,
    pub content: Option<&'static mut SkillInheritanceMenuItemContent>,
    pub name: &'static Il2CppString,
    pub index: i32,
    pub full_index: i32,
    pub attribute: i32,
    cursor_color: Color,
    active_text_color: Color,
    inactive_text_color: Color,
    pub original_skill_index: i32,
    pub skill: Option<&'static mut SkillData>,
    pub sort_id: i32,
    pub skill_level: i32,
    pub skill_cost: i32,
    pub decide_handler: Option<&'static mut Action1<Unit>>,
}

impl SkillInheritanceMenuItem {
    #[unity::class_method(0)] pub fn get_skill(&self) -> &'static SkillData; // Offset: 0x24A6320 Flags: 0
    #[unity::class_method(1)] pub fn set_skill(&self, value: &SkillData); // Offset: 0x24A6330 Flags: 0
    #[unity::class_method(8)] pub fn ctor(&self, sort_id: i32, skill: &SkillData, skill_level: i32, decide_event_handler: Option<&Action1<&Unit>>); // Offset: 0x24A33C0 Flags: 0
    #[unity::class_method(11)] pub fn set_initial_color(&self); // Offset: 0x24A6880 Flags: 0
    #[unity::class_method(12)] pub fn on_select(&self); // Offset: 0x24A6980 Flags: 0
    #[unity::class_method(13)] pub fn acall(&self) -> BasicMenuResult; // Offset: 0x24A6A20 Flags: 0
    #[unity::class_method(14)] pub fn is_inherited(&self) -> bool; // Offset: 0x24A63A0 Flags: 0
    #[unity::class_method(15)] pub fn is_enough_level(&self) -> bool; // Offset: 0x24A66A0 Flags: 0
    #[unity::class_method(16)] pub fn is_enough_sp(&self) -> bool; // Offset: 0x24A6790 Flags: 0
    #[unity::class_method(17)] pub fn reset_cost(&self); // Offset: 0x24A64A0 Flags: 0
    #[unity::class_method(18)] pub fn on_inherit(&self); // Offset: 0x24A6CE0 Flags: 0
    #[unity::class_method(19)] pub fn get_unit(&self) -> &'static Unit; // Offset: 0x24A6AF0 Flags: 0
    #[unity::class_method(20)] pub fn get_god(&self) -> &'static GodUnit; // Offset: 0x24A6BB0 Flags: 0

}
impl MenuItem for SkillInheritanceMenuItem {}

#[unity::class("App", "SkillInheritanceMenuItemContent")]
pub struct SkillInheritanceMenuItemContent {
    pub parent: BasicMenuItemContentFields,
    pub image_icon: &'static mut Image,
    pub text_name: &'static mut TextMeshProUGUI,
    pub text_level: &'static mut TextMeshProUGUI,
    obj: u64,
    pub text_cost: &'static mut TextMeshProUGUI,
}

impl MenuItemContent<SkillInheritanceMenuItem> for SkillInheritanceMenuItemContent {}