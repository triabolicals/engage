pub mod accessory;
pub mod config;
pub mod skill_inheritance;
pub mod mapunitcommand;
pub mod sortie;
use unity::engine::Color;
use unity::prelude::*;
use unity::system::Il2CppString;
use crate::menu::{BasicMenu, BasicMenuItemAttribute, BasicMenuResult};
use crate::tmpro::TextMeshProUGUI;
use crate::unityengine::{GameObject, RectTransform};

/// Represents the base MenuItem from which every other inherits.
///
/// A MenuItem is the class representing things such as an entry in a menu's list.
///
/// You can usually use this instead of a class inheriting from it at the risk of not having default implementations for functions.
#[unity::class("App", "BasicMenuItem")]
pub struct BasicMenuItem {
    pub menu: &'static mut BasicMenu<BasicMenuItem>,
    pub menu_item_content: &'static mut BasicMenuItemContent,
    pub name: &'static Il2CppString,
    pub index: i32,
    pub full_index: i32,
    pub attribute: i32,
    pub cursor_color: Color,
    pub active_text_color: Color,
    pub inactive_text_color: Color,
}
impl MenuItem for BasicMenuItem {}

#[unity::class("App", "BasicMenuItemContent")]
pub struct BasicMenuItemContent {
    parent: *const u8,
    pub menu_item: &'static mut BasicMenuItem,
    pub text_base: Color,
    pub text_blend: Color,
    pub frm_content: &'static mut GameObject,
}
impl<M: MenuItem> MenuItemContent<M> for BasicMenuItemContent {}

/// Interface for Classes that inherit BasicMenuItem
pub trait MenuItem: Il2CppClassData {
    #[unity::class_method(0, BasicMenuItem)] fn ctor_base(&self); // Offset: 0x2455FC0 Flags: 0
    #[unity::class_method(34, BasicMenuItem)] fn on_select_base(&self); // Offset: 0x2466570 Flags: 0
    #[unity::class_method(35, BasicMenuItem)] fn on_deselect_base(&self); // Offset: 0x24666D0 Flags: 0
    #[unity::class_method(36, BasicMenuItem)] fn on_cursor_move_end_base(&self); // Offset: 0x2466780 Flags: 0
    #[unity::class_method(37, BasicMenuItem)] fn on_close_base(&self); // Offset: 0x2466890 Flags: 0
    #[unity::class_method(4,  vtable)] fn get_name(&self) -> &'static Il2CppString; // Offset: 0x2465DB0 Flags: 0
    #[unity::class_method(5,  vtable)] fn get_width(&self) -> f32; // Offset: 0x2465DC0 Flags: 0
    #[unity::class_method(6,  vtable)] fn get_height(&self) -> f32; // Offset: 0x2465E80 Flags: 0
    #[unity::class_method(7,  vtable)] fn set_text_color(&self, color: Color, b_inactive: bool); // Offset: 0x2465F40 Flags: 0
    #[unity::class_method(8,  vtable)] fn build_attribute(&self) -> BasicMenuItemAttribute; // Offset: 0x2466410 Flags: 0
    #[unity::class_method(9,  vtable)] fn tick(&self); // Offset: 0x2466420 Flags: 0
    #[unity::class_method(10, vtable)] fn on_build(&self); // Offset: 0x2466430 Flags: 0
    #[unity::class_method(11, vtable)] fn on_build_menu_item_content(&self); // Offset: 0x2466560 Flags: 0
    #[unity::class_method(12, vtable)] fn on_select(&self); // Offset: 0x2466570 Flags: 0
    #[unity::class_method(13, vtable)] fn on_deselect(&self); // Offset: 0x24666D0 Flags: 0
    #[unity::class_method(14, vtable)] fn on_cursor_move_end(&self); // Offset: 0x2466780 Flags: 0
    #[unity::class_method(15, vtable)] fn on_close(&self); // Offset: 0x2466890 Flags: 0
    #[unity::class_method(16, vtable)] fn system_call(&self) -> BasicMenuResult; // Offset: 0x24668A0 Flags: 0
    #[unity::class_method(17, vtable)] fn key_call(&self) -> BasicMenuResult; // Offset: 0x24668B0 Flags: 0
    #[unity::class_method(18, vtable)] fn a_call(&self) -> BasicMenuResult; // Offset: 0x24668C0 Flags: 0
    #[unity::class_method(19, vtable)] fn b_call(&self) -> BasicMenuResult; // Offset: 0x24668D0 Flags: 0
    #[unity::class_method(20, vtable)] fn x_call(&self) -> BasicMenuResult; // Offset: 0x24668E0 Flags: 0
    #[unity::class_method(21, vtable)] fn y_call(&self) -> BasicMenuResult; // Offset: 0x24668F0 Flags: 0
    #[unity::class_method(22, vtable)] fn l_call(&self) -> BasicMenuResult; // Offset: 0x2466900 Flags: 0
    #[unity::class_method(23, vtable)] fn r_call(&self) -> BasicMenuResult; // Offset: 0x2466910 Flags: 0
    #[unity::class_method(24, vtable)] fn plus_call(&self) -> BasicMenuResult; // Offset: 0x2466920 Flags: 0
    #[unity::class_method(25, vtable)] fn minus_call(&self) -> BasicMenuResult; // Offset: 0x2466930 Flags: 0
    #[unity::class_method(26, vtable)] fn custom_call(&self) -> BasicMenuResult; // Offset: 0x2466940 Flags: 0

