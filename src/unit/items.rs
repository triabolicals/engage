use crate::gamedata::Gamedata;
use crate::gamedata::god::GodData;
use super::*;

#[unity::class("App", "UnitItem")]
pub struct UnitItem {
    pub index: i32,
    pub item: &'static ItemData,
    pub endurance: u8,
    pub refine_level: u8,
    pub flags :i32,
    pub engrave: Option<&'static GodData>,
    pub god_unit: Option<&'static GodUnit>,
}
#[unity::class("App", "UnitItemList")]
pub struct UnitItemList {
    pub unit_items: &'static mut Array<Option<&'static mut UnitItem>>
}

impl UnitItem {
    pub fn ctor_str(&self, key: &str) { self.ctor_iid(key.into()); }
    #[unity::class_method(1)] pub fn ctor(&self, item: &ItemData); // Offset: 0x1FAD9E0 Flags: 0
    #[unity::class_method(2)] pub fn ctor_iid(&self, iid: &Il2CppString); // Offset: 0x1FADB00 Flags: 0
    // #[unity::class_method(3)] pub fn ctor4(&self, index: i32); // Offset: 0x1FADD50 Flags: 0
    #[unity::class_method(5)] pub fn dispose(&self); // Offset: 0x1FAE010 Flags: 0
    #[unity::class_method(6)] pub fn op_implicit(p: &UnitItem) -> &'static ItemData; // Offset: 0x1F95D60 Flags: 0
    #[unity::class_method(11)] pub fn clear(&self); // Offset: 0x1FAE6C0 Flags: 0
    #[unity::class_method(12)] pub fn expend(&self) -> bool; // Offset: 0x1FAE780 Flags: 0
    #[unity::class_method(13)] pub fn do_trade(&self); // Offset: 0x1FAE7F0 Flags: 0
    #[unity::class_method(14)] pub fn do_transporter(&self); // Offset: 0x1FAE820 Flags: 0
    #[unity::class_method(15)] pub fn do_drop(&self); // Offset: 0x1FAE830 Flags: 0
    #[unity::class_method(16)] pub fn do_transfer(&self); // Offset: 0x1FAE840 Flags: 0
    #[unity::class_method(17)] pub fn can_expend(&self) -> bool; // Offset: 0x1FAE7D0 Flags: 0
    #[unity::class_method(18)] pub fn get_expend(&self) -> i32; // Offset: 0x1FAE850 Flags: 0
    // #[unity::class_method(19)] pub fn can_enchant(&self) -> bool; // Offset: 0x1FAE870 Flags: 0
    // #[unity::class_method(20)] pub fn can_enchant2(&self, unit: &Unit) -> bool; // Offset: 0x1FAE880 Flags: 0
    #[unity::class_method(21)] pub fn is_empty(&self) -> bool; // Offset: 0x1FAE8C0 Flags: 0
    #[unity::class_method(22)] pub fn is_exist(&self) -> bool; // Offset: 0x1FAE8D0 Flags: 0
    #[unity::class_method(24)] pub fn is_long_range(&self) -> bool; // Offset: 0x1FAE920 Flags: 0
    // #[unity::class_method(25)] pub fn get_data(&self) -> &'static ItemData; // Offset: 0x1FAE940 Flags: 0
    #[unity::class_method(26)] pub fn get_price(&self) -> i32; // Offset: 0x1FAE950 Flags: 0
    #[unity::class_method(27)] pub fn get_selling(&self) -> i32; // Offset: 0x1FAE960 Flags: 0
    #[unity::class_method(30)] pub fn is_none(&self) -> bool; // Offset: 0x1FAEB40 Flags: 0
    #[unity::class_method(31)] pub fn is_inventory(&self) -> bool; // Offset: 0x1FAEB60 Flags: 0
    // #[unity::class_method(32)] pub fn get_kind(&self) -> i32; // Offset: 0x1FAEB70 Flags: 0
    // #[unity::class_method(33)] pub fn get_use_type(&self) -> ItemDataUseTypes; // Offset: 0x1FAEB80 Flags: 0
    // #[unity::class_method(34)] pub fn get_rod_type(&self) -> ItemDataRodTypes; // Offset: 0x1FAEB90 Flags: 0
    #[unity::class_method(35)] pub fn is_range_target(&self) -> bool; // Offset: 0x1FAEBA0 Flags: 0
    #[unity::class_method(36)] pub fn is_range_heal(&self) -> bool; // Offset: 0x1FAEC10 Flags: 0
    #[unity::class_method(37)] pub fn is_range_rest_heal(&self) -> bool; // Offset: 0x1FAEC90 Flags: 0
    #[unity::class_method(38)] pub fn is_range_again(&self) -> bool; // Offset: 0x1FAED10 Flags: 0
    #[unity::class_method(39)] pub fn is_range_engage_add(&self) -> bool; // Offset: 0x1FAED90 Flags: 0
    #[unity::class_method(40)] pub fn is_bless(&self) -> bool; // Offset: 0x1FAEE10 Flags: 0
    #[unity::class_method(41)] pub fn is_bullet(&self) -> bool; // Offset: 0x1FAEE30 Flags: 0
    // #[unity::class_method(45)] pub fn get_attr(&self) -> ItemDataAttrs; // Offset: 0x1FAEEA0 Flags: 0
    // #[unity::class_method(46)] pub fn get_weapon_attr(&self) -> ItemDataWeaponAttrs; // Offset: 0x1FAEEB0 Flags: 0
    #[unity::class_method(47)] pub fn is_weapon(&self) -> bool; // Offset: 0x1FAEEC0 Flags: 0
    #[unity::class_method(48)] pub fn is_physical(&self) -> bool; // Offset: 0x1FAEED0 Flags: 0
    #[unity::class_method(49)] pub fn is_magic(&self) -> bool; // Offset: 0x1FAEEF0 Flags: 0
    #[unity::class_method(50)] pub fn is_breath(&self) -> bool; // Offset: 0x1FAEF10 Flags: 0
    #[unity::class_method(51)] pub fn is_surehit(&self) -> bool; // Offset: 0x1FAEF80 Flags: 0
    #[unity::class_method(52)] pub fn is_rod(&self) -> bool; // Offset: 0x1FAEFB0 Flags: 0
    // #[unity::class_method(53)] pub fn get_rod_type2(&self) -> ItemDataRodTypes; // Offset: 0x1FAEFD0 Flags: 0
    // #[unity::class_method(54)] pub fn is_single_rod(&self) -> bool; // Offset: 0x1FAEFF0 Flags: 0
    // #[unity::class_method(55)] pub fn get_kind2(&self) -> ItemDataKinds; // Offset: 0x1FAF080 Flags: 0
    // #[unity::class_method(56)] pub fn is_efficacy(&self, target: &Unit) -> bool; // Offset: 0x1FAF090 Flags: 0
    #[unity::class_method(57)] pub fn get_name(&self, with_refine_level: bool) -> &'static Il2CppString; // Offset: 0x1FAF2A0 Flags: 0
    #[unity::class_method(58)] pub fn get_original_name(&self) -> &'static Il2CppString; // Offset: 0x1FAF5F0 Flags: 0
    #[unity::class_method(59)] pub fn get_power(&self) -> i32; // Offset: 0x1FAF600 Flags: 0
    #[unity::class_method(60)] pub fn get_weight(&self) -> i32; // Offset: 0x1FAF7C0 Flags: 0
    #[unity::class_method(61)] pub fn get_hit(&self) -> i32; // Offset: 0x1FAF890 Flags: 0
    #[unity::class_method(62)] pub fn get_critical(&self) -> i32; // Offset: 0x1FAF990 Flags: 0
    #[unity::class_method(63)] pub fn get_avoid(&self) -> i32; // Offset: 0x1FAFA90 Flags: 0
    #[unity::class_method(64)] pub fn get_secure(&self) -> i32; // Offset: 0x1FAFB00 Flags: 0
    #[unity::class_method(65)] pub fn get_enhance(&self) -> &'static CapabilitySbyte; // Offset: 0x1FAFB70 Flags: 0
    // #[unity::class_method(66)] pub fn get_times(&self) -> i32; // Offset: 0x1FAFBC0 Flags: 0
    #[unity::class_method(67)] pub fn get_equip_skills(&self) -> Option<&'static SkillArray>; // Offset: 0x1FAFBD0 Flags: 0
    // #[unity::class_method(68)] pub fn get_sort(&self) -> i64; // Offset: 0x1FAFDA0 Flags: 0
    #[unity::class_method(69)] pub fn is_refined(&self) -> bool; // Offset: 0x1FAF5E0 Flags: 0
    // #[unity::class_method(70)] pub fn get_max_refine_level(&self) -> i32; // Offset: 0x1FAFE80 Flags: 0
    #[unity::class_method(71)] pub fn refine_data_exist(&self) -> bool; // Offset: 0x1FAFED0 Flags: 0
    // #[unity::class_method(72)] pub fn get_current_refine_data(&self) -> &'static ItemRefineData; // Offset: 0x1FAF700 Flags: 0
    // #[unity::class_method(73)] pub fn get_next_refine_data(&self) -> &'static ItemRefineData; // Offset: 0x1FAFF00 Flags: 0
    // #[unity::class_method(74)] pub fn get_refine_data_list(&self) -> List<&'static ItemRefineData>; // Offset: 0x1FAFFA0 Flags: 0
    #[unity::class_method(75)] pub fn can_evolve(&self) -> bool; // Offset: 0x1FAFFB0 Flags: 0
    // #[unity::class_method(76)] pub fn is_exist_evolve_data(&self) -> bool; // Offset: 0x1FB0010 Flags: 0
    // #[unity::class_method(77)] pub fn get_evolve_data_list(&self) -> List<&'static ItemEvolveData>; // Offset: 0x1FB0040 Flags: 0
    #[unity::class_method(78)] pub fn can_engrave(&self) -> bool; // Offset: 0x1FB0050 Flags: 0
    #[unity::class_method(79)] pub fn is_engraved(&self) -> bool; // Offset: 0x1FAF5D0 Flags: 0
    #[unity::class_method(80)] pub fn set_engrave(&self, god_data: &GodData); // Offset: 0x1FB0080 Flags: 0
    #[unity::class_method(81)] pub fn get_engrave(&self) -> Option<&'static GodData>; // Offset: 0x1FB00D0 Flags: 0

