use unity::prelude::*;
use crate::menu::BasicMenuSelect;
use crate::menu::menus::unit_select::UnitSelectRoot;
use crate::proc::SingletonProcInstFields;
use crate::unit::Unit;
use crate::unityengine::GameObject;
use crate::util::{get_instance, get_singleton_proc_instance, try_get_instance};

#[unity::class("App", "SortieSelectionUnitManager")]
pub struct SortieSelectionUnitManager {
    junk: [u8; 0x18],
    pub unit: Option<&'static mut Unit>,
}

impl SortieSelectionUnitManager {
    pub fn get_instance() -> Option<&'static mut SortieSelectionUnitManager> { Some(get_instance::<Self>()) }
    pub fn is_sortie_mode() -> bool {
        unsafe { sortie_is_sortie_mode( Self::get_instance().unwrap(), None )}
    }
    pub fn get_unit() -> &'static mut Unit {
        let instance = Self::get_instance().unwrap();
        unsafe { sortie_get_unit(instance, None) }
    }
}

#[unity::class("App", "SortieTopMenuManager")]
pub struct SortieTopMenuManager {}

impl SortieTopMenuManager {
    pub fn get_instance() -> Option<&'static mut SortieTopMenuManager> {
        Some(get_instance::<Self>())
    }
    pub fn get_menu_select() -> &'static mut BasicMenuSelect {
        unsafe { sortie_top_menu_get_menu_select(Self::get_instance().unwrap(), None)}
    }
}
#[unity::class("App", "SortieSequenceUnitSelect")]
pub struct SortieSequenceUnitSelect {
    pub proc: SingletonProcInstFields,
    pub game_object: &'static GameObject, // Offset 0x78, Attr: 1
    pub window: &'static UnitSelectRoot, // Offset 0x80, Attr: 1
    /*
    m_unit_select_menu: &BasicMenu, // Offset 0x88, Attr: 1
    m_root_animator: &Animator, // Offset 0x90, Attr: 1
    m_menu_select: &BasicMenuSelect, // Offset 0x98, Attr: 1
}

     */
}
impl SortieSequenceUnitSelect {
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
    #[unity::class_method(10)] pub fn open(&self); // Offset: 0x1FFA5D0 Flags: 0
    #[unity::class_method(11)] pub fn after_open(&self); // Offset: 0x1FFA8B0 Flags: 0
    #[unity::class_method(12)] pub fn reset_select(&self); // Offset: 0x1FFAF60 Flags: 0
    #[unity::class_method(13)] pub fn menu_tick(&self); // Offset: 0x1FFB100 Flags: 0
    #[unity::class_method(14)] pub fn close_all(&self); // Offset: 0x1FFB110 Flags: 0
    #[unity::class_method(15)] pub fn is_closed(&self) -> bool; // Offset: 0x1FFB180 Flags: 0
    #[unity::class_method(16)] pub fn post_closed(&self); // Offset: 0x1FFB230 Flags: 0
    #[unity::class_method(17)] pub fn disp_all(&self); // Offset: 0x1FFB240 Flags: 0
    #[unity::class_method(18)] pub fn hide_header_key_help(&self); // Offset: 0x1FFB360 Flags: 0
    #[unity::class_method(19)] pub fn release(&self); // Offset: 0x1FFB430 Flags: 0
    #[unity::class_method(20)] pub fn setting_title(&self); // Offset: 0x1FFAA40 Flags: 0
    #[unity::class_method(21)] pub fn close_title(&self); // Offset: 0x1FFB590 Flags: 0
}

#[skyline::from_offset(0x01fe8c00)]
fn sortie_is_sortie_mode(this: &SortieSelectionUnitManager, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01fe8db0)]
fn sortie_get_unit(this: &SortieSelectionUnitManager, method_info: OptionalMethod) -> &'static mut Unit;

#[unity::from_offset("App", "SortieTopMenuManager", "get_MenuSelect")]
fn sortie_top_menu_get_menu_select(this: &SortieTopMenuManager, method_info: OptionalMethod) -> &'static mut BasicMenuSelect;