    #[unity::class_method(2, BasicMenuItem)]
    fn get_menu_item_content<M>(&self) -> Option<&'static mut M> where M: MenuItemContent<Self>, Self: Sized; // Offset: 0x2465D40 Flags: 0

    #[unity::class_method(23, BasicMenuItem)] fn force_rebuild_layout(&self); // Offset: 0x2466340 Flags: 0
}

/// Interface for Classes that inherit BasicMenuItemContent
pub trait MenuItemContent<M: MenuItem>: Il2CppClassData {
    #[unity::class_method(0,  BasicMenuItemContent)] fn get_menu_item(&self) -> &'static M; // Offset: 0x2466950 Flags: 0
    #[unity::class_method(1,  BasicMenuItemContent)] fn get_name(&self) -> &'static Il2CppString; // Offset: 0x2466960 Flags: 0
    #[unity::class_method(2,  BasicMenuItemContent)] fn get_text(&self) -> &'static Il2CppString; // Offset: 0x2466970 Flags: 0
    #[unity::class_method(3,  BasicMenuItemContent)] fn set_text(&self, name: &Il2CppString); // Offset: 0x2466AA0 Flags: 0
    #[unity::class_method(4,  BasicMenuItemContent)] fn set_text_size(&self, font_size: i32); // Offset: 0x2466D10 Flags: 0
    #[unity::class_method(5,  BasicMenuItemContent)] fn set_text_base_color(&self, color: Color); // Offset: 0x2457550 Flags: 0
    #[unity::class_method(6,  BasicMenuItemContent)] fn set_text_blend_color(&self, color: Color); // Offset: 0x2456A30 Flags: 0
    #[unity::class_method(7,  BasicMenuItemContent)] fn set_frm_content(&self, frm_content: &GameObject); // Offset: 0x2466EE0 Flags: 0
    #[unity::class_method(8,  BasicMenuItemContent)] fn get_child_object(&self, ) -> &'static GameObject; // Offset: 0x2466CA0 Flags: 0
    #[unity::class_method(13, BasicMenuItemContent)] fn set_menu_item(&self, menu_item: &M); // Offset: 0x2465000 Flags: 0
    #[unity::class_method(22, BasicMenuItemContent)] fn force_rebuild_layout(&self); // Offset: 0x24677C0 Flags: 0
    #[unity::class_method(23, BasicMenuItemContent)] fn ctor(&self); // Offset: 0x245AF00 Flags: 0
    // #[unity::class_method(4, vtable)] fn get_text_component2(&self) -> &'static Text; // Offset: 0x2466EF0 Flags: 0
    #[unity::class_method(5,  vtable)] fn get_text_mesh_pro_component(&self) -> &'static TextMeshProUGUI; // Offset: 0x2467000 Flags: 0
    #[unity::class_method(6,  vtable)] fn get_rect_transform(&self) -> &'static RectTransform; // Offset: 0x2467110 Flags: 0
    #[unity::class_method(7,  vtable)] fn update_text_color(&self); // Offset: 0x2467220 Flags: 0
    #[unity::class_method(8,  vtable)] fn build(&self, menu_item: &M); // Offset: 0x2458F60 Flags: 0
    #[unity::class_method(9,  vtable)] fn build_font(&self); // Offset: 0x2467480 Flags: 0
    #[unity::class_method(10, vtable)] fn build_text(&self); // Offset: 0x2467490 Flags: 0
    #[unity::class_method(11, vtable)] fn build_text_color(&self); // Offset: 0x2467510 Flags: 0
    #[unity::class_method(12, vtable)] fn unbind(&self); // Offset: 0x24676F0 Flags: 0
    #[unity::class_method(13, vtable)] fn disable(&self); // Offset: 0x2467740 Flags: 0
    #[unity::class_method(14, vtable)] fn start(&self); // Offset: 0x24677A0 Flags: 0
    #[unity::class_method(15, vtable)] fn update(&self); // Offset: 0x24677B0 Flags: 0
}

