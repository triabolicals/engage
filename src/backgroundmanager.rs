use unity::prelude::*;
use crate::unityengine::Camera;

#[unity::class("App", "BackgroundManager")]
pub struct BackgroundManager {}

impl BackgroundManager {
    #[unity::class_method(0)] pub fn set_camera(camera: &Camera, enable: bool); // Offset: 0x2122000 Flags: 0
    #[unity::class_method(1)] pub fn set_capture(camera: &Camera, enable: bool); // Offset: 0x21220F0 Flags: 0
    #[unity::class_method(2)] pub fn set_wall_paper(enable: bool); // Offset: 0x2122210 Flags: 0
    #[unity::class_method(3)] pub fn set_blur(enable: bool); // Offset: 0x2122490 Flags: 0
    #[unity::class_method(4)] pub fn set_capture2(enable: bool); // Offset: 0x2122590 Flags: 0
    #[unity::class_method(5)] pub fn get_background() -> i32; // Offset: 0x2122730 Flags: 0
    // #[unity::class_method(6)] pub fn bind(type: BackgroundManagerBindType); // Offset: 0x2122780 Flags: 0
    #[unity::class_method(7)] pub fn bind(); // Offset: 0x2122980 Flags: 0
    #[unity::class_method(8)] pub fn update(); // Offset: 0x21229F0 Flags: 0
    #[unity::class_method(9)] pub fn unbind(); // Offset: 0x2122B00 Flags: 0
    #[unity::class_method(10)] pub fn is_bind() -> bool; // Offset: 0x2122EB0 Flags: 0
    #[unity::class_method(11)] pub fn is_captured() -> bool; // Offset: 0x2122F20 Flags: 0 }
}
