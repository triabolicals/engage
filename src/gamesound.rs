use unity::prelude::*;

use crate::{combat::Character, soundmanager::SoundSystemSoundHandle};
use crate::battle::BattleCalculator;
use crate::force::ForceType;
use crate::gamedata::{ChapterData, ItemData, PersonData};
use crate::proc::Bindable;
use crate::unit::Unit;
use crate::unityengine::GameObject;

#[unity::class("App", "GameSound")]
pub struct GameSound {}

impl GameSound {
    pub fn post_event<'a>(event_name: impl Into<&'a Il2CppString>, character: Option<&Character>) -> &'static GameSoundHandle {
        Self::post_event_(event_name.into(), character)
    }
    pub fn is_event_loaded<'a>(event_name: impl Into<&'a Il2CppString>) -> bool {
        Self::is_event_loaded(event_name.into())
    }
    #[unity::class_method(2)]  pub fn is_enable() -> bool; // Offset: 0x2287150 Flags: 0
    #[unity::class_method(34)] pub fn is_event_loaded_(event_name: &Il2CppString) -> bool; // Offset: 0x2289000 Flags: 0
    #[unity::class_method(35)] pub fn is_event_playing(event_name: &Il2CppString) -> bool; // Offset: 0x2289120 Flags: 0
    #[unity::class_method(39)] pub fn post_event_(event_name: &Il2CppString, character: Option<&Character>) -> &'static GameSoundHandle; // Offset: 0x2272FD0 Flags: 0
    #[unity::class_method(57)] pub fn stop_all_bgm(fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228AB90 Flags: 0
    #[unity::class_method(58)] pub fn stop_all_se(fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228AD90 Flags: 0
    #[unity::class_method(59)] pub fn stop_all_voice(fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228AE10 Flags: 0
    #[unity::class_method(60)] pub fn stop_all_env(fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228AE90 Flags: 0
    #[unity::class_method(61)] pub fn stop_all(fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228AF10 Flags: 0
    #[unity::class_method(62)] pub fn stop_all_common(event_name_base: &Il2CppString, fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228AC10 Flags: 0
    #[unity::class_method(81)] pub fn field_bgm_init(); // Offset: 0x228BDA0 Flags: 0
    #[unity::class_method(82)] pub fn field_bgm_final(); // Offset: 0x228BEB0 Flags: 0
    #[unity::class_method(83)] pub fn field_bgm_is_set_phase_bgm() -> bool; // Offset: 0x228BFE0 Flags: 0
    #[unity::class_method(84)] pub fn field_bgm_set_phase_bgm(chapter: &ChapterData, is_encount: bool); // Offset: 0x228C0F0 Flags: 0
    #[unity::class_method(85)] pub fn field_bgm_restore_phase_bgm(); // Offset: 0x228C220 Flags: 0
    #[unity::class_method(86)] pub fn field_bgm_set_phase_bgm2(player_phase_bgm: &Il2CppString, enemy_phase_bgm: &Il2CppString, ally_phase_bgm: &Il2CppString) -> bool; // Offset: 0x228C330 Flags: 0
    #[unity::class_method(87)] pub fn field_bgm_play(force_type: ForceType); // Offset: 0x228C470 Flags: 0
    #[unity::class_method(88)] pub fn field_bgm_stop(fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228C590 Flags: 0
    #[unity::class_method(89)] pub fn field_bgm_stop2(fade_msec: i32); // Offset: 0x228C630 Flags: 0
    #[unity::class_method(90)] pub fn field_bgm_pause(fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228C750 Flags: 0
    #[unity::class_method(91)] pub fn field_bgm_pause2(fade_msec: i32); // Offset: 0x228C7F0 Flags: 0
    #[unity::class_method(92)] pub fn field_bgm_resume(fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228C910 Flags: 0
    #[unity::class_method(93)] pub fn field_bgm_resume2(fade_msec: i32); // Offset: 0x228C9B0 Flags: 0
    #[unity::class_method(94)] pub fn field_bgm_set_volume(vol: f32, fade_speed_type: GameSoundFadeSpeedType); // Offset: 0x228CAD0 Flags: 0
    #[unity::class_method(95)] pub fn field_bgm_set_volume2(vol: f32, fade_msec: i32); // Offset: 0x228CB70 Flags: 0
    #[unity::class_method(96)] pub fn field_bgm_set_war_situation_param(war_situation_state_name: &Il2CppString); // Offset: 0x228CCA0 Flags: 0
    #[unity::class_method(97)] pub fn field_bgm_restore_war_situation_param(); // Offset: 0x228CDC0 Flags: 0
    #[unity::class_method(98)] pub fn field_bgm_start_special_battle_bgm_continue_turn(); // Offset: 0x228CED0 Flags: 0
    #[unity::class_method(99)] pub fn field_bgm_set_special_battle_bgm_continue_turn_for_rewind(turn: i32); // Offset: 0x228CFE0 Flags: 0
    #[unity::class_method(100)] pub fn field_bgm_set_first_played_flag(); // Offset: 0x228D100 Flags: 0
    #[unity::class_method(101)] pub fn field_bgm_change_force_type<B>(force_type: ForceType, proc: &B) where B: Bindable; // Offset: 0x228D210 Flags: 0
    #[unity::class_method(102)] pub fn field_bgm_change_force_type_imm(force_type: ForceType); // Offset: 0x228D340 Flags: 0
    #[unity::class_method(103)] pub fn field_bgm_play_special_battle_bgm(event_name: &Il2CppString); // Offset: 0x228D460 Flags: 0
    #[unity::class_method(104)] pub fn field_bgm_pause_special_battle_bgm(event_name: &Il2CppString); // Offset: 0x228D580 Flags: 0
    #[unity::class_method(105)] pub fn field_bgm_is_special_battle_bgm_exist(event_name: &Il2CppString) -> bool; // Offset: 0x228D6A0 Flags: 0
    #[unity::class_method(106)] pub fn field_bgm_to_pre_battle_bgm(calculator: &BattleCalculator, is_simple_battle: bool); // Offset: 0x228D7C0 Flags: 0
    #[unity::class_method(107)] pub fn field_bgm_to_main_battle_bgm(calculator: &BattleCalculator, is_simple_battle: bool); // Offset: 0x228E560 Flags: 0
    #[unity::class_method(108)] pub fn field_bgm_return_from_battle_bgm(calculator: &BattleCalculator, is_simple_battle: bool); // Offset: 0x228E660 Flags: 0
    #[unity::class_method(109)] pub fn field_bgm_is_special_battle_bgm(calculator: &BattleCalculator) -> bool; // Offset: 0x228D930 Flags: 0
    #[unity::class_method(110)] pub fn field_bgm_get_special_battle_bgm_event_name(calculator: &BattleCalculator) -> &'static Il2CppString; // Offset: 0x228DEB0 Flags: 0
    #[unity::class_method(111)] pub fn is_leader_enemy_unit(unit: &Unit) -> bool; // Offset: 0x228E720 Flags: 0
    #[unity::class_method(112)] pub fn is_combat_bgm_exist(person: &PersonData) -> bool; // Offset: 0x228E6F0 Flags: 0
    #[unity::class_method(117)] pub fn set_state_phase(turn: ForceType); // Offset: 0x228F0D0 Flags: 0
    #[unity::class_method(118)] pub fn set_state_field_situation(name: &Il2CppString); // Offset: 0x228F270 Flags: 0
    #[unity::class_method(119)] pub fn set_state_special_battle(name: &Il2CppString); // Offset: 0x228E420 Flags: 0
    #[unity::class_method(120)] pub fn restore_special_battle_param(); // Offset: 0x228F3C0 Flags: 0
    #[unity::class_method(121)] pub fn push_state_map_or_combat(name: &Il2CppString); // Offset: 0x228F500 Flags: 0
    #[unity::class_method(122)] pub fn pop_state_map_or_combat(); // Offset: 0x228F620 Flags: 0
    #[unity::class_method(129)] pub fn item_get(item: &ItemData, is_force_important: bool); // Offset: 0x227F020 Flags: 0
    #[unity::class_method(130)] pub fn money_get(); // Offset: 0x227F6B0 Flags: 0
    #[unity::class_method(131)] pub fn join_unit(); // Offset: 0x228FDA0 Flags: 0
    #[unity::class_method(132)] pub fn select_unit(); // Offset: 0x228FE20 Flags: 0
    #[unity::class_method(133)] pub fn exchange_unit(); // Offset: 0x228FEA0 Flags: 0
    #[unity::class_method(134)] pub fn cancel(); // Offset: 0x228FF20 Flags: 0
    #[unity::class_method(135)] pub fn failure(); // Offset: 0x228FFA0 Flags: 0
    #[unity::class_method(157)]
    pub fn talk_voice(event_name: &Il2CppString); // Offset: 0x2291E00 Flags: 0
    #[unity::class_method(158)]
    pub fn talk_voice2(event_name: &Il2CppString, character: Option<&Character>); // Offset: 0x2291EC0 Flags: 0
    #[unity::class_method(159)]
    pub fn person_voice(person_switch_name: &Il2CppString, engage_switch_name: Option<&Il2CppString>,event_name: &Il2CppString); // Offset: 0x2291F40 Flags: 0
    #[unity::class_method(160)]
    pub fn person_voice2(person_switch_name: &Il2CppString, engage_switch_name: Option<&Il2CppString>, event_name: Option<&Il2CppString>, character: Option<&Character>); // Offset: 0x2291FD0 Flags: 0
    #[unity::class_method(161)]
    pub fn person_voice3(game_object: &GameObject, person_switch_name: &Il2CppString, engage_switch_name: Option<&Il2CppString>, event_name: Option<&Il2CppString>); // Offset: 0x22921E0 Flags: 0
    #[unity::class_method(162)]
    pub fn person_voice4(go: &GameObject, person_switch_name: &Il2CppString, engage_switch_name: Option<&Il2CppString>, event_name: Option<&Il2CppString>, character: Option<&Character>); // Offset: 0x2292270 Flags: 0
    /*
    #[unity::class_method(0)] pub fn get_fade_msec_by_fade_speed_type(fade_speed_type: GameSoundFadeSpeedType) -> i32; // Offset: 0x2287070 Flags: 0
    #[unity::class_method(1)] pub fn get_string_by_fade_speed_type(fade_speed_type: GameSoundFadeSpeedType) -> &'static Il2CppString; // Offset: 0x22870A0 Flags: 0

    #[unity::class_method(3)] pub fn is_game_skip() -> bool; // Offset: 0x22872E0 Flags: 0
    #[unity::class_method(4)] pub fn is_use_game_sound_mode() -> bool; // Offset: 0x2287320 Flags: 0
    #[unity::class_method(5)] pub fn get_audio_listener_object() -> &'static GameObject; // Offset: 0x2287330 Flags: 0
    #[unity::class_method(6)] pub fn add_base_paths(); // Offset: 0x2287410 Flags: 0
    #[unity::class_method(7)] pub fn get_patch_ver_num() -> i32; // Offset: 0x22875E0 Flags: 0
    #[unity::class_method(8)] pub fn get_patch_package_file_name(patch_index: i32) -> &'static Il2CppString; // Offset: 0x22875F0 Flags: 0
    #[unity::class_method(9)] pub fn get_patch_package_file_path(patch_index: i32) -> &'static Il2CppString; // Offset: 0x22876A0 Flags: 0
    #[unity::class_method(10)] pub fn load_package_files(); // Offset: 0x2287860 Flags: 0
    #[unity::class_method(11)] pub fn load_package_file(patch_index: i32) -> bool; // Offset: 0x2287980 Flags: 0
    #[unity::class_method(12)] pub fn load_package_file_impl(package_file_path: &Il2CppString) -> bool; // Offset: 0x22879F0 Flags: 0
    #[unity::class_method(13)] pub fn convert_to_dlcsound_bank_name(original_bank_name: &Il2CppString, package_file_name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x2287B40 Flags: 0
    #[unity::class_method(14)] pub fn initialize(); // Offset: 0x2287BE0 Flags: 0
    #[unity::class_method(15)] pub fn load_default_sound_banks(); // Offset: 0x2287C60 Flags: 0
    #[unity::class_method(16)] pub fn post_initialize(); // Offset: 0x2288010 Flags: 0
    #[unity::class_method(17)] pub fn is_initialized() -> bool; // Offset: 0x22871C0 Flags: 0
    #[unity::class_method(18)] pub fn reset_master_volume(); // Offset: 0x2288240 Flags: 0
    #[unity::class_method(19)] pub fn serialize(stream: &Stream); // Offset: 0x2288350 Flags: 0
    #[unity::class_method(20)] pub fn deserialize(stream: &Stream); // Offset: 0x22883E0 Flags: 0
    #[unity::class_method(21)] pub fn set_language(language: LanguageVoices); // Offset: 0x2288470 Flags: 0
    #[unity::class_method(22)] pub fn set_state_language(language: LanguageVoices); // Offset: 0x2288190 Flags: 0
    #[unity::class_method(23)] pub fn load_bank(bank_name: &Il2CppString); // Offset: 0x22885B0 Flags: 0
    #[unity::class_method(24)] pub fn load_bank_async(bank_name: &Il2CppString); // Offset: 0x22886E0 Flags: 0
    #[unity::class_method(25)] pub fn load_banks(bank_names: &Array<String>); // Offset: 0x2288810 Flags: 0
    #[unity::class_method(26)] pub fn load_banks_async(bank_names: &Array<String>); // Offset: 0x22888D0 Flags: 0
    #[unity::class_method(27)] pub fn unload_bank(bank_name: &Il2CppString); // Offset: 0x2288990 Flags: 0
    #[unity::class_method(28)] pub fn unload_banks(bank_names: &Array<String>); // Offset: 0x2288AC0 Flags: 0
    #[unity::class_method(29)] pub fn is_loading_bank() -> bool; // Offset: 0x2288B80 Flags: 0
    #[unity::class_method(30)] pub fn is_loading_bank2(bank_name: &Il2CppString) -> bool; // Offset: 0x2288C90 Flags: 0
    #[unity::class_method(31)] pub fn load_chapter_bank(chapter: &ChapterData); // Offset: 0x2288DC0 Flags: 0
    #[unity::class_method(32)] pub fn unload_chapter_bank(); // Offset: 0x2288EC0 Flags: 0
    #[unity::class_method(33)] pub fn clear_prepare(); // Offset: 0x2288F80 Flags: 0

    #[unity::class_method(36)] pub fn find_sound_handle_by_event_name(event_name: &Il2CppString) -> &'static GameSoundHandle; // Offset: 0x2289240 Flags: 0
    #[unity::class_method(37)] pub fn find_sound_handles_by_event_name(event_name: &Il2CppString) -> List<&'static GameSoundHandle>; // Offset: 0x22893A0 Flags: 0
    #[unity::class_method(38)] pub fn get_sound_handle_list(prefix: &Il2CppString) -> List<&'static GameSoundHandle>; // Offset: 0x2289630 Flags: 0


    #[unity::class_method(113)] pub fn load_system_voice(person_switch_name: &Il2CppString) -> &'static GameSoundResultLoad; // Offset: 0x228E850 Flags: 0
    #[unity::class_method(114)] pub fn unload_system_voice(person_switch_name: &Il2CppString); // Offset: 0x228EA70 Flags: 0
    #[unity::class_method(115)] pub fn load_system_voice_for_engage_in_combat(engage_switch_name: &Il2CppString) -> &'static GameSoundResultLoad; // Offset: 0x228EC10 Flags: 0
    #[unity::class_method(116)] pub fn unload_system_voice_for_engage_in_combat(engage_switch_name: &Il2CppString); // Offset: 0x228EF30 Flags: 0

    #[unity::class_method(123)] pub fn set_switch_person(name: &Il2CppString) -> bool; // Offset: 0x228F730 Flags: 0
    #[unity::class_method(124)] pub fn set_switch_person2(game_object: &GameObject, name: &Il2CppString) -> bool; // Offset: 0x228F880 Flags: 0
    #[unity::class_method(125)] pub fn set_switch_engage(name: &Il2CppString) -> bool; // Offset: 0x228F9E0 Flags: 0
    #[unity::class_method(126)] pub fn set_switch_engage2(game_object: &GameObject, name: &Il2CppString) -> bool; // Offset: 0x228FB30 Flags: 0
    #[unity::class_method(127)] pub fn convert_hero_engage_switch_name(name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x228EE20 Flags: 0
    #[unity::class_method(128)] pub fn set_switch_weapon(weapon_name: &Il2CppString, game_object: &GameObject, item_kind: ItemDataKinds); // Offset: 0x228FC90 Flags: 0
    */
}

