use unity::{prelude::*, il2cpp::object::Array};
use super::DynValue;

// Namespace: App, Token: 0x2000A57
#[unity::class("App", "ScriptSystem")]
pub struct ScriptSystem { }

impl ScriptSystem {
    #[unity::class_method(0)] pub fn include(args: &Array<&DynValue>); // Offset: 0x1ED7D70 Flags: 0
    #[unity::class_method(1)] pub fn yield_impl(args: &Array<&DynValue>); // Offset: 0x1ED7FE0 Flags: 0
    #[unity::class_method(2)] pub fn jump_impl(args: &Array<&DynValue>); // Offset: 0x1ED8050 Flags: 0
    #[unity::class_method(3)] pub fn log(args: &Array<&DynValue>); // Offset: 0x1ED8190 Flags: 0
    #[unity::class_method(4)] pub fn warning(args: &Array<&DynValue>); // Offset: 0x1ED81A0 Flags: 0
    #[unity::class_method(5)] pub fn time_get_delta(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED81B0 Flags: 0
    #[unity::class_method(6)] pub fn skip_is_black_out(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED8250 Flags: 0
    #[unity::class_method(7)] pub fn skip_escape(args: &Array<&DynValue>); // Offset: 0x1ED82C0 Flags: 0
    #[unity::class_method(8)] pub fn skip_push_state_and_disable(args: &Array<&DynValue>); // Offset: 0x1ED8340 Flags: 0
    #[unity::class_method(9)] pub fn skip_pop_state(args: &Array<&DynValue>); // Offset: 0x1ED8360 Flags: 0
    #[unity::class_method(10)] pub fn talk(args: &Array<&DynValue>); // Offset: 0x1ED8370 Flags: 0
    #[unity::class_method(11)] pub fn talk_begin_continue(args: &Array<&DynValue>); // Offset: 0x1ED8500 Flags: 0
    #[unity::class_method(12)] pub fn talk_end_continue(args: &Array<&DynValue>); // Offset: 0x1ED8510 Flags: 0
    #[unity::class_method(13)] pub fn dialog(args: &Array<&DynValue>); // Offset: 0x1ED8590 Flags: 0
    #[unity::class_method(14)] pub fn notice(args: &Array<&DynValue>); // Offset: 0x1ED8670 Flags: 0
    #[unity::class_method(15)] pub fn mess_load(args: &Array<&DynValue>); // Offset: 0x1ED8740 Flags: 0
    #[unity::class_method(16)] pub fn mess_free(args: &Array<&DynValue>); // Offset: 0x1ED87F0 Flags: 0
    #[unity::class_method(17)] pub fn mess_is_exist(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED88A0 Flags: 0
    #[unity::class_method(18)] pub fn mess_set_argument_impl(args: &Array<&DynValue>, offset: i32); // Offset: 0x1ED8420 Flags: 0
    #[unity::class_method(19)] pub fn mess_set_argument_impl2(index: i32, value: &DynValue); // Offset: 0x1ED8990 Flags: 0
    #[unity::class_method(20)] pub fn mess_set_argument(args: &Array<&DynValue>); // Offset: 0x1ED8B10 Flags: 0
    #[unity::class_method(21)] pub fn scene_get_name(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED8BF0 Flags: 0
    #[unity::class_method(22)] pub fn scene_load(args: &Array<&DynValue>); // Offset: 0x1ED8CB0 Flags: 0
    #[unity::class_method(23)] pub fn scene_unload(args: &Array<&DynValue>); // Offset: 0x1ED8D80 Flags: 0
    #[unity::class_method(24)] pub fn puppet_demo(args: &Array<&DynValue>); // Offset: 0x1ED8E20 Flags: 0
    #[unity::class_method(25)] pub fn cut_scene(args: &Array<&DynValue>); // Offset: 0x1ED8EE0 Flags: 0
    #[unity::class_method(26)] pub fn movie(args: &Array<&DynValue>); // Offset: 0x1ED8EF0 Flags: 0
    #[unity::class_method(27)] pub fn telop(args: &Array<&DynValue>); // Offset: 0x1ED9000 Flags: 0
    #[unity::class_method(28)] pub fn mode_select(args: &Array<&DynValue>); // Offset: 0x1ED90E0 Flags: 0
    #[unity::class_method(29)] pub fn win_rule(args: &Array<&DynValue>); // Offset: 0x1ED90F0 Flags: 0
    #[unity::class_method(30)] pub fn tutorial(args: &Array<&DynValue>); // Offset: 0x1ED91C0 Flags: 0
    #[unity::class_method(31)] pub fn camera_get_main(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED92E0 Flags: 0
    // #[unity::class_method(32)] pub fn get_event_camera(camera: &Camera) -> &'static EventCamera; // Offset: 0x1ED9480 Flags: 0
    #[unity::class_method(33)] pub fn camera_set_main(args: &Array<&DynValue>); // Offset: 0x1ED9540 Flags: 0
    #[unity::class_method(34)] pub fn variable_entry(args: &Array<&DynValue>); // Offset: 0x1ED9880 Flags: 0
    #[unity::class_method(35)] pub fn variable_is_exist(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1ED9A80 Flags: 0
    #[unity::class_method(36)] pub fn variable_set(args: &Array<&DynValue>); // Offset: 0x1ED9C10 Flags: 0
    #[unity::class_method(37)] pub fn variable_get(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDA040 Flags: 0
    #[unity::class_method(38)] pub fn fade_in(args: &Array<&DynValue>); // Offset: 0x1EDA300 Flags: 0
    #[unity::class_method(39)] pub fn fade_out(args: &Array<&DynValue>); // Offset: 0x1EDA3C0 Flags: 0
    #[unity::class_method(40)] pub fn is_fading(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDA530 Flags: 0
    #[unity::class_method(41)] pub fn is_loading(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDA5D0 Flags: 0
    /*
    #[unity::class_method(42)] pub fn variable_entry_monitor(args: &Array<&DynValue>); // Offset: 0x1EDA670 Flags: 0
    #[unity::class_method(43)] pub fn variable_clear_monitor(args: &Array<&DynValue>); // Offset: 0x1EDA680 Flags: 0
    #[unity::class_method(44)] pub fn get_debug_buttons(args: &Array<&DynValue>, index: i32) -> NpadButton; // Offset: 0x1EDA690 Flags: 0
    #[unity::class_method(45)] pub fn debug_is_button(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDAAA0 Flags: 0
    #[unity::class_method(46)] pub fn debug_is_trigger(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDAB00 Flags: 0
    #[unity::class_method(47)] pub fn debug_hook_button(args: &Array<&DynValue>); // Offset: 0x1EDAB60 Flags: 0
    #[unity::class_method(48)] pub fn debug_hook_stick_l(args: &Array<&DynValue>); // Offset: 0x1EDAB70 Flags: 0
    #[unity::class_method(49)] pub fn debug_hook_stick_r(args: &Array<&DynValue>); // Offset: 0x1EDAB80 Flags: 0
    #[unity::class_method(50)] pub fn debug_create_menu(args: &Array<&DynValue>); // Offset: 0x1EDAB90 Flags: 0
    #[unity::class_method(51)] pub fn debug_set_param(args: &Array<&DynValue>); // Offset: 0x1EDABA0 Flags: 0
    #[unity::class_method(52)] pub fn debug_get_param(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDABB0 Flags: 0
    #[unity::class_method(53)] pub fn debug_is_proc_exists(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDAC70 Flags: 0
    #[unity::class_method(54)] pub fn debug_is_auto_play(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDAD30 Flags: 0
    #[unity::class_method(55)] pub fn object_is_exist(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDADF0 Flags: 0
    #[unity::class_method(56)] pub fn object_create(args: &Array<&DynValue>); // Offset: 0x1EDAFC0 Flags: 0
    #[unity::class_method(57)] pub fn object_delete(args: &Array<&DynValue>); // Offset: 0x1EDB290 Flags: 0
    #[unity::class_method(58)] pub fn object_set_active(args: &Array<&DynValue>); // Offset: 0x1EDB380 Flags: 0
    #[unity::class_method(59)] pub fn object_activate(args: &Array<&DynValue>); // Offset: 0x1EDB480 Flags: 0
    #[unity::class_method(60)] pub fn object_deactivate(args: &Array<&DynValue>); // Offset: 0x1EDB560 Flags: 0
    #[unity::class_method(61)] pub fn has_content(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDB640 Flags: 0
    #[unity::class_method(62)] pub fn debug_gpu_perf(args: &Array<&DynValue>); // Offset: 0x1EDB7F0 Flags: 0
    #[unity::class_method(63)] pub fn debug_add_coroutine(args: &Array<&DynValue>); // Offset: 0x1EDB800 Flags: 0
    #[unity::class_method(64)] pub fn random_get(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDB810 Flags: 0
    #[unity::class_method(65)] pub fn sub_prefix(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDB910 Flags: 0
    #[unity::class_method(66)] pub fn string_contains(args: &Array<&DynValue>) -> &'static DynValue; // Offset: 0x1EDBAB0 Flags: 0
    #[unity::class_method(67)] pub fn get_main_camera() -> &'static Camera; // Offset: 0x1EDBC40 Flags: 0
    #[unity::class_method(68)] pub fn background_color_set(args: &Array<&DynValue>); // Offset: 0x1EDBDC0 Flags: 0
    #[unity::class_method(69)] pub fn regist(script: &EventScript); // Offset: 0x1EDBF10 Flags: 0
    #[unity::class_method(70)] pub fn ctor(&self); // Offset: 0x1EDD0B0 Flags: 0
     #[unity::class_method(0, vtable)] pub fn equals(&self, obj: Object) -> bool; // Offset: 0x37DF1A0 Flags: 0
    #[unity::class_method(1, vtable)] pub fn finalize(&self); // Offset: 0x37DF1E0 Flags: 0
    #[unity::class_method(2, vtable)] pub fn get_hash_code(&self) -> i32; // Offset: 0x37DF1F0 Flags: 0
    #[unity::class_method(3, vtable)] pub fn to_string(&self) -> &'static Il2CppString; // Offset: 0x37DF210 Flags: 0
     */
}