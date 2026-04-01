use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
pub use unity::{il2cpp::object::Array, prelude::*};
use unity::engine::{Color, Sprite};
use crate::battle::BattleInfoSide;
use crate::bit::BitStruct;
use crate::calculator::CalculatorManager;
use crate::gamedata::job::BattleStyleTypes;
use crate::gamedata::terrain::TerrainData;
use crate::unit::{CapabilitySbyte, Unit};
use super::{*, item::ItemData};

#[unity::class("App", "SkillArrayList")]
pub struct SkillArrayEntityList {
    pub item: &'static mut Array<SkillArrayEntity>,
    pub size: i32,
    pub version: i32,
    sync_root: *const u8,
}

impl Deref for SkillArrayEntityListFields {
    type Target = [SkillArrayEntity];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.item.m_items.as_ptr(), self.size as usize) }
    }
}

impl DerefMut for SkillArrayEntityListFields  {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.item.m_items.as_mut_ptr(), self.size as usize) }
    }
}

#[unity::class("App", "WeaponLevels")]
pub struct WeaponLevels {
    pub levels: &'static mut Array<i8>,
}
#[unity::class("App", "SkillArray")]
pub struct SkillArray {
    pub bit: &'static mut BitStruct,
    pub list: &'static mut SkillArrayEntityList,
    pub flags: SkillDataFlags, // u64,
    pub cycles: i32,
    pub timing: i32,
    pub efficacy: i32,
    pub efficacy_ignore: i32,
    pub bad_states: SkillDataStates, //i32,
    pub bad_ignore: i32,
    pub weapon_levels: &'static WeaponLevels,
}
impl Deref for SkillArrayFields {
    type Target = [SkillArrayEntity];
    fn deref(&self) -> &Self::Target {
        &self.list.item
    }
}

// #[repr(C)]
pub struct SkillArrayEntity { pub value : u32, }

#[unity::class("App", "SkillData")]
pub struct SkillData {
    pub parent: StructBaseFields,
    pub commands: *const u8,  // from the calculator part
    pub sid: &'static Il2CppString,
    pub name: Option<&'static Il2CppString>,
    pub help: Option<&'static Il2CppString>,
    pub command_name: Option<&'static Il2CppString>,
    pub command_help: Option<&'static Il2CppString>,
    pub command_warning: Option<&'static Il2CppString>,
    pub root_command_sid: Option<&'static Il2CppString>,
    pub icon_type: i32,
    pub pad1: i32,
    pub icon_label: Option<&'static Il2CppString>,
    pub icon_bmap: Option<&'static Il2CppString>,
    pub stand: i32,
    pub action: i32,
    pub timing: i32,
    pub target: i32,
    pub equip_iids: Option<&'static Array<&'static Il2CppString>>,
    pub group: i32,
    pub priority: u8,
    pub layer: i32,
    pub order: i8,
    pub cycle: i32,
    pub frequency: i32,
    pub condition: Option<&'static Il2CppString>,
    pub give_target: i32,
    pub pad2: i32,
    pub give_condition: &'static Il2CppString,
    pub give_sids: &'static Array<&'static Il2CppString>,
    pub remove_sids: &'static Array<&'static Il2CppString>,
    pub sync_conditions: &'static Array<&'static Il2CppString>,
    pub sync_sids: &'static Array<&'static Il2CppString>,
    pub rebirth_sid: &'static Il2CppString,
    pub engage_sid: &'static Il2CppString,
    pub change_sids: &'static Array<&'static Il2CppString>,
    pub attack_range:  Option<&'static Il2CppString>,
    pub overlap_range:  Option<&'static Il2CppString>,
    pub overlap_terrain:  Option<&'static Il2CppString>,
    pub zoc_range:  Option<&'static Il2CppString>,
    pub zoc_type: i32,
    pub pad3: i32,
    pub cooperation_skill:  Option<&'static Il2CppString>,
    pub horse_skill: Option<&'static Il2CppString>,
    pub covert_skill: Option<&'static Il2CppString>,
    pub heavy_skill: Option<&'static Il2CppString>,
    pub fly_skill: Option<&'static Il2CppString>,
    pub magic_skill: Option<&'static Il2CppString>,
    pub pranas_kill: Option<&'static Il2CppString>,
    pub dragons_kill: Option<&'static Il2CppString>,
    pub act_names: Option<&'static Array<&'static Il2CppString>>,
    pub act_operations: Option<&'static Array<&'static Il2CppString>>,
    pub act_values: Option<&'static Array<&'static Il2CppString>>,
    pub around_center: i32,
    pub around_target: i32,
    pub around_condition: Option<&'static Il2CppString>,
    pub around_name: Option<&'static Il2CppString>,
    pub around_operation: Option<&'static Il2CppString>,
    pub around_value: Option<&'static Il2CppString>,
    pub bad_state: i32,
    pub bad_ignore: i32,
    pub efficacy: i32,
    pub efficacy_ignore: i32,
    pub efficacy_value: i32,
    pub flag: i64,
    pub private_flag: i32,
    pub work: i32,
    pub work_operation: Option<&'static Il2CppString>,
    pub work_value :f32,
    pub power: i32,
    pub life: i32,
    pub cost: i32,
    pub rewarp: i32,
    pub removable: i32,
    pub vision_count: i32,
    pub range_target: i32,
    pub range_i: i32,
    pub range_o: i32,
    pub range_add: i32,
    pub range_extend: i32,
    pub move_self: i32,
    pub move_target: i32,
    pub enhance_level: i8,
    pub enhance_value: &'static CapabilitySbyte,
    pub weapon_prohibit: &'static mut WeaponMask,
    pub weapon_level: &'static mut WeaponLevels,
    pub effect: Option<&'static Il2CppString>,
    pub inheritance_cost: u16,
    pub inheritance_sort: u16,
    pub pad4: i32,
    pub give_skills: &'static SkillArray,
    pub remove_skills: &'static SkillArray,
    pub sync_skills: &'static SkillArray,
    pub rebirth_skill: Option<&'static SkillData>,
    pub engage_skill: Option<&'static SkillData>,
    pub change_skills: &'static mut Array<&'static mut SkillData>,
    pub low_skill: Option<&'static SkillData>,
    pub high_skill: Option<&'static SkillData>,
    pub root_command_skill: Option<&'static SkillData>,
    pub timing_mask: i32,
    pub cycle_mask: i32,
    pub sort_key: i32,
    pub act_funcs: *const u8,
    pub around_funcs: *const u8,
    pub style_skills: &'static mut Array<&'static mut SkillData>,
    pub weapon_level_mask: &'static mut WeaponMask,
    condition_command: *const u8,
    give_condition_command: *const u8,
    around_condition_command: *const u8,
    sync_condition_commands: *const u8,
    pub equip_items: &'static List<ItemData>,
    pub default_equip_item: Option<&'static ItemData>,
    pub prefixless_sid :&'static Il2CppString,
}
impl Gamedata for SkillData{}

