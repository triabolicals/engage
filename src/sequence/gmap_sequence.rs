use super::*;
use crate::{gmapspotmanager::gmap_spot_manager_open_next_chapters, proc::Bindable};
use unity::{system::List, prelude::*};
use crate::gamedata::chapter::ChapterData;
use crate::util::get_singleton_proc_instance;

#[repr(C)]
#[unity::class("App", "GmapSequence")]
pub struct GmapSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub is_loaded: bool,
    pub scene_name: Option<&'static Il2CppString>,
    pub scene_mode: i32,
    pub now_spot: &'static GmapSpot,
    pub changing_spot: &'static GmapSpot,
    changing_path: *const u8,
    pub dispos_spot: &'static GmapSpot,
    gmap_camera: *const u8,
    path_controller: *const u8,
    virtual_sphere: *const u8,
    whole_map: *const u8,
    pub map_info: &'static GmapMapInfoContent,

}
impl Bindable for GmapSequence {}

impl GmapSequence {
    pub const HASH: i32 = 971901799;
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
}

#[unity::class("App", "GmapMapInfoContent")]
pub struct GmapMapInfoContent {}

impl GmapMapInfoContent {
    pub fn close(&self) { unsafe { mapinfo_close(self, None); }}
}

#[unity::class("App", "GmapSpot")]
pub struct GmapSpot {
    pub global_flag_name: &'static Il2CppString,
    pub chapters: &'static List<ChapterData>,
}

impl GmapSpot {
    pub fn get_chapter(&self) -> &'static ChapterData { unsafe { gmapspot_get_chapter(self, None) }  }
    pub fn is_completed(&self) -> bool { unsafe { gmapspot_is_completed(self, None) }}
    #[unity::class_method(7)] pub fn get_spot_state(&self) -> GmapSpotState; // Offset: 0x2B37310 Flags: 0
    #[unity::class_method(8)] pub fn set_spot_state(&self, value: GmapSpotState); // Offset: 0x2B44CB0 Flags: 0
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GmapSpotState {
    ReserveHide = 0, // Attr: 17
    Hide = 1, // Attr: 17
    ReserveActive = 2, // Attr: 17
    Active = 3, // Attr: 17
    ReserveCannotEnter = 4, // Attr: 17
    CannotEnter = 5, // Attr: 17
    ReserveBroken = 6, // Attr: 17
    Broken = 7, // Attr: 17
    CanSearch = 8, // Attr: 17
}

#[unity::from_offset("App", "GmapSpot", "get_Chapter")]
fn gmapspot_get_chapter(this: &GmapSpot, method_info: OptionalMethod) -> &'static ChapterData;

#[unity::from_offset("App", "GmapSpot", "IsCompleted")]
fn gmapspot_is_completed(this: &GmapSpot, method_info: OptionalMethod) -> bool; 

#[skyline::from_offset(0x0252a640)]
pub fn mapinfo_close(this: &GmapMapInfoContent, method_info: OptionalMethod); 