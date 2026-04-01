use paste::paste;
use unity::prelude::Il2CppClassData;

#[macro_export]
macro_rules! impl_action_handlers {
    ([$nested:ident, $class:expr, $class_name:ident], $($arg:ident: $arg_ty:ty),*) => {
        #[unity::class("", $class)]
        #[nested_from_type($nested)]
        pub struct $class_name {
            pub method_ptr: *const u8,
            invoke_impl: *const u8,
            pub target_obj: *const u8,
            pub method: *const MethodInfo,
        }
        impl $class_name {
            #[unity::class_method(0)] pub fn ctor<T>(&self, obj: &T, m: &unity::prelude::MethodInfo) where T: unity::prelude::Il2CppClassData + Sized;
            #[unity::class_method(1)] pub fn invoke(&self, $($arg: $arg_ty),*);
        }
    };
    ([$namespace:expr, $class:expr, $class_name:ident], $($arg:ident: $arg_ty:ty),*) => {
        #[unity::class($namespace, $class)]
        pub struct $class_name {
            pub method_ptr: *const u8,
        }
        impl $class_name{
            #[unity::class_method(0)] pub fn ctor<T>(&self, obj: &T, m: &unity::prelude::MethodInfo) where T: unity::prelude::Il2CppClassData + Sized;
            #[unity::class_method(1)] pub fn invoke(&self, $($arg: $arg_ty),*);
        }
    };
}

#[macro_export]
macro_rules! method_ptr_convert{
    ($method:ident($($arg_ty:ty), *) -> $ return_type: ty) => {
        unsafe { std::mem::transmute::<_, fn($($arg_ty),*, &MethodInfo) -> $return_type>($method.method_ptr) }
    };
    ($method:ident($($arg_ty:ty), *)) => {
        unsafe { std::mem::transmute::<_, fn($($arg_ty),*,&MethodInfo) >($method.method_ptr) }
    };
}
