//! Methods, type and traits related to menus. Deeply tied with [`ProcInst`](crate::proc::ProcInst).

use modular_bitfield::{bitfield, specifiers::B2};
use unity::{prelude::*, system::List};
use crate::proc::{desc::ProcDesc, Bindable, ProcInst, ProcInstFields};
pub mod content;
pub mod savedata;
pub mod menu_item;
pub mod menus;
pub use menu_item::{config::*, BasicMenuItem, BasicMenuItemFields, MenuItem};
pub use content::{BasicMenuContent, BasicMenuContentFields, MenuContent};

/// Represents the base Menu from which every other inherits.
///
/// A Menu is a [ProcInst](crate::proc::ProcInst) with fields to keep every element together
///
/// You can usually use this instead of a class inheriting from it at the risk of missing fields and methods.
/// 
//
#[unity::class("App", "BasicMenu")] pub struct BasicMenuClass{}

#[unity::class("App", "BasicMenu")]
pub struct BasicMenu<T: 'static> {
    pub proc: ProcInstFields,
    pub menu_content: &'static mut BasicMenuContent,
    pub menu_item_list: &'static mut List<T>,
    pub full_menu_item_list: &'static mut List<T>,
    pub status_field: &'static mut BasicMenuStatus,
    pub result: i32,
    pub scroll_precede_input_a: bool,
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

