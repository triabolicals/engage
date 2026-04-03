use num_derive::FromPrimitive;
pub use unity::prelude::*;
pub use unity::engine::Color;
use unity::il2cpp::object::Array;
pub use unity::system::*;
use crate::bit::*;
use crate::gamedata::god::GodData;
use crate::gamedata::item::ItemData;
use crate::gamedata::job::JobData;
use crate::gamedata::person::PersonData;
use crate::gamedata::{Gamedata, StructBaseFields};
use crate::god::GodUnit;
use crate::unit::Unit;

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum AssetTableStates {
    None = 0, // Attr: 17
    Engaging = 1, // Attr: 17
    EngageAttack = 2, // Attr: 17
    EngageLinkAttackMain = 3, // Attr: 17
    EngageLinkAttackSub = 4, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum AssetTableModes {
    Common = 0, // Attr: 17
    OnMap = 1, // Attr: 17
    Combat = 2, // Attr: 17
    GMap = 3, // Attr: 17
    Num = 4, // Attr: 17
}

#[unity::class("App", "AssetTable")]
#[static_fields(AssetTableStaticFields)]
pub struct AssetTable {
    pub parent: StructBaseFields,
    pub preset_name: Option<&'static Il2CppString>,
    pub mode: i32,
    __: i32,
    pub conditions: Option<&'static mut Array<&'static mut Il2CppString>>,
    pub body_model: Option<&'static Il2CppString>,
    pub dress_model: Option<&'static Il2CppString>,
    pub head_model: Option<&'static Il2CppString>,
    pub hair_model: Option<&'static Il2CppString>,
    pub ride_model: Option<&'static Il2CppString>,
    pub ride_dress_model: Option<&'static Il2CppString>,
    pub left_hand: Option<&'static Il2CppString>,
    pub right_hand: Option<&'static Il2CppString>,
    pub trail: Option<&'static Il2CppString>,
    pub magic: Option<&'static Il2CppString>,
    pub body_anim: Option<&'static Il2CppString>, 
    pub ride_anim: Option<&'static Il2CppString>,
    pub info_anim: Option<&'static Il2CppString>,
    pub talk_anim: Option<&'static Il2CppString>,
    pub demo_anim: Option<&'static Il2CppString>,
    pub hub_anim: Option<&'static Il2CppString>,
    pub colors: [u8; 24],
    /*
    pub hair_r: u8,
    pub hair_g: u8,
    pub hair_b: u8,
    pub grad_r: u8,
    pub grad_g: u8,
    pub grad_b: u8,
    pub skin_r: u8,
    pub skin_g: u8,
    pub skin_b: u8,
    pub toon_r: u8,
    pub toon_g: u8,
    pub toon_b: u8,
    pub mask_color_100_r: u8,
    pub mask_color_100_g: u8,
    pub mask_color_100_b: u8,
    pub mask_color_075_r: u8,
    pub mask_color_075_g: u8,
    pub mask_color_075_b: u8,
    pub mask_color_050_r: u8,
    pub mask_color_050_g: u8,
    pub mask_color_050_b: u8,
    pub mask_color_025_r: u8,
    pub mask_color_025_g: u8,
    pub mask_color_025_b: u8,
     */
    pub unity_colors: [Color; 8],
    pub accessories: [&'static mut AssetTableAccessory; 8],
    pub accessory_list: &'static AssetTableAccessoryList,
    pub scale_stuff: [f32; 19], 
    ___: i32,
    pub voice: Option<&'static Il2CppString>,
    pub foot_steps: Option<&'static Il2CppString>,
    pub material: Option<&'static Il2CppString>,
    pub comment: Option<&'static Il2CppString>,
    pub condition_indexes: &'static mut AssetTableConditionIndexes,
}
impl Gamedata for AssetTable {}

impl AssetTable {
    pub fn add_condition_key<'a>(key: impl Into<&'a Il2CppString>) {
        Self::class().get_static_fields::<AssetTableStaticFields>().condition_flags.add_by_key(key.into());
    }
    #[unity::class_method(157)] pub fn get_condition_names() -> &'static List<&'static Il2CppString>; // Offset: 0x211D240 Flags: 0
    #[unity::class_method(158)] pub fn get_condition_hits() -> &'static List2<i32>; // Offset: 0x211D2B0 Flags: 0
    // #[unity::class_method(158)] pub fn get_condition_hits() -> &'static SimpleList<i32>; // Offset: 0x211D2B0 Flags: 0
    #[unity::class_method(160)] pub fn has_color(color: Color) -> bool; // Offset: 0x211D450 Flags: 0
}

#[unity::class("", "ConditionFlags")]
#[nested_from_type(AssetTable)]
pub struct AssetTableConditionFlags {
    pub bits: BitStruct,
    pub keys: &'static List<Il2CppString>,
    pub hits: &'static List2<i32>,
    //pub hits: &'static SimpleList<i32>,
    pub dic: &'static Dictionary<'static, &'static Il2CppString, i32>,
}

impl AssetTableConditionFlags {
    pub fn add_by_key<'a>(&self, key: impl Into<&'a Il2CppString>) { self.add_by_key_(key.into()); }


    #[unity::class_method(3)] pub fn clear(&self); // Offset: 0x1BAFA50 Flags: 0
    #[unity::class_method(4)] pub fn test(&self, index: i32) -> bool; // Offset: 0x1BAFC20 Flags: 0
    #[unity::class_method(5)] pub fn test2(&self, key: &Il2CppString) -> bool; // Offset: 0x1BAFC70 Flags: 0
    #[unity::class_method(7)] pub fn add_by_key_(&self, key: &Il2CppString); // Offset: 0x1BAFDD0 Flags: 0
    #[unity::class_method(8)] pub fn add_item(&self, item: &ItemData); // Offset: 0x1BAFF70 Flags: 0
    #[unity::class_method(10)] pub fn get_state(unit: &Unit) -> AssetTableStates; // Offset: 0x1BB0100 Flags: 0
    #[unity::class_method(11)] pub fn add_unit(&self, unit: &Unit); // Offset: 0x1BB0200 Flags: 0
    /*
    #[unity::class_method(6)] pub fn add_keys(&self, keys: &Array<&Il2CppString>); // Offset: 0x1BAFCD0 Flags: 0
    #[unity::class_method(9)] pub fn add4(&self, force: ForceType); // Offset: 0x1BB0070 Flags: 0
    #[unity::class_method(12)] pub fn is_simple_mode(&self) -> bool; // Offset: 0x1BB07A0 Flags: 0
    #[unity::class_method(13)] pub fn add_gender(&self, gender: Gender, dress_gender: Gender); // Offset: 0x1BB07B0 Flags: 0
    #[unity::class_method(14)] pub fn add_gender_from_person(&self, person: &PersonData); // Offset: 0x1BB0900 Flags: 0
    #[unity::class_method(15)] pub fn add_gender_from_unit(&self, unit: &Unit); // Offset: 0x1BB0D60 Flags: 0
    #[unity::class_method(16)] pub fn add_gender_god_data(&self, goid_data: &GodData); // Offset: 0x1BB10F0 Flags: 0
    #[unity::class_method(17)] pub fn add_person_job(&self, person: &PersonData, job: &JobData); // Offset: 0x1BB13F0 Flags: 0
    #[unity::class_method(18)] pub fn add(&self, person: &PersonData, job: &JobData, force: ForceType); // Offset: 0x1BB1410 Flags: 0
    #[unity::class_method(19)] pub fn add_skills(&self, skills: &SkillArray); // Offset: 0x1BB1710 Flags: 0
    #[unity::class_method(20)] pub fn add9(&self, state: AssetTableStates, god_data: &GodData, is_darkness: bool); // Offset: 0x1BB0440 Flags: 0
    #[unity::class_method(21)] pub fn replace_gid2_eid(&self, gid: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1BB1930 Flags: 0
     */
}

