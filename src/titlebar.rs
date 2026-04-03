//! Wrappers to open and close a TitleBar when entering a menu.

use unity::il2cpp::object::Array;
use unity::system::List;
use crate::unityengine::{MonoBehaviorFields, Animator, GameObject, UnityComponent};
use unity::prelude::*;
use crate::tmpro::TextMeshProUGUI;
use crate::keyhelp::KeyHelpTitleBarController;

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum TitleBarFooterType{
    Gold = 1,
    PieceOfBond = 2,
    Refine = 4,
    RefineGod = 8,
    Proof = 16,
    RelayTicket = 32,
    None = 0,
    GoldAndBond = 3,
    GoldAndRefine = 5,
    GoldAndBondAndRefine = 7,
    GoldAndBondAndRelayTicket = 35,
}

#[repr(i32)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum KeyHelpButton {
    A = 0,
    B = 1,
    X = 2,
    Y = 3,
    L = 4,
    R = 5,
    LR = 6,
    ZL = 7,
    ZR = 8,
    ZlZr = 9,
    Plus = 10,
    Minus = 11,
    Up = 12,
    Down = 13,
    UpDown = 14,
    Left = 15,
    Right = 16,
    LeftRight = 17,
    LStick = 18,
    RStick = 19,
}

#[unity::class("App", "TitleBar")]
pub struct TitleBar {
    parent: MonoBehaviorFields,
    pub m_animator_header: &'static Animator, // Offset 0x18, Attr: 6
    pub m_animator_footer: &'static Animator, // Offset 0x20, Attr: 6
    pub title0: Option<&'static mut TitleBarTitle>, // Offset 0x28, Attr: 6
    pub title1: Option<&'static mut TitleBarTitle>, // Offset 0x30, Attr: 6
    pub values0: &'static TitleBarValues, // Offset 0x38, Attr: 6
    pub values1: &'static TitleBarValues, // Offset 0x40, Attr: 6
    pub count_time: f32, // Offset 0x48, Attr: 6
    pub current_title: Option<&'static mut TitleBarTitle>, // Offset 0x50, Attr: 1
    pub current_values: &'static TitleBarValues, // Offset 0x58, Attr: 1
    pub header_title: Option<&'static Il2CppString>, // Offset 0x60, Attr: 1
    pub header_title_help: Option<&'static Il2CppString>, // Offset 0x68, Attr: 1
    pub header_key_help: Option<&'static Il2CppString>, // Offset 0x70, Attr: 1
    pub footer_type: TitleBarFooterType, // Offset 0x78, Attr:
    pub is_show_header: bool, // Offset 0x7C, Attr: 1
    pub is_show_footer: bool, // Offset 0x7D, Attr: 1
    pub is_open_header: bool, // Offset 0x7E, Attr: 1
    pub is_open_footer: bool, // Offset 0x7F, Attr: 1
    pub is_same_footer: bool, // Offset 0x80, Attr: 1
    /*
    m_num_display: &'static TitleBarNum, // Offset 0x88, Attr: 1
    m_num_current: &'static TitleBarNum, // Offset 0x90, Attr: 1
    m_num_count_value: &'static TitleBarNum, // Offset 0x98, Attr: 1
    m_num_count: f32, // Offset 0xA0, Attr: 1
    // m_value_count_list: List<&ValueCountController>, // Offset 0xA8, Attr: 1
    junk: [u8; 0x40],
    pub current_title: Option<&'static Title>,

     */
}
impl UnityComponent for TitleBar {}

#[unity::class("", "Values")]
#[nested_from_type(TitleBar)]
pub struct TitleBarValues {
    pub root: &'static GameObject, // Offset 0x10, Attr: 6
    pub animator: &'static Animator, // Offset 0x18, Attr: 6
    pub material_obj_list: &'static List<GameObject>, // Offset 0x20, Attr: 6
    pub piece_of_bond_object: &'static GameObject, // Offset 0x28, Attr: 6
    pub piece_of_bond_value: &'static TextMeshProUGUI, // Offset 0x30, Attr: 6
    pub money_object: &'static GameObject, // Offset 0x38, Attr: 6
    pub money_value: &'static TextMeshProUGUI, // Offset 0x40, Attr: 6
}

#[unity::class("", "Title")]
#[nested_from_type(TitleBar)]
pub struct TitleBarTitle {
    pub root: &'static GameObject, // Offset 0x10, Attr: 6
    pub animator: &'static Animator, // Offset 0x18, Attr: 6
    pub title_text: &'static TextMeshProUGUI, // Offset 0x20, Attr: 6
    pub help_text: &'static TextMeshProUGUI, // Offset 0x28, Attr: 6
    pub unit_obj: &'static GameObject, // Offset 0x30, Attr: 6
    pub unit_value: &'static TextMeshProUGUI, // Offset 0x38, Attr: 6
    pub unit_max_value: &'static TextMeshProUGUI, // Offset 0x40, Attr: 6
    pub key_help: &'static KeyHelpTitleBarController, // Offset 0x48, Attr: 6
}

