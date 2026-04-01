use unity::prelude::*;
use crate::gamedata::person::PersonData;
use crate::proc::Bindable;
use super::Unit;

#[unity::class("App", "UnitUtil")] 
pub struct UnitUtil {}

impl UnitUtil {
    pub fn join_unit(pid: impl AsRef<str>) -> Option<&'static mut Unit> { Self::join_unit_(pid.into()) }
    #[unity::class_method(1)] pub fn reset_unit_force(); // Offset: 0x1C72F70 Flags: 0
    #[unity::class_method(2)] pub fn join_unit_(pid: &Il2CppString) -> Option<&'static mut Unit>; // Offset: 0x1C738D0 Flags: 0
    #[unity::class_method(3)] pub fn join_unit_person(person: &PersonData) -> Option<&'static mut Unit>; // Offset: 0x1C73960 Flags: 0
    #[unity::class_method(4)] pub fn join_message<B>(proc: &B, person1: Option<&PersonData>, person2: Option<&PersonData>, person3: Option<&PersonData>) where B: Bindable; // Offset: 0x1C73C00 Flags: 0
    #[unity::class_method(23)] pub fn get_vision_owner(vision_unit: &Unit) -> Option<&'static Unit>; // Offset: 0x1C764B0 Flags: 0
    #[unity::class_method(38)] pub fn unit_update();
    #[unity::class_method(34)] pub fn summon_create(owner: &Unit, rank: i32, person: &PersonData) -> Option<&'static mut Unit>; // Offset: 0x1C772A0 Flags: 0
    #[unity::class_method(30)] pub fn vision_create(owner: &Unit); // Offset: 0x1C76A90 Flags: 0
    #[unity::class_method(31)] pub fn can_vision_delete(owner: &Unit) -> bool; // Offset: 0x1C76CA0 Flags: 0
    #[unity::class_method(32)] pub fn vision_delete(owner: &Unit); // Offset: 0x1C76CE0 Flags: 0 
    #[unity::class_method(33)] pub fn get_summon_unit(owner: &Unit) -> Option<&'static Unit>; // Offset: 0x1C76E40 Flags: 0
    /*
    #[unity::class_method(5)] pub fn is_joined_unit(person: &PersonData) -> bool; // Offset: 0x1C73BE0 Flags: 0
    #[unity::class_method(6)] pub fn band_activate(unit: &Unit); // Offset: 0x1C73E30 Flags: 0
    #[unity::class_method(7)] pub fn refine_weapon(unit: &Unit, owner_item_index: i32, refine_level: i32) -> &'static UnitItem; // Offset: 0x1C73FE0 Flags: 0
    #[unity::class_method(8)] pub fn evolve_weapon(unit: &Unit, owner_item_index: i32, evolve_iid: &Il2CppString) -> &'static UnitItem; // Offset: 0x1C74120 Flags: 0
    #[unity::class_method(9)] pub fn engrave_weapon(unit: &Unit, owner_item_index: i32, god_data: &GodData) -> &'static UnitItem; // Offset: 0x1C74490 Flags: 0
    #[unity::class_method(10)] pub fn clear_engrave_weapon(unit: &Unit, owner_item_index: i32); // Offset: 0x1C745D0 Flags: 0
    #[unity::class_method(11)] pub fn item_disposal_for_dead(unit: &Unit); // Offset: 0x1C745E0 Flags: 0
    #[unity::class_method(12)] pub fn is_attack_range(unit: &Unit, unit_item: &UnitItem, attack_x: i32, attack_z: i32, attack_size: i32, target_x: i32, target_z: i32, target_size: i32) -> bool; // Offset: 0x1C74790 Flags: 0
    #[unity::class_method(13)] pub fn is_attack_range2(unit: &Unit, unit_item: &UnitItem, range: i32) -> bool; // Offset: 0x1C74F20 Flags: 0
    #[unity::class_method(14)] pub fn can_chain_attack(unit: &Unit, chain: &Unit, target: &Unit, unit_item: &UnitItem) -> bool; // Offset: 0x1C750A0 Flags: 0
    #[unity::class_method(15)] pub fn can_auto_equip_when_adding_item(unit: &Unit) -> bool; // Offset: 0x1C75430 Flags: 0
    #[unity::class_method(16)] pub fn can_auto_equip_when_adding_item2(unit: &Unit, item: &ItemData) -> bool; // Offset: 0x1C75480 Flags: 0
    #[unity::class_method(17)] pub fn can_auto_equip_when_adding_item3(unit: &Unit, unit_item: &UnitItem) -> bool; // Offset: 0x1C759C0 Flags: 0
    #[unity::class_method(18)] pub fn get_round_grow(percent: i32) -> i32; // Offset: 0x1C759D0 Flags: 0
    #[unity::class_method(19)] pub fn get_vision_person_impl() -> &'static PersonData; // Offset: 0x1C75A00 Flags: 0
    #[unity::class_method(20)] pub fn get_vision_count_impl(owner: &Unit) -> i32; // Offset: 0x1C75A90 Flags: 0
    #[unity::class_method(21)] pub fn can_vision_create_impl(owner: &Unit, x: i32, z: i32) -> bool; // Offset: 0x1C75EB0 Flags: 0
    #[unity::class_method(22)] pub fn for_each_vision(owner: &Unit, func: Action<&Unit>); // Offset: 0x1C76070 Flags: 0

    #[unity::class_method(24)] pub fn is_vision_unit(owner: &Unit, unit: &Unit) -> bool; // Offset: 0x1C76570 Flags: 0
    #[unity::class_method(25)] pub fn has_vision_unit(owner: &Unit) -> bool; // Offset: 0x1C76610 Flags: 0
    #[unity::class_method(26)] pub fn can_vision_create(owner: &Unit) -> bool; // Offset: 0x1C76650 Flags: 0
    #[unity::class_method(27)] pub fn get_vision_offsets() -> &'static Array<Vector2Int>; // Offset: 0x1C76770 Flags: 0
    #[unity::class_method(28)] pub fn for_each_creatable_vision(owner: &Unit, owner_x: i32, owner_z: i32, func: Action<&SkillData, i32, i32>); // Offset: 0x1C76850 Flags: 0
    #[unity::class_method(29)] pub fn get_creatable_vision_count(owner: &Unit, owner_x: i32, owner_z: i32) -> i32; // Offset: 0x1C76680 Flags: 0
    #[unity::class_method(33)] pub fn get_summon_unit(owner: &Unit) -> &'static Unit; // Offset: 0x1C76E40 Flags: 0

    #[unity::class_method(35)] pub fn summon_delete(owner: &Unit); // Offset: 0x1C77570 Flags: 0
    #[unity::class_method(36)] pub fn vision_delete_impl(unit: &Unit); // Offset: 0x1C77710 Flags: 0
    #[unity::class_method(37)] pub fn summon_delete_impl(unit: &Unit); // Offset: 0x1C77620 Flags: 0
// Offset: 0x1C77800 Flags: 0
    #[unity::class_method(39)] pub fn force_gathter(x: i32, z: i32, types: &Array<Type>); // Offset: 0x1C78030 Flags: 0
    #[unity::class_method(40)] pub fn get_free_point(unit: &Unit, tx: i32, tz: i32) -> bool; // Offset: 0x1C78370 Flags: 0
    #[unity::class_method(41)] pub fn try_add_reliance(unit_a: &Unit, unit_b: &Unit, name: &Il2CppString) -> bool; // Offset: 0x1C78660 Flags: 0
    #[unity::class_method(42)] pub fn try_add_reliance2(unit_a: &Unit, unit_b: &Unit, value: i32) -> bool; // Offset: 0x1C786A0 Flags: 0
    #[unity::class_method(43)] pub fn try_add_exp(unit: &Unit, god_unit: &GodUnit, exp: i32) -> bool; // Offset: 0x1C78770 Flags: 0
    #[unity::class_method(44)] pub fn get_die_type(unit: &Unit) -> UnitUtilDieType; // Offset: 0x1C78890 Flags: 0
    #[unity::class_method(45)] pub fn try_get_exist_message(mid: &Il2CppString, label: &Il2CppString, footer: &Il2CppString) -> bool; // Offset: 0x1C78B90 Flags: 0
    #[unity::class_method(46)] pub fn get_die_message(unit: &Unit, ascii: &Il2CppString) -> &'static Il2CppString; // Offset: 0x1C78CC0 Flags: 0
    #[unity::class_method(47)] pub fn get_die_message2(unit: &Unit) -> &'static Il2CppString; // Offset: 0x1C78F00 Flags: 0
    #[unity::class_method(48)] pub fn get_die_god_message(unit: &Unit) -> &'static Il2CppString; // Offset: 0x1C78F60 Flags: 0
    #[unity::class_method(49)] pub fn get_voice_id(unit: &Unit) -> &'static Il2CppString; // Offset: 0x1C790B0 Flags: 0
    #[unity::class_method(50)] pub fn is_same_unit(unit: &Unit, target: &Unit) -> bool; // Offset: 0x1C79130 Flags: 0
    #[unity::class_method(51)] pub fn is_weakness(unit: &Unit) -> bool; // Offset: 0x1C791A0 Flags: 0
    #[unity::class_method(52)] pub fn reset_chapter(chapter: &ChapterData); // Offset: 0x1C79430 Flags: 0
    #[unity::class_method(53)] pub fn get_summon_persons(rank: PersonDataRanks, color: PersonDataColors) -> List<&'static PersonData>; // Offset: 0x1C79850 Flags: 0
    #[unity::class_method(54)] pub fn get_summon_rank() -> PersonDataRanks; // Offset: 0x1C79B60 Flags: 0
    #[unity::class_method(55)] pub fn calc_summon(person: &PersonData, rank: PersonDataRanks, skill: &SkillData, color: PersonDataColors, dbg_rank: PersonDataRanks) -> bool; // Offset: 0x1C79C60 Flags: 0
    #[unity::class_method(56)] pub fn is_multi_change_god(unit: &Unit) -> bool; // Offset: 0x1C79FE0 Flags: 0
    #[unity::class_method(57)] pub fn is_out_of_view(position: Vector3) -> bool; // Offset: 0x1C7A0F0 Flags: 0
    #[unity::class_method(58)] pub fn try_multi_change_god(unit: &Unit) -> bool; // Offset: 0x1C7A1D0 Flags: 0
    #[unity::class_method(59)] pub fn get_help_key(unit: &Unit) -> &'static Il2CppString; // Offset: 0x1C7A580 Flags: 0
    #[unity::class_method(60)] pub fn god_save_equip(); // Offset: 0x1C7A610 Flags: 0
    #[unity::class_method(61)] pub fn god_load_equip(); // Offset: 0x1C7A730 Flags: 0
    #[unity::class_method(62)] pub fn get_move_first_add(unit: &Unit) -> i32; // Offset: 0x1C7A910 Flags: 0
    #[unity::class_method(63)] pub fn get_move_first_add2(unit: &Unit, x: i32, z: i32) -> i32; // Offset: 0x1C7AA40 Flags: 0
    #[unity::class_method(64)] pub fn get_move_first_add3(move_type: JobDataMoveTypes, move_power: i32, x: i32, z: i32) -> i32; // Offset: 0x1C7AE30 Flags: 0
    #[unity::class_method(65)] pub fn get_total_item_count(iid: &Il2CppString) -> i32; // Offset: 0x1C7B010 Flags: 0
    #[unity::class_method(66)] pub fn get_total_item_count2(item: &ItemData) -> i32; // Offset: 0x1C7B0A0 Flags: 0
    #[unity::class_method(67)] pub fn clear_enhance_all(); // Offset: 0x1C7B340 Flags: 0

     */
}