    // #[unity::class_method(83)] pub fn get_engage_weapon_god(&self) -> &'static GodUnit; // Offset: 0x1FB04D0 Flags: 0
    // #[unity::class_method(84)] pub fn set_engage_weapon_god(&self, unit: &Unit); // Offset: 0x1FB04E0 Flags: 0
    // #[unity::class_method(85)] pub fn set_engage_weapon_god2(&self, god_unit: &GodUnit); // Offset: 0x1FB0530 Flags: 0
    // #[unity::class_method(86)] pub fn clear_engage_weapon_god(&self); // Offset: 0x1FB21F0 Flags: 0
    // #[unity::class_method(87)] pub fn update_engage_weapon_param(&self); // Offset: 0x1FB0560 Flags: 0
    // #[unity::class_method(90)] pub fn get_iid(&self) -> &'static Il2CppString; // Offset: 0x1FA3C60 Flags: 0
    // #[unity::class_method(91)] pub fn get_index(&self) -> i32; // Offset: 0x1FB2990 Flags: 0
    #[unity::class_method(92)] pub fn is_flags(&self, flags: i32) -> bool; // Offset: 0x1FB29A0 Flags: 0
    #[unity::class_method(93)] pub fn set_flags(&self, flags: i32); // Offset: 0x1FB29B0 Flags: 0
    // #[unity::class_method(94)] pub fn clear_flags(&self, flags: UnitItemFlags); // Offset: 0x1FAE810 Flags: 0
    #[unity::class_method(95)] pub fn set_flags2(&self, flags: i32, enable: bool); // Offset: 0x1FB2970 Flags: 0
    #[unity::class_method(96)] pub fn get_is_equipped(&self) -> bool; // Offset: 0x1FB29C0 Flags: 0
    #[unity::class_method(97)] pub fn set_is_equipped(&self, value: bool); // Offset: 0x1FB29D0 Flags: 0
    #[unity::class_method(98)] pub fn is_drop(&self) -> bool; // Offset: 0x1FB29F0 Flags: 0
    #[unity::class_method(99)] pub fn set_is_drop(&self, value: bool); // Offset: 0x1FB2A00 Flags: 0
    // #[unity::class_method(100)] pub fn get_is_skip_log(&self) -> bool; // Offset: 0x1FB2A20 Flags: 0
    // #[unity::class_method(101)] pub fn set_is_skip_log(&self, value: bool); // Offset: 0x1FB2A30 Flags: 0
    #[unity::class_method(102)] pub fn get_is_enchant(&self) -> bool; // Offset: 0x1FB2A50 Flags: 0
    #[unity::class_method(103)] pub fn reset_enchant_item(); // Offset: 0x1FB2B30 Flags: 0
    #[unity::class_method(104)] pub fn set_enchant_item(item: &ItemData); // Offset: 0x1FB2BA0 Flags: 0
    #[unity::class_method(105)] pub fn set_enchant_hash(hash: i32); // Offset: 0x1FB2CE0 Flags: 0
    #[unity::class_method(106)] pub fn get_enchant_hash() -> i32; // Offset: 0x1FB2D50 Flags: 0
    #[unity::class_method(107)] pub fn get_flags_value(&self) -> i32; // Offset: 0x1FB2DC0 Flags: 0
    #[unity::class_method(108)] pub fn set_flags_value(&self, value: i32); // Offset: 0x1FB2DD0 Flags: 0
    #[unity::class_method(109)] pub fn get_endurance(&self) -> i32; // Offset: 0x1FB2DE0 Flags: 0
    #[unity::class_method(110)] pub fn set_endurance(&self, value: i32); // Offset: 0x1FB2DF0 Flags: 0
    #[unity::class_method(111)] pub fn get_refine_level(&self) -> i32; // Offset: 0x1FB2E00 Flags: 0
    #[unity::class_method(112)] pub fn set_refine_level(&self, value: i32); // Offset: 0x1FB2E10 Flags: 0
    #[unity::class_method(114)] pub fn get_give_skills(&self) -> Option<&'static SkillArray>; // Offset: 0x1FB2E60 Flags: 0
}

