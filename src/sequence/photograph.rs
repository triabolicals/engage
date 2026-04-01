use unity::il2cpp::object::Array;
use crate::proc::{Bindable, ProcInstFields};
use unity::prelude::*;
use unity::system::List;
use crate::gamedata::{GamedataArray, StructDataArrayFields};
use crate::unityengine::{GameObject, Transform, UnityComponent};
use crate::util::get_singleton_proc_instance;

mod dispos;
mod camera;

pub use dispos::*;
pub use camera::*;

#[unity::class("App", "PhotographTopSequence")]
pub struct PhotographTopSequence{
    pub proc: ProcInstFields,
}
impl Bindable for PhotographTopSequence{}

impl PhotographTopSequence{
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
    pub fn get_photograph_sequence() -> Option<&'static mut PhotographSequence> {
        get_singleton_proc_instance::<Self>().and_then(|proc| proc.get_child())
            .map(|proc| proc.cast_mut::<PhotographSequence>())
    }
}

#[unity::class("App", "PhotographSequence")]
pub struct PhotographSequence {
    pub proc: ProcInstFields,
    pub reserved_label: i32,
    menu_content: u64,
    pub spot_obj: &'static GameObject,
    pub dispos_manager: &'static mut PhotographDisposManager,
    pub camera_controller: &'static mut PhotographCameraController,
    pub animal_locator_name_list: &'static List<Il2CppString>,
}
impl Bindable for PhotographSequence{}

impl AsRef<ProcInstFields> for PhotographSequence{
    fn as_ref(&self) -> &ProcInstFields{ &self.proc }
}
impl AsMut<ProcInstFields> for PhotographSequence{
    fn as_mut(&mut self) -> &mut ProcInstFields{ &mut self.proc }
}


#[unity::class("App", "PhotographPauseData")]
pub struct PhotographPauseData {
    pub parent: StructDataArrayFields,
    pub pause_name: &'static Il2CppString, // Offset 0x28, Attr: 1
    pub no: i32, // Offset 0x30, Attr: 1
    pub mid: &'static Il2CppString, // Offset 0x38, Attr: 1
    pub anime_frame: i32, // Offset 0x40, Attr: 1
    pub face_anime: &'static Il2CppString, // Offset 0x48, Attr: 1
    pub chara_id_list: &'static Array<Il2CppString>, // Offset 0x50, Attr: 1
}
impl GamedataArray for PhotographPauseData {}

impl PhotographPauseData {
    #[unity::class_method(1)] pub fn get_pause_name(&self) -> &'static Il2CppString; // Offset: 0x2694960 Flags: 0
    #[unity::class_method(13)] pub fn get_name(&self) -> &'static Il2CppString; // Offset: 0x2693830 Flags: 0
}

#[unity::class("RootMotion.FinalIK", "LookAtIK")]
pub struct LookAtIK {
    parent: [u8; 0x28],
    pub solver: &'static mut IKSolverLookAt,
}
impl UnityComponent for LookAtIK {}
#[unity::class("RootMotion.FinalIK", "IKSolverLookAt")]
pub struct IKSolverLookAt {
    parent: [u8; 0x48],
    pub target: &'static Transform,
    spine: u64,
    head: u64,
    eyes: u64,
    pub body_weight: f32, // Offset 0x78, Attr: 6
    pub head_weight: f32, // Offset 0x7C, Attr: 6
    pub eyes_weight: f32, // Offset 0x80, Attr: 6
    pub clamp_weight: f32, // Offset 0x84, Attr: 6
    pub clamp_weight_head: f32, // Offset 0x88, Attr: 6
    pub clamp_weight_eyes: f32, // Offset 0x8C, Attr: 6
    pub clamp_smoothing: i32, // Offset 0x90, Attr: 6
}


