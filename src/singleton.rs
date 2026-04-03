//! Utilities to be used with the [`get_generic_class!`](unity::get_generic_class) macro.

#[repr(C)]
#[unity::class("App", "SingletonProcInst`1")]
pub struct SingletonProcInstClass {}

#[repr(C)]
#[unity::class("App", "SingletonClass`1")]
pub struct SingletonClass {}

#[repr(C)]
#[unity::class("App", "SingletonMonoBehaviour`1")]
pub struct SingletonMonoBehaviour { }

#[repr(C)]
#[unity::class("App", "SingletonScriptableObject`1")]
pub struct SingletonScriptableObject {}

#[repr(C)]
#[unity::class("App", "SingletonPool`2")]
pub struct SingletonPool{}