//! Methods and traits related to the [`ProcInst`] system.

use unity::prelude::*;
use unity::system::action::SystemDelegate;

pub mod desc;
use desc::*;

pub mod inst;
pub mod hash;

pub use inst::*;
pub use hash::*;

#[repr(i32)]
pub enum VsyncMode {
    Normal = 0,
    Slow = 1,
    Fast = 2,
}

#[unity::class("App", "Proc")] pub struct Proc {}

impl Proc {
    pub fn find_by_name<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static mut ProcInst> { Self::find_by_name_(name.into()) }
    pub fn kill_by_name<'a>(name: impl Into<&'a Il2CppString>) { Self::kill_by_name_(name.into()) }
    // #[unity::class_method(6, generic)] pub fn find_by_class<T>() -> Option<&'static mut T> where T: Bindable; // Offset: -1 Flags: 1
    #[unity::class_method(3)] pub fn find_by_name_(name: &Il2CppString) -> Option<&'static mut ProcInst>; // Offset: 0x281A5B0 Flags: 0
    #[unity::class_method(4)] pub fn kill_by_name_(name: &Il2CppString); // Offset: 0x281A810 Flags: 0
    #[unity::class_method(10)] pub fn get_root_hi() -> &'static mut ProcInst; // Offset: 0x281AB50 Flags: 0
    #[unity::class_method(11)] pub fn get_root_def() -> &'static mut ProcInst; // Offset: 0x281ABD0 Flags: 0
    #[unity::class_method(12)] pub fn get_root_low() -> &'static mut ProcInst; // Offset: 0x281AC50 Flags: 0
    #[unity::class_method(13)] pub fn end() -> &'static mut ProcDesc; // Offset: 0x280AA50 Flags: 0
    #[unity::class_method(15)] pub fn jump(label: i32) -> &'static mut ProcDesc; // Offset: 0x281AD90 Flags: 0
    #[unity::class_method(17)] pub fn label(label: i32) -> &'static mut ProcDesc; // Offset: 0x281AEB0 Flags: 0
    #[unity::class_method(34)] pub fn wait_time(second: f32) -> &'static mut ProcDesc; // Offset: 0x281B7C0 Flags: 0
    #[unity::class_method(51)] pub fn vsync(mode: VsyncMode) -> &'static mut ProcDesc; // Offset: 0x281C2F0 Flags: 0
    #[unity::class_method(52)] pub fn wait_is_loading() -> &'static mut ProcDesc; // Offset: 0x281C390 Flags: 0
}

/// Trait to simulate inheritance for [`ProcInst`].
/// 
/// A method expecting a `&impl Bindable` or `<P: Bindable>(parent: &P, ...)` will accept any type that inherits from [`ProcInst`].
pub trait Bindable: Il2CppClassData + Sized {
    fn create_bind(&self, parent: &impl Bindable, descs: &'static mut Il2CppArray<&'static mut ProcDesc>, name: impl AsRef<str>) {
        unsafe { procinst_createbind(self, parent, descs, name.as_ref().into(), None) }
    }
    #[unity::class_method(5,  ProcInst)] fn create_bind_no_desc<B>(&self, parent: &B) -> &'static mut ProcInst where B: Bindable; // Offset: 0x281E080 Flags: 0
    #[unity::class_method(13, ProcInst)] fn get_parent(&self) -> Option<&'static mut ProcInst>; // Offset: 0x281EAD0 Flags: 0
    fn get_child2<B: Bindable>(&self) -> Option<&'static mut B> {
        let klass = B::class();
        self.get_child().filter(|i| i.klass.get_name() == klass.get_name() && i.klass.get_namespace() == klass.get_namespace())
            .map(|u| unsafe { std::mem::transmute(u) })
    }
    #[unity::class_method(14, ProcInst)] fn get_child(&self) -> Option<&'static mut ProcInst>; // Offset: 0x281EAE0 Flags: 0
    #[unity::class_method(17, ProcInst)] fn get_label(&self) -> i32; // Offset: 0x281EB10 Flags: 0
    #[unity::class_method(27, ProcInst)] fn jump(&self, label: i32); // Offset: 0x281E6F0 Flags: 0
}
#[unity::from_offset("App", "Proc", "Call")]
fn proc_call<D: SystemDelegate>(
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

impl<T: Bindable> SystemDelegate for ProcVoidFunction<T> { }
impl<T: Bindable> SystemDelegate for ProcVoidMethod<T> { }
impl<T> SystemDelegate for ProcBoolMethod<T> { }

impl<T: Bindable> ProcVoidMethod<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate the target.
    pub fn new(
        target: impl Into<Option<&'static T>>,
        method: extern "C" fn(&'static mut T, OptionalMethod),
    ) -> &'static mut ProcVoidMethod<T> {
        ProcVoidMethod::<T>::instantiate().map(|proc| {
            proc.ctor(target.into(), proc.klass.get_methods()[1]);
            proc.method_ptr = method as _;
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
            proc.ctor(target.into(), proc.klass.get_methods()[1]);
            proc.method_ptr = method as _;
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

impl<T: Bindable> ProcVoidFunction<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate to target.
    pub fn new(
        target: impl Into<Option<&'static T>>,
        method: extern "C" fn(&'static mut T, OptionalMethod),
    ) -> &'static mut ProcVoidFunction<T> {
        ProcVoidFunction::<T>::instantiate().map(|proc| {
            proc.ctor(target.into(), proc.klass.get_methods()[1]);
            proc.method_ptr = method as _;
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

