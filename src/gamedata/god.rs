pub use unity::{prelude::*, system::{List, Dictionary, Stack}, il2cpp::object::Array};
use crate::bit::BitField32Methods;
use crate::gamedata::{*, person::PersonData, item::ItemData, skill::SkillArray, WeaponMask};
use crate::gamedata::job::BattleStyleTypes;
use crate::unit::{CapabilitySbyte, Unit};

#[unity::class("App", "GodData")]
pub struct GodData {
    pub parent: StructBaseFields,
    pub gid: &'static Il2CppString,
    pub mid: &'static Il2CppString,
    pub nickname: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub sound_id: &'static Il2CppString,
    pub asset_id: &'static Il2CppString,
    pub face_icon_name: &'static Il2CppString,
    pub face_icon_name_darkness: &'static Il2CppString,
    pub ring_name: Option<&'static Il2CppString>,
    pub ring_help: Option<&'static Il2CppString>,
    pub unit_icon_id: Option<&'static Il2CppString>,
    pub change: Option<&'static Array<&'static Il2CppString>>,
    pub link: Option<&'static Il2CppString>,
    pub haunt: Option<&'static Il2CppString>,
    pub level: i32,
    pub force_type: i32,
    pub female: i32,
    pub good_weapon: i32,
    pub sort: i16,
    pub engage_count: i8,
    pub engage_attack: Option<&'static Il2CppString>,
    pub engage_attack_rampage: Option<&'static Il2CppString>,
    pub engage_attack_link: Option<&'static Il2CppString>,
    pub link_gid: Option<&'static Il2CppString>,
    pub gbid: Option<&'static Il2CppString>,
    pub grow_table: Option<&'static Il2CppString>,
    pub level_cap: u8,
    pub unlock_level_cap_flag: Option<&'static Il2CppString>,
    pub engrave_word: Option<&'static Il2CppString>,
    pub engrave_power: i8,
    pub engrave_weight: i8,
    pub engrave_hit: i8,
    pub engrave_critical: i8,
    pub engrave_avoid: i8,
    pub engrave_secure: i8,
    pub syncho_enhance: &'static mut CapabilitySbyte,
    pub main_data: &'static GodData,
    pub change_data: &'static mut Array<&'static mut GodData>,
    pub change_index: i32,
    pub ascii_name: Option<&'static Il2CppString>,
    pub flag: &'static mut GodDataFlagField,
}
impl Gamedata for GodData {}

impl GodData {
    pub fn set_engrave_value(&self, index: i32, value: i8){
        match index {
            0 => self.set_engrave_avoid(value),
            1 => self.set_engrave_critical(value),
            2 => self.set_engrave_hit(value),
            3 => self.set_engrave_power(value),
            4 => self.set_engrave_secure(value),
            5 => self.set_engrave_weight(value),
            _ => {},
        }
    }
    pub fn get_engrave_value(&self, index: i32) -> i8 {
        match index {
            0 => self.engrave_avoid,
            1 => self.engrave_critical,
            2 => self.engrave_hit,
            3 => self.engrave_power,
            4 => self.engrave_secure,
            5 => self.engrave_weight,
            _ => 0,
        }
    }
    pub fn get_link_dictionary() -> &'static Dictionary<'static, &'static Il2CppString, &'static GodData> {
        Self::class().get_static_fields::<GodDataStaticFields>().link_dics
    }

