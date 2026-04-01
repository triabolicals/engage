use unity::prelude::*;
use unity::system::List;
use crate::combat::{Character, CharacterAppearance};
use crate::gamedata::accessory::AccessoryData;
use crate::proc::{Bindable, ProcInstFields, ProcSceneSequenceFields};
use crate::scene::ProcScene;
use crate::unit::Unit;
use crate::unityengine::{GameObject, UnityObject};
use crate::util::get_singleton_proc_instance;

#[unity::class("App", "HubAccessoryRoom")]
pub struct HubAccessoryRoom {
    pub proc: ProcSceneSequenceFields,
    pub disable_list: &'static mut List<GameObject>,
    pub return_scene_name: &'static Il2CppString,
    pub mode: i32,
    pub load_count: i32,
    pub camera_pos: Option<&'static mut HubAccessoryRoomCamera>,
    pub last_hash: i32,
    pub character: Option<&'static mut Character>,
    pub loading_appearance: Option<&'static CharacterAppearance>,
    pub loading_chara: Option<&'static mut Character>,
    pub last_pid: &'static mut Il2CppString,
}

impl HubAccessoryRoom {
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
    #[unity::class_method(0)] pub fn get_scene_name(&self) -> &'static Il2CppString; // Offset: 0x2173740 Flags: 0
    #[unity::class_method(1)] pub fn get_return_scene_name(&self) -> &'static Il2CppString; // Offset: 0x2173790 Flags: 0
    #[unity::class_method(2)] pub fn set_unit(base_unit: &Unit, accessory_data: Option<&AccessoryData>, is_delay_load: bool, is_amiibo: bool); // Offset: 0x21737A0 Flags: 0
    #[unity::class_method(3)] pub fn set_view_mode(position: HubAccessoryRoomViewMode); // Offset: 0x2173C70 Flags: 0
    #[unity::class_method(4)] pub fn ctor(&self, shop: HubAccessoryRoomShop); // Offset: 0x2173D50 Flags: 0
    #[unity::class_method(5)] pub fn additive_scene(&self); // Offset: 0x2173E30 Flags: 0
    #[unity::class_method(6)] pub fn open_title(&self); // Offset: 0x2174010 Flags: 0
    #[unity::class_method(7)] pub fn init(&self); // Offset: 0x21741F0 Flags: 0
    #[unity::class_method(8)] pub fn un_additive_scene(&self); // Offset: 0x2174380 Flags: 0
    #[unity::class_method(9)] pub fn exit(&self); // Offset: 0x21743F0 Flags: 0
    #[unity::class_method(10)] pub fn exit_other(&self); // Offset: 0x21747E0 Flags: 0
    #[unity::class_method(11)] pub fn exit_after(&self); // Offset: 0x2174880 Flags: 0
    #[unity::class_method(12)] pub fn is_character_loading(&self) -> bool; // Offset: 0x2174910 Flags: 0
    #[unity::class_method(13)] pub fn main(&self); // Offset: 0x2174A10 Flags: 0
    #[unity::class_method(14)] pub fn get_camera_pos(&self) -> &'static HubAccessoryRoomCamera; // Offset: 0x2174A30 Flags: 0
    #[unity::class_method(15)] pub fn set_camera_pos(&self, value: &HubAccessoryRoomCamera); // Offset: 0x2174A40 Flags: 0
    #[unity::class_method(16)] pub fn set_unit_core(&self, unit: &Unit, target: i64, is_delay_load: bool); // Offset: 0x2173990 Flags: 0
    #[unity::class_method(17)] pub fn load_character(&self, appearance: &CharacterAppearance, pid: &Il2CppString); // Offset: 0x2174BC0 Flags: 0
    #[unity::class_method(18)] pub fn destroy_current_char(&self); // Offset: 0x2174A50 Flags: 0
    #[unity::class_method(19)] pub fn set_view_mode_core(&self, mode: HubAccessoryRoomViewMode); // Offset: 0x2173D40 Flags: 0
    // #[unity::class_method(20)] pub fn on_shutdown(&self); // Offset: 0x2174D00 Flags: 0
    #[unity::class_method(21)] pub fn create_bind<B>(proc: &B, shop: HubAccessoryRoomShop) where B: Bindable; // Offset: 0x2174D70 Flags: 0
}
impl Bindable for HubAccessoryRoom {}
impl ProcScene for HubAccessoryRoom {}
impl AsRef<ProcInstFields> for HubAccessoryRoom {
    fn as_ref(&self) -> &ProcInstFields { &self.proc.parent }
}

impl AsMut<ProcInstFields> for HubAccessoryRoom {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc.parent
    }
}

#[unity::class("App", "HubAccessoryRoomCamera")]
pub struct HubAccessoryRoomCamera {}
impl UnityObject for HubAccessoryRoomCamera {}


#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum HubAccessoryRoomViewMode {
    UnitSelect = 0, // Attr: 17
    AccessorySelect = 1, // Attr: 17
    Preview = 2, // Attr: 17
}

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum HubAccessoryRoomShop {
    Hub = 0, // Attr: 17
    Amiibo = 1, // Attr: 17
}