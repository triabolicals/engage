use unity::engine::Color;
use unity::il2cpp::object::Array;
use unity::system::Dictionary;
use crate::combat::{Character, CharacterAppearance};
use crate::unityengine::GameObject;
use crate::util::get_singleton_proc_instance;
use super::*;

#[unity::class("App", "EventDemoSequence")]
pub struct EventDemoSequence {
    pub proc: SingletonProcInstFields,
    pub scene_name: &'static Il2CppString,  //0x78
    pub scene_mode: i32,    //0x80
    pub demo_name: &'static Il2CppString, // Offset 0x88, Attr: 1
    pub mess_file_name: &'static Il2CppString, // Offset 0x90, Attr: 1
    pub cur_mess_label_index: i32, // Offset 0x98, Attr: 1
    pub cur_mess_label: &'static Il2CppString, // Offset 0xA0, Attr: 1
    cmd_infos_exec_before: &'static Array<&'static CmdInfo>, // Offset 0xA8, Attr: 1
    cmd_infos_exec_after: &'static Array<&'static CmdInfo>, // Offset 0xB0, Attr: 1
    cur_cmd_info_index: i32, // Offset 0xB8, Attr: 1
    event_cmd_seq: i32, // Offset 0xBC, Attr: 1
    event_cmd_text_exec_before: &'static Dictionary<'static, &'static Il2CppString, &'static Il2CppString>, // Offset 0xC0, Attr: 1
    event_cmd_text_exec_after:  &'static Dictionary<'static, &'static Il2CppString, &'static Il2CppString>, // Offset 0xC8, Attr: 1
    pub func_dictionary: &'static Dictionary<'static, &'static Il2CppString, &'static mut EventDemoSequenceCmdFunc>, // Offset 0xD0, Attr: 1
    /*
    m_is_puppet_talk_pause: bool, // Offset 0xD8, Attr: 1
    m_is_fade_out_in_start: bool, // Offset 0xD9, Attr: 1
    m_do_later_set_camera: &IDisposable, // Offset 0xE0, Attr: 1
    m_light_setup_info: &EventDemoSequenceLightSetupInfo, // Offset 0xE8, Attr: 1
    m_cube_material_handle: &ResourceHandle, // Offset 0xF0, Attr: 1
    m_mess_loaded_list: List<&Il2CppString>, // Offset 0xF8, Attr: 1
    m_character_work_dictionary: Dictionary<&Il2CppString, &EventDemoSequenceCharacterWork>, // Offset 0x100, Attr: 1
    m_split_view_work_dictionary: Dictionary<&Il2CppString, &EventDemoSequenceSplitViewWork>, // Offset 0x108, Attr: 1
    m_effect_work_dictionary: Dictionary<&Il2CppString, &EventDemoSequenceEffectWork>, // Offset 0x110, Attr: 1
    m_telop_effect_resource_object: &ResourceObject, // Offset 0x118, Attr: 1
    m_picture_controller: &EventPictureController, // Offset 0x120, Attr: 1
    */
}
impl EventDemoSequence {
    pub fn get_instance() -> Option<&'static mut Self> { get_singleton_proc_instance::<Self>() }
    pub fn get_func<'a>(&self, key: impl Into<&'a Il2CppString>) -> Option<&'static mut EventDemoSequenceCmdFunc> {
        self.func_dictionary.get_item(key.into())
    }
    // #[unity::class_method(0)] pub fn analysis_cmd_text(&self, cmd_texts: &Il2CppString) -> &'static Array<CmdInfo>; // Offset: 0x24D3AF0 Flags: 0
   //  #[unity::class_method(1)] pub fn get_cmd_info_from_cmd_lines(&self, cmd_line: &Il2CppString) -> &'static CmdInfo; // Offset: 0x24D3C50 Flags: 0
    //#[unity::class_method(2)] pub fn exec_event_cmd(&self, cmd_infos: &Array<CmdInfo>, cmd_info_index: i32) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D4130 Flags: 0
    // #[unity::class_method(3)] pub fn exec_event_cmd_impl(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D41B0 Flags: 0
    #[unity::class_method(4)] pub fn func_label(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D4210 Flags: 0
    #[unity::class_method(5)] pub fn func_jump(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D4220 Flags: 0
    #[unity::class_method(6)] pub fn func_wait(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D4370 Flags: 0
    #[unity::class_method(7)] pub fn func_variant(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D44F0 Flags: 0
    #[unity::class_method(8)] pub fn set_back_ground(&self, sky_box_material_name: &Il2CppString); // Offset: 0x24D4820 Flags: 0
    #[unity::class_method(9)] pub fn get_sky_box_material_name_auto(&self, base_name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x24D4990 Flags: 0
    #[unity::class_method(10)] pub fn func_set_back_ground(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D4BE0 Flags: 0
    #[unity::class_method(11)] pub fn func_set_back_ground_auto(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D4C50 Flags: 0
   // #[unity::class_method(12)] pub fn disable_cameras(&self); // Offset: 0x24D4CD0 Flags: 0
    // #[unity::class_method(13)] pub fn create_current_camera(&self, src_camera_name: &Il2CppString, is_chara_camera_target_hero_female: bool, is_split_view_camera: bool) -> &'static Camera; // Offset: 0x24D4DF0 Flags: 0
    // #[unity::class_method(14)] pub fn create_camera(&self, camera_name: &Il2CppString, src_camera_name: &Il2CppString, is_chara_camera_target_hero_female: bool, is_split_view_camera: bool) -> &'static Camera; // Offset: 0x24D4E70 Flags: 0
    // #[unity::class_method(15)] pub fn get_src_camera(&self, src_camera_name: &Il2CppString, is_warning: bool) -> &'static Camera; // Offset: 0x24D5420 Flags: 0
    // #[unity::class_method(16)] pub fn create_parent_object_of_camera(&self, camera: &Camera) -> &'static GameObject; // Offset: 0x24D5580 Flags: 0
   // #[unity::class_method(17)] pub fn set_camera_for_chara(&self, camera: &Camera, character: &Character); // Offset: 0x24D5810 Flags: 0
    // #[unity::class_method(18)] pub fn get_character_camera_adjust_object(&self, character: &Character) -> &'static GameObject; // Offset: 0x24D59D0 Flags: 0
    // #[unity::class_method(19)] pub fn init_character_camera_adjust_transform(&self, character: &Character); // Offset: 0x24D5A40 Flags: 0
    // #[unity::class_method(20)] pub fn set_camera_for_scene(&self, camera: &Camera); // Offset: 0x24D5CC0 Flags: 0

    #[unity::class_method(21)] pub fn func_camera_set_chara_camera(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D5D50 Flags: 0
    #[unity::class_method(22)] pub fn set_chara_camera_no_delay(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D6040 Flags: 0
    #[unity::class_method(23)] pub fn func_camera_set_scene_camera(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D6B80 Flags: 0
    #[unity::class_method(24)] pub fn set_scene_camera_no_delay(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D6E60 Flags: 0
    // #[unity::class_method(25)] pub fn get_split_view_work(&self, split_view_name: &Il2CppString, is_warning: bool) -> &'static EventDemoSequenceSplitViewWork; // Offset: 0x24D7070 Flags: 0
    #[unity::class_method(26)] pub fn func_split_view_begin_split_view_camera_only(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D7120 Flags: 0
    #[unity::class_method(27)] pub fn func_split_view_end_split_view_camera_only(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D7230 Flags: 0
    #[unity::class_method(28)] pub fn func_split_view_create(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D7440 Flags: 0
    #[unity::class_method(29)] pub fn func_split_view_set_chara_camera(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D75E0 Flags: 0
    #[unity::class_method(30)] pub fn func_split_view_set_scene_camera(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D7A40 Flags: 0
    #[unity::class_method(31)] pub fn func_split_view_set_active(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D7C90 Flags: 0
    #[unity::class_method(32)] pub fn func_split_view_play_anim(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D7E10 Flags: 0
    #[unity::class_method(33)] pub fn func_split_view_wait_anim_end(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D7FC0 Flags: 0
    // #[unity::class_method(34)] pub fn disable_lights(&self); // Offset: 0x24D8170 Flags: 0
    // #[unity::class_method(35)] pub fn setup_light(&self, light_setup_info: &EventDemoSequenceLightSetupInfo); // Offset: 0x24D67C0 Flags: 0
    #[unity::class_method(36)] pub fn func_light_setup(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D8290 Flags: 0
    #[unity::class_method(37)] pub fn func_light_setup_auto(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D86C0 Flags: 0
    // #[unity::class_method(38)] pub fn set_light_common(&self, light_name: &Il2CppString, rot_offset: Vector3); // Offset: 0x24D85B0 Flags: 0
    #[unity::class_method(39)] pub fn func_fade_in(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D8910 Flags: 0
    #[unity::class_method(40)] pub fn func_fade_out(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D8B60 Flags: 0
    #[unity::class_method(41)] pub fn func_white_fade_in(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D8BA0 Flags: 0
    #[unity::class_method(42)] pub fn func_white_fade_out(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D8BE0 Flags: 0
    // #[unity::class_method(43)] pub fn fade_in_out_impl(&self, cmd_info: &CmdInfo, color: Color, is_fade_in: bool) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D8950 Flags: 0
    // #[unity::class_method(44)] pub fn get_character_work(&self, pid: &Il2CppString, is_warning: bool) -> &'static EventDemoSequenceCharacterWork; // Offset: 0x24D6650 Flags: 0
    #[unity::class_method(45)] pub fn get_character_appearance(&self, pid: &Il2CppString, is_warning: bool) -> &'static CharacterAppearance; // Offset: 0x24D6470 Flags: 0
    #[unity::class_method(46)] pub fn get_character_locator(&self, pid: &Il2CppString, is_warning: bool) -> &'static GameObject; // Offset: 0x24D8C20 Flags: 0
    #[unity::class_method(47)] pub fn get_character(&self, pid: &Il2CppString, is_warning: bool) -> Option<&'static Character>; // Offset: 0x24D6530 Flags: 0
    #[unity::class_method(48)] pub fn find_character_locator(&self, pos_string: &Il2CppString) -> &'static GameObject; // Offset: 0x24D8D40 Flags: 0
    #[unity::class_method(49)] pub fn play_character_anim(&self, character: &Character, facial_anim_name: &Il2CppString, body_anim_name: &Il2CppString, transition_duration: f32); // Offset: 0x24D9140 Flags: 0
    // #[unity::class_method(50)] pub fn get_asset_table(&self, pid: &Il2CppString, cloth_type: EventDemoSequenceClothType) -> &'static AssetTableResult; // Offset: 0x24D9240 Flags: 0
    #[unity::class_method(51)] pub fn func_character_create(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D9390 Flags: 0
    #[unity::class_method(52)] pub fn func_character_delete(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D9BC0 Flags: 0
    #[unity::class_method(53)] pub fn func_character_adjust_pos(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24D9D40 Flags: 0
    #[unity::class_method(54)] pub fn func_character_show_hide(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DA3B0 Flags: 0
    #[unity::class_method(55)] pub fn func_character_set_animator(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DA6C0 Flags: 0
    #[unity::class_method(56)] pub fn func_character_play_motion(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DAAB0 Flags: 0
    #[unity::class_method(57)] pub fn func_character_wait_motion(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DB100 Flags: 0
    #[unity::class_method(58)] pub fn set_character_angle(&self, character_self: &Character, character_eye_target: &Character, character_head_target: &Character); // Offset: 0x24DB3B0 Flags: 0
    #[unity::class_method(59)] pub fn func_character_set_angle(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DB6F0 Flags: 0
    #[unity::class_method(60)] pub fn set_character_angle2(&self, self_pid: &Il2CppString, eye_target_pid: &Il2CppString, head_target_pid: &Il2CppString) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DBAE0 Flags: 0
    #[unity::class_method(61)] pub fn func_character_reset_angle(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DBDA0 Flags: 0
    #[unity::class_method(62)] pub fn func_character_set_rotate(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DC0A0 Flags: 0
   // #[unity::class_method(63)] pub fn set_character_rotate(&self, character: &Character, character_work: &EventDemoSequenceCharacterWork, rotate_y: f32, sec: f32); // Offset: 0x24DC6A0 Flags: 0
    #[unity::class_method(64)] pub fn func_character_equip_weapon(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DC780 Flags: 0
    #[unity::class_method(65)] pub fn func_character_equip_no_weapon(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DCC50 Flags: 0
    #[unity::class_method(66)] pub fn func_character_equip_fishing_rod(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DCE70 Flags: 0
    #[unity::class_method(67)] pub fn func_character_equip_no_fishing_rod(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DD230 Flags: 0
    //#[unity::class_method(68)] pub fn get_effect_work(&self, effect_name: &Il2CppString, is_warning: bool) -> &'static EventDemoSequenceEffectWork; // Offset: 0x24DD460 Flags: 0
    #[unity::class_method(69)] pub fn func_create_effect(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DD510 Flags: 0
    #[unity::class_method(70)] pub fn func_delete_effect(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DD640 Flags: 0
    #[unity::class_method(76)] pub fn func_talk_face_begin(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DDBF0 Flags: 0
    #[unity::class_method(77)] pub fn func_picture_show(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DDC30 Flags: 0
    #[unity::class_method(78)] pub fn func_picture_hide(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DDF50 Flags: 0
    #[unity::class_method(79)] pub fn func_chapter_title_show(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DE0C0 Flags: 0
    #[unity::class_method(80)] pub fn func_sound_event(&self, cmd_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x24DE1E0 Flags: 0
    #[unity::class_method(83)] pub fn find_game_object(name: &Il2CppString) -> Option<&'static GameObject>; // Offset: 0x24D5220 Flags: 0

}
#[unity::class("", "CmdFunc")]
#[nested_from_type(EventDemoSequence)]
pub struct EventDemoSequenceCmdFunc {
    pub method_ptr: *const u8,
    invoke_impl: *const u8,
    pub target_obj: &'static EventDemoSequence,
    pub method: Option<&'static MethodInfo>,
}

