use unity::prelude::{Il2CppClassData, Il2CppString, OptionalMethod};
use crate::unityengine::GameObject;
use unity::il2cpp::object::Array;

pub trait ProcScene: Il2CppClassData + Sized {
    fn load_scene<'a>(&self, scene_name: impl Into<&'a Il2CppString>, mode: LoadSceneMode){ 
        unsafe { proc_load_scene(self, scene_name.into(), mode, None) }
    }
}

#[skyline::from_offset(0x264a140)]
fn proc_load_scene<T: ProcScene>(this: &T, scene_name: &Il2CppString, mode: LoadSceneMode, optional_method: OptionalMethod);

pub struct Scene {
    pub handle: i32,
}
impl Scene {
    pub fn get_root_game_objects(&self) -> &'static mut Array<&'static mut GameObject> {
        unsafe { get_root_game_objects(self, None) }
    }
}
pub struct SceneManager{}

impl SceneManager {
    pub fn get_scene_by_name(name: &Il2CppString) -> Scene { unsafe { scene_manager_get_scene_by_name(name, None) } }
    pub fn set_active_scene(scene: Scene) { unsafe { scene_manager_set_active_scene(scene, None); } }
    pub fn get_active_scene() -> Option<Scene> { unsafe { scene_manager_get_active_scene(None) } }
}
#[repr(i32)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LoadSceneMode {
    Single = 0,
    Additive = 1,
}

#[skyline::from_offset(0x02f8a5f0)]
fn scene_manager_get_scene_by_name(name: &Il2CppString, optional_method: OptionalMethod) -> Scene;

#[skyline::from_offset(0x2f8a500)]
fn scene_manager_set_active_scene(scene: Scene, optional_method: OptionalMethod);

#[skyline::from_offset(0x2f8a410)]
fn scene_manager_get_active_scene(optional_method: OptionalMethod) -> Option<Scene>;

#[skyline::from_offset(0x2f89fe0)]
fn get_root_game_objects(this: &Scene, optional_method: OptionalMethod) -> &'static mut Array<&'static mut GameObject>;