use num_derive::FromPrimitive;
use unity::engine::Color;
use unity::engine::ui::Image;
use unity::prelude::*;
use unity::system::List;
use crate::gamedata::{ItemData, JobData, WeaponMask};
use crate::menu::{BasicMenu, BasicMenuContentFields, BasicMenuItem, BasicMenuItemAttribute, BasicMenuItemFields, MenuItem};
use crate::menu::menu_item::{BasicMenuItemContent, MenuItemContent};
use crate::proc::Bindable;
use crate::tmpro::TextMeshProUGUI;
use crate::unit::Unit;
use crate::uniticon::UnitIcon;
use crate::unitinfo::UnitStatusSetter;
use crate::unityengine::{MonoBehaviorFields, GameObject, UnityComponent};

#[unity::class("App", "ClassChange")]
pub struct ClassChange{}
impl ClassChange {
    #[unity::class_method(0)] pub fn get_select_job_list(job_data: &JobData) -> &'static List<&'static ClassChangeJobData>; // Offset: 0x1EA1BE0 Flags: 0
    #[unity::class_method(1)] pub fn get_job_list_all() -> &'static List<ClassChangeJobData>; // Offset: 0x1EA2050 Flags: 0
    #[unity::class_method(2)] pub fn get_job_list(unit: &Unit, item: &ItemData) -> &'static List<&'static JobData>; // Offset: 0x1EA2310 Flags: 0
    #[unity::class_method(3)] pub fn get_job_list_by_master(unit: &Unit) -> &'static List<&'static JobData>; // Offset: 0x1EA24A0 Flags: 0
    #[unity::class_method(4)] pub fn get_job_list_by_change(unit: &Unit) -> &'static List<&'static JobData>; // Offset: 0x1EA2540 Flags: 0
    #[unity::class_method(5)] pub fn add_to_list_for_master(job_list: &List<&JobData>, unit: &Unit, high_jobs: &List<&JobData>); // Offset: 0x1EA25E0 Flags: 0
    #[unity::class_method(6)] pub fn add_to_list_for_change(job_list: &List<&JobData>, unit: &Unit); // Offset: 0x1EA2710 Flags: 0
    #[unity::class_method(7)] pub fn add_to_list(job_list: List<&JobData>, unit: &Unit, job: &JobData, include_current_job: bool, ignore_aptitude_check: bool); // Offset: 0x1EA2C70 Flags: 0
    #[unity::class_method(8)] pub fn get_relational_jobs(job: &JobData) -> &'static List<&'static JobData>; // Offset: 0x1EA2F10 Flags: 0
}

#[unity::class("App", "ClassChangeJobMenu")]
pub struct ClassChangeJobMenu{}
impl ClassChangeJobMenu {
    #[unity::class_method(0)] pub fn create<B>(proc: &B, class_change_root: &ClassChangeRoot) -> &'static ClassChangeJobMenu where B: Bindable; // Offset: 0x1EA3100 Flags: 0
    #[unity::class_method(2)] pub fn after_build(&self); // Offset: 0x1EA35D0 Flags: 0
    #[unity::class_method(3)] pub fn get_name(&self) -> &'static Il2CppString; // Offset: 0x1EA3840 Flags: 0
    #[unity::class_method(5)] pub fn sort_menu_item(&self); // Offset: 0x1EA4330 Flags: 0
    #[unity::class_method(6)] pub fn set_unit_info_all(&self); // Offset: 0x1EA3760 Flags: 0
    #[unity::class_method(7)] pub fn set_unit_info_all2(&self, after_change_job_data: &ClassChangeJobData); // Offset: 0x1EA3540 Flags: 0
    #[unity::class_method(8)] pub fn set_unit_info_after(&self, unit_before: &Unit, data: &ClassChangeJobData); // Offset: 0x1EA4A40 Flags: 0
    #[unity::class_method(9)] pub fn set_job_details(&self, data: &ClassChangeJobData); // Offset: 0x1EA36D0 Flags: 0
    #[unity::class_method(10)] pub fn get_selected_unit_copy() -> &'static mut Unit; // Offset: 0x1EA4680 Flags
}
impl Bindable for ClassChangeJobMenu {}


#[unity::class("App", "ClassChangeRoot")]
pub struct ClassChangeRoot{
    parent: MonoBehaviorFields,
    pub menu_content: &'static ClassChangeJobMenuContent, // Offset 0x18, Attr: 6
    pub unit_name: &'static TextMeshProUGUI, // Offset 0x20, Attr: 6
    pub unit_info_before: &'static UnitStatusSetter, // Offset 0x28, Attr: 6
    pub unit_info_after: &'static UnitStatusSetter, // Offset 0x30, Attr: 6
    pub weapon_icon_list: &'static List<&'static mut Image>, // Offset 0x38, Attr: 6
}
impl UnityComponent for ClassChangeRoot {}