impl BasicMenuItem {
    pub fn new() -> &'static mut BasicMenuItem {
        let item = BasicMenuItem::instantiate().unwrap();
        unsafe { basicmenuitem_ctor(item, None); }
        item
    }
    pub fn ctor(&self) { unsafe { basicmenuitem_ctor(self, None); } }

    pub fn new_impl<Methods: BasicMenuItemMethods>() -> &'static mut BasicMenuItem {
        let custom_class = BasicMenuItem::class().clone();
        let item = il2cpp::instantiate_class(&custom_class).unwrap();

        unsafe { basicmenuitem_ctor(item, None); }

        item.get_class_mut()
            .get_virtual_method_mut("GetName")
            .map(|method| method.method_ptr = Methods::get_name as _)
            .unwrap();

        // item
        //     .get_class_mut()
        //     .get_virtual_method_mut("GetHelpText")
        //     .map(|method| method.method_ptr = Methods::get_name as _);

        item.get_class_mut()
            .get_virtual_method_mut("ACall")
            .map(|method| method.method_ptr = Methods::a_call as _)
            .unwrap();

        item.get_class_mut()
            .get_virtual_method_mut("BuildAttribute")
            .map(|method| method.method_ptr = Methods::build_attributes as _)
            .unwrap();

        item
    }
    pub fn is_attribute_disable(&self) -> bool { unsafe { basicmenuitem_is_attribute_disable(self, None) } }
    /// Updates the text in BasicMenuItemContent
    pub fn rebuild_text(&self) {
        unsafe { build_content_text(self.menu_item_content, None); }
    }
    pub fn force_rebuild(&self) { unsafe { basicmenuitem_force_rebuild(self, None); }  }
}
pub trait BasicMenuItemMethods {
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString;

    extern "C" fn get_help_text(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString {
        "".into()
    }

    extern "C" fn a_call(_this: &'static mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::se_decide()
    }

    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::Enable
    }
    extern "C" fn custom_call(_this: &'static mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::new()
    }
}

#[skyline::from_offset(0x2455fc0)]
fn basicmenuitem_ctor(this: &BasicMenuItem, method_info: OptionalMethod);

#[skyline::from_offset(0x2457540)]
fn basicmenuitem_is_attribute_disable(this: &BasicMenuItem, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02467490)]
fn build_content_text(this: &BasicMenuItemContent, method_info: OptionalMethod);

#[unity::from_offset("App", "BasicMenuItem", "ForceRebuildLayout")]
fn basicmenuitem_force_rebuild(this: &BasicMenuItem, method_info: OptionalMethod);