impl SkillArray {
    pub fn find_sid<'a>(&self, sid: impl Into<&'a Il2CppString>) -> Option<&'static SkillData> { self.find_by_sid(sid.into()) }
    pub fn add_sid<'a>(&self, sid: impl Into<&'a Il2CppString>,category: SkillDataCategorys, age: i32) -> bool{
        self.add_sid_(sid.into(), category, age)
    }
    pub fn replace_sid(&self, sid: &Il2CppString, skill: &SkillData) {
        let index = self.index_of_sid(sid);
        if index == -1 { return; }
        let category = self.get_category_index(index);
        self.replace_index(index, skill, category);
    }
    #[unity::class_method(0)] pub fn ctor(&self); // Offset: 0x247DCA0 Flags: 0
    #[unity::class_method(1)] pub fn ctor2(&self, src: &SkillArray); // Offset: 0x247DDA0 Flags: 0
    #[unity::class_method(3)] pub fn get_flags(&self) -> SkillDataFlags; // Offset: 0x247E100 Flags: 0
    /*
    #[unity::class_method(4)] pub fn test(&self, sid: &Il2CppString) -> bool; // Offset: 0x247E110 Flags: 0
    #[unity::class_method(5)] pub fn test2(&self, skill: &SkillData) -> bool; // Offset: 0x247E1D0 Flags: 0
    #[unity::class_method(6)] pub fn test3(&self, flags: SkillDataFlags) -> bool; // Offset: 0x247E1F0 Flags: 0
    #[unity::class_method(7)] pub fn test4(&self, states: SkillDataStates) -> bool; // Offset: 0x247E200 Flags: 0
    #[unity::class_method(8)] pub fn test5(&self, timing: SkillDataTimings) -> bool; // Offset: 0x247E210 Flags: 0
    #[unity::class_method(9)] pub fn test6(&self, timing1: SkillDataTimings, timing2: SkillDataTimings) -> bool; // Offset: 0x247E220 Flags: 0
    #[unity::class_method(10)] pub fn test7(&self, mask: SkillDataTimingMasks) -> bool; // Offset: 0x247E240 Flags: 0
    #[unity::class_method(11)] pub fn test8(&self, cycle: SkillDataCycles) -> bool; // Offset: 0x247E250 Flags: 0
    #[unity::class_method(12)] pub fn test9(&self, cycle1: SkillDataCycles, cycle2: SkillDataCycles) -> bool; // Offset: 0x247E260 Flags: 0
    #[unity::class_method(13)] pub fn test10(&self, mask: SkillDataCycleMasks) -> bool; // Offset: 0x247E280 Flags: 0
    #[unity::class_method(14)] pub fn test11(&self, target: SkillDataTargets) -> bool; // Offset: 0x247E290 Flags: 0
     */
    #[unity::class_method(15)] pub fn get_bad_states(&self) -> SkillDataStates; // Offset: 0x247E3E0 Flags: 0
    #[unity::class_method(16)] pub fn set_by_sid(&self, sid: &Il2CppString, category: SkillDataCategorys) -> bool; // Offset: 0x247E3F0 Flags: 0
    #[unity::class_method(17)] pub fn set_by_skill(&self, skill: &SkillData, category: SkillDataCategorys) -> bool; // Offset: 0x247EAE0 Flags: 0
    #[unity::class_method(19)] pub fn set_by_array(&self, src: &SkillArray); // Offset: 0x247F8D0 Flags: 0
    #[unity::class_method(20)] pub fn add_sid_(&self, sid: &Il2CppString, category: SkillDataCategorys, age: i32) -> bool; // Offset: 0x2480100 Flags: 0
    #[unity::class_method(21)] pub fn add_skill(&self, skill: &SkillData, category: SkillDataCategorys, age: i32) -> bool; // Offset: 0x2480750 Flags: 0
    #[unity::class_method(22)] pub fn add_without_update(&self, skill: &SkillData, category: SkillDataCategorys, age: i32) -> bool; // Offset: 0x2480D10 Flags: 0
    #[unity::class_method(23)] pub fn update(&self); // Offset: 0x2481130 Flags: 0
    #[unity::class_method(26)] pub fn add_from_skill_array(&self, src: &SkillArray) -> bool; // Offset: 0x24820C0 Flags: 0
    #[unity::class_method(27)] pub fn remove_sid(&self, sid: &Il2CppString) -> bool; // Offset: 0x2482850 Flags: 0
    #[unity::class_method(28)] pub fn remove_skill_array(&self, src: &SkillArray) -> bool; // Offset: 0x2482BF0 Flags: 0
    #[unity::class_method(29)] pub fn remove_skill(&self, skill: &SkillData) -> bool; // Offset: 0x2483080 Flags: 0
    #[unity::class_method(30)] pub fn remove_by_flag(&self, flags: SkillDataFlags) -> bool; // Offset: 0x24833A0 Flags: 0
    #[unity::class_method(31)] pub fn replace_index(&self, index: i32, skill: &SkillData, category: SkillDataCategorys) -> bool; // Offset: 0x2483760 Flags: 0
    #[unity::class_method(32)] pub fn replace_skill(&self, skill: &SkillData) -> bool; // Offset: 0x2483AA0 Flags: 0
    #[unity::class_method(33)] pub fn move_skill(&self, old_index: i32, new_index: i32) -> bool; // Offset: 0x2483E80 Flags: 0
    #[unity::class_method(34)] pub fn sort(&self); // Offset: 0x2484130 Flags: 0
    #[unity::class_method(35)] pub fn get_give_skill(&self, give: &SkillData) -> &'static SkillData; // Offset: 0x2484260 Flags: 0
    #[unity::class_method(43)] pub fn can_give(&self, give: &SkillData) -> bool; // Offset: 0x24856F0 Flags: 0
    #[unity::class_method(44)] pub fn commit(&self, unit: &Unit); // Offset: 0x2485A10 Flags: 0
    #[unity::class_method(45)] pub fn get_item_range_i(&self, unit: &Unit, item: &ItemData) -> i32; // Offset: 0x2485D80 Flags: 0
    #[unity::class_method(46)] pub fn get_item_range_o(&self, unit: &Unit, item: &ItemData) -> i32; // Offset: 0x2485FF0 Flags: 0
    #[unity::class_method(47)] pub fn get_item_distance(&self, unit: &Unit, item: &ItemData) -> i32; // Offset: 0x2486220 Flags: 0
    #[unity::class_method(48)] pub fn get_rod_range_extend(&self, item: &ItemData) -> i32; // Offset: 0x24863F0 Flags: 0
    #[unity::class_method(49)] pub fn update_impl(&self); // Offset: 0x24865C0 Flags: 0
    #[unity::class_method(50)] pub fn clear(&self); // Offset: 0x2486790 Flags: 0
    #[unity::class_method(51)] pub fn copy(&self, src: &SkillArray); // Offset: 0x2486860 Flags: 0
    #[unity::class_method(52)] pub fn change(&self, index: i32); // Offset: 0x2486AC0 Flags: 0
    #[unity::class_method(53)] pub fn is_exist(&self) -> bool; // Offset: 0x2486FC0 Flags: 0
    #[unity::class_method(54)] pub fn is_ignore(&self, state: SkillDataStates) -> bool; // Offset: 0x2487010 Flags: 0
    #[unity::class_method(55)] pub fn index_of_sid(&self, sid: &Il2CppString) -> i32; // Offset: 0x2487020 Flags: 0
    #[unity::class_method(56)] pub fn index_of_skill(&self, skill: &SkillData) -> i32; // Offset: 0x2487170 Flags: 0
    #[unity::class_method(57)] pub fn get_count(&self) -> i32; // Offset: 0x2487250 Flags: 0
    #[unity::class_method(59)] pub fn get_skill(&self, index: i32) -> Option<&'static SkillData>; // Offset: 0x2487370 Flags: 0
    #[unity::class_method(61)] pub fn get_entity(&self, index: i32) -> SkillArrayEntity; // Offset: 0x2487540 Flags: 0
    #[unity::class_method(62)] pub fn get_age_index(&self, index: i32) -> i32; // Offset: 0x24875B0 Flags: 0
    #[unity::class_method(63)] pub fn get_age_skill(&self, skill: &SkillData) -> i32; // Offset: 0x2487620 Flags: 0
    #[unity::class_method(64)] pub fn get_decay(&self, index: i32) -> i32; // Offset: 0x2487760 Flags: 0
    #[unity::class_method(65)] pub fn get_category_index(&self, index: i32) -> SkillDataCategorys; // Offset: 0x24877E0 Flags: 0
    #[unity::class_method(66)] pub fn get_category_skill(&self, skill: &SkillData) -> SkillDataCategorys; // Offset: 0x2487850 Flags: 0
    #[unity::class_method(67)] pub fn find_by_sid(&self, sid: &Il2CppString) -> Option<&'static SkillData>; // Offset: 0x2487990 Flags: 0
    #[unity::class_method(68)] pub fn find_by_flag(&self, flags: SkillDataFlags) -> Option<&'static SkillData>; // Offset: 0x2487B90 Flags: 0
    #[unity::class_method(69)] pub fn find_by_state(&self, states: SkillDataStates) -> Option<&'static SkillData>; // Offset: 0x2487CD0 Flags: 0
    #[unity::class_method(70)] pub fn find_by_work(&self, work: SkillDataWorks) -> &'static SkillData; // Offset: 0x2487E10 Flags: 0
    // #[unity::class_method(71)] pub fn update_aging(&self, cycle: SkillDataCycles, ignore: Func<&SkillData, bool>) -> bool; // Offset: 0x2487F50 Flags: 0
    // #[unity::class_method(72)] pub fn update_aging2(&self, cycle1: SkillDataCycles, cycle2: SkillDataCycles, ignore: Func<&SkillData, bool>) -> bool; // Offset: 0x2488590 Flags: 0
    // #[unity::class_method(73)] pub fn update_aging_impl(&self, mask: SkillDataCycleMasks, ignore: Func<&SkillData, bool>) -> bool; // Offset: 0x2487F60 Flags: 0
    #[unity::class_method(74)] pub fn clear_cycles(&self); // Offset: 0x24885B0 Flags: 0
    #[unity::class_method(77)] pub fn calc_work(&self, value: i32, work: SkillDataWorks) -> i32; // Offset: 0x24891D0 Flags: 0
    #[unity::class_method(78)] pub fn get_power(&self) -> i32; // Offset: 0x2489450 Flags: 0
    #[unity::class_method(79)] pub fn get_efficacy_value(&self, target: &Unit) -> i32; // Offset: 0x24895A0 Flags: 0
    #[unity::class_method(80)] pub fn get_efficacy_mask(&self, target: &Unit) -> SkillDataAttrs; // Offset: 0x2489750 Flags: 0
    #[unity::class_method(81)] pub fn is_efficacy(&self, target: &Unit) -> bool; // Offset: 0x2489790 Flags: 0
    #[unity::class_method(82)] pub fn get_weapon_levels(&self) -> &'static WeaponLevels; // Offset: 0x24897D0 Flags: 0
    #[unity::class_method(83)] pub fn get_is_equip_skill_first_null(&self) -> bool; // Offset: 0x24897E0 Flags: 0
    #[unity::class_method(84)] pub fn set_is_equip_skill_first_null(&self, value: bool); // Offset: 0x24897F0 Flags: 0
}
impl SkillData {
    #[unity::class_method(0)] pub fn load(); // Offset: 0x2489800 Flags: 0
    #[unity::class_method(168)] pub fn ctor(&self); // Offset: 0x248A320 Flags: 0
    #[unity::class_method(169)] pub fn get_calculator(&self) -> &'static CalculatorManager; // Offset: 0x248A630 Flags: 0
    #[unity::class_method(170)] pub fn get_operations(operation: &Il2CppString) -> SkillDataOperations; // Offset: 0x248A6B0 Flags: 0
    #[unity::class_method(172)] pub fn read_skill(sid: &Il2CppString) -> &'static SkillData; // Offset: 0x248A9F0 Flags: 0
    #[unity::class_method(173)] pub fn on_build(&self); // Offset: 0x248AA90 Flags: 0
    #[unity::class_method(174)] pub fn calc_change_skills(&self) -> &'static Array<&'static SkillData>; // Offset: 0x248AEE0 Flags: 0
    #[unity::class_method(175)] pub fn on_completed(&self); // Offset: 0x248B110 Flags: 0
    #[unity::class_method(176)] pub fn group_assign(); // Offset: 0x248D0C0 Flags: 0
    #[unity::class_method(177)] pub fn on_completed_end(&self); // Offset: 0x248D260 Flags: 0
    #[unity::class_method(178)] pub fn on_release(&self); // Offset: 0x248D660 Flags: 0
    #[unity::class_method(179)] pub fn get_style_skill(&self, style: BattleStyleTypes) -> &'static SkillData; // Offset: 0x248D920 Flags: 0
    #[unity::class_method(180)] pub fn set_style_skill(&self, style: BattleStyleTypes, name: &Il2CppString); // Offset: 0x248CFA0 Flags: 0
    #[unity::class_method(184)] pub fn has_overlap_terrain(&self) -> bool; // Offset: 0x248DB50 Flags: 0
    #[unity::class_method(185)] pub fn get_overlap_terrain(&self) -> Option<&'static TerrainData>; // Offset: 0x248DB80 Flags: 0
    #[unity::class_method(187)] pub fn get_weapon_level_mask(&self) -> &'static WeaponMask; // Offset: 0x248DCB0 Flags: 0
    #[unity::class_method(188)] pub fn get_item(iid: &Il2CppString) -> &'static ItemData; // Offset: 0x248DCC0 Flags: 0
    #[unity::class_method(189)] pub fn get_name(&self) -> &'static Il2CppString; // Offset: 0x248DD50 Flags: 0
    #[unity::class_method(190)] pub fn get_help(&self) -> &'static Il2CppString; // Offset: 0x248DDE0 Flags: 0
    #[unity::class_method(191)] pub fn get_command_name(&self) -> &'static Il2CppString; // Offset: 0x248DE70 Flags: 0
    #[unity::class_method(192)] pub fn get_command_help(&self) -> &'static Il2CppString; // Offset: 0x248DF00 Flags: 0
    #[unity::class_method(193)] pub fn get_prefixless_sid(&self) -> &'static Il2CppString; // Offset: 0x248DF90 Flags: 0
    #[unity::class_method(194)] pub fn get_font_color(is_engage: bool, is_active: bool) -> Color; // Offset: 0x248DFA0 Flags: 0
    #[unity::class_method(196)] pub fn is_command_skill(&self) -> bool; // Offset: 0x248E2C0 Flags: 0
    // #[unity::class_method(197)] pub fn is_condition_impl(&self, command: &CalculatorCommand, obj1: Object, obj2: Object) -> bool; // Offset: 0x248E2E0 Flags: 0
    #[unity::class_method(198)] pub fn has_condition(&self) -> bool; // Offset: 0x248E320 Flags: 0
    #[unity::class_method(199)] pub fn is_condition(&self) -> bool; // Offset: 0x248E330 Flags: 0
    #[unity::class_method(200)] pub fn is_condition_units(&self, current: &Unit, reverse: &Unit) -> bool; // Offset: 0x248E370 Flags: 0
    #[unity::class_method(201)] pub fn is_condition_info_side(&self, current: &BattleInfoSide, reverse: &BattleInfoSide) -> bool; // Offset: 0x248E3B0 Flags: 0
    #[unity::class_method(202)] pub fn is_give_condition(&self, current: &Unit, reverse: &Unit) -> bool; // Offset: 0x248E3F0 Flags: 0
    #[unity::class_method(203)] pub fn is_give_condition_info_side(&self, current: &BattleInfoSide, reverse: &BattleInfoSide) -> bool; // Offset: 0x248E430 Flags: 0
    // #[unity::class_method(204)] pub fn execute_impl(&self, func: &SkillDataFunc, obj1: Object, obj2: Object) -> bool; // Offset: 0x248E470 Flags: 0
    #[unity::class_method(205)] pub fn has_execute_act(&self) -> bool; // Offset: 0x248E550 Flags: 0
    #[unity::class_method(206)] pub fn execute_act(&self, current: &Unit, reverse: &Unit) -> bool; // Offset: 0x248E5A0 Flags: 0
    #[unity::class_method(207)] pub fn execute_act2(&self, current: &BattleInfoSide, reverse: &BattleInfoSide) -> bool; // Offset: 0x248E670 Flags: 0
    #[unity::class_method(208)] pub fn is_style_skill(&self) -> bool; // Offset: 0x248E740 Flags: 0
    #[unity::class_method(209)] pub fn is_sync_condition(&self, unit: &Unit, index: i32) -> bool; // Offset: 0x248E750 Flags: 0
    #[unity::class_method(210)] pub fn is_around_condition_target(&self, unit: &Unit, target: &Unit) -> bool; // Offset: 0x248E7B0 Flags: 0
    #[unity::class_method(211)] pub fn is_around_condition(&self, unit: &Unit, target: &Unit) -> bool; // Offset: 0x248E980 Flags: 0
    #[unity::class_method(212)] pub fn is_around_condition2(&self, unit: &Unit, center: &Unit, target: &Unit) -> bool; // Offset: 0x248F370 Flags: 0
    #[unity::class_method(213)] pub fn is_around_condition3(&self, unit: &Unit, center: &Unit, target: &Unit, range_i: i32, range_o: i32) -> bool; // Offset: 0x248FC70 Flags: 0
    #[unity::class_method(214)] pub fn has_execute_around(&self) -> bool; // Offset: 0x2490570 Flags: 0
    #[unity::class_method(215)] pub fn execute_around(&self, unit: &Unit, target: &Unit) -> bool; // Offset: 0x24905C0 Flags: 0
    #[unity::class_method(216)] pub fn execute_around2(&self, current: &BattleInfoSide, reverse: &BattleInfoSide) -> bool; // Offset: 0x2490690 Flags: 0
    // #[unity::class_method(217)] pub fn can_frequency(&self, frequency: SkillDataFrequencies) -> bool; // Offset: 0x2490760 Flags: 0
    #[unity::class_method(218)] pub fn can_equip(&self) -> bool; // Offset: 0x2490780 Flags: 0
    #[unity::class_method(219)] pub fn has_effect(&self) -> bool; // Offset: 0x2490790 Flags: 0
    #[unity::class_method(220)] pub fn is_hide(&self) -> bool; // Offset: 0x24907A0 Flags: 0
    #[unity::class_method(221)] pub fn is_moving(&self) -> bool; // Offset: 0x24907B0 Flags: 0
    #[unity::class_method(222)] pub fn is_all_range(&self) -> bool; // Offset: 0x24907D0 Flags: 0
    #[unity::class_method(223)] pub fn try_get_icon(&self) -> &'static Sprite; // Offset: 0x24907E0 Flags: 0
    #[unity::class_method(224)] pub fn try_get_effcy_icon(&self, is_outline: bool) -> Option<&'static Sprite>; // Offset: 0x24907F0 Flags: 0
    #[unity::class_method(225)] pub fn get_equip_item(&self, index: i32) -> Option<&'static ItemData>; // Offset: 0x2490800 Flags: 0
    #[unity::class_method(226)] pub fn has_equip_item(&self) -> bool; // Offset: 0x24908A0 Flags: 0
    #[unity::class_method(227)] pub fn has_null_item(&self) -> bool; // Offset: 0x24908F0 Flags: 0
    #[unity::class_method(228)] pub fn can_target(&self, unit: &Unit, target: &Unit) -> bool; // Offset: 0x2490A40 Flags: 0
    #[unity::class_method(229)] pub fn can_range(&self, x: i32, z: i32, target: &Unit) -> bool; // Offset: 0x2490DB0 Flags: 0
    #[unity::class_method(230)] pub fn can_range2(&self, unit: &Unit, target: &Unit) -> bool; // Offset: 0x2491090 Flags: 0
    #[unity::class_method(231)] pub fn can_range3(&self, range: i32) -> bool; // Offset: 0x2491610 Flags: 0
    #[unity::class_method(232)] pub fn can_support(&self, unit: &Unit, target: &Unit) -> bool; // Offset: 0x2491640 Flags: 0
    #[unity::class_method(233)] pub fn can_self_target(&self) -> bool; // Offset: 0x24919B0 Flags: 0
    #[unity::class_method(234)] pub fn is_re_rewarp(&self) -> bool; // Offset: 0x24919E0 Flags: 0
    #[unity::class_method(235)] pub fn is_enchantment_skill(&self) -> bool; // Offset: 0x2491A40 Flags: 0
    #[unity::class_method(236)] pub fn is_fang_curse_skill(&self) -> bool; // Offset: 0x2491AF0 Flags: 0
    #[unity::class_method(237)] pub fn get_fang_curse_level(&self) -> i32; // Offset: 0x2491BA0 Flags: 0
    #[unity::class_method(238)] pub fn calc_work(&self, value: i32) -> i32; // Offset: 0x2489350 Flags: 0
    #[unity::class_method(239)] pub fn get_timing_mask(timing: SkillDataTimings) -> SkillDataTimingMasks; // Offset: 0x2491C60 Flags: 0
    #[unity::class_method(240)] pub fn get_timing_mask2(timing1: SkillDataTimings, timing2: SkillDataTimings) -> SkillDataTimingMasks; // Offset: 0x2491C70 Flags: 0
    #[unity::class_method(241)] pub fn get_timing_mask3(timing1: SkillDataTimings, timing2: SkillDataTimings, timing3: SkillDataTimings) -> SkillDataTimingMasks; // Offset: 0x2491C90 Flags: 0
    #[unity::class_method(246)] pub fn get_efficacy_skills(attrs: SkillDataAttrs) -> &'static SkillArray; // Offset: 0x2491DD0 Flags: 0
    #[unity::class_method(247)] pub fn get_hero_skill() -> &'static SkillData; // Offset: 0x2498980 Flags: 0
    #[unity::class_method(248)] pub fn get_stun_skill() -> &'static SkillData; // Offset: 0x24989F0 Flags: 0
    #[unity::class_method(249)] pub fn get_dance_skill() -> &'static SkillData; // Offset: 0x2498A60 Flags: 0
    #[unity::class_method(250)] pub fn get_morph_skill() -> &'static SkillData; // Offset: 0x2498AD0 Flags: 0
    #[unity::class_method(251)] pub fn get_poison_skill() -> &'static SkillData; // Offset: 0x2498B40 Flags: 0
    #[unity::class_method(252)] pub fn get_leader_skill() -> &'static SkillData; // Offset: 0x2498BB0 Flags: 0
    #[unity::class_method(253)] pub fn get_fang_curse_skill() -> &'static SkillData; // Offset: 0x2498C20 Flags: 0
    #[unity::class_method(254)] pub fn get_multi_change_skill() -> &'static SkillData; // Offset: 0x2498C90 Flags: 0
    #[unity::class_method(255)] pub fn get_enchant_skill() -> &'static SkillData; // Offset: 0x2498D00 Flags: 0
    #[unity::class_method(256)] pub fn get_enchantment_skill() -> &'static SkillData; // Offset: 0x2498D70 Flags: 0
    #[unity::class_method(257)] pub fn get_immortal_skill() -> &'static SkillData; // Offset: 0x2498DE0 Flags: 0
    #[unity::class_method(258)] pub fn get_transporter_skill() -> &'static SkillData; // Offset: 0x2498E50 Flags: 0
    #[unity::class_method(259)] pub fn get_full_bullet_skill() -> &'static SkillData; // Offset: 0x2498EC0 Flags: 0
    #[unity::class_method(260)] pub fn get_chain_attack_guard_skill() -> &'static SkillData; // Offset: 0x2498F30 Flags: 0
    #[unity::class_method(261)] pub fn get_not_terrain_damage_skill() -> &'static SkillData; // Offset: 0x2498FA0 Flags: 0
    #[unity::class_method(262)] pub fn get_gaze_diagonally_skill() -> &'static SkillData; // Offset: 0x2499010 Flags: 0
    #[unity::class_method(276)] pub fn can_override_skill(&self) -> bool; // Offset: 0x24994A0 Flags: 0
}