#[allow(non_upper_case_globals)]
/// Function Keys are used in App.EventDemoSequence$$SetupCommands (0x24de270)
/// Methods of fn(&EventDemoSequence, &CmdInfo) -> CmdResult
impl EventDemoSequenceCmdFunc {
    pub const SetBackground: &'static str = "背景";
    pub const SetSceneCamera: &'static str = "シーンカメラ";
    pub const SoundEvent: &'static str = "サウンドイベント";
    pub const CharacterSetAnimator: &'static str = "キャラアニメーター切替";
    pub const PlayCharacterMotion: &'static str = "キャラモーション再生";
    pub const ShowPicture: &'static str = "一枚絵表示";
    pub const HidePicture: &'static str = "一枚絵非表示";

    #[unity::class_method(0)] pub fn ctor(&self, object: &EventDemoSequence, method_info: &MethodInfo); // Offset: 0x1E564A0 Flags: 0
    #[unity::class_method(1)] pub fn invoke(&self, cmd_func_info: &CmdInfo) -> EventDemoSequenceEventCmdResult; // Offset: 0x1E564C0 Flags: 0
}

#[unity::class("", "CmdInfo")]
#[nested_from_type(EventDemoSequence)]
pub struct CmdInfo {
    pub func: &'static mut EventDemoSequenceCmdFunc, // Offset 0x10, Attr: 1
    pub cmd_name: &'static Il2CppString, // Offset 0x18, Attr: 1
    pub args: &'static mut Array<&'static mut Il2CppString>, // Offset 0x20, Attr: 1
    repeat_counter: i32, // Offset 0x28, Attr: 1
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EventDemoSequenceEventCmdResult{
    Continue = 0,
    ContinueNextFrame = 1,
    Retry = 2,
    RetryNextFrame = 3,
    CmdNotFound = 4,
}