#[unity::class("", "Num")]
#[nested_from_type(TitleBar)]
pub struct TitleBarNum {
    pub gold: i32, // Offset 0x10, Attr: 6
    pub piece_of_bond: i32, // Offset 0x14, Attr: 6
    pub refine_silver: i32, // Offset 0x18, Attr: 6
    pub refine_steel: i32, // Offset 0x1C, Attr: 6
    pub refine_iron: i32, // Offset 0x20, Attr: 6
    pub refine_god_list: &'static Array<i32>, // Offset 0x28, Attr: 6
    pub proof_master: i32, // Offset 0x30, Attr: 6
    pub proof_change: i32, // Offset 0x34, Attr: 6
    pub proof_enchant: i32, // Offset 0x38, Attr: 6
    pub proof_gunner: i32, // Offset 0x3C, Attr: 6
    pub relay_ticket: i32, // Offset 0x40, Attr: 6
}

impl TitleBar {
    pub fn show_header() { Self::get_instance().show_header_(); }
    pub fn open_header(title: impl AsRef<str>, help: impl AsRef<str>, key_help_id: impl AsRef<str>) -> bool {
        Self::get_instance().open_header_(title.into(), help.into(), key_help_id.into())
    }
    pub fn open_header_sortie(title: impl AsRef<str>, help: impl AsRef<str>, key_help_id: impl AsRef<str>, unit_num: i32, unit_max_num: i32) {
        Self::get_instance().open_header_sortie_(title.into(), help.into(), key_help_id.into(), unit_num, unit_max_num);
    }
    pub fn close_header()  { Self::get_instance().close_header_(); }
    pub fn hide_header() { Self::get_instance().hide_header_(); }

    pub fn show_footer() { Self::get_instance().show_footer_(); }
    pub fn hide_footer() { Self::get_instance().hide_footer_(); }
    pub fn open_footer(ty: TitleBarFooterType) { Self::get_instance().open_footer_(ty); }
    pub fn update() { Self::get_instance().update_(); }
    pub fn update_footer_values() { Self::get_instance().update_footer_values_(); }

    #[unity::class_method(0)] pub fn get_instance() -> &'static mut TitleBar; // Offset: 0x21ECAF0 Flags: 0
    #[unity::class_method(7)] pub fn update_(&self); // Offset: 0x21ED1C0 Flags: 0
    #[unity::class_method(8)] pub fn hide_header_(&self); // Offset: 0x21ED990 Flags: 0
    #[unity::class_method(9)] pub fn hide_header_key_help(&self, key_help: &Il2CppString); // Offset: 0x21EDA30 Flags: 0
    #[unity::class_method(10)] pub fn hide_footer_(&self); // Offset: 0x21EDB20 Flags: 0
    #[unity::class_method(11)] pub fn show_header_(&self); // Offset: 0x21EDD30 Flags: 0
    #[unity::class_method(12)] pub fn show_header_key_help_(&self); // Offset: 0x21EDDD0 Flags: 0
    #[unity::class_method(13)] pub fn show_footer_(&self); // Offset: 0x21EDE30 Flags: 0
    #[unity::class_method(14)] pub fn is_open_header(&self) -> bool; // Offset: 0x21EDED0 Flags: 0
    #[unity::class_method(15)] pub fn is_open_footer(&self) -> bool; // Offset: 0x21EDEE0 Flags: 0
    #[unity::class_method(16)] pub fn open_header_(&self, title: &Il2CppString, title_help: &Il2CppString, key_help_id: &Il2CppString) -> bool; // Offset: 0x21EDEF0 Flags: 0
    #[unity::class_method(17)] pub fn open_header_sortie_(&self, title: &Il2CppString, title_help: &Il2CppString, key_help_id: &Il2CppString, unit_num: i32, unit_max_num: i32); // Offset: 0x21EE2F0 Flags: 0
    #[unity::class_method(18)] pub fn open_footer_(&self, ty: TitleBarFooterType); // Offset: 0x21EE510 Flags: 0
    #[unity::class_method(19)] pub fn close_header_(&self); // Offset: 0x21EF1D0 Flags: 0
    #[unity::class_method(20)] pub fn close_footer_(&self); // Offset: 0x21EF380 Flags: 0
    #[unity::class_method(21)] pub fn update_footer_values_(&self); // Offset: 0x21EF170 Flags: 0
    #[unity::class_method(22)] pub fn set_unit_num(&self, num: i32, max_num: i32); // Offset: 0x21EE3E0 Flags: 0
    #[unity::class_method(23)] pub fn transit_header(&self, title: &Il2CppString, title_help: &Il2CppString, key_help_id: &Il2CppString); // Offset: 0x21EDAD0 Flags: 0
    #[unity::class_method(28)] pub fn set_title(&self, title: &Il2CppString, title_help: &Il2CppString, key_help_id: &Il2CppString); // Offset: 0x21EE0C0 Flags: 0
    #[unity::class_method(35)] pub fn init_footer_value(&self, values: &TitleBarValues); // Offset: 0x21EE780 Flags: 0
    #[unity::class_method(36)] pub fn set_footer_value(&self); // Offset: 0x21ED700 Flags: 0
    #[unity::class_method(37)] pub fn set_footer_count_value(&self); // Offset: 0x21EF690 Flags: 0
    #[unity::class_method(39)] pub fn set_material_value(&self, values: &TitleBarValues, index: i32, value: i32); // Offset: 0x21F0980 Flags: 0
}