use super::*;
use crate::util::get_singleton_proc_instance;

#[unity::class("App", "SortieSequence")]
pub struct SortieSequence {
   pub proc: ProcInstFields,
}

impl Bindable for SortieSequence {}

impl SortieSequence {
   pub const HASH: i32 = -1624221522;
   pub fn get_instance() -> Option<&'static mut Self> {
      get_singleton_proc_instance::<Self>()
   }
   /*
   #[unity::class_method(0)] pub fn create_bind_map(proc: &ProcInst) -> &'static ProcInst; // Offset: 0x1FE8E40 Flags: 0
   #[unity::class_method(1)] pub fn create_bind_hub(super: &ProcInst) -> &'static ProcInst; // Offset: 0x1FE9FA0 Flags: 0
   #[unity::class_method(2)] pub fn create_bind_edit(super: &ProcInst) -> &'static ProcInst; // Offset: 0x1FE9FB0 Flags: 0
   #[unity::class_method(3)] pub fn create_bind_impl<B>(B: &ProcInst, mode: SortieSequenceModes) -> &'static ProcInst; // Offset: 0x1FE8E50 Flags: 0
   */
   #[unity::class_method(4)] pub fn decide_to_battle(); // Offset: 0x1FEA080 Flags: 0
   #[unity::class_method(5)] pub fn is_decided_to_battle() -> bool; // Offset: 0x1FEA130 Flags: 0
   #[unity::class_method(6)] pub fn get_mode(&self) -> SortieSequenceModes; // Offset: 0x1FEA180 Flags: 0
   #[unity::class_method(7)] pub fn ctor(&self, mode: SortieSequenceModes); // Offset: 0x1FE9FC0 Flags: 0
   #[unity::class_method(8)] pub fn on_dispose(&self); // Offset: 0x1FEA190 Flags: 0
   #[unity::class_method(9)] pub fn on_shutdown(&self); // Offset: 0x1FEA4C0 Flags: 0
   #[unity::class_method(10)] pub fn setup_units(&self); // Offset: 0x1FEA530 Flags: 0
   #[unity::class_method(11)] pub fn post_setup_units(&self); // Offset: 0x1FEA730 Flags: 0
   #[unity::class_method(12)] pub fn try_show_tutorial(&self); // Offset: 0x1FEA7C0 Flags: 0
   #[unity::class_method(13)] pub fn notice(&self); // Offset: 0x1FEA8A0 Flags: 0
   #[unity::class_method(14)] pub fn reset_managers(&self); // Offset: 0x1FEACD0 Flags: 0
   #[unity::class_method(15)] pub fn show_help(&self); // Offset: 0x1FEAE30 Flags: 0
   #[unity::class_method(16)] pub fn hide_help(&self); // Offset: 0x1FEAED0 Flags: 0
   #[unity::class_method(17)] pub fn play_bgm(&self); // Offset: 0x1FEAF50 Flags: 0
   #[unity::class_method(18)] pub fn stop_bgm(&self); // Offset: 0x1FEB020 Flags: 0
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SortieSequenceModes {
   Map = 0, // Attr: 17
   Hub = 1, // Attr: 17
   Edit = 2, // Attr: 17
}