#[repr(C)]
pub struct AssetTableStaticFields { 
    pub preset_name: &'static List<Il2CppString>,
    pub search_lists: &'static mut Array<&'static mut List<AssetTable>>,
    pub condition_indexes: &'static Dictionary<'static, &'static Il2CppString, i32>,
    pub condition_flags: &'static mut AssetTableConditionFlags,
    pub null_sound: AssetTableSound,
    pub null_color: Color,
    pub shared: &'static AssetTableResult,
}

impl AssetTableStaticFields {
    pub fn get() -> &'static mut Self {
        AssetTable::class().get_static_fields_mut::<AssetTableStaticFields>()
    }
    pub fn get_condition_index(key: impl Into<&'static Il2CppString> ) -> i32 {
        let mut return_value = 0;
        let found =  Self::get().condition_indexes.try_get_value(key.into(), &mut return_value);
        if found { return_value.clone() } else { -1 }
    }
}

#[repr(C)]
pub struct AssetTableSound {
    pub voice: Option<&'static Il2CppString>,
    pub footstep: Option<&'static Il2CppString>,
    pub material: Option<&'static Il2CppString>,
}

#[unity::class("", "Result")]
#[nested_from_type(AssetTable)]
pub struct AssetTableResult {
    pub pid: Option<&'static Il2CppString>,
    pub jid: Option<&'static Il2CppString>,
    pub body_model: &'static Il2CppString,
    pub dress_model: &'static Il2CppString,
    pub head_model: &'static Il2CppString,
    pub hair_model: &'static Il2CppString,
    pub ride_model: Option<&'static Il2CppString>,
    pub ride_dress_model: Option<&'static Il2CppString>,
    pub left_hand: &'static Il2CppString,
    pub right_hand: &'static Il2CppString,
    pub trail: &'static Il2CppString,
    pub magic: &'static Il2CppString,
    pub body_anim: Option<&'static Il2CppString>,
    pub ride_anim: Option<&'static Il2CppString>,
    pub unity_colors: [Color; 8],
    pub scale_stuff: [f32; 19], 
    __ : i32,
    pub sound: AssetTableSound,
    pub info_anims: Option<&'static Il2CppString>,
    pub talk_anims: Option<&'static Il2CppString>,
    pub demo_anims: Option<&'static Il2CppString>,
    pub hub_anims: Option<&'static Il2CppString>,
    pub force_id: Option<&'static Il2CppString>,
    pub weapon_id: Option<&'static Il2CppString>,
    pub body_anims: &'static mut List<Il2CppString>,
    pub accessory_list: &'static mut AssetTableAccessoryList,
    pub accessory_dictionary: &'static Dictionary<'static, &'static Il2CppString, &'static AssetTableAccessory>,
}
impl AssetTableResult {
    pub fn get_anim(&mut self, idx: i32) -> Option<&mut &'static Il2CppString> {
        match idx {
            0 => self.info_anims.as_mut(),
            1 => self.talk_anims.as_mut(),
            2 => self.demo_anims.as_mut(),
            3 => self.hub_anims.as_mut(),
            _ => None,
        }
    }
    pub fn get_from_pid<'a>(mode: i32, pid: impl Into<&'a Il2CppString>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult { Self::get_from_pid_(mode, pid.into(), conditions) }

    #[unity::class_method(88)] pub fn ctor(&self); // Offset: 0x1BB22C0 Flags: 0
    #[unity::class_method(89)] pub fn setup_for_unit(&self, mode: i32, unit: &Unit, equipped: Option<&ItemData>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB2430 Flags: 0
    #[unity::class_method(90)] pub fn setup_for_god(&self, mode: i32, god_data: Option<&GodData>, is_darkness: bool, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB2D80 Flags: 0
    #[unity::class_method(91)] pub fn setup_for_asset_table(&self, data: &AssetTable) -> &'static mut AssetTableResult; // Offset: 0x1BB2E90 Flags: 0
    #[unity::class_method(92)] pub fn setup_for_person(&self, mode: i32, person: Option<&PersonData>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB4180 Flags: 0
    #[unity::class_method(93)] pub fn setup_for_person_job_item(&self, mode: i32, person: Option<&PersonData>, job: Option<&JobData>, equipped: Option<&ItemData>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB4290 Flags: 0
    // #[unity::class_method(94)] pub fn setup6(&self, mode: i32, person: &PersonData, job: &JobData, god: &GodData, equipped: &ItemData, force: i32, state: i32, is_darkness: bool, conditions: &Array<String>) -> &'static mut AssetTableResult; // Offset: 0x1BB43A0 Flags: 0
    #[unity::class_method(95)] pub fn commit_mode(&self, mode: i32); // Offset: 0x1BB44D0 Flags: 0
    #[unity::class_method(96)] pub fn commit(&self, mode: i32, person: Option<&PersonData>, job: Option<&JobData>, equipped: Option<&ItemData>); // Offset: 0x1BB2A80 Flags: 0
    #[unity::class_method(97)] pub fn commit_god(&self, mode: i32, god_data: &GodData); // Offset: 0x1BB2E60 Flags: 0
    #[unity::class_method(98)] pub fn commit_asset_table(&self, data: &AssetTable); // Offset: 0x1BB2EE0 Flags: 0

    #[unity::class_method(102)] pub fn commit_accessory(&self, accessory: &AssetTableAccessory); // Offset: 0x1BB48B0 Flags: 0
    #[unity::class_method(103)] pub fn replace(&self, mode: i32); // Offset: 0x1BB3BE0 Flags: 0
    #[unity::class_method(106)] pub fn clear(&self); // Offset: 0x1BB2750 Flags: 0

    #[unity::class_method(114)] pub fn get_hash_code(&self) -> i32; // Offset: 0x1BB4FA0 Flags: 0

    #[unity::class_method(116)] pub fn get_for_talk_pid(pid: &Il2CppString) -> &'static mut AssetTableResult; // Offset: 0x1BB5A90 Flags: 0
    #[unity::class_method(117)] pub fn get_for_talk_unit(unit: &Unit) -> &'static mut AssetTableResult; // Offset: 0x1BB5CF0 Flags: 0
    #[unity::class_method(118)] pub fn get_for_demo(pid: &Il2CppString, is_default: bool, is_plain: bool) -> &'static mut AssetTableResult; // Offset: 0x1BB5F60 Flags: 0
    #[unity::class_method(119)] pub fn get_kizuna_condition(conditions: &Array<&Il2CppString>) -> &'static Array<&'static Il2CppString>; // Offset: 0x1BB62E0 Flags: 0
    #[unity::class_method(120)] pub fn get_for_kizuna(pid: &Il2CppString, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB63C0 Flags: 0
    #[unity::class_method(121)] pub fn get_hub_condition(conditions: &Array<&Il2CppString>) -> &'static Array<&'static Il2CppString>; // Offset: 0x1BB6560 Flags: 0
    #[unity::class_method(122)] pub fn get_for_hub(pid: &Il2CppString, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB6660 Flags: 0
    // #[unity::class_method(123)] pub fn get_for_hub_direct(pid: &Il2CppString, condisions: &Array<String>) -> &'static mut AssetTableResult; // Offset: 0x1BB6800 Flags: 0
    #[unity::class_method(124)] pub fn get_for_accessory(unit: &Unit) -> &'static mut AssetTableResult; // Offset: 0x1BB68B0 Flags: 0
    #[unity::class_method(125)] pub fn get_from_unit(mode: i32, unit: &Unit, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB6AF0 Flags: 0
    #[unity::class_method(126)] pub fn get_from_unit_item(mode: i32, unit: &Unit, equipped: Option<&ItemData>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB5ED0 Flags: 0
    #[unity::class_method(127)] pub fn get_for_unit_info(unit: &Unit) -> &'static mut AssetTableResult; // Offset: 0x1BB6BC0 Flags: 0
    #[unity::class_method(128)] pub fn get_for_unit_info_item(unit: &Unit, equipped: Option<&ItemData>) -> &'static mut AssetTableResult; // Offset: 0x1BB6C50 Flags: 0
    #[unity::class_method(129)] pub fn get_for_unit_info_god(god_data: &GodData, is_darkness: bool) -> &'static mut AssetTableResult; // Offset: 0x1BB6DE0 Flags: 0
    #[unity::class_method(130)] pub fn get_for_unit_info_god_unit(god_unit: &GodUnit) -> &'static mut AssetTableResult; // Offset: 0x1BB7010 Flags: 0
    #[unity::class_method(131)] pub fn get_for_unit_hub(unit: &Unit) -> &'static mut AssetTableResult; // Offset: 0x1BB70A0 Flags: 0
    #[unity::class_method(132)] pub fn get_for_talk_god(god_data: &GodData) -> &'static mut AssetTableResult; // Offset: 0x1BB7270 Flags: 0
    #[unity::class_method(133)] pub fn get_for_talk_god_unit(god_unit: &GodUnit) -> &'static mut AssetTableResult; // Offset: 0x1BB7400 Flags: 0
    #[unity::class_method(134)] pub fn get_for_demo_god(god_data: &GodData) -> &'static mut AssetTableResult; // Offset: 0x1BB7600 Flags: 0
    #[unity::class_method(135)] pub fn get_for_hub_god(god_data: &GodData) -> &'static mut AssetTableResult; // Offset: 0x1BB77F0 Flags: 0
    #[unity::class_method(136)] pub fn get_from_god_unit(mode: i32, god_unit: &GodUnit, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB7980 Flags: 0
    #[unity::class_method(137)] pub fn get_from_god_data(mode: i32, god_data: &GodData, is_darkness: bool, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB6F80 Flags: 0
    // #[unity::class_method(138)] pub fn get_from_item(mode: i32, item: &ItemData) -> &'static mut AssetTableResult; // Offset: 0x1BB7A60 Flags: 0
    // #[unity::class_method(139)] pub fn get_from_shop(mode: i32, item: &ItemData) -> &'static mut AssetTableResult; // Offset: 0x1BB7B90 Flags: 0
    #[unity::class_method(140)] pub fn get_from_preset(name: &Il2CppString) -> &'static mut AssetTableResult; // Offset: 0x1BB7CA0 Flags: 0
    #[unity::class_method(144)] pub fn get_from_pid_(mode: i32, pid: &Il2CppString, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult; // Offset: 0x1BB5BE0 Flags: 0
}

