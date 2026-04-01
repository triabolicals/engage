//! Methods and traits related to the [`ProcInst`] system.

use unity::prelude::*;

pub mod desc;
use desc::*;

pub mod inst;
pub mod hash;

pub use inst::*;
pub use hash::*;

#[unity::class("App", "Proc")]
pub struct Proc {}

impl Proc {
    pub fn find_by_name<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static mut ProcInst> { Self::find_by_name_(name.into()) }
    
    pub fn get_root_hi() -> &'static mut ProcInst {
        unsafe { proc_getroothi(None) }
    }
    pub fn get_root_def() -> &'static mut ProcInst {
        unsafe { proc_getrootdef(None) }
    }
    pub fn get_root_low() -> &'static mut ProcInst {
        unsafe { proc_getrootlow(None) }
    }
    pub fn vsync(vsync_mode: i32) -> &'static mut ProcDesc { unsafe { proc_vsync(vsync_mode, None) } }
    pub fn wait_is_loading() -> &'static mut ProcDesc {
        unsafe { proc_waitisloading(None) }
    }
    #[unity::class_method(3)] pub fn find_by_name_(name: &Il2CppString) -> Option<&'static mut ProcInst>; // Offset: 0x281A5B0 Flags: 0
    #[unity::class_method(4)] pub fn kill_by_name_(name: &Il2CppString); // Offset: 0x281A810 Flags: 0
}

/// Trait to simulate inheritance for [`ProcInst`].
/// 
/// A method expecting a `&impl Bindable` or `<P: Bindable>(parent: &P, ...)` will accept any type that inherits from [`ProcInst`].
pub trait Bindable: Il2CppClassData + Sized {
    fn create_bind(&self, parent: &impl Bindable, descs: &'static mut Il2CppArray<&'static mut ProcDesc>, name: impl AsRef<str>) {
        unsafe { procinst_createbind(self, parent, descs, name.as_ref().into(), None) }
    }
    fn create_bind_no_desc(&self, parent: &impl Bindable) { unsafe { procinst_createbind_no_desc(self, parent, None) } }

    #[unity::class_method(13, ProcInst)] fn get_parent(&self) -> Option<&'static mut ProcInst>; // Offset: 0x281EAD0 Flags: 0
    fn get_child(&self) -> Option<&'static mut ProcInst> { unsafe { procinst_get_child(self, None) } }
    fn jump(&self, label: i32) { unsafe { procinst_jump(self, label, None) } }

}

#[unity::from_offset("App", "Proc", "WaitIsLoading")]
fn proc_waitisloading(
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "Vsync")]
fn proc_vsync(
    vsync_mode: i32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "GetRootHi")]
fn proc_getroothi(
    method_info: OptionalMethod,
) -> &'static mut ProcInst;

#[unity::from_offset("App", "Proc", "GetRootDef")]
fn proc_getrootdef(
    method_info: OptionalMethod,
) -> &'static mut ProcInst;

#[unity::from_offset("App", "Proc", "GetRootLow")]
fn proc_getrootlow(
    method_info: OptionalMethod,
) -> &'static mut ProcInst;

#[unity::from_offset("App", "Proc", "Label")]
fn proc_label(
    label: i32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "WaitTime")]
fn proc_wait_time(
    seconds: f32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "Call")]
fn proc_call<D: Delegate>(
    method: &'static mut D,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "WaitWhileTrue")]
fn proc_wait_while_true<T>(
    method: &'static mut ProcBoolMethod<T>,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "WaitWhileFalse")]
fn proc_wait_while_false<T>(
    method: &'static mut ProcBoolMethod<T>,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Proc", "End")]
fn proc_end(
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[skyline::from_offset(0x280a980)]
fn proc_jump_true<T>(
    method: &'static mut ProcBoolMethod<T>,
    label: i32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

/// A structure representing a call to a method that returns nothing.
#[repr(C)]
#[unity::class("App", "ProcVoidMethod")]
pub struct ProcVoidMethod<T: 'static + Bindable> {
    method_ptr: *const u8,
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: Option<&'static T>,
    // MethodInfo
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: *const u8,
    // ...
}

pub trait Delegate { }

impl<T: Bindable> Delegate for ProcVoidFunction<T> { }
impl<T: Bindable> Delegate for ProcVoidMethod<T> { }

impl<T: Bindable> ProcVoidMethod<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate the target.
    pub fn new(
        target: impl Into<Option<&'static T>>,
        method: extern "C" fn(&'static mut T, OptionalMethod),
    ) -> &'static mut ProcVoidMethod<T> {
        ProcVoidMethod::<T>::instantiate().map(|proc| {
            proc.method_ptr = method as _;
            proc.target = target.into();
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        }).unwrap()
    }
}

#[repr(C)]
#[unity::class("App", "ProcBoolMethod")]
pub struct ProcBoolMethod<T: 'static> {
    method_ptr: *const u8,
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: Option<&'static T>,
    // MethodInfo
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: *const u8,
    // ...
}

/// A structure representing a call to a method that returns a bool.
impl<T> ProcBoolMethod<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate the target.
    pub fn new(
        target: impl Into<Option<&'static T>>,
        method: extern "C" fn(&'static mut T, OptionalMethod) -> bool,
    ) -> &'static mut ProcBoolMethod<T> {
        ProcBoolMethod::<T>::instantiate().map(|proc| {
            proc.method_ptr = method as _;
            proc.target = target.into();
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        }).unwrap()
    }
}

#[repr(C)]
#[unity::class("App", "ProcVoidFunction")]
pub struct ProcVoidFunction<T: 'static> {
    method_ptr: extern "C" fn(&'static mut T, OptionalMethod),
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: Option<&'static T>,
    // MethodInfo
    method: *const MethodInfo,
    // ...
}

impl<T> ProcVoidFunction<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate to target.
    pub fn new(
        target: impl Into<Option<&'static T>>,
        method: extern "C" fn(&'static mut T, OptionalMethod),
    ) -> &'static mut ProcVoidFunction<T> {
        ProcVoidFunction::<T>::instantiate().map(|proc| {
            proc.method_ptr = method;
            proc.target = target.into();
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        }).unwrap()
    }
}

#[repr(C)]
pub struct RawValueStack {
    count: i32,
    // ValueTypes array
    values: &'static Il2CppArray<&'static ValueType>,
}

#[repr(C)]
pub struct ValueType;

