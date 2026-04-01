use unity::engine::Color;
pub use super::*;

pub mod tag;
pub use tag::*;
use crate::combat::Character;
use crate::proc::ProcInst;
use crate::unityengine::GameObject;
use crate::util::get_singleton_proc_instance;
#[unity::class("App", "Talk")] pub struct Talk {}

impl Talk {
    #[unity::class_method(7)] pub fn get_playing_mid() -> &'static Il2CppString; // Offset: 0x20B8E10 Flags: 0
}

#[unity::class("App.Talk3D", "TalkSequence")]
pub struct TalkSequence {
    pub proc: SingletonProcInstFields,
    prefab_root_object: &'static GameObject, // Offset 0x78, Attr: 1
    m_talk_sound: u64, // &TalkSound, // Offset 0x80, Attr: 1
    m_tag_parser: &'static TalkTagParser, // Offset 0x88, Attr: 1
    m_talk_ptr: &'static mut TalkPtr, // Offset 0x90, Attr: 1
    m_reserved_tag: u64, //&TalkTag, // Offset 0x98, Attr: 1
    m_replace_text: u64, // &TalkSequenceReplaceText, // Offset 0xA0, Attr: 1
    m_bind_proc: Option<&'static ProcInst>, // Offset 0xA8, Attr: 1
    m_error_message_builder: u64, // &TalkBuilder, // Offset 0xB0, Attr: 1
    m_is_start_by_continuous_number: bool, // Offset 0xB8, Attr: 1
    m_is_flushed_sound_list: bool, // Offset 0xB9, Attr: 1
    m_is_show_map_terrain_info: bool, // Offset 0xBA, Attr: 1
    m_is_bind_map_camera: bool, // Offset 0xBB, Attr: 1
    m_is_pushed_unit_info: bool, // Offset 0xBC, Attr: 1
    m_loaded_sound_bank_name_list: u64, //&'static List<Il2CppString>, // Offset 0xC0, Attr: 1
    pub mid: &'static Il2CppString, // Offset 0xC8, Attr: 1
    talk_type: i32, // Offset 0xD0, Attr: 1
    active_pid: &'static Il2CppString, // Offset 0xD8, Attr: 1
    str_to_lower_trigger: bool, // Offset 0xE0, Attr: 1
    is_latest_str_patchim1: bool, // Offset 0xE1, Attr: 1
    is_latest_str_patchim2: bool, // Offset 0xE2, Attr: 1
    pub m_is_loading_character: bool, // Offset 0xE3, Attr: 6
    m_loading_character_timeout_counter: f32, // Offset 0xE4, Attr: 1
    // m_replace_talker_name_dictionary: Dictionary<&Il2CppString, &Il2CppString>, // Offset 0xE8, Attr: 1
}