impl SkillArrayEntity {
    pub fn is_hidden(&self) -> bool { self.get_skill().map(|s| s.flag & 1 != 0).unwrap_or(false) }
    pub fn get_index(&self) -> i32 { (self.value as i32) & 0xFFF }
    pub fn set_index(&mut self, value: i32) { self.value = (self.value & 0xFFFFF000) | (value as u32); }
    pub fn get_group(&self) -> i32 { (self.value as i32 >> 12) & 0xFF }
    pub fn set_group(&mut self, value: i32) {
        let v = (value as u32 & 0xFF) << 12;
        self.value = (self.value & 0xFFF00FFF) | v;
    }
    pub fn get_age(&self) -> i32 { (self.value as i32 >> 20) & 0xFF }
    pub fn set_age(&mut self, value: i32) {
        let age = (value as u32 & 0xFF) << 20;
        self.value = (self.value & 0xF00FFFFF) | age;
    }
    pub fn get_category(&self) -> SkillDataCategorys {
        SkillDataCategorys::from_u32(self.value >> 28).unwrap_or(SkillDataCategorys::None)
    }
    pub fn set_category(&mut self, value: SkillDataCategorys) {
        let v = (value as u32) << 28;
        self.value = v | (self.value & 0xFFFFFFF);
    }
    pub fn get_skill(&self) -> Option<&'static SkillData> { SkillData::try_index_get(self.get_index()) }
    pub fn get_skill_mut(&self) -> Option<&'static mut SkillData> { SkillData::try_index_get_mut(self.get_index()) }
}

