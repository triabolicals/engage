use num_derive::{FromPrimitive, ToPrimitive};
use super::*;
use unity::prelude::Il2CppString;
use crate::unit::Unit;
use crate::util::get_singleton_proc_instance;

pub mod human;
pub mod battle;
pub mod summon;

#[repr(i32)]
#[derive(PartialEq, Clone, FromPrimitive, ToPrimitive)]
pub enum MapSequenceLabel {
    Init = 0,
    Tick = 1,
    Sortie = 2,
    ResumeMap = 3,
    ResumeSortie = 4,
    SkipSortie = 5,
    MapStart = 6,
    MapBegin = 7,
    TurnBegin = 8,
    TurnBeginAfterRewind = 9,
    TurnBranch = 10,
    TurnBranchAfterRewind = 11,
    TurnHuman = 12,
    TurnAI = 13,
    TurnLink = 14,
    TurnReplay = 15,
    TurnEnd = 16,
    Complete = 17,
    GameOver = 18,
    RelayUnsettled = 19,
    SaveDataLoad = 20,
    RestartLoad = 21,
    RelayLoad = 22,
    RelayLoadError = 23,
    RelaySkipReplay = 24,
    RelayReplayToTakeOver = 25,
    VersusLoad = 26,
    VersusEdit = 27,
    End = 28,
    Tail = 29,
}

#[unity::class("App", "MapSequence")]
pub struct MapSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,    //0x61
    pub is_loaded: bool,    //0x62
    pub scene_name: &'static Il2CppString,  //0x68
    pub scene_mode: i32,    //0x70
    pub padding: i32, //    0x074
    pub is_completed: bool,
    pub is_sortie_cancel: bool,
    challenge_failer: bool,
    pre_load_handle: u64,
    pub time: f32,
    //
}