impl<T> BasicMenu<T> {
    pub fn new_with_content<M: MenuContent>(menu_item_list: &List<T>, content: &M) -> &'static mut BasicMenu<T> {
        let instance: &'static mut BasicMenu<T> = BasicMenu::<T>::instantiate().unwrap();
        unsafe { basicmenu_ctor2(instance, menu_item_list, content, None) };
        instance
    }
    pub fn new(menu_item_list: &List<T>, menu_content: &BasicMenuContent) -> &'static mut BasicMenu<T> {
        let instance: &'static mut BasicMenu<T> = BasicMenu::<T>::instantiate().unwrap();
        unsafe { basicmenu_ctor(instance, menu_item_list, menu_content, None) };
        instance
    }
    pub fn add_item(&mut self, item: &'static mut T) {
        self.full_menu_item_list.add(item);
    }
    #[unity::class_method(22, BasicMenuClass)] pub fn get_item(&self, item_index: i32) -> Option<&'static mut T>; // Offset: 0x2454AE0 Flags: 0

    pub fn cast<M: BasicMenuMethods>(&self) -> &'static M { unsafe { std::mem::transmute::<&Self, &'static M>(self) } }
    pub fn cast_mut<M: BasicMenuMethods>(&mut self) -> &'static mut M { unsafe { std::mem::transmute::<&mut Self, &mut M>(self) } }
}
impl<T> AsRef<ProcInstFields> for BasicMenu<T> {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl<T> AsMut<ProcInstFields> for BasicMenu<T> {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}
impl<T> Bindable for BasicMenu<T> {}

/// Interface for basic menu inheritance
pub trait BasicMenuMethods: Il2CppClassData {
    #[unity::class_method(5,  BasicMenuClass)] fn bind_parent_menu(&self); // Offset: 0x2454310 Flags: 0
    #[unity::class_method(6,  BasicMenuClass)] fn build(&self); // Offset: 0x245BB20 Flags: 0
    #[unity::class_method(7,  BasicMenuClass)] fn set_active(&self, active: bool); // Offset: 0x245C3D0 Flags: 0
    #[unity::class_method(8,  BasicMenuClass)] fn set_active_all(&self, active: bool); // Offset: 0x245C480 Flags: 0
    #[unity::class_method(28, BasicMenuClass)] fn set_show_row_num(&self, row_num: i32); // Offset: 0x245CD60 Flags: 0
    #[unity::class_method(42, BasicMenuClass)] fn save_select(&self, sel: &BasicMenuSelect); // Offset: 0x245D070 Flags: 0
    #[unity::class_method(43, BasicMenuClass)] fn restore_select(&self, sel: &BasicMenuSelect); // Offset: 0x245D0A0 Flags: 0
    #[unity::class_method(86, BasicMenuClass)] fn set_transform_as_sub_menu<M>(&self, parent_menu: &M, parent_menu_item: &BasicMenuItem) where M: BasicMenuMethods; // Offset: 0x245E330 Flags: 0
    #[unity::class_method(92, BasicMenuClass)] fn rebuild(&self); // Offset: 0x245E7E0 Flags: 0
    #[unity::class_method(93, BasicMenuClass)] fn rebuild_instant(&self, is_keep_item_index: bool); // Offset: 0x245E840 Flags: 0
    #[unity::class_method(94, BasicMenuClass)] fn rebuild_instant2(&self, menu_select: &BasicMenuSelect); // Offset: 0x245E930 Flags: 0
    #[unity::class_method(96, BasicMenuClass)] fn close(&self); // Offset: 0x245E980 Flags: 0
    #[unity::class_method(97, BasicMenuClass)] fn close_parent_menu(&self); // Offset: 0x245E9E0 Flags: 0
    #[unity::class_method(98, BasicMenuClass)] fn delete_parent_menu(&self); // Offset: 0x245EAB0 Flags: 0
    #[unity::class_method(100, BasicMenuClass)] fn tick_input_base(&self) -> bool; // Offset: 0x245ED80
    #[unity::class_method(110, BasicMenuClass)] fn key_call_base(&self) -> BasicMenuResult; // Offset: 0x2461870 Flags: 0
    #[unity::class_method(111, BasicMenuClass)] fn a_call_base(&self) -> BasicMenuResult; // Offset: 0x2461910 Flags: 0
    #[unity::class_method(112, BasicMenuClass)] fn b_call_base(&self) -> BasicMenuResult; // Offset: 0x24619B0 Flags: 0
    #[unity::class_method(113, BasicMenuClass)] fn x_call_base(&self) -> BasicMenuResult; // Offset: 0x2461A50 Flags: 0
    #[unity::class_method(114, BasicMenuClass)] fn y_call_base(&self) -> BasicMenuResult; // Offset: 0x2461AF0 Flags: 0
    #[unity::class_method(115, BasicMenuClass)] fn l_call_base(&self) -> BasicMenuResult; // Offset: 0x2461B90 Flags: 0
    #[unity::class_method(116, BasicMenuClass)] fn r_call_base(&self) -> BasicMenuResult; // Offset: 0x2461C30 Flags: 0
    #[unity::class_method(117, BasicMenuClass)] fn plus_call_base(&self) -> BasicMenuResult; // Offset: 0x2461CD0 Flags: 0
    #[unity::class_method(118, BasicMenuClass)] fn minus_call_base(&self) -> BasicMenuResult; // Offset: 0x2461D70 Flags: 0
    #[unity::class_method(119, BasicMenuClass)] fn custom_call_base(&self) -> BasicMenuResult; // Offset: 0x2461E10 Flags: 0
    #[unity::class_method(5,  vtable)] fn on_start(&self); // Offset: 0x281F340 Flags: 0
    #[unity::class_method(6,  vtable)] fn on_tick(&self); // Offset: 0x281F350 Flags: 0
    #[unity::class_method(8,  vtable)] fn on_persistent(&self); // Offset: 0x281F3C0 Flags: 0
    #[unity::class_method(9,  vtable)] fn on_create(&self); // Offset: 0x245C830 Flags: 0
    #[unity::class_method(10, vtable)] fn on_dispose(&self); // Offset: 0x245C8B0 Flags: 0
    #[unity::class_method(11, vtable)] fn on_bind(&self); // Offset: 0x245BA30 Flags: 0
    #[unity::class_method(12, vtable)] fn on_unbind(&self); // Offset: 0x245BA40 Flags: 0
    #[unity::class_method(13, vtable)] fn on_singleton_create(&self); // Offset: 0x281F410 Flags: 0
    #[unity::class_method(14, vtable)] fn on_singleton_dispose(&self); // Offset: 0x281F420 Flags: 0
    #[unity::class_method(15, vtable)] fn on_shutdown(&self); // Offset: 0x281F430 Flags: 0
    #[unity::class_method(18, vtable)] fn create_default_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc>; // Offset: 0x245B530 Flags: 0
    #[unity::class_method(20, vtable)] fn open_anime(&self); // Offset: 0x245C5F0 Flags: 0
    #[unity::class_method(21, vtable)] fn open_anime_all(&self); // Offset: 0x245C610 Flags: 0
    #[unity::class_method(22, vtable)] fn close_anime(&self); // Offset: 0x245C6B0 Flags: 0
    #[unity::class_method(23, vtable)] fn close_anime_all(&self); // Offset: 0x245C6D0 Flags: 0
    #[unity::class_method(24, vtable)] fn on_build(&self, is_first_build: bool); // Offset: 0x245C770 Flags: 0
    #[unity::class_method(25, vtable)] fn after_build(&self); // Offset: 0x2453C30 Flags: 0
    #[unity::class_method(26, vtable)] fn on_close(&self); // Offset: 0x2453D80 Flags: 0
    #[unity::class_method(27, vtable)] fn on_cursor_move_end(&self); // Offset: 0x245C790 Flags: 0
    #[unity::class_method(28, vtable)] fn on_resume(&self); // Offset: 0x245C810 Flags: 0
    #[unity::class_method(29, vtable)] fn on_suspend(&self); // Offset: 0x245C820 Flags: 0
    #[unity::class_method(30, vtable)] fn get_name(&self) -> &'static Il2CppString; // Offset: 0x245CBA0 Flags: 0
    #[unity::class_method(31, vtable)] fn clamp_menu_item_index(&self, item_index: i32) -> i32; // Offset: 0x245CBF0 Flags: 0
    #[unity::class_method(32, vtable)] fn get_show_row_max(&self) -> i32; // Offset: 0x245CE10 Flags: 0
    #[unity::class_method(33, vtable)] fn get_build_row_num(&self) -> i32; // Offset: 0x245CE20 Flags: 0
    #[unity::class_method(34, vtable)] fn set_select_index(&self, select_index: i32); // Offset: 0x245CE90 Flags: 0
    #[unity::class_method(35, vtable)] fn adjust_scroll_index(&self); // Offset: 0x245D010 Flags: 0
    #[unity::class_method(36, vtable)] fn is_system_call(&self) -> bool; // Offset: 0x245D610 Flags: 0
    #[unity::class_method(37, vtable)] fn rebuild_instant3(&self, menu_select: &BasicMenuSelect, display_index: i32); // Offset: 0x245E950 Flags: 0
    #[unity::class_method(38, vtable)] fn tick(&self); // Offset: 0x245EB40 Flags: 0
    #[unity::class_method(39, vtable)] fn tick_input(&self) -> bool; // Offset: 0x245ED80 Flags: 0
    #[unity::class_method(40, vtable)] fn key_up(&self, is_trigger: bool); // Offset: 0x2460EE0 Flags: 0
    #[unity::class_method(41, vtable)] fn key_down(&self, is_trigger: bool); // Offset: 0x2460F00 Flags: 0
    #[unity::class_method(42, vtable)] fn key_left(&self, is_trigger: bool); // Offset: 0x2460F20 Flags: 0
    #[unity::class_method(43, vtable)] fn key_right(&self, is_trigger: bool); // Offset: 0x2460FB0 Flags: 0
    #[unity::class_method(44, vtable)] fn move_up(&self, is_trigger: bool); // Offset: 0x2461040 Flags: 0
    #[unity::class_method(45, vtable)] fn move_down(&self, is_trigger: bool); // Offset: 0x2461270 Flags: 0
    #[unity::class_method(46, vtable)] fn page_up(&self, is_trigger: bool); // Offset: 0x2461490 Flags: 0
    #[unity::class_method(47, vtable)] fn page_down(&self, is_trigger: bool); // Offset: 0x2461630 Flags: 0
    #[unity::class_method(48, vtable)] fn system_call(&self) -> BasicMenuResult; // Offset: 0x24617D0 Flags: 0
    #[unity::class_method(49, vtable)] fn key_call(&self) -> BasicMenuResult; // Offset: 0x2461870 Flags: 0
    #[unity::class_method(50, vtable)] fn a_call(&self) -> BasicMenuResult; // Offset: 0x2461910 Flags: 0
    #[unity::class_method(51, vtable)] fn b_call(&self) -> BasicMenuResult; // Offset: 0x24619B0 Flags: 0
    #[unity::class_method(52, vtable)] fn x_call(&self) -> BasicMenuResult; // Offset: 0x2461A50 Flags: 0
    #[unity::class_method(53, vtable)] fn y_call(&self) -> BasicMenuResult; // Offset: 0x2461AF0 Flags: 0
    #[unity::class_method(54, vtable)] fn l_call(&self) -> BasicMenuResult; // Offset: 0x2461B90 Flags: 0
    #[unity::class_method(55, vtable)] fn r_call(&self) -> BasicMenuResult; // Offset: 0x2461C30 Flags: 0
    #[unity::class_method(56, vtable)] fn plus_call(&self) -> BasicMenuResult; // Offset: 0x2461CD0 Flags: 0
    #[unity::class_method(57, vtable)] fn minus_call(&self) -> BasicMenuResult; // Offset: 0x2461D70 Flags: 0
    #[unity::class_method(58, vtable)] fn custom_call(&self) -> BasicMenuResult; // Offset: 0x2461E10 Flags: 0
    #[unity::class_method(59, vtable)] fn play_cursor_se(&self); // Offset: 0x2461EB0 Flags: 0
    #[unity::class_method(60, vtable)] fn play_decide_se(&self); // Offset: 0x2461F30 Flags: 0
    #[unity::class_method(61, vtable)] fn play_decide_big_se(&self); // Offset: 0x2461FB0 Flags: 0
    #[unity::class_method(62, vtable)] fn play_cancel_se(&self); // Offset: 0x2462030 Flags: 0
    #[unity::class_method(63, vtable)] fn get_tutorial(&self) -> Option<&'static Il2CppString>; // Offset: 0x24620B0 Flags: 0
}
impl<T> BasicMenuMethods for BasicMenu<T> {}


#[unity::from_offset("App", "BasicMenu", ".ctor")]
fn basicmenu_ctor<P: BasicMenuMethods + ?Sized, T>(
    this: &P,
    menu_item_list: &List<T>,
    menu_content: &BasicMenuContent,
    method_info: OptionalMethod,
);

#[unity::from_offset("App", "BasicMenu", ".ctor")]
fn basicmenu_ctor2<P: BasicMenuMethods + ?Sized, T, M: MenuContent>(
    this: &P,
    menu_item_list: &List<T>,
    menu_content: &M,
    method_info: OptionalMethod,
);

#[unity::from_offset("App", "BasicMenu", "SetTransformAsSubMenu")]
fn basicmenu_set_transform_as_sub_menu<P: BasicMenuMethods + ?Sized, T: BasicMenuMethods + ?Sized>(this: &P, parent: &T, parent_item: &BasicMenuItem, method_info: OptionalMethod);

#[unity::class("", "StatusField")]
#[nested_from_type(BasicMenuClass)]
pub struct BasicMenuStatus{
    pub value: i32,
}


#[unity::class("App", "BasicMenuSelect")]
pub struct BasicMenuSelect {
    pub index: i32,
    pub scroll: i32,
}


pub trait MenuSequence {
    fn bind(parent: &impl Bindable) {
        let proc = ProcInst::instantiate().unwrap();
        let descs = Il2CppArray::from_slice(Self::get_proc_desc(proc)).unwrap();

        proc.create_bind(parent, descs, Self::proc_name());
    }