bitflags::bitflags! {
	#[repr(C)]
	#[derive(Copy, Clone, PartialEq, Eq)]
	pub struct SkillDataFlags: u64 {
        const Invisible = 1;
        const EngageAttack = 2;
        const EngageCharge = 4;
        const EngageLink = 8;
        const EngageWait = 16;
        const EngageSummon = 32;
        const IgnoreEngageAttacking = 64;
        const IgnoreNoEngageAttacking = 128;
        const EnableChaining = 256;
        const EnableDestory = 512;
        const EnableCannon = 1024;
        const EnableRod = 2048;
        const IgnoreAlone = 4096;
        const IgnoreMultiAttacking = 8192;
        const IgnoreTraining = 16384;
        const IgnoreTraial = 32768;
        const IgnoreSimulation = 65536;
        const ExclusiveDance = 131072;
        const RevengeAutoEquip = 262144;
        const SwapOrder = 524288;
        const InterruptOrder = 1048576;
        const ContinueBattle = 2097152;
        const ForceLateOrder = 4194304;
        const EachSupport = 8388608;
        const Reactable = 16777216;
        const Remagicable = 33554432;
        const BeforeMove = 67108864;
        const AllowChainAttack = 134217728;
        const AllowChainGuard = 268435456;
        const AllowEngageGuard = 536870912;
        const ForceChainAttack = 1073741824;
        const JoinChainAttack = 2147483648;
        const RangeReliance = 4294967296;
        const PickupReliance = 8589934592;
        const MoveCostFree = 17179869184;
        const MoveEnemyPass = 34359738368;
        const ResetDisorder = 68719476736;
        const ItemHealAround = 137438953472;
        const ItemHealGive = 274877906944;
        const SelfHealRod = 549755813888;
        const OnlyRecvoerRod = 1099511627776;
        const DecayEnhance = 2199023255552;
        const SubEngageCountLimit = 4398046511104;
        const ReverseCount = 8796093022208;
        const ReCooking = 17592186044416;
        const BasisSkill = 35184372088832;
        const Unstoppable = 70368744177664;
        const HideChangeGod = 140737488355328;
        const OverExpChange = 281474976710656;
        const MoveFly = 562949953421312;
        const ViewRestriction = 1125899906842624;
        const HasIconBmap = 9007199254740992;
        const HasContract = 18014398509481984;
        const HauntChainAttack = 36028797018963968;
        const HasRootCommand = 72057594037927936;
        const HasZOC = 144115188075855872;
        const HasWork = 288230376151711744;
        const HasVision = 576460752303423488;
        const NotCondition = 1152921504606846976;
        const HasCondition = 2305843009213693952;
        const HasEnhance = 4611686018427387904;
        const HasRangeTarget = 9223372036854775808;
        const IgnoreMask = 127168;
	}
}

