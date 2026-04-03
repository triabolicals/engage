use unity::prelude::*;
use unity::system::List;
use crate::menu::MenuContent;
use crate::proc::{Bindable, ProcInstFields};
use crate::unityengine::{MonoBehaviorFields, GameObject, UnityComponent};

#[unity::class("", "ConfigRoot")]
pub struct ConfigRoot {
    pub parent: MonoBehaviorFields,
    pub config_menu_content_object: &'static GameObject,
}
impl ConfigRoot {
    #[unity::class_method(0)] pub fn load_prefab_async(); // Offset: 0x2539F10 Flags: 0
    #[unity::class_method(1)] pub fn unload_prefab(); // Offset: 0x2539F90 Flags: 0
    #[unity::class_method(2)] pub fn is_loading_prefab() -> bool; // Offset: 0x253A010 Flags: 0
    #[unity::class_method(3)] pub fn create() -> &'static GameObject; // Offset: 0x2539230 Flags: 0
    #[unity::class_method(4)] pub fn destroy(game_object: &GameObject); // Offset: 0x253A090 Flags: 0
    #[unity::class_method(5)] pub fn get_menu_content(&self) -> &'static ConfigMenuContent; // Offset: 0x25392C0 Flags: 0
    #[unity::class_method(6)] pub fn ctor(&self); // Offset: 0x253A100 Flags: 0
}
impl UnityComponent for ConfigRoot {}

#[unity::class("", "ConfigMenuContent")]
pub struct ConfigMenuContent{}

impl MenuContent for ConfigMenuContent {}
impl UnityComponent for ConfigMenuContent {}

#[unity::class("", "ConfigMenu")]
pub struct ConfigMenu<T: 'static> {
    pub proc: ProcInstFields,
    pub menu_content: *const u8,
    pub menu_item_list: &'static mut List<T>,
    pub full_menu_item_list: &'static mut List<T>,
    pad: [u8; 0x10],
    pub row_num: i32,
    pub show_row_num: i32,
    pub select_index: i32,
    pub select_index_old: i32,
    pub scroll_index: i32,
    pub scroll_index_old: i32,
    pub reserved_select_index: i32,
    pub reserved_scroll_index: i32,
    pub reserved_show_row_num: i32,
    pub memory_display_index: i32,
    pub suspend: i32,
}

// Workaround to not specify a generic type when using as a static method
impl ConfigMenu<()> {
    pub fn create_bind(parent: &impl Bindable) {
        unsafe { configmenu_createbind(parent, None) }
    }
}

impl<T> ConfigMenu<T> {
    pub fn add_item(&mut self, item: &'static mut T) {
        self.full_menu_item_list.add(item);
    }
}

impl<T> AsRef<ProcInstFields> for ConfigMenu<T> {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl<T> AsMut<ProcInstFields> for ConfigMenu<T> {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}

#[unity::from_offset("", "ConfigMenu", "CreateBind")]
fn configmenu_createbind<T: Bindable + ?Sized>(parent: &T, method_info: OptionalMethod); // Apparently returns a GameObject?