    pub fn get_level_data(&self) -> Option<&'static mut List<GodGrowthDataLevelData>> {
        self.grow_table.and_then(|ggid| GodGrowthData::get_level_data(ggid))
    }
    #[unity::class_method(42)] pub fn get_engage_attack(&self) -> Option<&'static Il2CppString>; // Offset: 0x232DB70 Flags: 0
    #[unity::class_method(43)] pub fn set_engage_attack(&self, value: Option<&Il2CppString>); // Offset: 0x232DB80 Flags: 0
    #[unity::class_method(46)] pub fn get_engage_attack_link(&self) -> Option<&'static Il2CppString>; // Offset: 0x232DBB0 Flags: 0
    #[unity::class_method(47)] pub fn set_engage_attack_link(&self, value: &Il2CppString); // Offset: 0x232DBC0 Flags: 0
    #[unity::class_method(48)] pub fn get_link_gid(&self) -> Option<&'static Il2CppString>; // Offset: 0x232DBD0 Flags: 0
    #[unity::class_method(49)] pub fn set_link_gid(&self, value: &Il2CppString); // Offset: 0x232DBE0 Flags: 0
    #[unity::class_method(61)] pub fn set_engrave_power(&self, value: i8); // Offset: 0x232DCA0 Flags: 0
    #[unity::class_method(63)] pub fn set_engrave_weight(&self, value: i8); // Offset: 0x232DCC0 Flags: 0
    #[unity::class_method(65)] pub fn set_engrave_hit(&self, value: i8); // Offset: 0x232DCE0 Flags: 0
    #[unity::class_method(67)] pub fn set_engrave_critical(&self, value: i8); // Offset: 0x232DD00 Flags: 0
    #[unity::class_method(69)] pub fn set_engrave_avoid(&self, value: i8); // Offset: 0x232DD20 Flags: 0
    #[unity::class_method(71)] pub fn set_engrave_secure(&self, value: i8); // Offset: 0x232DD40 Flags: 0
    #[unity::class_method(74)] pub fn get_main_data(&self) -> &'static GodData; // Offset: 0x232DD70 Flags: 0
    #[unity::class_method(75)] pub fn set_main_data(&self, value: &GodData); // Offset: 0x232DD80 Flags: 0
    #[unity::class_method(80)] pub fn get_ascii_name(&self) -> &'static Il2CppString; // Offset: 0x232DDD0 Flags: 0
    #[unity::class_method(81)] pub fn set_ascii_name(&self, value: &Il2CppString); // Offset: 0x232DDE0 Flags: 0
    #[unity::class_method(95)] pub fn get_link_god_data(&self) -> Option<&'static GodData>; // Offset: 0x232E040 Flags: 0
    #[unity::class_method(96)] pub fn get_engage_haunt_unit(&self) -> Option<&'static Unit>; // Offset: 0x232E0D0 Flags: 0
    #[unity::class_method(97)] pub fn try_get_link(person: &PersonData) -> Option<&'static GodData>; // Offset: 0x232E0E0 Flags: 0
    #[unity::class_method(106)] pub fn is_hero(&self) -> bool; // Offset: 0x232E900 Flags: 0
    /*
    pub fn get_engrave_avoid(&self) -> i8 { unsafe{ goddata_get_engrave_avoid(self, None) }}
    pub fn get_engage_attack_link(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_engage_link(self, None)}}
    pub fn get_engrave_critical(&self) -> i8 { unsafe{ goddata_get_engrave_critical(self, None) }}
    pub fn get_engrave_hit(&self) -> i8 { unsafe{ goddata_get_engrave_hit(self, None) }}
    pub fn get_engrave_power(&self) -> i8 { unsafe{ goddata_get_engrave_power(self, None) }}
    pub fn get_engrave_secure(&self) -> i8 { unsafe{ goddata_get_engrave_secure(self, None) }}
    pub fn get_engrave_weight(&self) -> i8 { unsafe{ goddata_get_engrave_weight(self, None) }}
    pub fn get_force_type(&self) -> i32 { unsafe {  god_data_force_type(self, None)}}
    pub fn get_link_gid(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_link_gid(self, None)}}
    pub fn get_link(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_link(self, None) }}
    pub fn get_ascii_name(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_ascii(self, None) }}
    pub fn get_flag(&self) -> &'static mut WeaponMask { unsafe { god_data_get_flag(self, None)}}
    pub fn get_grow_table(&self) -> Option<&'static Il2CppString> { unsafe { god_data_get_grow_table(self, None)}}
    pub fn get_link_god_data(&self) -> Option<&'static GodData> { unsafe { goddata_get_link_god_data(self, None)}}
    pub fn load() { unsafe { goddata_load(None); }}
    pub fn on_complete(&self) { unsafe{ god_data_on_complete(self, None); }}

    pub fn set_engage_attack(&self, value: &Il2CppString) { unsafe { goddata_set_engage_attack(self, value, None); }}
    pub fn set_engage_attack_link(&self, value: &Il2CppString) { unsafe { god_data_set_engage_link(self, value, None); } }
    pub fn set_engrave_avoid(&self, value: i8) { unsafe{ goddata_set_engrave_avoid(self, value, None); }}
    pub fn set_engrave_critical(&self, value: i8) { unsafe{ goddata_set_engrave_critical(self, value, None); }}
    pub fn set_engrave_hit(&self, value: i8) { unsafe{ goddata_set_engrave_hit(self, value, None); }}
    pub fn set_engrave_power(&self, value: i8)  { unsafe{ goddata_set_engrave_power(self, value, None); }}
    pub fn set_engrave_secure(&self, value: i8)  { unsafe{ goddata_set_engrave_secure(self, value, None); }}
    pub fn set_engrave_weight(&self, value: i8) { unsafe{ goddata_set_engrave_weight(self, value, None); }}
    pub fn set_link_gid(&self, value: &Il2CppString) { unsafe { god_data_set_link_gid(self, value, None); }}
    pub fn set_link(&self, value: &Il2CppString) { unsafe { god_data_set_link(self, value, None); }}
    pub fn set_main_data(&self, value: &GodData) { unsafe { god_data_set_main_data(self, value, None)}}
    pub fn set_ascii_name(&self, value: &Il2CppString) { unsafe { god_data_set_ascii(self, value, None); }}
    */
}

