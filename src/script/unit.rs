use unity::{prelude::*, il2cpp::object::Array};
use crate::force::ForceType;
use crate::gamedata::PersonData;
use crate::unit::Unit;
use super::{EventScript, DynValue};

// Namespace: App, Token: 0x2000A5A
#[unity::class("App", "ScriptUnit")]
pub struct ScriptUnit { }

impl ScriptUnit {
    #[unity::class_method(0)] pub fn force_unit_get_first(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2195E40 Flags: 0
    #[unity::class_method(1)] pub fn force_unit_get_next(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21960A0 Flags: 0
    #[unity::class_method(2)] pub fn force_unit_delete(args: &Array<&DynValue>); // Offset: 0x21963E0 Flags: 0
    #[unity::class_method(3)] pub fn unit_is_exist(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2196650 Flags: 0
    #[unity::class_method(4)] pub fn unit_get_by_pid(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2196720 Flags: 0
    #[unity::class_method(5)] pub fn unit_get_by_pos(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21969D0 Flags: 0
    #[unity::class_method(6)] pub fn unit_get_pid(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2196D80 Flags: 0
    #[unity::class_method(7)] pub fn unit_get_jid(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2196ED0 Flags: 0
    #[unity::class_method(8)] pub fn unit_get_mpid(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21970A0 Flags: 0
    #[unity::class_method(9)] pub fn unit_is_status(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21971F0 Flags: 0
    #[unity::class_method(10)] pub fn unit_set_status(args: &Array<&DynValue>); // Offset: 0x21973B0 Flags: 0
    #[unity::class_method(11)] pub fn unit_clear_status(args: &Array<&DynValue>); // Offset: 0x2197500 Flags: 0
    #[unity::class_method(12)] pub fn unit_set_show(args: &Array<&DynValue>); // Offset: 0x2197650 Flags: 0
    #[unity::class_method(13)] pub fn unit_get_force(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2197860 Flags: 0
    #[unity::class_method(14)] pub fn unit_get_level(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2197930 Flags: 0
    #[unity::class_method(15)] pub fn unit_get_hp(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2197A00 Flags: 0
    #[unity::class_method(16)] pub fn unit_get_move_cost(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x2197AD0 Flags: 0
    #[unity::class_method(17)] pub fn unit_set_hp(args: &Array<&DynValue>); // Offset: 0x2197D60 Flags: 0
    #[unity::class_method(18)] pub fn unit_reset_param(args: &Array<&DynValue>); // Offset: 0x2198030 Flags: 0
    #[unity::class_method(19)] pub fn unit_update(args: &Array<&DynValue>); // Offset: 0x21980C0 Flags: 0
    #[unity::class_method(20)] pub fn unit_get_capability(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21980D0 Flags: 0
    #[unity::class_method(21)] pub fn unit_die(args: &Array<&DynValue>); // Offset: 0x2199300 Flags: 0
    #[unity::class_method(22)] pub fn unit_die_without_event(args: &Array<&DynValue>); // Offset: 0x21995A0 Flags: 0
    #[unity::class_method(23)] pub fn unit_transfer_impl(unit: &Unit, force: ForceType); // Offset: 0x2199780 Flags: 0
    #[unity::class_method(24)] pub fn unit_transfer(args: &Array<&DynValue>); // Offset: 0x2199A00 Flags: 0
    #[unity::class_method(25)] pub fn unit_delete(args: &Array<&DynValue>); // Offset: 0x2199AE0 Flags: 0
    #[unity::class_method(26)] pub fn unit_join_impl(args: &Array<&DynValue>, index: i32) -> Option<&'static PersonData>; // Offset: 0x2199CB0 Flags: 0
    #[unity::class_method(27)] pub fn unit_join(args: &Array<&DynValue>); // Offset: 0x2199DC0 Flags: 0
    #[unity::class_method(28)] pub fn unit_join_silent(args: &Array<&DynValue>); // Offset: 0x2199EC0 Flags: 0
    #[unity::class_method(29)] pub fn unit_set_private_skill(args: &Array<&DynValue>); // Offset: 0x2199F00 Flags: 0
    #[unity::class_method(30)] pub fn unit_clear_private_skill(args: &Array<&DynValue>); // Offset: 0x219A060 Flags: 0
    #[unity::class_method(31)] pub fn unit_has_private_skill(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219A1C0 Flags: 0
    #[unity::class_method(32)] pub fn unit_has_whole_skill(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219A380 Flags: 0
    #[unity::class_method(33)] pub fn unit_get_x(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219A5E0 Flags: 0
    #[unity::class_method(34)] pub fn unit_get_z(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219A780 Flags: 0
    #[unity::class_method(35)] pub fn unit_can_enter(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219A920 Flags: 0
    #[unity::class_method(36)] pub fn unit_get_item_count(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219ABA0 Flags: 0
    #[unity::class_method(37)] pub fn unit_get_item_range_i(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219ACF0 Flags: 0
    #[unity::class_method(38)] pub fn unit_get_item_range_o(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219AF20 Flags: 0
    #[unity::class_method(39)] pub fn unit_set_item_equip(args: &Array<&DynValue>); // Offset: 0x219B100 Flags: 0
    #[unity::class_method(40)] pub fn unit_put_off_item(args: &Array<&DynValue>); // Offset: 0x219B290 Flags: 0
    #[unity::class_method(41)] pub fn ai_set_active(args: &Array<&DynValue>); // Offset: 0x219B4D0 Flags: 0
    #[unity::class_method(42)] pub fn ai_get_active(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219B640 Flags: 0
    #[unity::class_method(43)] pub fn ai_get_band_no(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219B7F0 Flags: 0
    #[unity::class_method(44)] pub fn ai_set_band_no(args: &Array<&DynValue>); // Offset: 0x219B990 Flags: 0
    #[unity::class_method(45)] pub fn ai_set_priority(args: &Array<&DynValue>); // Offset: 0x219BB60 Flags: 0
    #[unity::class_method(46)] pub fn ai_set_sequence(args: &Array<&DynValue>); // Offset: 0x219BD00 Flags: 0
    #[unity::class_method(47)] pub fn ai_get_sequence(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219BF60 Flags: 0
    #[unity::class_method(48)] pub fn ai_set_rerewarp(args: &Array<&DynValue>); // Offset: 0x219C190 Flags: 0
    #[unity::class_method(49)] pub fn ai_get_rerewarp_position(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219C480 Flags: 0
    #[unity::class_method(50)] pub fn ai_set_reject_power0_attack(args: &Array<&DynValue>); // Offset: 0x219C740 Flags: 0
    #[unity::class_method(51)] pub fn ai_clear_move_limit(args: &Array<&DynValue>); // Offset: 0x219C8C0 Flags: 0
    #[unity::class_method(52)] pub fn can_enter(unit: &Unit, x: i32, z: i32) -> bool; // Offset: 0x219C9F0 Flags: 0
    #[unity::class_method(53)] pub fn try_get_move_pos(args: &Array<&DynValue>, unit: &Unit, x: i32, z: i32, flag: i32) -> bool; // Offset: 0x219CAE0 Flags: 0
    #[unity::class_method(54)] pub fn unit_set_pos(args: &Array<&DynValue>); // Offset: 0x219CEC0 Flags: 0
    #[unity::class_method(55)] pub fn unit_move_pos(args: &Array<&DynValue>); // Offset: 0x219CFF0 Flags: 0
    #[unity::class_method(56)] pub fn unit_sync_sky_castle(args: &Array<&DynValue>); // Offset: 0x219D2D0 Flags: 0
    #[unity::class_method(57)] pub fn unit_jump_pos(args: &Array<&DynValue>); // Offset: 0x219D480 Flags: 0
    #[unity::class_method(58)] pub fn unit_rotation(args: &Array<&DynValue>); // Offset: 0x219F090 Flags: 0
    #[unity::class_method(59)] pub fn unit_warp_in(args: &Array<&DynValue>); // Offset: 0x219F310 Flags: 0
    #[unity::class_method(60)] pub fn unit_warp_out(args: &Array<&DynValue>); // Offset: 0x219F400 Flags: 0
    #[unity::class_method(61)] pub fn unit_translation(args: &Array<&DynValue>); // Offset: 0x219F4F0 Flags: 0
    #[unity::class_method(62)] pub fn unit_is_action(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219F7C0 Flags: 0
    #[unity::class_method(63)] pub fn unit_play_anim(args: &Array<&DynValue>); // Offset: 0x219F940 Flags: 0
    #[unity::class_method(64)] pub fn unit_get_engage_count(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219FA50 Flags: 0
    #[unity::class_method(65)] pub fn unit_set_engage_count(args: &Array<&DynValue>); // Offset: 0x219FB90 Flags: 0
    #[unity::class_method(66)] pub fn unit_get_engaging(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x219FDD0 Flags: 0
    #[unity::class_method(67)] pub fn update_image(); // Offset: 0x219FF50 Flags: 0
    #[unity::class_method(68)] pub fn unit_set_engaging(args: &Array<&DynValue>); // Offset: 0x219FFE0 Flags: 0
    #[unity::class_method(69)] pub fn unit_set_god_unit(args: &Array<&DynValue>); // Offset: 0x21A02A0 Flags: 0
    #[unity::class_method(70)] pub fn unit_get_god_unit(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21A05F0 Flags: 0
    #[unity::class_method(71)] pub fn god_unit_create(args: &Array<&DynValue>); // Offset: 0x21A07C0 Flags: 0
    #[unity::class_method(72)] pub fn god_unit_delete(args: &Array<&DynValue>); // Offset: 0x21A0870 Flags: 0
    #[unity::class_method(73)] pub fn god_unit_set_darkness(args: &Array<&DynValue>); // Offset: 0x21A0920 Flags: 0
    #[unity::class_method(74)] pub fn god_unit_set_escape(args: &Array<&DynValue>); // Offset: 0x21A0A60 Flags: 0
    #[unity::class_method(75)] pub fn god_unit_exists(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21A0BD0 Flags: 0
    #[unity::class_method(76)] pub fn god_data_get_mgid(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21A0D70 Flags: 0
    #[unity::class_method(77)] pub fn unit_get_god_state_count(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21A0ED0 Flags: 0
    #[unity::class_method(78)] pub fn unit_get_god_state(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21A1010 Flags: 0
    #[unity::class_method(79)] pub fn unit_set_god_state(args: &Array<&DynValue>); // Offset: 0x21A11E0 Flags: 0
    #[unity::class_method(80)] pub fn unit_reliance_permit_aplus(args: &Array<&DynValue>); // Offset: 0x21A1400 Flags: 0
    #[unity::class_method(81)] pub fn unit_shine(args: &Array<&DynValue>); // Offset: 0x21A1480 Flags: 0
    #[unity::class_method(82)] pub fn unit_set_hp_stock(args: &Array<&DynValue>); // Offset: 0x21A15F0 Flags: 0
    #[unity::class_method(83)] pub fn unit_get_hp_stock(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21A17A0 Flags: 0
    #[unity::class_method(84)] pub fn unit_get_hp_stock_max(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x21A18E0 Flags: 0
    #[unity::class_method(85)] pub fn regist(script: &EventScript); // Offset: 0x21A1A20 Flags: 0
}
