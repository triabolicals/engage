//! Structures representing a singular entry from the gamedata files in memory.

use std::ops::Deref;
use unity::{prelude::*, system::{List, ListFields}};
use unity::il2cpp::object::Array;

pub mod accessory;
pub mod person;
pub mod skill;
pub mod job;
pub mod dispos;
pub mod item;
pub mod cook;
pub mod animal;
pub mod god;
pub mod ring;
pub mod assettable;
pub mod terrain;
pub mod shop;
pub mod ai;
pub mod achieve;
pub mod music;
pub mod chapter;
pub mod hub;
pub mod reward;
mod anim_db;

pub use god::GodData;
pub use job::JobData;
pub use person::PersonData;
pub use chapter::ChapterData;
pub use item::ItemData;

#[unity::class("App", "StructBase")]
pub struct StructBase {
    pub index: i32,
    pub hash: i32,
    pub key: &'static Il2CppString,
}


#[unity::class("App", "StructTemplate`1")]
#[static_fields(StructTemplateStaticFields)]
pub struct StructTemplate {
    pub parent: StructBaseFields,
}
#[repr(C)]
pub struct StructTemplateStaticFields {
    header: u64,
    pub dictionary: &'static mut StructDictionary,
}

#[unity::class("App", "StructData`1")] pub struct StructData { }

#[derive(Clone, Copy)]
pub struct StructDataStaticFields<T: 'static> {
    pub s_list: &'static StructList<T>,
    pub loaded: bool,
}

#[unity::class("App", "StructList<`1>")]
pub struct StructList<T: 'static> {
    pub list: ListFields<T>,
}

impl<T> Deref for StructListFields<T> {
    type Target = ListFields<T>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
use std::ops::DerefMut;
use unity::system::{Dictionary, List2Fields, ListVirtual};
use crate::gamedata::skill::{SkillArray, SkillData};
use crate::ut::Ut;

impl<T> DerefMut for StructListFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl<T> StructList<T> {
    pub fn add(&mut self, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Add").unwrap();
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };

        add(self, element, method.method_info);
    }
    pub fn insert(&mut self, index: i32, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Insert").unwrap();
        let insert = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, i32, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };
        insert(self, index, element, method.method_info);
    }
}

impl<T> StructListFields<T> {
    pub fn len(&self) -> usize {
        self.size as _
    }
    pub fn capacity(&self) -> usize {
        self.items.len() as _
    }
}



#[unity::class("App", "StructDictionary`1")]
pub struct StructDictionary {
    pub key_list: &'static mut List<Il2CppString>,
    pub index_key: &'static mut Dictionary<'static, &'static Il2CppString, i32>,
    pub hash_key: &'static mut Dictionary<'static, i32, i32>,
}
impl StructDictionary {
    /*
    #[unity::class_method(0)] pub fn clear(&self); 
    #[unity::class_method(1)] pub fn get_public_names(&self) -> &'static Array<String>; 
    #[unity::class_method(2)] pub fn get_key(&self, index: i32) -> &'static Il2CppString; 
    #[unity::class_method(3)] pub fn get_index(&self, key: &Il2CppString) -> i32; 
    #[unity::class_method(4)] pub fn get_index_from_hash(&self, hash: i32) -> i32; 

     */
    #[unity::class_method(5)] pub fn is_exist(&self) -> bool; 
    #[unity::class_method(6)] pub fn key_exists(&self, key: &Il2CppString) -> bool; 
    // #[unity::class_method(7)] pub fn prefixless(key: &Il2CppString) -> &'static Il2CppString; 
    #[unity::class_method(8)] pub fn add(&self, key: &Il2CppString, index: i32); 
    #[unity::class_method(9)] pub fn add_with_hash(&self, key: &Il2CppString, index: i32, hash: i32); 
    // #[unity::class_method(10)] pub fn get_list(&self) -> List<&'static Il2CppString>; 
}

#[unity::class("App", "WeaponMask")]
pub struct WeaponMask {
    pub value: i32,
}

