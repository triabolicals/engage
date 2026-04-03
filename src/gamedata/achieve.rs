use num_derive::FromPrimitive;
use unity::prelude::*;
use unity::system::List;
use crate::gamedata::person::PersonData;
use super::{ChapterData, Gamedata, GodData, StructBaseFields};

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum AchieveDataStatus {
    None = 0,
    Cleared = 1,
    Showed = 2,
    Completed = 3,
}

#[unity::class("App", "AchieveData")]
pub struct AchieveData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub category: i32,
    pub kind: i32,
    pub count: i32,
    pub arg: &'static Il2CppString,
    pub count_unit: &'static Il2CppString,
    pub kizuna_rewards: i32,
    pub chapter: &'static Il2CppString,
    pub flagname: &'static Il2CppString,
}

impl AchieveData {
    #[unity::class_method(0)] pub fn load(); // Offset: 0x27C6930 Flags: 0
    #[unity::class_method(1)] pub fn is_grouping(kind: i32) -> bool; // Offset: 0x27C69E0 Flags: 0
    #[unity::class_method(2)] pub fn is_count(kind: i32) -> bool; // Offset: 0x27C6A00 Flags: 0
    #[unity::class_method(17)] pub fn get_reward(&self) -> i32; // Offset: 0x27C6B30 Flags: 0
    #[unity::class_method(24)] pub fn get_chapter(&self) -> Option<&'static ChapterData>; // Offset: 0x27C6DA0 Flags: 0
    #[unity::class_method(25)] pub fn get_person(&self) -> Option<&'static PersonData>; // Offset: 0x27C6E50 Flags: 0
    #[unity::class_method(26)] pub fn get_god(&self) -> Option<&'static GodData>; // Offset: 0x27C6F00 Flags: 0
    #[unity::class_method(27)] pub fn get_name2(&self) -> &'static Il2CppString; // Offset: 0x27C6FB0 Flags: 0
    #[unity::class_method(28)] pub fn get_status(&self) -> AchieveDataStatus; // Offset: 0x27C7380 Flags: 0
    #[unity::class_method(29)] pub fn set_cleared(&self) -> bool; // Offset: 0x27C7460 Flags: 0
    #[unity::class_method(30)] pub fn set_showed(&self) -> bool; // Offset: 0x27C7680 Flags: 0
    #[unity::class_method(31)] pub fn set_complete(&self) -> bool; // Offset: 0x27C76D0 Flags: 0
    #[unity::class_method(32)] pub fn is_can_get(&self) -> bool; // Offset: 0x27C7720 Flags: 0
    #[unity::class_method(33)] pub fn set_status(&self, status: AchieveDataStatus); // Offset: 0x27C74B0 Flags: 0
    #[unity::class_method(34)] pub fn get_value(&self) -> i32; // Offset: 0x27C7760 Flags: 0
    #[unity::class_method(35)] pub fn regist_global_flags(); // Offset: 0x27C7930 Flags: 0
    #[unity::class_method(36)] pub fn get_flag_name(kind: i32) -> &'static Il2CppString; // Offset: 0x27C7880 Flags: 0
    #[unity::class_method(37)] pub fn get_flag_name2(kind: i32, footer: &Il2CppString) -> &'static Il2CppString; // Offset: 0x27C84D0 Flags: 0
    #[unity::class_method(38)] pub fn get_flag_name3(kind: &Il2CppString) -> &'static Il2CppString; // Offset: 0x27C8550 Flags: 0
    #[unity::class_method(39)] pub fn get_count_current(&self, footer: &Il2CppString) -> i32; // Offset: 0x27C85B0 Flags: 0
    #[unity::class_method(40)] pub fn get_kind_count(kind: &Il2CppString) -> i32; // Offset: 0x27C87C0 Flags: 0
    #[unity::class_method(41)] pub fn set_clear_reliance(person: &PersonData); // Offset: 0x27C8910 Flags: 0
    #[unity::class_method(42)] pub fn add_count_reliance_b(); // Offset: 0x27C8CB0 Flags: 0
    #[unity::class_method(43)] pub fn add_count_reliance_a(); // Offset: 0x27C91D0 Flags: 0
    #[unity::class_method(44)] pub fn add_count_reliance_s(); // Offset: 0x27C9230 Flags: 0
    #[unity::class_method(45)] pub fn add_count_sortie(person: &PersonData); // Offset: 0x27C9290 Flags: 0
    #[unity::class_method(46)] pub fn add_count_encount(); // Offset: 0x27C9950 Flags: 0
    #[unity::class_method(47)] pub fn add_count_battle(); // Offset: 0x27C99B0 Flags: 0
    #[unity::class_method(48)] pub fn add_count_defeat(person: &PersonData); // Offset: 0x27C9A10 Flags: 0
    #[unity::class_method(49)] pub fn add_count_rod(); // Offset: 0x27C9B00 Flags: 0
    #[unity::class_method(50)] pub fn add_count_critical(); // Offset: 0x27C9B60 Flags: 0
    #[unity::class_method(51)] pub fn add_count_engage(); // Offset: 0x27C9BC0 Flags: 0
    #[unity::class_method(52)] pub fn add_count_engage_attack(); // Offset: 0x27C9C20 Flags: 0
    #[unity::class_method(53)] pub fn add_count_avoidance(); // Offset: 0x27C9C80 Flags: 0
    #[unity::class_method(54)] pub fn add_count_chain_guard(); // Offset: 0x27C9CE0 Flags: 0
    #[unity::class_method(55)] pub fn add_count_chain_attack(); // Offset: 0x27C9D40 Flags: 0
    #[unity::class_method(56)] pub fn set_value_chain_unit(unit_num: i32); // Offset: 0x27C9DA0 Flags: 0
    #[unity::class_method(57)] pub fn add_count_break(); // Offset: 0x27CA2D0 Flags: 0
    #[unity::class_method(58)] pub fn add_count_smash(); // Offset: 0x27CA330 Flags: 0
    #[unity::class_method(59)] pub fn add_count_mini_game(); // Offset: 0x27CA390 Flags: 0
    #[unity::class_method(60)] pub fn add_count_ring_form(add_value: i32); // Offset: 0x27CA3F0 Flags: 0
    #[unity::class_method(61)] pub fn add_count_ring_mix(); // Offset: 0x27CA460 Flags: 0
    #[unity::class_method(62)] pub fn add_count_bonds_ring_c(add_value: i32); // Offset: 0x27CA4C0 Flags: 0
    #[unity::class_method(63)] pub fn add_count_bonds_ring_b(add_value: i32); // Offset: 0x27CA530 Flags: 0
    #[unity::class_method(64)] pub fn add_count_bonds_ring_a(add_value: i32); // Offset: 0x27CA5A0 Flags: 0
    #[unity::class_method(65)] pub fn add_count_bonds_ring_s(add_value: i32); // Offset: 0x27CA610 Flags: 0
    #[unity::class_method(66)] pub fn add_count_ring_cleaning(); // Offset: 0x27CA680 Flags: 0
    #[unity::class_method(67)] pub fn set_value_investment_filene(level: i32); // Offset: 0x27CA6E0 Flags: 0
    #[unity::class_method(68)] pub fn set_value_investment_brodia(level: i32); // Offset: 0x27CA750 Flags: 0
    #[unity::class_method(69)] pub fn set_value_investment_ircion(level: i32); // Offset: 0x27CA7C0 Flags: 0
    #[unity::class_method(70)] pub fn set_value_investment_solum(level: i32); // Offset: 0x27CA830 Flags: 0
    #[unity::class_method(71)] pub fn set_clear_investment_all(); // Offset: 0x27CA8A0 Flags: 0
    #[unity::class_method(72)] pub fn add_count_investment_money(money: i32); // Offset: 0x27CAA80 Flags: 0
    #[unity::class_method(73)] pub fn add_count_cook_all(); // Offset: 0x27CAAF0 Flags: 0
    #[unity::class_method(74)] pub fn add_count_cook_g(); // Offset: 0x27CAB50 Flags: 0
    #[unity::class_method(75)] pub fn add_count_cook_f(); // Offset: 0x27CABB0 Flags: 0
    #[unity::class_method(76)] pub fn add_count_cook_e(); // Offset: 0x27CAC10 Flags: 0
    #[unity::class_method(77)] pub fn add_count_cook_d(); // Offset: 0x27CAC70 Flags: 0
    #[unity::class_method(78)] pub fn add_count_cook_c(); // Offset: 0x27CACD0 Flags: 0
    #[unity::class_method(79)] pub fn add_count_cook_b(); // Offset: 0x27CAD30 Flags: 0
    #[unity::class_method(80)] pub fn add_count_cook_a(); // Offset: 0x27CAD90 Flags: 0
    #[unity::class_method(81)] pub fn add_count_cook_s(); // Offset: 0x27CADF0 Flags: 0
    #[unity::class_method(82)] pub fn add_count_cook_ss(); // Offset: 0x27CAE50 Flags: 0
    #[unity::class_method(83)] pub fn add_count_sleep(); // Offset: 0x27CAEB0 Flags: 0
    #[unity::class_method(84)] pub fn add_count_wake_up_c(); // Offset: 0x27CAF10 Flags: 0
    #[unity::class_method(85)] pub fn add_count_wake_up_b(); // Offset: 0x27CAF70 Flags: 0
    #[unity::class_method(86)] pub fn add_count_wake_up_a(); // Offset: 0x27CAFD0 Flags: 0
    #[unity::class_method(87)] pub fn add_count_wake_up_s(); // Offset: 0x27CB030 Flags: 0
    #[unity::class_method(88)] pub fn add_count_unit_battle(); // Offset: 0x27CB090 Flags: 0
    #[unity::class_method(89)] pub fn add_count_unit_battle_win(); // Offset: 0x27CB0F0 Flags: 0
    #[unity::class_method(90)] pub fn add_count_god_battle(); // Offset: 0x27CB150 Flags: 0
    #[unity::class_method(91)] pub fn add_count_god_battle_win(); // Offset: 0x27CB1B0 Flags: 0
    #[unity::class_method(92)] pub fn add_count_buy_weapon(); // Offset: 0x27CB210 Flags: 0
    #[unity::class_method(93)] pub fn add_count_buy_sword(); // Offset: 0x27CB270 Flags: 0
    #[unity::class_method(94)] pub fn add_count_buy_lance(); // Offset: 0x27CB2D0 Flags: 0
    #[unity::class_method(95)] pub fn add_count_buy_axe(); // Offset: 0x27CB330 Flags: 0
    #[unity::class_method(96)] pub fn add_count_buy_bow(); // Offset: 0x27CB390 Flags: 0
    #[unity::class_method(97)] pub fn add_count_buy_knife(); // Offset: 0x27CB3F0 Flags: 0
    #[unity::class_method(98)] pub fn add_count_buy_magic(); // Offset: 0x27CB450 Flags: 0
    #[unity::class_method(99)] pub fn add_count_buy_fist(); // Offset: 0x27CB4B0 Flags: 0
    #[unity::class_method(100)] pub fn add_count_sell_weapon(add_value: i32); // Offset: 0x27CB510 Flags: 0
    #[unity::class_method(101)] pub fn add_count_buy_item(); // Offset: 0x27CB580 Flags: 0
    #[unity::class_method(102)] pub fn add_count_sell_item(add_value: i32); // Offset: 0x27CB5E0 Flags: 0
    #[unity::class_method(103)] pub fn add_count_buy_rod(); // Offset: 0x27CB650 Flags: 0
    #[unity::class_method(104)] pub fn add_count_buy_accessories(); // Offset: 0x27CB6B0 Flags: 0
    #[unity::class_method(105)] pub fn add_count_change_accessories(); // Offset: 0x27CB710 Flags: 0
    #[unity::class_method(106)] pub fn add_count_forging(); // Offset: 0x27CB770 Flags: 0
    #[unity::class_method(107)] pub fn add_count_forging_sword(); // Offset: 0x27CB7D0 Flags: 0
    #[unity::class_method(108)] pub fn add_count_forging_lance(); // Offset: 0x27CB830 Flags: 0
    #[unity::class_method(109)] pub fn add_count_forging_axe(); // Offset: 0x27CB890 Flags: 0
    #[unity::class_method(110)] pub fn add_count_forging_bow(); // Offset: 0x27CB8F0 Flags: 0
    #[unity::class_method(111)] pub fn add_count_forging_knife(); // Offset: 0x27CB950 Flags: 0
    #[unity::class_method(112)] pub fn add_count_forging_magic(); // Offset: 0x27CB9B0 Flags: 0
    #[unity::class_method(113)] pub fn add_count_forging_fist(); // Offset: 0x27CBA10 Flags: 0
    #[unity::class_method(114)] pub fn add_count_engrave(); // Offset: 0x27CBA70 Flags: 0
    #[unity::class_method(115)] pub fn set_value_play_time(time: f32); // Offset: 0x27CBAD0 Flags: 0
    #[unity::class_method(116)] pub fn set_clear_chapter(chapter: &ChapterData); // Offset: 0x27CBB80 Flags: 0
    #[unity::class_method(117)] pub fn add_count_net_match(); // Offset: 0x27CC050 Flags: 0
    #[unity::class_method(118)] pub fn add_count_net_match_win(); // Offset: 0x27CC0B0 Flags: 0
    #[unity::class_method(119)] pub fn add_count_relay_battle(); // Offset: 0x27CC110 Flags: 0
    #[unity::class_method(120)] pub fn add_count_relay_battle_win(); // Offset: 0x27CC170 Flags: 0
    #[unity::class_method(121)] pub fn set_challenge_rank(route: i32, level: i32); // Offset: 0x27CC1D0 Flags: 0
    #[unity::class_method(122)] pub fn play_report_add_count_class_change(); // Offset: 0x27CC340 Flags: 0
    #[unity::class_method(123)] pub fn play_report_add_count_push_ups_normal(); // Offset: 0x27CC3A0 Flags: 0
    #[unity::class_method(124)] pub fn play_report_add_count_push_ups_hard(); // Offset: 0x27CC400 Flags: 0
    #[unity::class_method(125)] pub fn play_report_add_count_push_ups_expert(); // Offset: 0x27CC460 Flags: 0
    #[unity::class_method(126)] pub fn play_report_add_count_push_ups_muscle(); // Offset: 0x27CC4C0 Flags: 0
    #[unity::class_method(127)] pub fn play_report_add_count_ads_normal(); // Offset: 0x27CC520 Flags: 0
    #[unity::class_method(128)] pub fn play_report_add_count_ads_hard(); // Offset: 0x27CC580 Flags: 0
    #[unity::class_method(129)] pub fn play_report_add_count_ads_expert(); // Offset: 0x27CC5E0 Flags: 0
    #[unity::class_method(130)] pub fn play_report_add_count_ads_muscle(); // Offset: 0x27CC640 Flags: 0
    #[unity::class_method(131)] pub fn play_report_add_count_squat_normal(); // Offset: 0x27CC6A0 Flags: 0
    #[unity::class_method(132)] pub fn play_report_add_count_squat_hard(); // Offset: 0x27CC700 Flags: 0
    #[unity::class_method(133)] pub fn play_report_add_count_squat_expert(); // Offset: 0x27CC760 Flags: 0
    #[unity::class_method(134)] pub fn play_report_add_count_squat_muscle(); // Offset: 0x27CC7C0 Flags: 0
    #[unity::class_method(135)] pub fn play_report_add_count_dragon_ride_normal(); // Offset: 0x27CC820 Flags: 0
    #[unity::class_method(136)] pub fn play_report_add_count_dragon_ride_hard(); // Offset: 0x27CC880 Flags: 0
    #[unity::class_method(137)] pub fn play_report_add_count_dragon_ride_expert(); // Offset: 0x27CC8E0 Flags: 0
    #[unity::class_method(138)] pub fn play_report_add_count_fishing_rod_smoll(); // Offset: 0x27CC940 Flags: 0
    #[unity::class_method(139)] pub fn play_report_add_count_fishing_rod_normal(); // Offset: 0x27CC9A0 Flags: 0
    #[unity::class_method(140)] pub fn play_report_add_count_fishing_rod_all_purpose(); // Offset: 0x27CCA00 Flags: 0
    #[unity::class_method(141)] pub fn play_report_add_count_cleaning_form_god(gid: &Il2CppString); // Offset: 0x27CCA60 Flags: 0
    #[unity::class_method(142)] pub fn play_report_add_count_challenge(route: i32); // Offset: 0x27CCD70 Flags: 0
    #[unity::class_method(143)] pub fn play_report_add_count_challenge_clear(route: i32); // Offset: 0x27CCF10 Flags: 0
    #[unity::class_method(144)] pub fn play_report_add_count_relay_battle(route: i32); // Offset: 0x27CD0B0 Flags: 0
    #[unity::class_method(145)] pub fn play_report_add_count_relay_battle_inherit(); // Offset: 0x27CD250 Flags: 0
    #[unity::class_method(146)] pub fn play_report_add_count_versus_casual(); // Offset: 0x27CD2B0 Flags: 0
    #[unity::class_method(147)] pub fn play_report_add_count_versus_casual_win(); // Offset: 0x27CD310 Flags: 0
    #[unity::class_method(148)] pub fn play_report_add_count_versus_casual_lose(); // Offset: 0x27CD370 Flags: 0
    #[unity::class_method(149)] pub fn play_report_add_count_versus_ranked(); // Offset: 0x27CD3D0 Flags: 0
    #[unity::class_method(150)] pub fn play_report_add_count_versus_ranked_win(); // Offset: 0x27CD430 Flags: 0
    #[unity::class_method(151)] pub fn play_report_add_count_versus_ranked_lose(); // Offset: 0x27CD490 Flags: 0
    #[unity::class_method(152)] pub fn play_report_add_count_versus_ranked_defense_win(); // Offset: 0x27CD4F0 Flags: 0
    #[unity::class_method(153)] pub fn play_report_add_count_versus_ranked_defense_lose(); // Offset: 0x27CD550 Flags: 0
    #[unity::class_method(154)] pub fn play_report_get_count(kinds: i32) -> i32; // Offset: 0x27CD5B0 Flags: 0
    #[unity::class_method(155)] pub fn play_report_get_count_cleaning_form_god(gid: &Il2CppString) -> i32; // Offset: 0x27CD860 Flags: 0
    #[unity::class_method(156)] pub fn try_set_cleard(data: &AchieveData) -> bool; // Offset: 0x27C8BC0 Flags: 0
    #[unity::class_method(157)] pub fn is_prohibited() -> bool; // Offset: 0x27C9810 Flags: 0
    #[unity::class_method(158)] pub fn is_valid(flag_name: &Il2CppString) -> bool; // Offset: 0x27CD750 Flags: 0
    #[unity::class_method(159)] pub fn add_count(kind: i32, add_value: i32); // Offset: 0x27C8D10 Flags: 0
    #[unity::class_method(160)] pub fn commit_value(kind: i32, value: i32); // Offset: 0x27C9E10 Flags: 0
    #[unity::class_method(161)] pub fn get_kind_list(kind: i32) -> &'static List<&'static AchieveData>; // Offset: 0x27C8420 Flags: 0
    #[unity::class_method(162)] pub fn update_show_queue(); // Offset: 0x27CD9E0 Flags: 0
    #[unity::class_method(163)] pub fn clear_show_queue(); // Offset: 0x27CDBE0 Flags: 0
    #[unity::class_method(164)] pub fn dequeue_show_data() -> &'static AchieveData; // Offset: 0x27CDCC0 Flags: 0
}
impl Gamedata for AchieveData {}