#[unity::class("App", "ClassChangeJobMenuContent")]
pub struct ClassChangeJobMenuContent {
    pub parent: BasicMenuContentFields,
    pub cost_text_level: &'static TextMeshProUGUI, // Offset 0xE8, Attr: 6
    pub cost_weapon_icon_obj_list: &'static List<&'static GameObject>, // Offset 0xF0, Attr: 6
    pub cost_weapon_text: &'static TextMeshProUGUI, // Offset 0xF8, Attr: 6
    pub cost_item_image: &'static Image, // Offset 0x100, Attr: 6
    pub cost_item_title: &'static TextMeshProUGUI, // Offset 0x108, Attr: 6
    pub cost_item_value: &'static TextMeshProUGUI, // Offset 0x110, Attr: 6
    pub help_text: &'static TextMeshProUGUI, // Offset 0x118, Attr: 6
    pub skill_rood_obj: &'static GameObject, // Offset 0x120, Attr: 6
    pub skill_image: &'static mut Image, // Offset 0x128, Attr: 6
    pub skill_name: &'static TextMeshProUGUI, // Offset 0x130, Attr: 6
    pub skill_get_level: &'static TextMeshProUGUI, // Offset 0x138, Attr: 6
    pub skill_help_text: &'static TextMeshProUGUI, // Offset 0x140, Attr: 6
}

#[unity::class("", "ChangeJobData")]
#[nested_from_type(ClassChange)]
pub struct ClassChangeJobData {
    pub job: &'static JobData, // Offset 0x10, Attr: 1
    pub job_weapon_mask: &'static WeaponMask, // Offset 0x18, Attr: 1
    pub original_job_weapon_mask: &'static WeaponMask, // Offset 0x20, Attr: 1
    pub proof_type: ChangeJobDataProofTypes, // Offset 0x28, Attr: 1
    pub cost_level: &'static Il2CppString, // Offset 0x30, Attr: 1
    pub is_enough_level: bool, // Offset 0x38, Attr: 1
    pub cost_weapon_mask: &'static WeaponMask, // Offset 0x40, Attr: 1
    pub equippable_weapon_mask: &'static WeaponMask, // Offset 0x48, Attr: 1
    pub is_enough_item: bool, // Offset 0x50, Attr: 1
    pub is_gender: bool, // Offset 0x51, Attr: 1
    pub is_default_job: bool, // Offset 0x52, Attr: 1
    ex_item1: Option<&'static ItemData>, // Offset 0x58, Attr: 1
    ex_item2: Option<&'static ItemData>, // Offset 0x60, Attr: 1
}
impl ClassChangeJobData {
    #[unity::class_method(29)] pub fn cc_check(&self, unit: &Unit) -> bool; // Offset: 0x19C6700 Flags: 0
}

#[unity::class("", "ClassChangeJobMenuItem")]
#[nested_from_type(ClassChangeJobMenu)]
pub struct ClassChangeJobMenuItem {
    pub menu: &'static mut BasicMenu<BasicMenuItem>,
    pub menu_item_content: &'static mut BasicMenuItemContent,
    pub name: &'static Il2CppString,
    pub index: i32,
    pub full_index: i32,
    pub m_attribute: i32,
    pub cursor_color: Color,
    pub active_text_color: Color,
    pub inactive_text_color: Color,
    pub pad: i32,
    pub job_data: &'static mut ClassChangeJobData, // Offset 0x68, Attr: 1
    pub attribute: BasicMenuItemAttribute, // Offset 0x70, Attr: 1
}
impl MenuItem for ClassChangeJobMenuItem {}

#[unity::class("App", "ClassChangeJobMenuItemContent")]
pub struct ClassChangeJobMenuItemContent {
    pub parent: BasicMenuItemFields,
    pub unit_icon: &'static mut UnitIcon, // Offset 0x48, Attr: 6
    pub title: &'static TextMeshProUGUI, // Offset 0x50, Attr: 6
    // pub weapon_level_list: List<&ClassChangeJobMenuItemContentWeaponLevelItem>, // Offset 0x58, Attr: 6
    // pub name_ranks: &'static TextMeshProUGUI, // Offset 0x60, Attr: 6
}
impl MenuItemContent<ClassChangeJobMenuItem> for ClassChangeJobMenuContent {}

#[unity::class("", "ConfirmDialog")]
#[nested_from_type(ClassChangeJobMenu)]
pub struct ClassChangeJobMenuConfirmDialog {}
impl ClassChangeJobMenuConfirmDialog {
    #[unity::class_method(0)]
    pub fn create_bind<B>(proc: &B, data: &ClassChangeJobData)
    where B: Bindable; // Offset: 0x19C76C0 Flags: 0
}

#[repr(C)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum ChangeJobDataProofTypes {
    Master = 0,
    Change = 1,
    Enchant = 2,
    Gunner = 3,
}