pub trait Gamedata: Il2CppClassData + Sized {
    fn get<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static Self> { Self::get_(name.into()) }
    fn get_mut<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static mut Self> { Self::get_mut_(name.into()) }
    fn get_index<'a>(name: impl Into<&'a Il2CppString>) -> i32 { Self::get_index_(name.into()) }

    #[unity::class_method("Load")] fn load();

    #[unity::class_method(0, StructData)] fn add_public_label(instance: &Self);
    #[unity::class_method(4, StructData)] fn completed();
    #[unity::class_method(5, StructData)] fn unload();
    #[unity::class_method(6, StructData)] fn get_(name: &Il2CppString) -> Option<&'static Self>;
    #[unity::class_method(6, StructData)] fn get_mut_(name: &Il2CppString) -> Option<&'static mut Self>;
    #[unity::class_method(9, StructData)] fn try_index_get(index: i32) -> Option<&'static Self>;
    #[unity::class_method(9, StructData)] fn try_index_get_mut(index: i32) -> Option<&'static mut Self>;
    #[unity::class_method(10, StructData)] fn try_get_hash(hash: i32) -> Option<&'static Self>;
    #[unity::class_method(10, StructData)] fn try_get_hash_mut(hash: i32) -> Option<&'static mut Self>;
    #[unity::class_method(18, StructData)] fn get_list() -> Option<&'static StructList<Self>>;
    #[unity::class_method(18, StructData)] fn get_list_mut() -> Option<&'static mut StructList<Self>>;
    #[unity::class_method(14, StructData)] fn get_index_(name: &Il2CppString) -> i32;
    #[unity::class_method(17, StructData)] fn get_count() -> i32;
    fn get_dictionary() -> &'static mut StructDictionary {
        let klass = get_generic_class!(StructTemplate<Self>).unwrap();
        klass.get_static_fields_mut::<StructTemplateStaticFields>().dictionary
    }
    fn add(new: &'static mut Self){
        let dictionary = Self::get_dictionary();
        let struct_base = unsafe { std::mem::transmute::<&mut Self, &mut StructBase>(new) };
        if !dictionary.key_exists(struct_base.key){
            let new_index = Self::get_count();
            let hash = Ut::hash_fnv_1_string(struct_base.key);
            struct_base.hash = hash;
            struct_base.index = new_index;
            Self::add_public_label(new);
            if let Some(list) = Self::get_list_mut() { list.add(new); }
        }
    }
    #[unity::class_method(4, vtable)] fn on_build(&self);
    #[unity::class_method(5, vtable)] fn on_completed(&self);
    #[unity::class_method(6, vtable)] fn on_completed_end(&self);
    #[unity::class_method(7, vtable)] fn on_release(&self);
}
/// Struct for various gamedata that are stored in different arrays/groups
/// Examples: DisposData, GodGrowthData, ShopData,
#[unity::class("App", "StructDataArray`1")]
pub struct StructDataArray {
    pub parent: StructBaseFields,
    pub array_name: &'static Il2CppString,
}

/// List that contains all the arrays/groups of the StructDataArray gamedata
#[unity::class("App", "StructArrayList`1")]
pub struct StructArrayList<T: 'static> {
    pub parent: List2Fields<&'static mut StructDataArrayList<T>>,
}

impl<T> Deref for StructArrayListFields<T> {
    type Target = List2Fields<&'static mut StructDataArrayList<T>>;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl<T> DerefMut for StructArrayListFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}


#[unity::class("App", "StructDataArrayList`1")] pub struct StructDataArrayListClass {}

/// List that stores all StructDataArray belong to a single array/group
/// Example: "Player" group in DisposData
#[unity::class("App", "StructDataArrayList`1")]
pub struct StructDataArrayList<T: 'static>  {
    pub parent: StructListFields<T>,
    pub array_name: &'static Il2CppString,
    pub array_hash: i32, 
}