impl UnitItemList {
    pub fn has_item_iid(&self, iid: &str) -> bool { ItemData::get(iid).map(|item| self.has_item(item)).unwrap_or(false) }
    pub fn add_item_no_duplicate(&self, item: &ItemData){ if !self.has_item(item) { self.add(item); } }
    pub fn add_iid_no_duplicate(&self, iid: &str){
        let item = ItemData::get(iid);
        if !self.has_item_iid(iid) && item.is_some() { self.add(item.unwrap()); }
    }
    #[unity::class_method(0)] pub fn ctor(&self); // Offset: 0x1FB33F0 Flags: 0
    #[unity::class_method(1)] pub fn clear(&self); // Offset: 0x1FB36D0 Flags: 0
    #[unity::class_method(2)] pub fn copy_from(&self, from: &UnitItemList); // Offset: 0x1FB37F0 Flags: 0
    #[unity::class_method(3)] pub fn get_empty_index(&self) -> i32; // Offset: 0x1FB3990 Flags: 0
    #[unity::class_method(4)] pub fn add(&self, item: &ItemData) -> i32; // Offset: 0x1FB3AB0 Flags: 0
    #[unity::class_method(5)] pub fn add_unit_item(&self, unit_item: &UnitItem) -> i32; // Offset: 0x1FB3C00 Flags: 0
    #[unity::class_method(6)] pub fn move_item(&self, from: i32, to: i32); // Offset: 0x1FB3E00 Flags: 0
    #[unity::class_method(7)] pub fn close_up(&self); // Offset: 0x1FB3FF0 Flags: 0
    #[unity::class_method(8)] pub fn equip(&self, index: i32); // Offset: 0x1FB4760 Flags: 0
    #[unity::class_method(9)] pub fn take_off(&self, index: i32) -> bool; // Offset: 0x1FB4900 Flags: 0
    #[unity::class_method(10)] pub fn put_off(&self, index: i32, closeup: bool) -> bool; // Offset: 0x1FB4950 Flags: 0
    #[unity::class_method(11)] pub fn put_off_all_item(&self); // Offset: 0x1FB4BD0 Flags: 0
    #[unity::class_method(12)] pub fn is_equipped(&self) -> bool; // Offset: 0x1FB4C70 Flags: 0
    #[unity::class_method(13)] pub fn get_equipped(&self) -> &'static UnitItem; // Offset: 0x1FB4C90 Flags: 0
    #[unity::class_method(14)] pub fn get_equipped2(&self, index: i32) -> &'static UnitItem; // Offset: 0x1FB4CF0 Flags: 0
    #[unity::class_method(15)] pub fn get_index_equipped(&self) -> i32; // Offset: 0x1FB47E0 Flags: 0
    #[unity::class_method(16)] pub fn get_index_enchantable(&self, item: &ItemData) -> i32; // Offset: 0x1FB4D60 Flags: 0
    #[unity::class_method(17)] pub fn has_enchanted(&self) -> bool; // Offset: 0x1FB4EA0 Flags: 0
    #[unity::class_method(18)] pub fn get_hold(&self, unit: &Unit) -> &'static UnitItem; // Offset: 0x1FB4FC0 Flags: 0
    #[unity::class_method(19)] pub fn get_index_hold(&self, unit: &Unit) -> i32; // Offset: 0x1FB5020 Flags: 0
    #[unity::class_method(20)] pub fn has_rod(&self, unit: &Unit, can_use_check: bool) -> bool; // Offset: 0x1FB56E0 Flags: 0
    #[unity::class_method(21)] pub fn has_heal_rod(&self, unit: &Unit, can_use_check: bool) -> bool; // Offset: 0x1FB5D90 Flags: 0
    #[unity::class_method(22)] pub fn has_heal_rod_for_oneself(&self, unit: &Unit, can_use_check: bool) -> bool; // Offset: 0x1FB6450 Flags: 0
    #[unity::class_method(23)] pub fn has_support_rod(&self, unit: &Unit, can_use_check: bool, is_oneself: bool) -> bool; // Offset: 0x1FB6470 Flags: 0
    #[unity::class_method(24)] pub fn has_interference_rod(&self, unit: &Unit, can_us_check: bool) -> bool; // Offset: 0x1FB6B60 Flags: 0
    #[unity::class_method(25)] pub fn has_critical_weapon(&self, unit: &Unit, can_use_check: bool) -> bool; // Offset: 0x1FB7220 Flags: 0
    #[unity::class_method(26)] pub fn has_efficacy_weapon(&self, unit: &Unit, target: &Unit, can_use_check: bool) -> bool; // Offset: 0x1FB7A00 Flags: 0
    #[unity::class_method(27)] pub fn has_drop_item(&self) -> bool; // Offset: 0x1FB8290 Flags: 0
    #[unity::class_method(28)] pub fn has_item(&self, item: &ItemData) -> bool; // Offset: 0x1FB8360 Flags: 0
    #[unity::class_method(29)] pub fn find_item(&self, item: &ItemData) -> i32; // Offset: 0x1FB8380 Flags: 0
    #[unity::class_method(30)] pub fn put_engage_item(&self, god_unit: Option<&GodUnit>, engaged: bool); // Offset: 0x1FB84C0 Flags: 0
    #[unity::class_method(31)] pub fn get_item_count(&self, item: &ItemData) -> i32; // Offset: 0x1FB8930 Flags: 0
    #[unity::class_method(34)] pub fn get_item(&self, index: i32) -> Option<&'static mut UnitItem>; // Offset: 0x1FB8CA0 Flags: 0
    #[unity::class_method(35)] pub fn get_count(&self) -> i32; // Offset: 0x1FB8CE0 Flags: 0
}