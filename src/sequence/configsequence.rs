use unity::prelude::*;
use crate::proc::{ProcInstFields, Bindable};

#[unity::class("", "ConfigSequence")]
pub struct ConfigSequence {
    pub proc: ProcInstFields,
}

impl ConfigSequence {
    pub const HASH: i32 = 1743174389;
    #[unity::class_method(0)] pub fn load_resources(&self); // Offset: 0x253A110 Flags: 0
    #[unity::class_method(1)] pub fn unload_resources(&self); // Offset: 0x253A190 Flags: 0
    #[unity::class_method(2)] pub fn is_loading_resources(&self) -> bool; // Offset: 0x253A210 Flags: 0
    #[unity::class_method(3)] pub fn start_sequence(&self); // Offset: 0x253A290 Flags: 0
    #[unity::class_method(4)] pub fn create_config_menu(&self); // Offset: 0x253A470 Flags: 0
    #[unity::class_method(5)] pub fn end_sequence(&self); // Offset: 0x253A4A0 Flags: 0
    #[unity::class_method(6)] pub fn destroy_config_menu(&self); // Offset: 0x253A5E0 Flags: 0
    #[unity::class_method(7)] pub fn create_bind<B>(parent: &B) where B: Bindable; // Offset: 0x253A650 Flags: 0
    #[unity::class_method(8)] pub fn on_dispose(&self); // Offset: 0x253AC00 Flags: 0
    #[unity::class_method(9)] pub fn ctor(&self); // Offset: 0x253ABF0 Flags: 0
}

impl AsRef<ProcInstFields> for ConfigSequence {
    fn as_ref(&self) -> &ProcInstFields {
      &self.proc
    }
}

impl AsMut<ProcInstFields> for ConfigSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}

impl Bindable for ConfigSequence { }