#[unity::class("", "Accessory")]
#[nested_from_type(AssetTable)]
pub struct AssetTableAccessory {
    pub locator: Option<&'static Il2CppString>,
    pub model: Option<&'static Il2CppString>, 
}

#[unity::class("", "ConditionIndexes")]
#[nested_from_type(AssetTable)]
pub struct AssetTableConditionIndexes {
    pub list: &'static mut List2<&'static mut List2<i32>>,
    //pub list: &'static mut List<SimpleList<i32>>,
}
impl AssetTableConditionIndexes {
    #[unity::class_method(0)] pub fn clear(&self); // Offset: 0x1BB1A00 Flags: 0
    #[unity::class_method(3)] pub fn test(&self, flags: &AssetTableConditionFlags) -> bool; // Offset: 0x1BB1AD0 Flags: 0
    pub fn has_condition_index(&self, index: i32) -> bool { self.list.iter().map(|x| x.iter()).flatten().any(|x| *x == index) }
}

#[unity::class("", "AccessoryList")]
#[nested_from_type(AssetTable)]
pub struct AssetTableAccessoryList {
    pub list: ListFields<AssetTableAccessory>,
}

impl AssetTableAccessoryList {
    #[unity::class_method(0)] pub fn try_add(&self, accessory: &AssetTableAccessory); // Offset: 0x1BAF640 Flags: 0
    #[unity::class_method(12, vtable)] pub fn clear(&self); // Offset: 0x3DE9950 Flags: 2
}