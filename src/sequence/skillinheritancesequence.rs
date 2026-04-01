use unity::prelude::*;
use unity::system::List;
use crate::{
    unit::Unit,
    god::GodUnit,
    proc::{Bindable, desc::ProcDesc, ProcInstFields}
};

use super::hubrefineshopsequence::SortieSequenceItemShop;

#[unity::class("App", "SkillInheritanceSequence")]
pub struct SkillInheritanceSequence {
    pub proc: ProcInstFields,
    pub unit: Option<&'static mut Unit>,
    pub select_unit_god_list: &'static mut List<GodUnit>,
}
impl AsRef<ProcInstFields> for SkillInheritanceSequence {
    fn as_ref(&self) -> &ProcInstFields { &self.proc }
}
impl AsMut<ProcInstFields> for SkillInheritanceSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields { &mut self.proc }
}
impl Bindable for SkillInheritanceSequence { }

impl SkillInheritanceSequence {
    pub fn new() -> &'static mut Self {
        let item = Self::instantiate().unwrap();
        item.klass = SortieSequenceItemShop::class().clone();
        item
    }
    pub fn create_bind<B: Bindable>(parent: &B) { unsafe { skillinheritancesequence_create_bind(parent, None) } }
    pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc> {
        unsafe { skillinheritancesequence_createdesc(self, None) }
    }
    #[unity::class_method(11)] pub fn get_select_unit(&self) -> &'static Unit; // Offset: 0x24A8C60 Flags: 0
    pub fn start_sequence(&self) { unsafe { skillinheritance_start_sequence(self, None); } }
    pub fn end_sequence(&self) { unsafe { skillinheritance_end_sequence(self, None); } }
}

#[unity::from_offset("App", "SkillInheritanceSequence", "CreateDesc")]
fn skillinheritancesequence_createdesc(this: &SkillInheritanceSequence, method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc>;

#[unity::from_offset("App", "SkillInheritanceSequence", "CreateBind")]
fn skillinheritancesequence_create_bind<B: Bindable>(proc: &B, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillInheritanceSequence", "StartSequence")]
fn skillinheritance_start_sequence(this: &SkillInheritanceSequence, method_info: OptionalMethod);

#[unity::from_offset("App", "SkillInheritanceSequence", "EndSequence")]
fn skillinheritance_end_sequence(this: &SkillInheritanceSequence, method_info: OptionalMethod);