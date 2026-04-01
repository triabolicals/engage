use unity::{prelude::*, engine::Color};
use crate::unityengine::{GameObject, UnityComponent};
use crate::util::get_singleton_scriptable_object;

#[unity::class("App", "GameColor")]
pub struct GameColor {
    parent: u64,
    pub sample: Color,
    pub allow_attack_position: Color,
    pub prohibited_attack_position: Color,
    pub selected_attack_position: Color,
    pub default_character: Color,
    pub disable_character: Color,
    pub help_character: Color,
    pub character_refinement: Color,
    pub increase_value: Color,
    pub decrease_value: Color,
    pub max_value: Color,
    pub insufficient_value: Color,
    pub engage_move: Color,
    pub engage_move_disabled: Color,
    pub green_text_command: Color,
    pub yellow_text: Color,
    pub support_conversation_unavail: Color,
    pub skill_e: Color,
    pub default_color: Color,
    pub second_color: Color,
    pub dialog: Color,
    pub unselectable_color: Color,
    pub page_navigation_on: Color,
    pub page_navigation_off: Color,
    pub icon_select: Color,
    pub icon_deselect: Color,
    pub cursor_white: Color,
    pub cursor_red: Color,
    pub waiting: Color,
    pub danger_zone_display: Color,
    pub emblem_ring_color: Color,
    pub dark_emblem_ring_color: Color,
    pub window_disable: Color,
    pub illumination: Color,
    pub contour: Color,
}

impl GameColor {
    pub fn get() -> &'static mut Option<&'static mut GameColor> { get_singleton_scriptable_object::<GameColor>() }
}
#[unity::class("App", "GameUI")]
pub struct GameUI {}

impl GameUI {
    pub fn get_instance() -> Option<&'static mut Self> { crate::util::try_get_instance_monobehaviour::<GameUI>() }
    #[unity::class_method(0)] pub fn get_root() -> &'static GameObject; // Offset: 0x250DF40 Flags: 0
    #[unity::class_method(1)] pub fn get_canvas(priority: GameUIPriority) -> &'static GameObject; // Offset: 0x250DFC0 Flags: 0
    #[unity::class_method(2)] pub fn get_canvas2(name: &Il2CppString, priority: GameUIPriority) -> &'static GameObject; // Offset: 0x250E1D0 Flags: 0
    #[unity::class_method(3)] pub fn try_create_canvas(parent: &GameObject, name: &Il2CppString, priority: GameUIPriority) -> Option<&'static GameObject>; // Offset: 0x250E080 Flags: 0
    #[unity::class_method(4)] pub fn try_set_sort_order(go: &GameObject, priority: GameUIPriority); // Offset: 0x250E300 Flags: 0
}
impl UnityComponent for GameUI {}
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameUIPriority {
    MapUnit = 4294967096, // Attr: 17
    MapUI = 4294967196, // Attr: 17
    UnitInfo = 4294967246, // Attr: 17
    Default = 0, // Attr: 17
    LevelUp = 10, // Attr: 17
    TerrainInfo = 50, // Attr: 17
    BasicMenu = 500, // Attr: 17
    TitleBar = 600, // Attr: 17
    Tutorial = 700, // Attr: 17
    Help = 800, // Attr: 17
    Telop = 1000, // Attr: 17
    Movie = 1050, // Attr: 17
    Achieve = 1100, // Attr: 17 TalKUI?
    Fade = 1200, // Attr: 17
    Dialog = 1500, // Attr: 17
    Debug = 2000, // Attr: 17
}