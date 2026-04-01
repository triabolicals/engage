use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use unity::prelude::*;
use crate::util::get_singleton_proc_instance;
use super::*;

#[repr(C)]
#[unity::class("App", "MainSequence")]
pub struct MainSequence {
    // Start SingletonProcInst here
    pub proc: ProcInstFields,
    is_resume: bool,
    is_loaded: bool,
    // End here
    pub scene_name: &'static mut Il2CppString,
    pub scene_mode: i32,
    pub pad: i32,
}

#[repr(C)]
pub struct MainSequenceStaticFields {
    pub jump_label: MainSequenceLabel,
    pub fake_label: i32,
    pub initialized: bool,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MainSequenceLabel {
    None = 0,
    Startup = 1,
    TitleLoop = 2,
    TitleLoopFromMainMenu = 3,
    MainMenu = 4,
    Chapter = 5,
    Gmap = 6,
    Kizuna = 7,
    Hub = 8,
    HubToSavePosition = 9,
    Ending = 10,
    NextChapter = 11,
    Map = 12,
    Complete = 13,
    GameOver = 14,
    ChapterSave = 15,
    AfterChapterSave = 16,
    SetSaveDataLoadTarget = 17,
    SaveDataLoad = 18,
    SaveDataLoadFailed = 19,
    SaveDataVersionFailed = 20,
    DataLoadFailed = 21,
    AfterLoadFailed = 22,
    ContentsResume = 23,
    RelayDebug = 24,
    Relay = 25,
    Versus = 26,
    Challenge = 27,
    BackToTitle = 28,
    End = 29,
}

impl Bindable for MainSequence {}

impl AsRef<ProcInstFields> for MainSequence {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl AsMut<ProcInstFields> for MainSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}
impl MainSequence {
    pub const HASH: i32 = -339912801;
    pub fn get_instance() -> Option<&'static mut Self> {
        get_singleton_proc_instance::<Self>()
    }
    #[unity::class_method(7)] pub fn post_initialize(&self); // Offset: 0x1EDD920 Flags: 0
    #[unity::class_method(8)] pub fn load_public(&self); // Offset: 0x1EDDA10 Flags: 0
    #[unity::class_method(9)] pub fn load_resource(&self); // Offset: 0x1EDDA80 Flags: 0
    #[unity::class_method(10)] pub fn post_load_resource(&self); // Offset: 0x1EDDC40 Flags: 0
    #[unity::class_method(11)] pub fn begin_silent_volume(&self); // Offset: 0x1EDDCE0 Flags: 0
    #[unity::class_method(12)] pub fn end_silent_volume(&self); // Offset: 0x1EDDCF0 Flags: 0
    #[unity::class_method(13)] pub fn on_persistent(&self); // Offset: 0x1EDDD00 Flags: 0
    #[unity::class_method(14)] pub fn branch_start(&self); // Offset: 0x1EDDD80 Flags: 0
    #[unity::class_method(15)] pub fn branch_chapter_start(&self); // Offset: 0x1EDDDF0 Flags: 0
    #[unity::class_method(16)] pub fn load_chapter_bank(&self); // Offset: 0x1EDDF20 Flags: 0
    #[unity::class_method(17)] pub fn try_jump_to_kizuna(&self); // Offset: 0x1EDE020 Flags: 0
    #[unity::class_method(18)] pub fn try_jump_to_continue_map(&self); // Offset: 0x1EDE310 Flags: 0
    #[unity::class_method(19)] pub fn try_jump_to_hub(&self); // Offset: 0x1EDE4F0 Flags: 0
    #[unity::class_method(20)] pub fn try_jump_to_gmap(&self); // Offset: 0x1EDE610 Flags: 0
    #[unity::class_method(21)] pub fn try_jump_to_next_chapter(&self); // Offset: 0x1EDE3F0 Flags: 0
    #[unity::class_method(22)] pub fn hub_to_save_position(&self); // Offset: 0x1EDE6E0 Flags: 0
    #[unity::class_method(23)] pub fn game_reset(&self); // Offset: 0x1EDE760 Flags: 0
    #[unity::class_method(24)] pub fn auto_save(&self); // Offset: 0x1EDE820 Flags: 0
    #[unity::class_method(25)] pub fn set_save_data_load_target(&self); // Offset: 0x1EDE840 Flags: 0
    #[unity::class_method(26)] pub fn save_data_load(&self); // Offset: 0x1EDE990 Flags: 0
    #[unity::class_method(27)] pub fn save_data_load_result(&self); // Offset: 0x1EDEC40 Flags: 0
    #[unity::class_method(28)] pub fn save_data_release(&self); // Offset: 0x1EDED70 Flags: 0
    #[unity::class_method(29)] pub fn save_data_normalize(&self); // Offset: 0x1EDEE40 Flags: 0
    #[unity::class_method(30)] pub fn save_data_branch_first(&self); // Offset: 0x1EDEE50 Flags: 0
    #[unity::class_method(31)] pub fn save_data_branch_second(&self); // Offset: 0x1EDEF90 Flags: 0
    #[unity::class_method(32)] pub fn save_data_load_failed(&self); // Offset: 0x1EDF090 Flags: 0
    #[unity::class_method(33)] pub fn save_data_version_failed(&self); // Offset: 0x1EDF120 Flags: 0
    #[unity::class_method(34)] pub fn data_load_failed(&self); // Offset: 0x1EDF1B0 Flags: 0
    #[unity::class_method(35)] pub fn delete_temporary(&self); // Offset: 0x1EDF240 Flags: 0
    #[unity::class_method(36)] pub fn game_sound_reset(&self); // Offset: 0x1EDF250 Flags: 0
    #[unity::class_method(39)] pub fn load_logo(&self); // Offset: 0x1EDF330 Flags: 0
    #[unity::class_method(40)] pub fn show_logo(&self); // Offset: 0x1EDF3A0 Flags: 0
    #[unity::class_method(41)] pub fn show_icon(&self); // Offset: 0x1EDF3D0 Flags: 0
}