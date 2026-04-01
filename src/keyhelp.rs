use unity::engine::MonoBehaviorFields;
use unity::prelude::*;
use unity::system::List;
use crate::gamedata::GamedataArray;
use crate::titlebar::KeyHelpButton;
use crate::unityengine::{GameObject, UnityComponent, UnityObject};
use crate::util::get_instance;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KeyHelpType {
    A = 0, // Attr: 17
    B = 1, // Attr: 17
    X = 2, // Attr: 17
    Y = 3, // Attr: 17
    L = 4, // Attr: 17
    R = 5, // Attr: 17
    LR = 6, // Attr: 17
    ZL = 7, // Attr: 17
    ZR = 8, // Attr: 17
    Plus = 9, // Attr: 17
    Minus = 10, // Attr: 17
    Num = 11, // Attr: 17
}

#[unity::class("App", "KeyHelpData")]
pub struct KeyHelpData {
    pub parent: crate::gamedata::StructDataArrayFields,
    pub button_index: i8, // Offset 0x28, Attr: 1
    pub mid: &'static Il2CppString, // Offset 0x30, Attr: 1
}
impl GamedataArray for KeyHelpData {}

#[unity::class("App", "KeyHelpTitleBarController")]
pub struct KeyHelpTitleBarController {
    parent: MonoBehaviorFields,
    pub help_object: &'static List<GameObject>,
}
impl KeyHelpTitleBarController {
    #[unity::class_method(0)] pub fn set_key_help_message(&self, key_help_id: &Il2CppString); // Offset: 0x1BDA5C0 Flags: 0
    #[unity::class_method(1)] pub fn hide(&self); // Offset: 0x1BDA9F0 Flags: 0
    #[unity::class_method(2)] pub fn set_text(&self, game_object: &GameObject, text: &Il2CppString); // Offset: 0x1BDA920 Flags: 0
    pub fn set_text_by_key(&self, key: KeyHelpButton, text: &Il2CppString) {
        let obj = &self.help_object[key as usize];
        obj.set_active2(true);
        self.set_text(obj, text);
    }
    pub fn disable(&self, key: KeyHelpButton) {
        if let Some(obj) = self.help_object.get(key as usize) {
            obj.set_active2(false);
        }
    }
    pub fn enable(&self, key: KeyHelpButton) {
        if let Some(obj) = self.help_object.get(key as usize) {
            obj.set_active2(true);
        }
    }
}
impl UnityComponent for KeyHelpTitleBarController {}
impl UnityObject for KeyHelpTitleBarController {}

#[unity::class("App", "KeyHelp")]
pub struct KeyHelp {
    parent: [u8; 0x10],
    pub ui: &'static KeyHelpUI,
}
impl KeyHelp {
    pub fn get_instance() -> &'static Self { get_instance::<KeyHelp>() }
    pub fn get_key_help_controller() -> Option<&'static mut KeyHelpController> {
        Self::get_instance().ui.game_object.get_component_by_type::<KeyHelpController>()
    }
    #[unity::class_method(0)] pub fn is_creating() -> bool; // Offset: 0x1BD8FE0 Flags: 0
    #[unity::class_method(1)] pub fn set_visible(is_visible: bool); // Offset: 0x1BD90A0 Flags: 0
    #[unity::class_method(2)] pub fn add_by_type(ty: i32, text: &Il2CppString); // Offset: 0x1BD9170 Flags: 0
    #[unity::class_method(3)] pub fn add_by_key_help_id(key_help_id: &Il2CppString); // Offset: 0x1BD9250 Flags: 0
    #[unity::class_method(4)] pub fn clear(); // Offset: 0x1BD9320 Flags: 0
}

#[unity::class("", "UI")]
#[nested_from_type(KeyHelp)]
pub struct KeyHelpUI {
    prefab_handle: u64,
    pub game_object: &'static GameObject, // Offset 0x18, Attr: 1
    elements: u64,
    // m_elements: Dictionary<KeyHelpType, &UIElement>, // Offset 0x20, Attr: 1
    pub m_index: i32, // Offset 0x28, Attr: 1
}

#[unity::class("App", "KeyHelpController")]
pub struct KeyHelpController {
    parent: MonoBehaviorFields,
    pub help_object: &'static List<GameObject>, // Offset 0x18, Attr: 1
    pub interval: f32, // Offset 0x20, Attr: 1
}

impl KeyHelpController {
    #[unity::class_method(0)] pub fn set_key_help_message(&self, key_help_id: &Il2CppString); // Offset: 0x1BD9540 Flags: 0
    #[unity::class_method(1)] pub fn hide(&self); // Offset: 0x1BD9D30 Flags: 0
    #[unity::class_method(2)] pub fn set_text(&self, game_object: &GameObject, text: &Il2CppString); // Offset: 0x1BD9C60 Flags: 0
    pub fn set_text_by_key(&self, key: KeyHelpButton, text: &Il2CppString) {
        let obj = &self.help_object[key as usize];
        obj.set_active2(true);
        self.set_text(obj, text);
    }
    pub fn disable(&self, key: KeyHelpButton) {
        if let Some(obj) = self.help_object.get(key as usize) {
            obj.set_active2(false);
        }
    }
    pub fn enable(&self, key: KeyHelpButton) {
        if let Some(obj) = self.help_object.get(key as usize) {
            obj.set_active2(true);
        }
    }
}
impl UnityComponent for KeyHelpController {}