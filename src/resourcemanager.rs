use unity::prelude::*;
use unity::system::action::{Action, Action1};
use unity::system::Dictionary;
use crate::unityengine::{GameObject, Transform};

#[unity::class("App", "ResourceManager")]
pub struct ResourceManager {}

#[repr(C)]
pub struct ResourceManagerStaticFields {
    junk: [u8; 0x40],
    pub files: &'static Dictionary<'static, &'static Il2CppString, bool>,

}
impl ResourceManager {
    pub fn file_exist<'a>(path: impl Into<&'a Il2CppString>) -> bool {
        unsafe { resource_manager_file_exist(path.into(), None) }
    }
    pub fn is_loading() -> bool { unsafe { is_loading(None) } }

    pub fn instantiate<'a>(path: impl Into<&'a Il2CppString>, parent_transform: Option<&Transform>) -> Option<&'static GameObject>{
        unsafe {
            resource_manager_instantiate_with_transform(path.into(), parent_transform, None)
        }
    }
    pub fn instantiate2<'a>(path: impl Into<&'a Il2CppString>, parent: &GameObject) -> Option<&'static GameObject> {
        Self::instantiate2_(path.into(), parent)
    }

    #[unity::class_method(22)] pub fn instantiate2_(path: &Il2CppString, parent: &GameObject) -> Option<&'static GameObject>; // Offset: 0x1FFE9A0 Flags: 0

    #[unity::class_method(17, generic)] pub fn load_global_async<T>(path: &Il2CppString, completed: Option<&'static Action>) where T: Il2CppClassData; // Offset: -1 Flags: 1
    #[unity::class_method(18)] pub fn release_global(path: &Il2CppString); // Offset: 0x1FFF030 Flags: 0
}



#[unity::class("App", "ResourceHandle")]
pub struct ResourceHandle {
    pub path: &'static Il2CppString,
    handle: [u8; 0x18],
}

#[unity::class("App", "ResourceGameObject")]
pub struct ResourceGameObject { }

impl ResourceGameObject {
    #[unity::class_method(0)] pub fn ctor(&self); // Offset: 0x2013AE0 Flags: 0
    #[unity::class_method(3, TResourceHandle)] pub fn load_async(&self, path: &Il2CppString, completed: Option<&'static Action1<GameObject>>);
    #[unity::class_method(2, TResourceHandle)] pub fn get_asset(&self) -> Option<&'static GameObject>;
    /*
    pub fn load_async(&self, path: &Il2CppString, completed: Option<&'static Action1<GameObject>>) {
        let t_resource_class = get_generic_class!(TResourceHandle<GameObject>).unwrap();
        let method = t_resource_class.get_methods()[3];
        let fn_call = unsafe { std::mem::transmute::<_, fn(&Self, &Il2CppString, Option<&'static Action1<GameObject>>, &MethodInfo)> (method.method_ptr) };
        fn_call(self, path, completed, method);
    }
    
     */
    #[unity::class_method(8, ResourceHandle)] pub fn release(&self); // Offset: 0x2013BF0 Flags: 0
}

#[unity::class("App", "TResourceHandle`1")]
pub struct TResourceHandle { }

impl TResourceHandle {
    #[unity::class_method(2)] pub fn get_asset(&self) -> Option<&'static GameObject>; // Offset: 0xFFFFFFFFFFFFFFFF Flags: 0
    #[unity::class_method(3)] pub fn load_async(&self, path: &Il2CppString, completed: OptionalMethod); // Offset: 0xFFFFFFFFFFFFFFFF Flags: 0
}

#[unity::from_offset("App", "ResourceManager", "IsLoading")]
pub fn is_loading(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x020169f0)]
fn resource_manager_file_exist(path: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x20167d0)]
fn resource_manager_instantiate_with_transform(path: &Il2CppString, x: Option<&Transform>, method_info: OptionalMethod) -> Option<&'static GameObject>;