impl TalkSequence {
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
    #[unity::class_method(0)] pub fn get_mid(&self) -> &'static Il2CppString; // Offset: 0x20C5550 Flags: 0
    #[unity::class_method(1)] pub fn set_mid(&self, value: &Il2CppString); // Offset: 0x20C5560 Flags: 0
    #[unity::class_method(14)] pub fn get_replace_talker_name(&self, pid: &Il2CppString) -> &'static Il2CppString; // Offset: 0x20C5620 Flags: 0
    #[unity::class_method(15)] pub fn add_replace_talker_name(&self, pid: &Il2CppString, talker_name: &Il2CppString); // Offset: 0x20C56A0 Flags: 0
    /*
    #[unity::class_method(16)] pub fn reserve_talk_prefab(); // Offset: 0x20C5770 Flags: 0
    #[unity::class_method(17)] pub fn create_talk_prefab() -> &'static GameObject; // Offset: 0x20C58B0 Flags: 0
    #[unity::class_method(18)] pub fn is_fast_forward() -> bool; // Offset: 0x20B8840 Flags: 0
    #[unity::class_method(19)] pub fn start(&self, mid: &Il2CppString, is_continuous_number: bool); // Offset: 0x20B83E0 Flags: 0
    #[unity::class_method(20)] pub fn restart(&self, mid: &Il2CppString, is_continuous_number: bool); // Offset: 0x20B83F0 Flags: 0
    #[unity::class_method(21)] pub fn start_impl(&self, mid: &Il2CppString, is_continuous_number: bool); // Offset: 0x20C59F0 Flags: 0
    #[unity::class_method(22)] pub fn init_talk(&self, mid: &Il2CppString); // Offset: 0x20C5CA0 Flags: 0
    #[unity::class_method(23)] pub fn try_next_label(&self); // Offset: 0x20C67B0 Flags: 0
    #[unity::class_method(24)] pub fn flush_sound_list(&self, is_exec_before_of_now_label: bool); // Offset: 0x20C6190 Flags: 0
    #[unity::class_method(25)] pub fn wait_camera(&self); // Offset: 0x20C6AC0 Flags: 0
    #[unity::class_method(26)] pub fn try_map_camera_bind(&self); // Offset: 0x20C6B00 Flags: 0
    #[unity::class_method(27)] pub fn try_map_camera_unbind(&self); // Offset: 0x20C6BA0 Flags: 0
    #[unity::class_method(28)] pub fn load_sound_bank(&self); // Offset: 0x20C6C30 Flags: 0
    #[unity::class_method(29)] pub fn wait_soundbank_loading(&self); // Offset: 0x20C6E50 Flags: 0
    #[unity::class_method(30)] pub fn load_scene(&self); // Offset: 0x20C6EF0 Flags: 0
    #[unity::class_method(31)] pub fn finish(&self, is_end_continuous_number: bool); // Offset: 0x20C69C0 Flags: 0
    #[unity::class_method(32)] pub fn release(&self); // Offset: 0x20C7270 Flags: 0
    #[unity::class_method(33)] pub fn get_active_character(&self) -> &'static Character; // Offset: 0x20C7500 Flags: 0
    #[unity::class_method(34)] pub fn get_person_voice(&self) -> &'static Il2CppString; // Offset: 0x20BEF90 Flags: 0
    #[unity::class_method(35)] pub fn try_play_voice(&self, character: &Character); // Offset: 0x20C76F0 Flags: 0
    #[unity::class_method(36)] pub fn stop_all_voice(&self); // Offset: 0x20BF980 Flags: 0
    #[unity::class_method(37)] pub fn persistent(&self); // Offset: 0x20C78D0 Flags: 0
    #[unity::class_method(38)] pub fn tick_sound_before(&self); // Offset: 0x20C78E0 Flags: 0
    #[unity::class_method(39)] pub fn tick_sound_after(&self); // Offset: 0x20C7920 Flags: 0
    #[unity::class_method(40)] pub fn tick(&self); // Offset: 0x20C79B0 Flags: 0
    #[unity::class_method(41)] pub fn process_message(&self, add_char_count: i32); // Offset: 0x20C7E90 Flags: 0
    #[unity::class_method(42)] pub fn close(&self); // Offset: 0x20C80E0 Flags: 0
    #[unity::class_method(43)] pub fn wait_close(&self); // Offset: 0x20C8210 Flags: 0
    #[unity::class_method(44)] pub fn check_continue(&self); // Offset: 0x20C82C0 Flags: 0
    #[unity::class_method(45)] pub fn begin_continue_talk(); // Offset: 0x20B85E0 Flags: 0
    #[unity::class_method(46)] pub fn end_continue_talk(); // Offset: 0x20B8700 Flags: 0
    #[unity::class_method(47)] pub fn start_key_wait(&self); // Offset: 0x20C8400 Flags: 0
    #[unity::class_method(48)] pub fn start_time_wait(&self, sec: f32); // Offset: 0x20C8410 Flags: 0
    #[unity::class_method(49)] pub fn start_fade_out(&self, time: f32, color: Color); // Offset: 0x20C8420 Flags: 0
    #[unity::class_method(50)] pub fn start_fade_in(&self, time: f32); // Offset: 0x20C84F0 Flags: 0
    #[unity::class_method(51)] pub fn set_replace_text(&self, text: &Il2CppString); // Offset: 0x20C8590 Flags: 0
    #[unity::class_method(52)] pub fn wait_game_skip_end(&self); // Offset: 0x20C85A0 Flags: 0
    #[unity::class_method(53)] pub fn skip(&self); // Offset: 0x20C85E0 Flags: 0
    #[unity::class_method(54)] pub fn start_text_scroll(&self); // Offset: 0x20C86D0 Flags: 0
    #[unity::class_method(55)] pub fn end_text_scroll(&self); // Offset: 0x20C4650 Flags: 0
    #[unity::class_method(56)] pub fn bind_parent(&self); // Offset: 0x20C6610 Flags: 0
    #[unity::class_method(57)] pub fn unbind_parent(&self); // Offset: 0x20C7220 Flags: 0
    // #[unity::class_method(58)] pub fn get_desc(&self) -> &'static Array<ProcDesc>; // Offset: 0x20C86E0 Flags: 0
    #[unity::class_method(59)] pub fn create<B>(parent: &B) -> &'static TalkSequence where B: Bindable; // Offset: 0x20B8360 Flags: 0

     */
}