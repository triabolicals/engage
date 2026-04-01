use unity::prelude::*;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use crate::util::get_singleton_proc_instance;
use super::*;

#[derive(PartialEq, Clone, FromPrimitive, ToPrimitive)]
pub enum MainMenuSequenceLabel {
    None = -1,
    Start = 0,
    DLCNews = 1,
    TopMenu = 2,
    ChangeSceneToGameStart = 3,
    ChangeSceneToTitle = 4,
    InitGameStart = 5,
    PlayerGenderSelect = 6,
    CameraZoomInToPlayer = 7,
    CameraZoomOutFromPlayer = 8,
    PlayerNameInput = 9,
    PlayerBirthdayInput = 10,
    DifficultySelect = 11,
    GameModeSelect = 12,
    GrowModeSelect = 13,
    NetworkServiceSelect = 14,
    NetworkLogin = 15,
    FinalConfirm = 16,
    ExecuteGameStart = 17,
    Continue = 18,
    Option = 19,
    SaveDataCopy = 20,
    SaveDataDelete = 21,
    LanguageSetting = 22,
    LanguageReload = 23,
    DLCBegin = 24,
    DLCShop = 25,
    DLCEnd = 26,
    ToTitleLoop = 27,
    ToStartGame = 28,
    ToContinueGame = 29,
    End = 30,
}

#[repr(C)]
#[unity::class("App", "MainMenuSequence")]
pub struct MainMenuSequence {
    pub proc: ProcInstFields,
    is_resume: bool,
    is_loaded: bool,
    pub prev_sequence: i32,
    pub now_sequence: i32,
    pub next_sequence: i32,
}

impl MainMenuSequence {
    pub const MAIN_MENU_SEQUENCE: i32 = -1912552174;
    pub fn get_instance() -> Option<&'static mut Self> {
        get_singleton_proc_instance::<Self>()
    }
    pub fn jump_to_next_sequence() {
        if let Some(instance) = Self::get_instance() {
            unsafe { mainmenusequence_jumptonextsequence(instance, None) };
        }
    }
}

impl AsRef<ProcInstFields> for MainMenuSequence {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl AsMut<ProcInstFields> for MainMenuSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}

impl Bindable for MainMenuSequence {}

#[unity::from_offset("App", "MainMenuSequence", "JumpToNextSequence")]
fn mainmenusequence_jumptonextsequence(this: &MainMenuSequence, method_info: OptionalMethod);