#[unity::class("", "FlagField")]
#[nested_from_type(GodData)]
pub struct GodDataFlagField {
    pub value: i32,
}

impl BitField32Methods for GodDataFlagField {}

#[allow(non_upper_case_globals)]
impl GodDataFlagField {
    pub const NoAddExp: i32 = 1;
    pub const EnableRingList: i32 = 2;
    pub const UnitIconDarkness: i32 = 1 << 2;
    pub const GaugeDarkness: i32 = 1 << 3;
    pub const OnlyEngageWeapon: i32 = 1 << 4;
    pub const Armlet: i32 = 1 << 5;
    pub const Hero: i32 = 1 << 31;
}


#[unity::class("App", "GodGrowthData")]
#[static_fields(GodGrowthDataStaticFields)]
pub struct GodGrowthData {
    pub parent: StructDataArrayFields,
    pub ggid: &'static Il2CppString,
    pub level: i32, 
    __: i32,
    pub inheritance_skills: Option<&'static mut Array<&'static Il2CppString>>,	
    pub synchro_skills: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_skills: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_items: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_cooperation: Option<&'static mut Array<&'static Il2CppString>>,
    pub engage_horses: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_coverts: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_heavys: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_flys: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_magics: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_pranas: Option<&'static mut Array<&'static Il2CppString>>,	
    pub engage_dragons: Option<&'static mut Array<&'static Il2CppString>>,
    pub aptitude: &'static mut WeaponMask,
    aptitude_cost_none: u16, // Offset 0xA0, Attr: 1
    aptitude_cost_sword: u16, // Offset 0xA2, Attr: 1
    aptitude_cost_lance: u16, // Offset 0xA4, Attr: 1
    aptitude_cost_axe: u16, // Offset 0xA6, Attr: 1
    aptitude_cost_bow: u16, // Offset 0xA8, Attr: 1
    aptitude_cost_dagger: u16, // Offset 0xAA, Attr: 1
    aptitude_cost_magic: u16, // Offset 0xAC, Attr: 1
    aptitude_cost_rod: u16, // Offset 0xAE, Attr: 1
    aptitude_cost_fist: u16, // Offset 0xB0, Attr: 1
    aptitude_cost_special: u16, // Offset 0xB2, Attr: 1
    pub aptitude_costs: &'static Array<u16>, // Offset 0xB8, Attr: 1
    pub flag: &'static GodGrowthDataFlagField, // Offset 0xC0, Attr: 1
    pub synchro_skill_array: &'static SkillArray, // Offset 0xC8, Attr: 1
    pub engage_skill_array: &'static SkillArray, // Offset 0xD0, Attr: 1
    pub style_engage_items: &'static GodGrowthDataStyleItems, // Offset 0xD8, Attr: 1
}
pub struct GodGrowthDataStaticFields {
    pub level_list: &'static Dictionary<'static, &'static Il2CppString, &'static mut List<GodGrowthDataLevelData>>,
}

impl GamedataArray for GodGrowthData {}

