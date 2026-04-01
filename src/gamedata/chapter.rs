use unity::il2cpp::object::Array;
use unity::prelude::{Il2CppString, Il2CppClassData};
use crate::gamedata::{Gamedata, StructBaseFields};
use crate::sequence::gmap_sequence::GmapSpotState;

#[unity::class("App", "ChapterData")]
pub struct ChapterData {
    pub parent: StructBaseFields,
    pub cid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub alpha: f32,
    pub mess: &'static Il2CppString,
    pub event: &'static Il2CppString,
    pub field: &'static Il2CppString,
    pub script_bmap: &'static Il2CppString,
    pub script_encount: &'static Il2CppString,
    pub script_kizuna: &'static Il2CppString,
    pub chapter_title: &'static Il2CppString,
    pub terrain: &'static Il2CppString,
    pub dispos: &'static Il2CppString,
    pub next_chapter: &'static Il2CppString,
    pub gmap_spot: &'static Il2CppString,
    pub gmap_spot_state: i32,
    pub gmap_spot_open_condition: Option<&'static Il2CppString>,
    pub gmap_spot_encount: i32,
    pub encount_jobs: &'static Array<&'static Il2CppString>,
    pub reward: &'static Il2CppString,
    pub hold_level: u8,
    pub progress: u8,
    pub flag: i32,
    pub sound_field_situation: &'static Il2CppString,
    pub player_phase_bgm: &'static Il2CppString,
    pub enemy_phase_bgm: &'static Il2CppString,
    pub ally_phase_bgm: &'static Il2CppString,
    pub player_encount_bgm: &'static Il2CppString,
    pub enemy_encount_bgm: &'static Il2CppString,
    pub sortie_bgm: &'static Il2CppString,
    pub kizuna_bgm: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub recommended_level: u8,
    pub nation: &'static Il2CppString,
    pub nation_name: &'static Il2CppString,
    pub net_kill_bonus_index: u8,
    pub hub: &'static Il2CppString, // Offset 0x128, Attr: 1
    pub prefixless_cid: &'static Il2CppString, // Offset 0x130, Attr: 1
    pub cleared_flag_name: &'static Il2CppString, // Offset 0x138, Attr: 1
    pub gmap_spot_flag_name: &'static Il2CppString, // Offset 0x140, Attr: 1
    pub place_name: &'static Il2CppString, // Offset 0x148, Attr: 1
}

impl Gamedata for ChapterData {}

impl ChapterData {
    #[unity::class_method(80)] pub fn get_cleared_flag_name(&self) -> &'static Il2CppString; // Offset: 0x2AF9B40 Flags: 0
    #[unity::class_method(81)] pub fn get_gmap_spot_flag_name(&self) -> &'static Il2CppString; // Offset: 0x2AF9B50 Flags: 0
    #[unity::class_method(82)] pub fn is_scenario(&self) -> bool; // Offset: 0x2AF9B60 Flags: 0
    #[unity::class_method(83)] pub fn is_dlc(&self) -> bool; // Offset: 0x2AF9B70 Flags: 0
    #[unity::class_method(84)] pub fn is_dlc_god(&self) -> bool; // Offset: 0x2AF9C20 Flags: 0
    #[unity::class_method(85)] pub fn is_dlc_evil(&self) -> bool; // Offset: 0x2AF9C80 Flags: 0
    #[unity::class_method(86)] pub fn is_last_evil(&self) -> bool; // Offset: 0x2AF9CE0 Flags: 0
    #[unity::class_method(87)] pub fn is_encountable_type(&self) -> bool; // Offset: 0x2AF9E80 Flags: 0
    #[unity::class_method(88)] pub fn is_training(&self) -> bool; // Offset: 0x2AF9E90 Flags: 0
    #[unity::class_method(89)] pub fn is_unknown(&self) -> bool; // Offset: 0x2AF9EC0 Flags: 0
    #[unity::class_method(90)] pub fn is_invalid(&self) -> bool; // Offset: 0x2AFA040 Flags: 0
    #[unity::class_method(91)] pub fn try_get_spot_state(&self, state: GmapSpotState) -> bool; // Offset: 0x2AFA150 Flags: 0
    #[unity::class_method(92)] pub fn try_set_spot_state(&self, state: GmapSpotState) -> bool; // Offset: 0x2AFA260 Flags: 0
    #[unity::class_method(94)] pub fn get_next_activating_sub_chapters(&self) -> &'static Array<&'static ChapterData>; // Offset: 0x2AFA520 Flags: 0
    #[unity::class_method(103)] pub fn get_next_chapter(&self) -> Option<&'static ChapterData>; // Offset: 0x2AF9DE0 Flags: 0
}
