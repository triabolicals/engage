use unity::{prelude::*, system::List};
use crate::unityengine::UnityComponent;

#[unity::class("App", "PhotographCameraController")]
pub struct PhotographCameraController{
    pub enable: bool,
    pub is_photograph_mode: bool,
    pub parameter_list: &'static mut List<PhotographCameraParameter>, // Offset 0x18, Attr: 1
    pub current_parameter: &'static PhotographCameraParameter, // Offset 0x20, Attr: 1
}
#[unity::class("App", "PhotographCameraParameter")]
pub struct PhotographCameraParameter {}
impl UnityComponent for PhotographCameraParameter {}

impl PhotographCameraController{
    #[unity::class_method(1)] pub fn enable(&self); // Offset: 0x2688620 Flags: 0
    #[unity::class_method(2)] pub fn disable(&self); // Offset: 0x2688630 Flags: 0
    #[unity::class_method(3)] pub fn set_is_photograph_mode(&self, is_photograph_mode: bool); // Offset: 0x2688700 Flags: 0
    #[unity::class_method(4)] pub fn update(&self); // Offset: 0x26887E0 Flags: 0
    #[unity::class_method(5)] pub fn set_camera_parameter(&self); // Offset: 0x26883E0 Flags: 0
}