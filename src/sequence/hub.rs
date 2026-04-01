use crate::hub::{HubLocatorGroup, HubMiniMap, HubPlayerController};
use crate::hub::access::HubAccessManager;
use crate::util::get_singleton_proc_instance;
pub use super::*;

#[unity::class("App", "HubSequence")]
pub struct HubSequence {
    pub proc: ProcSceneSequenceFields,
    pub script_func_name: &'static Il2CppString,
    fast_travel_id: &'static Il2CppString,
    talk_access: &'static (),
    is_background_bind: bool,
    is_key_help: bool,
    is_cave: bool,
    pub scene_name: &'static Il2CppString,
    pub start_name: &'static Il2CppString,
    hub_root: &'static (),
    hub_env: &'static (),
    pub hub_locator: &'static mut HubLocatorGroup,
    pub hub_player_controller: &'static mut HubPlayerController,
}

impl Bindable for HubSequence {}

impl HubSequence {
    pub const HASH: i32 = 570083351;
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
    pub fn gift_get<'a>(&self, reward_id: impl Into<&'a Il2CppString>, message_id: impl Into<&'a Il2CppString>) {
        self.gift_get_(reward_id.into(), message_id.into())
    }
    #[unity::class_method(2)] pub fn get_player(&self) -> Option<&'static mut HubPlayerController>; // Offset: 0x23EB740 Flags: 0
    #[unity::class_method(4)] pub fn get_locator_group(&self) -> Option<&'static mut HubLocatorGroup>; // Offset: 0x23EB760 Flags: 0
    #[unity::class_method(7)] pub fn get_mini_map(&self) -> &'static mut HubMiniMap; // Offset: 0x23EB790 Flags: 0
    #[unity::class_method(18)] pub fn get_current_access_data(&self) -> &'static mut HubAccessManager; // Offset: 0x23EB840 Flags: 0
    #[unity::class_method(113)] pub fn next_chapter(overwrite_chapter: &Il2CppString); // Offset: 0x23F1D00 
    #[unity::class_method(142)] pub fn gift_get_(&self, reward_id: &Il2CppString, message_id: &Il2CppString); // Offset: 0x23F38A0 Flags: 0

    #[unity::class_method(22, vtable)] pub fn get_scene_name(&self) -> &'static Il2CppString; // Offset: 0x264A090 Flags: 2
}