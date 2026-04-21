use unity::il2cpp::object::Array;
use unity::prelude::*;
use crate::gamedata::chapter::ChapterData;
use crate::sequence::gmap_sequence::GmapSpot;
use crate::util::try_get_instance;

#[unity::class("App", "GmapSpotManager")]
pub struct GmapSpotManager{
    parent: [u8; 0x10],
    pub spots: &'static Array<&'static mut GmapSpot>,
}

impl GmapSpotManager {
    pub fn get_instance() -> Option<&'static mut GmapSpotManager> { try_get_instance::<Self>() }
    pub fn set_state(cid: impl AsRef<str>, state: i32) {
        unsafe { gmap_spot_manager_set_state(cid.as_ref().into(), state, None) }
    }
    pub fn find_spot_cid<'a>(cid: impl Into<&'a Il2CppString>) -> Option<&'static mut GmapSpot> {
        Self::get_instance().and_then(|instance| instance.find_spot_by_cid(cid.into()))
    }
    #[unity::class_method(29)] pub fn open_next_chapters(chapter: &ChapterData); // Offset: 0x2B49AC0 Flags: 0
    #[unity::class_method(7)] pub fn find_spot_by_cid(&self, cid: &Il2CppString) -> Option<&'static mut GmapSpot>; // Offset: 0x2B48990 Flags: 0
}

#[unity::from_offset("App", "GmapSpotManager", "SetState")]
fn gmap_spot_manager_set_state(cid: &Il2CppString, state: i32, method_info: OptionalMethod);