#[unity::class("", "Handle")]
#[nested_from_type(GameSound)]
pub struct GameSoundHandle {}

impl GameSoundHandle {
    pub fn new(sound_handle: &SoundSystemSoundHandle) -> &'static Self {
        let handle_class = GameSound::class().get_nested_types().iter().find(|class| class.get_name() == "Handle").unwrap();
        let handle = Il2CppObject::<GameSoundHandle>::from_class(&handle_class).unwrap();
        handle.ctor(sound_handle);
        handle
    }
    
    fn ctor(&self, sound_handle: &SoundSystemSoundHandle) {
        unsafe { gamesound_handle_ctor(self, sound_handle, None) }
    }
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameSoundFadeSpeedType {
    Immediate = 0, // Attr: 17
    VeryFast = 1, // Attr: 17
    Fast = 2, // Attr: 17
    Normal = 3, // Attr: 17
    Slow = 4, // Attr: 17
    VerySlow = 5, // Attr: 17
}
/*
#[skyline::from_offset(0x2272fd0)]
extern "C" fn gamesound_postevent(event_name: &Il2CppString, character: Option<&Character>, method_info: OptionalMethod) -> *const u8;

#[unity::from_offset("App", "GameSound", "IsEventLoaded")]
extern "C" fn gamesound_iseventloaded(event_name: &Il2CppString, method_info: OptionalMethod) -> bool;

 */
#[skyline::from_offset(0x1e6db20)]
extern "C" fn gamesound_handle_ctor(this: &GameSoundHandle, sound_handle: &SoundSystemSoundHandle, method_info: OptionalMethod);


