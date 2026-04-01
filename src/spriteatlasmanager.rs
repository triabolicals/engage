use unity::engine::Sprite;
use unity::prelude::{Il2CppClassData, Il2CppString, OptionalMethod};
use unity::system::Dictionary;
use crate::gamedata::god::GodData;
use crate::gamedata::person::PersonData;
use crate::resourcemanager::ResourceHandle;

#[unity::class("App", "SpriteAtlasManager")]
pub struct SpriteAtlasManager {
    pub handle: &'static ResourceHandle,
    sprite_atlas: *const u8,
    pub cache_table: &'static mut Dictionary<'static, &'static Il2CppString, &'static Sprite>,
}
impl SpriteAtlasManager {
    pub fn try_get<'a>(&self, name: impl Into<&'a Il2CppString>) -> Option<&'static mut Sprite> {
        unsafe {  sprite_atlas_manager_try_get(self, name.into(), None) }
    }
}

#[unity::class("App", "FaceThumbnail")]
pub struct FaceThumbnail {}

impl FaceThumbnail {
    pub fn get_static_fields() -> &'static FaceThumbnailStaticFields {
        FaceThumbnail::class().get_static_fields::<FaceThumbnailStaticFields>()
    }
    pub fn get_path_person(person: &PersonData) -> Option<&'static Il2CppString> {
        unsafe { facethumb_nail_get_person(person, None) }
    }
    pub fn exists(name: &Il2CppString) -> bool {
        Self::get_static_fields().face_thumb.cache_table.entries.iter().
            any(|x| x.key.is_some_and(|key| key == name))
    }
    pub fn get_god(god: &GodData) -> Option<&'static mut Sprite>{
        unsafe { face_thumbnail_get_god(god, None) }
    }
    pub fn get_from_person(person: &PersonData) -> Option<&'static mut Sprite> {
        unsafe { facethumbnail_get_from_person(person, None) }
    }
}

pub struct FaceThumbnailStaticFields {
    pub path: &'static Il2CppString,
    pub face_thumb: &'static mut SpriteAtlasManager,
}

#[skyline::from_offset(0x02d521d0)]
pub fn facethumb_nail_get_person(person: &PersonData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;
#[skyline::from_offset(0x2d52090)]
fn facethumbnail_get_from_person(person: &PersonData, method_info: OptionalMethod) -> Option<&'static mut Sprite>;

#[skyline::from_offset(0x2d52270)]
fn face_thumbnail_get_god(god_data: &GodData, method_info: OptionalMethod) -> Option<&'static mut Sprite>;
#[unity::from_offset("App", "SpriteAtlasManager", "TryGet")]
fn sprite_atlas_manager_try_get(this: &SpriteAtlasManager, name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut Sprite>;