impl<T> Deref for StructDataArrayListFields<T> {
    type Target = ListFields<T>;
    fn deref(&self) -> &Self::Target {
        &self.parent.list
    }
}
impl<T> DerefMut for StructDataArrayListFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent.list
    }
}
impl<T: Il2CppClassData + Sized> StructDataArrayList<T> {
    pub fn add(&mut self, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Add").unwrap();
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };
        add(self, element, method.method_info);
    }
    pub fn insert(&mut self, index: i32, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Insert").unwrap();
        let insert = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, i32, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };
        insert(self, index, element, method.method_info);
    }
    pub fn new(name: &Il2CppString) -> &'static mut Self {
        let klass = get_generic_class!(StructDataArrayListClass<T>).unwrap();
        let list = klass.instantiate_as::<Self>().unwrap();
        list.ctor(name);
        list
    }
    // This assumes a filled out class
    #[unity::class_method(0)] pub fn ctor(&self, name: &Il2CppString);
}
//Making sure len() returns the size of the StructList instead of it's capacity 
impl<T> StructDataArrayListFields<T> {
    pub fn len(&self) -> usize { self.size as _ }
    pub fn capacity(&self) -> usize { self.items.len() as _ }
}

pub trait GamedataArray: Il2CppClassData + Sized {
    fn try_get_mut<'a>(array_name: impl Into<&'a Il2CppString>) -> Option<&'static mut StructDataArrayList<Self>>{
        Self::try_get_mut(array_name.into())
    }
    #[unity::class_method("Load")] fn load();
    #[unity::class_method("OnCompletedEnd")] fn on_completed_end(&self);
    #[unity::class_method("OnCompleted")] fn on_completed(&self);
    #[unity::class_method("OnBuild")] fn on_build(&self);
    #[unity::class_method(0, StructDataArray)] fn get_array_name(&self) -> &'static Il2CppString; 
    #[unity::class_method(1, StructDataArray)] fn set_array_name(&self, value: &Il2CppString); 
    // #[unity::class_method(2, StructDataArray)] fn get_array_list() -> &'static StructArrayList<Self>; 
    #[unity::class_method(7, StructDataArray)] fn add_array_list(list: &StructDataArrayList<Self>); 
    #[unity::class_method(8, StructDataArray)] fn completed(); 
    #[unity::class_method(9, StructDataArray)] fn unload(); 
    // #[unity::class_method(10)] fn get(name: &Il2CppString) -> StructDataArrayList<T>; 
    #[unity::class_method(12, StructDataArray)] fn try_get_mut_(name: &Il2CppString) -> Option<&'static mut StructDataArrayList<Self>>; 
    #[unity::class_method(14, StructDataArray)] fn try_get_from_hash(array_hash: i32) -> Option<&'static StructDataArrayList<Self>>; 
    #[unity::class_method(17, StructDataArray)] fn get_count() -> i32; 
    #[unity::class_method(18, StructDataArray)] fn get_list() -> Option<&'static StructArrayList<Self>>; 
    #[unity::class_method(18, StructDataArray)] fn get_list_mut() -> Option<&'static mut StructArrayList<Self>>; 

    /*
    fn get_list_mut() -> Option<&'static mut List<StructDataArrayList<Self>>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static mut List<StructDataArrayList<Self>>>>(
                method.unwrap().method_ptr,
            )
        };
        get_list(method.unwrap())
    }
    fn get_list() -> Option<&'static List<StructDataArrayList<Self>>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static List<StructDataArrayList<Self>>>>(
                method.unwrap().method_ptr,
            )
        };
        get_list(method.unwrap())
    }
    fn try_get_mut<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static mut StructDataArrayList<Self>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("TryGet")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("TryGet")));
        }
        if method.is_none() { return None; }
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static mut StructDataArrayList<Self>>> (
                method.unwrap().method_ptr,
            )
        };
        get(name.into(), method.unwrap())
    }
    fn unload() {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        }
        if method.is_none() { return; }
        let unload = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> ()> ( method.unwrap().method_ptr, )
        };
        unload(method.unwrap());
    }
    fn load() {
        let method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("Load")));
        if method.is_none() { return; }
        let load = unsafe { std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> ()> ( method.unwrap().method_ptr, )  };
        load(method.unwrap());
    }

    fn on_completed(&self) {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompleted")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompleted")));
        }
        if method.is_none() { return; }
        let fxn = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        fxn(self, method.unwrap());
    }
    fn on_completed_end(&self) {
        let mut method = Self::class().get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompletedEnd")));
        if method.is_none() {
            method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("OnCompletedEnd")));
        }
        if method.is_none() { return; }
        let fxn = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        fxn(self, method.unwrap());
    }

     */
}

