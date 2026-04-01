use unity::il2cpp::object::Array;
use unity::macro_context::Il2CppClass;
use unity::prelude::{Il2CppClassData, Il2CppString};
use unity::system::List;
use crate::bit::BitField32;
use crate::combat::Character;
use crate::gamedata::accessory::AccessoryData;
use crate::gamedata::ItemData;
use crate::sequence::photograph::{LookAtIK, PhotographPauseData};
use crate::unityengine::GameObject;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PhotographDisposInfoCharacterType {
    None = 0, // Attr: 17
    UnitM = 1, // Attr: 17
    UnitF = 2, // Attr: 17
    GodM = 3, // Attr: 17
    GodF = 4, // Attr: 17
    Mascot = 5, // Attr: 17
}

#[unity::class("App", "PhotographDisposInfo")]
pub struct PhotographDisposInfo {
    body_acc_data_list: &'static List<AccessoryData>, // Offset 0x10, Attr: 1
    pub body_acc_data: Option<&'static AccessoryData>, // Offset 0x18, Attr: 1
    face_acc_data_list: &'static List<AccessoryData>, // Offset 0x20, Attr: 1
    pub face_acc_data: Option<&'static AccessoryData>, // Offset 0x28, Attr: 1
    pub weapon_data_list: &'static List<ItemData>, // Offset 0x30, Attr: 1
    pub weapon_data: Option<&'static ItemData>, // Offset 0x38, Attr: 1
    pub m_is_visible: bool, // Offset 0x40, Attr: 1
    pub m_locator: &'static GameObject, // Offset 0x48, Attr: 1
    pub m_look_target: &'static GameObject, // Offset 0x50, Attr: 1
    m_pause_group_name_list: &'static Array<&'static Il2CppString>, // Offset 0x58, Attr: 1
    m_anime_controller_dic: u64, // Dictionary<&Il2CppString, &RuntimeAnimatorController>, // Offset 0x60, Attr: 1
    pub m_flag: &'static mut BitField32,
    pub m_is_loading_character: bool, // Offset 0x70, Attr: 1
    pub m_character_id_list: &'static List<Il2CppString>, // Offset 0x78, Attr: 1
    pub current_character_id: &'static Il2CppString, // Offset 0x80, Attr: 1
    pub m_character_type: PhotographDisposInfoCharacterType, // Offset 0x88, Attr: 1
    pub m_character_cmp: Option<&'static mut Character>, // Offset 0x90, Attr: 1
    pub m_is_look_at_camera: bool, // Offset 0x98, Attr: 1
    pub m_look_at_rate: f32, // Offset 0x9C, Attr: 1
    pub look_at_ik_eye: &'static mut LookAtIK, // Offset 0xA0, Attr: 1
    pub look_at_ik_body: &'static mut LookAtIK, // Offset 0xA8, Attr: 1
    pub pause_data_list: &'static List<PhotographPauseData>, // Offset 0xB0, Attr: 1
    pub current_pause_data: Option<&'static PhotographPauseData>, // Offset 0xB8, Attr: 1
    /*
    m_hold_item: &GameObject, // Offset 0xC0, Attr: 1
    m_hold_weapon: &GameObject, // Offset 0xC8, Attr: 1
    m_hold_weapon_data: &ItemData, // Offset 0xD0, Attr: 1
    m_hold_weapon_path: &Il2CppString, // Offset 0xD8, Attr: 1
    m_is_loading_weapon: bool, // Offset 0xE0, Attr: 1

     */
}
#[unity::class("App", "PhotographDisposManager")]
pub struct PhotographDisposManager {
    pub dispos_info_list: &'static mut List<PhotographDisposInfo>, // Offset 0x10, Attr: 1
    pub current_dispos_info: &'static mut PhotographDisposInfo, // Offset 0x18, Attr: 1
    pub is_record_dispos: bool, // Offset 0x20, Attr: 1
    pub character_id_old: &'static Il2CppString, // Offset 0x28, Attr: 1
    pause_data_old: &'static PhotographPauseData, // Offset 0x30, Attr: 1
    dispos_effect_list: &'static List<GameObject>, // Offset 0x38, Attr: 1
    body_acc_data_old: &'static AccessoryData, // Offset 0x40, Attr: 1
    face_acc_data_old: &'static AccessoryData, // Offset 0x48, Attr: 1
    weapon_data_old: &'static ItemData, // Offset 0x50, Attr: 1
}


impl PhotographDisposInfo {
    #[unity::class_method(28)] pub fn set_up_character(
        &self, character_id: Option<&Il2CppString>,
        pause_data: Option<&PhotographPauseData>,
        weapon_data: Option<&ItemData>, body_acc: Option<&AccessoryData>, face_acc: Option<&AccessoryData>, is_random: bool
    ); // Offset: 0x268F170 Flags: 0

    #[unity::class_method(30)]
    pub fn init_character(
        &self,
        character_id: Option<&Il2CppString>,
        pause_data: Option<&PhotographPauseData>,
        body_acc: Option<&AccessoryData>,
        face_acc: Option<&AccessoryData>,
        weapon_data: Option<&ItemData>,
        is_random: bool); // Offset: 0x268BA00 Flags: 0
    #[unity::class_method(31)] pub fn set_up_pause(&self); // Offset: 0x268CE70 Flags: 0
    #[unity::class_method(32)] pub fn setup_weapon(&self); // Offset: 0x268D530 Flags: 0
}

#[repr(C)]
/// Delegate Object to Initialize the Dispos Character
pub struct PhotographDisposInfo77{
    pub klass: &'static Il2CppClass,
    monitor: u64,
    pub this: &'static mut PhotographDisposInfo, // Offset 0x10, Attr: 6
    pub character_id: &'static Il2CppString, // Offset 0x18, Attr: 6
    pub body_acc: Option<&'static AccessoryData>, // Offset 0x20, Attr: 6
    pub face_acc: Option<&'static AccessoryData>, // Offset 0x28, Attr: 6
    pub character_cmp: &'static mut Character, // Offset 0x30, Attr: 6
}
impl PhotographDisposInfo77 {
    pub fn instantiate() -> &'static mut Self {
        let klass = PhotographDisposInfo::class().get_nested_types().iter().find(|k| k.get_name().contains("77")).unwrap();
        klass.instantiate_as::<Self>().unwrap()
    }
}