impl Bindable for MapSequence {}
impl MapSequence {
    pub const MAP_SEQUENCE: i32 = 1650205480;
    pub fn get_instance() -> Option<&'static mut Self> {
        get_singleton_proc_instance::<Self>()
    }
    #[unity::class_method(7)] pub fn init(&self); // Offset: 0x2363660 Flags: 0
    // #[unity::class_method(8)] pub fn get_grow_mode(grow: GrowMode) -> GrowMode; // Offset: 0x2363870 Flags: 0
    #[unity::class_method(9)] pub fn setup_chapter(&self); // Offset: 0x2363BE0 Flags: 0
    #[unity::class_method(10)] pub fn load_script(&self); // Offset: 0x2364010 Flags: 0
    #[unity::class_method(11)] pub fn unload_script(&self); // Offset: 0x2364170 Flags: 0
    #[unity::class_method(12)] pub fn opening_event(&self); // Offset: 0x2364270 Flags: 0
    #[unity::class_method(13)] pub fn map_opening_event_for_replay(&self); // Offset: 0x2364340 Flags: 0
    #[unity::class_method(14)] pub fn setup_field_a(&self); // Offset: 0x23643E0 Flags: 0
    #[unity::class_method(15)] pub fn setup_field_b(&self); // Offset: 0x2364A20 Flags: 0
    #[unity::class_method(16)] pub fn post_setup_field(&self); // Offset: 0x2364CF0 Flags: 0
    #[unity::class_method(17)] pub fn load_dispos(&self); // Offset: 0x23652C0 Flags: 0
    #[unity::class_method(18)] pub fn unload_dispos(&self); // Offset: 0x2365690 Flags: 0
    #[unity::class_method(19)] pub fn dispos_event(&self); // Offset: 0x23656A0 Flags: 0
    #[unity::class_method(20)] pub fn dispos_unit(&self); // Offset: 0x23656B0 Flags: 0
    #[unity::class_method(21)] pub fn load_async_actor(&self); // Offset: 0x2365900 Flags: 0
    // #[unity::class_method(23)] pub fn add_preload_combat_assets2(&self, result: &AssetTableResult); // Offset: 0x2365BB0 Flags: 0
    #[unity::class_method(24)] pub fn preload_combat_assets(&self); // Offset: 0x2365D30 Flags: 0
    #[unity::class_method(25)] pub fn load_menu(&self); // Offset: 0x2365F70 Flags: 0
    #[unity::class_method(26)] pub fn is_loading_menu(&self) -> bool; // Offset: 0x23660E0 Flags: 0
    #[unity::class_method(27)] pub fn unload_menu(&self); // Offset: 0x23662B0 Flags: 0
    #[unity::class_method(28)] pub fn is_loading(&self) -> bool; // Offset: 0x2366420 Flags: 0
    #[unity::class_method(29)] pub fn setup_view_mode(&self); // Offset: 0x23665D0 Flags: 0
    #[unity::class_method(30)] pub fn setup_map(&self); // Offset: 0x2366630 Flags: 0
    #[unity::class_method(31)] pub fn commit_temporary(&self); // Offset: 0x2367160 Flags: 0
    #[unity::class_method(32)] pub fn resume_branch(&self); // Offset: 0x23671C0 Flags: 0
    #[unity::class_method(33)] pub fn unload_actor(&self); // Offset: 0x23672F0 Flags: 0
    #[unity::class_method(34)] pub fn unload_combat_assets(&self); // Offset: 0x23674F0 Flags: 0
    #[unity::class_method(35)] pub fn cleanup_field(&self); // Offset: 0x2367620 Flags: 0
    #[unity::class_method(36)] pub fn cleanup_units(&self); // Offset: 0x2367B80 Flags: 0
    #[unity::class_method(37)] pub fn sortie(&self); // Offset: 0x2367C40 Flags: 0
    #[unity::class_method(38)] pub fn sortie_branch(&self); // Offset: 0x23681B0 Flags: 0
    #[unity::class_method(39)] pub fn pre_sortie(&self); // Offset: 0x2367DF0 Flags: 0
    #[unity::class_method(40)] pub fn post_sortie(&self); // Offset: 0x2368A90 Flags: 0
    #[unity::class_method(41)] pub fn map_begin(&self); // Offset: 0x2369270 Flags: 0
    #[unity::class_method(42)] pub fn map_history_begin(&self); // Offset: 0x2369FD0 Flags: 0
    #[unity::class_method(43)] pub fn post_map_begin(&self); // Offset: 0x236A040 Flags: 0
    #[unity::class_method(44)] pub fn map_end(&self); // Offset: 0x236A190 Flags: 0
    #[unity::class_method(45)] pub fn turn_begin(&self); // Offset: 0x236B5B0 Flags: 0
    #[unity::class_method(46)] pub fn turn_event(&self); // Offset: 0x236B9E0 Flags: 0
    #[unity::class_method(47)] pub fn turn_begin_after(&self); // Offset: 0x236BA80 Flags: 0
    #[unity::class_method(49)] pub fn try_wait_time(&self, time: f32); // Offset: 0x236BF20 Flags: 0
    #[unity::class_method(50)] pub fn turn_after_event(&self); // Offset: 0x236BF60 Flags: 0
    #[unity::class_method(51)] pub fn turn_end_event(&self); // Offset: 0x236C000 Flags: 0
    // #[unity::class_method(52)] pub fn get_under_roof_unit_count(&self, force: ForceType) -> i32; // Offset: 0x236C0A0 Flags: 0
    #[unity::class_method(53)] pub fn turn_skip(&self); // Offset: 0x236C410 Flags: 0
    #[unity::class_method(54)] pub fn get_first_unit(&self) -> &'static Unit; // Offset: 0x2366CC0 Flags: 0
    #[unity::class_method(55)] pub fn turn_action(&self); // Offset: 0x236C6C0 Flags: 0
    #[unity::class_method(56)] pub fn turn_effect(&self); // Offset: 0x236C7A0 Flags: 0
    #[unity::class_method(57)] pub fn turn_entrust(&self); // Offset: 0x236C8C0 Flags: 0
    #[unity::class_method(58)] pub fn auto_save(&self); // Offset: 0x236C9A0 Flags: 0
    #[unity::class_method(59)] pub fn turn_branch(&self); // Offset: 0x236C9F0 Flags: 0
    #[unity::class_method(60)] pub fn human_start(&self); // Offset: 0x236CB30 Flags: 0
    #[unity::class_method(61)] pub fn post_human_aibranch(&self); // Offset: 0x236CBA0 Flags: 0
    #[unity::class_method(62)] pub fn replay_start(&self); // Offset: 0x236D0D0 Flags: 0
    #[unity::class_method(63)] pub fn turn_end(&self); // Offset: 0x236D130 Flags: 0
    #[unity::class_method(64)] pub fn turn_next(&self); // Offset: 0x236D4C0 Flags: 0
    #[unity::class_method(65)] pub fn game_end_branch(&self); // Offset: 0x236D540 Flags: 0
    #[unity::class_method(66)] pub fn update_reliance(&self); // Offset: 0x236D7A0 Flags: 0
    #[unity::class_method(67)] pub fn create_complete_telop(&self); // Offset: 0x236D7B0 Flags: 0
    #[unity::class_method(68)] pub fn complete(&self); // Offset: 0x236D880 Flags: 0
    #[unity::class_method(69)] pub fn get_encount_reward(&self); // Offset: 0x236DE10 Flags: 0
    #[unity::class_method(70)] pub fn try_ending(&self); // Offset: 0x236E040 Flags: 0
    #[unity::class_method(71)] pub fn try_challenge_result(&self); // Offset: 0x236E130 Flags: 0
    #[unity::class_method(72)] pub fn try_restart_map_result(&self); // Offset: 0x236E8E0 Flags: 0
    #[unity::class_method(73)] pub fn game_over(&self); // Offset: 0x236EBB0 Flags: 0
    #[unity::class_method(74)] pub fn try_restart(&self); // Offset: 0x236EF10 Flags: 0
    #[unity::class_method(75)] pub fn save_data_load(&self); // Offset: 0x236F0D0 Flags: 0
    #[unity::class_method(76)] pub fn save_data_load_result(&self); // Offset: 0x236F1B0 Flags: 0
    #[unity::class_method(77)] pub fn save_data_release(&self); // Offset: 0x236F380 Flags: 0
    #[unity::class_method(78)] pub fn save_data_normalize(&self); // Offset: 0x236F450 Flags: 0
    // #[unity::class_method(79)] pub fn try_restart2(&self, target: GameUserRestartDataTargtes) -> bool; // Offset: 0x236F460 Flags: 0
    #[unity::class_method(80)] pub fn save_data_after(&self); // Offset: 0x236F510 Flags: 0
    #[unity::class_method(81)] pub fn restart_load(&self); // Offset: 0x236F680 Flags: 0
    #[unity::class_method(82)] pub fn unit_contienud(&self); // Offset: 0x236A9D0 Flags: 0
    #[unity::class_method(83)] pub fn unit_resurrect(&self); // Offset: 0x236AEA0 Flags: 0
    #[unity::class_method(84)] pub fn begin_silent_env(&self); // Offset: 0x236F6D0 Flags: 0
    #[unity::class_method(85)] pub fn end_silent_env(&self); // Offset: 0x236F740 Flags: 0
    #[unity::class_method(86)] pub fn download(&self); // Offset: 0x236F7B0 Flags: 0
    #[unity::class_method(87)] pub fn put_bonus(&self); // Offset: 0x236F800 Flags: 0
    #[unity::class_method(88)] pub fn upload(&self); // Offset: 0x236FA80 Flags: 0
    #[unity::class_method(89)] pub fn ranking_register_unit(&self); // Offset: 0x236FA90 Flags: 0
    #[unity::class_method(90)] pub fn versus_register_unit(&self); // Offset: 0x236FB30 Flags: 0
    #[unity::class_method(91)] pub fn relay_load(&self); // Offset: 0x236FC00 Flags: 0
    #[unity::class_method(92)] pub fn relay_load_error(&self); // Offset: 0x236FE60 Flags: 0
    #[unity::class_method(93)] pub fn relay_show_replay_player_name(&self); // Offset: 0x236FEF0 Flags: 0
    #[unity::class_method(94)] pub fn relay_hide_replay_player_name(&self); // Offset: 0x236FFC0 Flags: 0
    #[unity::class_method(95)] pub fn relay_message_show(&self); // Offset: 0x236FFD0 Flags: 0
    #[unity::class_method(96)] pub fn relay_show_win_rule_for_take_over(&self); // Offset: 0x23701F0 Flags: 0
    #[unity::class_method(97)] pub fn relay_skip_replay(&self); // Offset: 0x23703C0 Flags: 0
    #[unity::class_method(98)] pub fn relay_replay_to_take_over(&self); // Offset: 0x23704E0 Flags: 0
    #[unity::class_method(99)] pub fn is_relay(&self) -> bool; // Offset: 0x2362DC0 Flags: 0
    #[unity::class_method(100)] pub fn is_challenge(&self) -> bool; // Offset: 0x23691C0 Flags: 0
    #[unity::class_method(101)] pub fn versus_load(&self); // Offset: 0x2370590 Flags: 0
    #[unity::class_method(102)] pub fn versus_branch(&self); // Offset: 0x23706E0 Flags: 0
    #[unity::class_method(103)] pub fn start_map_edit(&self); // Offset: 0x2370990 Flags: 0
    #[unity::class_method(104)] pub fn is_versus(&self) -> bool; // Offset: 0x2362E40 Flags: 0
    #[unity::class_method(105)] pub fn try_patch(&self); // Offset: 0x2370A00 Flags: 0
    #[unity::class_method(107)] pub fn on_persistent(&self); // Offset: 0x2370B70 Flags: 0
    #[unity::class_method(108)] pub fn get_debug_log(&self) -> &'static Il2CppString; // Offset: 0x2370C70 Flags: 0
    #[unity::class_method(113)] pub fn timer_start(&self); // Offset: 0x23715E0 Flags: 0
    #[unity::class_method(114)] pub fn timer_stop_actor(&self); // Offset: 0x2371650 Flags: 0
    #[unity::class_method(115)] pub fn timer_stop_character(&self); // Offset: 0x23716A0 Flags: 0
    #[unity::class_method(116)] pub fn timer_stop(&self, name: &Il2CppString); // Offset: 0x2371690 Flags: 0
    #[unity::class_method(117)] pub fn try_game_over_rewind(&self); // Offset: 0x23716E0 Flags: 0
    #[unity::class_method(118)] pub fn stop_bgm(&self); // Offset: 0x2371780 Flags: 0
}