use unity::engine::Color;
use unity::prelude::*;
use crate::unityengine::UnityComponent;

/// You can usually use this instead of a class inheriting from it at the risk of missing fields and methods.
#[unity::class("TMPro", "TextMeshProUGUI")]
pub struct TextMeshProUGUI {}

impl TextMeshProUGUI {
    pub fn set_text(&self, source_text: &Il2CppString, sync_text_input_box: bool) {
        unsafe { tmptext_settext(self, source_text, sync_text_input_box, None) };
    }
    pub fn get_text(&self) -> &'static Il2CppString {
        unsafe { tmptext_gettext(self, None)}
    }
    pub fn set_color(&self, color: Color) {
        let set_color_method = unsafe { std::mem::transmute::<_, fn(&Self, f32, f32, f32, f32, &MethodInfo)>(self.klass.get_vtable()[23].method_ptr) };
        set_color_method(self, color.r, color.g, color.b, color.a, self.klass.get_vtable()[23].method_info);
    }
}

impl UnityComponent for TextMeshProUGUI {}

#[skyline::from_offset(0x2837690)]
fn tmptext_settext(this: &TextMeshProUGUI, source_text: &Il2CppString, sync_text_input_box: bool, method_info: OptionalMethod);

#[skyline::from_offset(0x028316d0)]
fn tmptext_gettext(this: &TextMeshProUGUI, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x02837720)]
fn tmp_text_set_text_with_arg(this: &TextMeshProUGUI, source_text: &Il2CppString, arg: f32, method_info: OptionalMethod);