impl GodGrowthData {
    #[unity::class_method(9)] pub fn get_engage_skills(&self) -> &'static mut Array<&'static Il2CppString>;
    #[unity::class_method(5)] pub fn get_inheritance_skills(&self) ->  Option<&'static mut Array<&'static Il2CppString>>;
    #[unity::class_method(55)] pub fn get_synchro_skill_array(&self) -> &'static SkillArray; // Offset: 0x2330F70 Flags: 0
    #[unity::class_method(56)] pub fn set_synchro_skill_array(&self, value: &SkillArray); // Offset: 0x2330F80 Flags: 0
    #[unity::class_method(57)] pub fn get_engage_skill_array(&self) -> &'static SkillArray; // Offset: 0x2330F90 Flags: 0
    #[unity::class_method(58)] pub fn set_engage_skill_array(&self, value: &SkillArray); // Offset: 0x2330FA0 Flags: 0
    #[unity::class_method(60)] pub fn on_completed(&self); // Offset: 0x23311A0 Flags: 0
    #[unity::class_method(61)] pub fn on_completed_end(&self); // Offset: 0x2332320 Flags: 0
    #[unity::class_method(65)] pub fn try_get_from_god_data(god: &GodData) -> Option<&'static mut List<GodGrowthData>>; // Offset: 0x2332960 Flags: 0
    #[unity::class_method(66)] pub fn try_get_from_ggid(ggid: &Il2CppString) -> Option<&'static List<&'static mut GodGrowthData>>; // Offset: 0x23329F0 Flags: 0
    pub fn get_level_data<'a>(key: impl Into<&'a Il2CppString>) -> Option<&'static mut List<GodGrowthDataLevelData>> {
        let level_list = Self::class().get_static_fields::<GodGrowthDataStaticFields>().level_list;
        let method = level_list.get_class().get_methods().iter()
            .find(|method| method.get_name() == Some(String::from("get_Item")))
            .unwrap();
        let get_keys = unsafe {
            std::mem::transmute::<_,
            extern "C" fn(&Dictionary<'static, &'static Il2CppString, &'static mut List<GodGrowthDataLevelData>>, &Il2CppString, &MethodInfo)
             -> Option<&'static mut List<GodGrowthDataLevelData>>>(
                method.method_ptr,
            )
        };
        get_keys(level_list, key.into(), method)
    }
    pub fn get_level_lists() -> &'static Dictionary<'static, &'static Il2CppString, &'static mut List<GodGrowthDataLevelData>> {
        Self::class().get_static_fields::<GodGrowthDataStaticFields>().level_list
    }
}
#[unity::class("", "FlagField")]
#[nested_from_type(GodGrowthData)]
pub struct GodGrowthDataFlagField { pub value: i32, }

impl BitField32Methods for GodGrowthDataFlagField {}

#[allow(non_upper_case_globals)]
impl GodGrowthDataFlagField {
    pub const UnlockSkillInheritance: i32 = 1;
    pub const AddEngageTurnLimit: i32 = 2;
    pub const SubEngageCountLimit: i32 = 4;
}

#[unity::class("", "LevelData")]
#[nested_from_type(GodGrowthData)]
pub struct GodGrowthDataLevelData {
    pub synchro_skills: &'static mut SkillArray,
    pub engaged_skills: &'static mut SkillArray,
    pub engage_skills: &'static mut SkillArray,
    pub style_items: &'static mut GodGrowthDataStyleItems,
    pub aptitude: &'static mut WeaponMask,
    pub flags: &'static GodGrowthDataFlagField,
}

pub struct GodDataStaticFields{
    pub link_dics: &'static Dictionary<'static, &'static Il2CppString, &'static GodData>,
}

#[unity::class("", "StyleItems")]
#[nested_from_type(GodGrowthData)]
pub struct GodGrowthDataStyleItems {
    pub items: &'static mut Array<&'static mut List<ItemData>>,
    pub count: i32,
}

impl GodGrowthDataStyleItems {
    #[unity::class_method(1)] pub fn add(&self, style: BattleStyleTypes, item: &ItemData); // Offset: 0x1CD8AA0 Flags: 0
    #[unity::class_method(1)] pub fn add_item(&self, style: i32, item: &ItemData); // Offset: 0x1CD8AA0 Flags: 0
    #[unity::class_method(3)] pub fn clear(&self); // Offset: 0x1CD8CD0 Flags: 0
}

impl Deref for GodGrowthDataStyleItemsFields {
    type Target = Array<&'static mut List<ItemData>>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl DerefMut for GodGrowthDataStyleItemsFields {
    fn deref_mut(&mut self) -> &mut Array<&'static mut List<ItemData>> {
        self.items
    }
}

#[unity::class("App", "Pool")]
pub struct Pool<T:'static> {
    pub list: &'static mut List<T>,
    pub stack: &'static mut Stack<T>,
}

#[unity::class("App", "GodLevelData")]
pub struct GodLevelData {
    pub parent: StructBaseFields,
    pub level: &'static Il2CppString, // Offset 0x20, Attr: 1
    pub exp: i32, // Offset 0x28, Attr: 1
    pub reliance_level: &'static Il2CppString, // Offset 0x30, Attr: 1
    pub cost: i32, // Offset 0x38, Attr: 1
}

impl Gamedata for GodLevelData {}