bitflags::bitflags! {
	#[repr(C)]
	#[derive(Copy, Clone, PartialEq, Eq)]
	pub struct SkillDataStates: i32 {
        const None = 0;
        const Poison = 1;
        const DeadlyPoison = 2;
        const SeverePoison = 4;
        const Heal = 8;
        const Sleep = 16;
        const Silence = 32;
        const Charm = 64;
        const Confusion = 128;
        const Freeze = 256;
        const Weakness = 512;
        const Stun = 1024;
        const Interact = 2048;
        const Decoy = 4096;
        const NotEnhance = 8192;
        const Enhance = 65536;
        const Immovable = 131072;
        const NotMove = 262144;
        const NotWeaponWeight = 524288;
        const NotChainAttacked = 1048576;
        const IgnoreDebug = 1 << 31;
        const PoisonMask = 7;
	}
}
#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum SkillDataAttrs {
    None = 0, // Attr: 17
    Walk = 1, // Attr: 17
    Horse = 2, // Attr: 17
    Heavy = 4, // Attr: 17
    Fly = 8, // Attr: 17
    Dragon = 16, // Attr: 17
    Evil = 32, // Attr: 17
    Morph = 64, // Attr: 17
    Mediuth = 128, // Attr: 17
    Duma = 256, // Attr: 17
    Loptous = 512, // Attr: 17
    Veld = 1024, // Attr: 17
    Idenn = 2048, // Attr: 17
    Nergal = 4096, // Attr: 17
    Fodeth = 8192, // Attr: 17
    Ashnard = 16384, // Attr: 17
    Astarte = 32768, // Attr: 17
    Gimle = 65536, // Attr: 17
    Hydra = 131072, // Attr: 17
    Nemesis = 262144, // Attr: 17
}
#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataTimings {
    None = 0, // Attr: 17
    Always = 1, // Attr: 17
    BattleBefore = 2, // Attr: 17
    BattleDetail = 3, // Attr: 17
    BattleInvoke = 4, // Attr: 17
    BattleStart = 5, // Attr: 17
    OrderStart = 6, // Attr: 17
    ActionStart = 7, // Attr: 17
    AttackStart = 8, // Attr: 17
    AttackBranch = 9, // Attr: 17
    HitBefore = 10, // Attr: 17
    HitAfter = 11, // Attr: 17
    HitAffect = 12, // Attr: 17
    AttackEnd = 13, // Attr: 17
    ActionEnd = 14, // Attr: 17
    OrderEnd = 15, // Attr: 17
    BattleEnd = 16, // Attr: 17
    BattleResult = 17, // Attr: 17
    BattleAfter = 18, // Attr: 17
    Around = 19, // Attr: 17
    Support = 20, // Attr: 17
    BattleCommand = 21, // Attr: 17
    ActionCommand = 22, // Attr: 17
    OverlapCommand = 23, // Attr: 17
    SupportCommand = 24, // Attr: 17
    FixedNone = 25, // Attr: 17
    FixedDone = 26, // Attr: 17
    PhaseStart = 27, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataTimingMasks {
    None = 0, // Attr: 17
    Full = 1048575, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataCycleMasks {
    None = 0, // Attr: 17
    Full = 1048575, // Attr: 17
}

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum SkillDataStands {
    None = 0, // Attr: 17
    Offence = 1, // Attr: 17
    Defence = 2, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataActions {
    None = 0, // Attr: 17
    Offence = 1, // Attr: 17
    Defence = 2, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataTargets {
    Target = 0, // Attr: 17
    Enemy = 1, // Attr: 17
    Friend = 2, // Attr: 17
    Destroy = 3, // Attr: 17
    Pierce = 4, // Attr: 17
    Range = 5, // Attr: 17
    Around = 6, // Attr: 17
    Overlap = 7, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataCycles {
    None = 0, // Attr: 17
    Map = 1, // Attr: 17
    PhaseBefore = 2, // Attr: 17
    PhaseAfter = 3, // Attr: 17
    Fixed = 4, // Attr: 17
    Engaged = 5, // Attr: 17
    Battled = 6, // Attr: 17
    BattledOf = 7, // Attr: 17
    BattledDf = 8, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataOperations {
    None = 0, // Attr: 17
    Equal = 1, // Attr: 17
    Add = 2, // Attr: 17
    Sub = 3, // Attr: 17
    Mul = 4, // Attr: 17
    Div = 5, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataWorks {
    None = 0, // Attr: 17
    ItemHealScale = 1, // Attr: 17
    JobGrowChange = 2, // Attr: 17
    TotalGrowChange = 3, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataZocs {
    None = 0, // Attr: 17
    CostMin = 1, // Attr: 17
    CostMax = 2, // Attr: 17
    NotMove = 3, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataLayers {
    A = 1, // Attr: 17
    B = 2, // Attr: 17
    C = 4, // Attr: 17
    D = 8, // Attr: 17
}

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum SkillDataCategorys {
    None = 0, // Attr: 17
    Person = 1, // Attr: 17
    Job = 2, // Attr: 17
    Item = 3, // Attr: 17
    Equip = 4, // Attr: 17
    God = 5, // Attr: 17
    Ring = 6, // Attr: 17
    Hub = 7, // Attr: 17
    Support = 8, // Attr: 17
    Battle = 9, // Attr: 17
    Private = 10, // Attr: 17
    Inheritance = 11, // Attr: 17
    Command = 12, // Attr: 17
}