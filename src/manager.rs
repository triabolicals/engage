//! List of small classes that have manager in the name
use unity::prelude::*;
use crate::mess::Mess;
use crate::unityengine::Camera;

#[repr(C)]
#[unity::class("App", "LoadingManager")]
pub struct LoadingManager { }

impl LoadingManager {
    #[unity::class_method(0)]pub fn bind(); // Offset: 0x1BEE4E0 Flags: 0
    #[unity::class_method(1)] pub fn bind2(mode: LoadingManagerModes); // Offset: 0x1BEE550 Flags: 0
    #[unity::class_method(3)]pub fn update(); // Offset: 0x1BEE750 Flags: 0
    #[unity::class_method(4)]pub fn unbind(); // Offset: 0x1BEE830 Flags: 0
    #[unity::class_method(5)]pub fn is_bind() -> bool; // Offset: 0x1BEE8F0 Flags: 0
    #[unity::class_method(6)]pub fn try_fade_bind(); // Offset: 0x1BEE960 Flags: 0
}
#[unity::class("App", "NoticeManager")]
pub struct NoticeManager {}

impl NoticeManager {
    #[unity::class_method(0)] pub fn add(kind: NoticeManagerKinds, text: &Il2CppString); // Offset: 0x1F1B540 Flags: 0
    #[unity::class_method(0)] pub fn add_by_mid(kind: NoticeManagerKinds, text: &Il2CppString); // Offset: 0x1F1B540 Flags: 0
    pub fn add_facility<'a, S: Into<&'a Il2CppString>>(message: S) {
        Self::add(NoticeManagerKinds::Facility, message.into());
    }
    pub fn add_facility_by_mid<'a, S: Into<&'a Il2CppString>>(message: S) {
        Self::add(NoticeManagerKinds::Facility, Mess::get(message));
    }
}

#[unity::class("App", "BackgroundManager")]
pub struct BackgroundManager {}

impl BackgroundManager {
    #[unity::class_method(0)] pub fn set_camera(camera: &Camera, enable: bool); // Offset: 0x2122000 Flags: 0
    #[unity::class_method(1)] pub fn set_capture(camera: &Camera, enable: bool); // Offset: 0x21220F0 Flags: 0
    #[unity::class_method(2)] pub fn set_wall_paper(enable: bool); // Offset: 0x2122210 Flags: 0
    #[unity::class_method(3)] pub fn set_blur(enable: bool); // Offset: 0x2122490 Flags: 0
    #[unity::class_method(4)] pub fn set_capture2(enable: bool); // Offset: 0x2122590 Flags: 0
    #[unity::class_method(5)] pub fn get_background() -> i32; // Offset: 0x2122730 Flags: 0
    #[unity::class_method(7)] pub fn bind(); // Offset: 0x2122980 Flags: 0
    #[unity::class_method(8)] pub fn update(); // Offset: 0x21229F0 Flags: 0
    #[unity::class_method(9)] pub fn unbind(); // Offset: 0x2122B00 Flags: 0
    #[unity::class_method(10)] pub fn is_bind() -> bool; // Offset: 0x2122EB0 Flags: 0
    #[unity::class_method(11)] pub fn is_captured() -> bool; // Offset: 0x2122F20 Flags: 0 }
}
#[unity::class("App", "RenderManager")]
pub struct RenderManager { }

impl RenderManager {
    #[unity::class_method(0)] pub fn push_render_scale(name: &Il2CppString); // Offset: 0x2011800 Flags: 0
    #[unity::class_method(1)] pub fn push_render_scale2(scale: f32); // Offset: 0x20119D0 Flags: 0
    #[unity::class_method(2)] pub fn push_render_scale3(name: &Il2CppString, name_on_gpu_save: &Il2CppString); // Offset: 0x2011C00 Flags: 0
    #[unity::class_method(3)] pub fn push_render_scale4(scale: f32, scale_on_gpu_save: f32); // Offset: 0x2011D90 Flags: 0
    #[unity::class_method(4)] pub fn pop_render_scale(); // Offset: 0x2011EA0 Flags: 0
    #[unity::class_method(5)] pub fn push_lod_bias(lod_bias: f32); // Offset: 0x2011F90 Flags: 0
    #[unity::class_method(6)] pub fn push_lod_bias2(lod_bias: f32, lod_bias_on_gpu_saved: f32); // Offset: 0x2012000 Flags: 0
    #[unity::class_method(7)] pub fn pop_lod_bias(); // Offset: 0x2012200 Flags: 0
    #[unity::class_method(8)] pub fn push_cross_fade_animation_duration(duration: f32); // Offset: 0x20122F0 Flags: 0
    #[unity::class_method(9)] pub fn pop_cross_fade_animation_duration(); // Offset: 0x20124C0 Flags: 0
    #[unity::class_method(10)] pub fn push_custom_blur(name: &Il2CppString); // Offset: 0x20125F0 Flags: 0
    #[unity::class_method(11)] pub fn push_custom_blur2(blur: i32); // Offset: 0x20127B0 Flags: 0
    #[unity::class_method(12)] pub fn pop_custom_blur(); // Offset: 0x2012950 Flags: 0
    #[unity::class_method(13)] pub fn push_color_rate(name: &Il2CppString); // Offset: 0x2012A10 Flags: 0
    #[unity::class_method(14)] pub fn push_color_rate2(rate: f32); // Offset: 0x2012B10 Flags: 0
    #[unity::class_method(15)] pub fn pop_color_rate(); // Offset: 0x2012CB0 Flags: 0
    #[unity::class_method(16)] pub fn reset(); // Offset: 0x2012D70 Flags: 0
    #[unity::class_method(17)] pub fn update_gpu_performance_mode(); // Offset: 0x20130E0 Flags: 0
    #[unity::class_method(18)] pub fn get_float(name: &Il2CppString, def: f32) -> f32; // Offset: 0x2011900 Flags: 0
    #[unity::class_method(19)] pub fn get_int(name: &Il2CppString, def: i32) -> i32; // Offset: 0x20126F0 Flags: 0
    // #[unity::class_method(20, generic)] pub fn get_stack_peek<T>(stack: Stack<T>, def: T) -> T; // Offset: -1 Flags: 1
    #[unity::class_method(21)] pub fn commit_scale(); // Offset: 0x2011AE0 Flags: 0
    #[unity::class_method(22)] pub fn commit_lod_bias(); // Offset: 0x2012110 Flags: 0
    #[unity::class_method(23)] pub fn commit_blur(); // Offset: 0x2012880 Flags: 0
    #[unity::class_method(24)] pub fn commit_color(); // Offset: 0x2012FE0 Flags: 0
    #[unity::class_method(25)] pub fn commit_color_rate(); // Offset: 0x2012BE0 Flags: 0
    #[unity::class_method(26)] pub fn commit_cross_fade_animation_duration(); // Offset: 0x2012430 Flags: 0
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LoadingManagerModes {
    None = 0, // Attr: 17
    Map = 1, // Attr: 17
    Hub = 2, // Attr: 17
    Gmap = 3, // Attr: 17
    Cook = 4, // Attr: 17
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NoticeManagerKinds {
    None = 0, // Attr: 17
    Facility = 1, // Attr: 17
    Kizuna = 2, // Attr: 17
    Tutorial = 3, // Attr: 17
    Notebook = 4, // Attr: 17
    RingList = 5, // Attr: 17
    Num = 6, // Attr: 17
}