    fn get_proc_desc(_this: &'static ProcInst) -> Vec<&'static mut ProcDesc> {
        vec![ProcDesc::end()]
    }

    fn proc_name() -> &'static str {
        "MenuSequence"
    }
}


/// The return type for Call methods on classes inheriting from BasicMenuItem.
///
/// Used to play a sound related to the action performed.
///
/// A ``new()`` method is available as a Builder pattern for situations where the result you desire does not have a method.
#[repr(C)]
#[bitfield]
pub struct BasicMenuResult {
    pub close_this: bool,   //1
    pub close_parent: bool, //2
    pub close_all: bool,    //4
    pub delete_this: bool,  //8

    pub delete_parent: bool,    //16
    pub delete_all: bool,   //32
    #[skip]
    __: bool,
    #[skip(getters)]
    pub se_decide: bool,    //128

    #[skip(getters)]
    pub se_decide2: bool,   //256
    #[skip(getters)]
    pub se_cancel: bool,    //512
    #[skip]
    __: bool,

    #[skip(getters)]
    pub se_miss: bool,  //2048
    #[skip(getters)]
    pub se_cursor: bool,    //4096
    pub do_nothing: bool,   //8192
    #[skip]
    padding: B2,
}

impl BasicMenuResult {
    pub fn se_cursor() -> Self {
        Self::new().with_se_cursor(true)
    }
    pub fn se_decide() -> Self {
        Self::new().with_se_decide(true)
    }
    pub fn close_decide() -> Self {
        Self::new().with_close_this(true).with_se_decide(true)
    }
    pub fn se_miss() -> Self {
        Self::new().with_se_miss(true)
    }
    pub fn close_parent_decide() -> Self {
        Self::new().with_close_parent(true).with_se_decide(true)
    }
    pub fn delete_decide() -> Self {
        Self::new().with_delete_this(true).with_se_decide(true)
    }
    pub fn close_cancel() -> Self {
        Self::new().with_se_cancel(true).with_close_this(true)
    }
}

#[repr(i32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BasicMenuItemAttribute {
    Enable = 1,
    Disable = 2,
    Hide = 4,
    Blank = 8,
    Select = 16,
}