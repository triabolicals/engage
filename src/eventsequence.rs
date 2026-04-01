//! Methods related to the flow of maps and their scripting.

use unity::prelude::*;

use crate::{proc::ProcInst, proc::ProcInstFields, script::DynValue};
use crate::force::ForceType;
use crate::map::inspectors::{MapInspector, MapInspectorKind, PokeInspector};
use crate::unit::Unit;

#[repr(C)]
#[unity::class("App", "EventSequence")]
pub struct EventSequence {
    pub proc: ProcInstFields,
    
}

impl EventSequence {
    /// Note: the arguments are currently Option until more is figure out. They serve no purpose at the moment.
    pub fn try_create_bind(
        proc: &ProcInst,
        func: &'static DynValue,
        _pre_call: Option<()>,
        _post_call: Option<()>,
        _args: Option<()>,
    ) {
        // TODO: Figure out the arguments
        unsafe {
            eventsequence_trycreatebind(proc, func, 0 as _, 0 as _, 0 as _, None);
        }
    }
    #[unity::class_method(17)] pub fn try_create_bind2(proc: &ProcInst, inspector: &MapInspector); // Offset: 0x24E3EC0 Flags: 0
    #[unity::class_method(18)] pub fn is_binding_ui() -> bool; // Offset: 0x24E3FA0 Flags: 0
    #[unity::class_method(19)] pub fn try_create_bind3(proc: &ProcInst, name: &Il2CppString) -> bool; // Offset: 0x24E4080 Flags: 0
    #[unity::class_method(20)] pub fn turn(proc: &ProcInst, turn: i32, force: ForceType) -> bool; // Offset: 0x24E4210 Flags: 0
    #[unity::class_method(21)] pub fn turn_after(proc: &ProcInst, turn: i32, force: ForceType) -> bool; // Offset: 0x24E4230 Flags: 0
    #[unity::class_method(22)] pub fn turn_end(proc: &ProcInst, turn: i32, force: ForceType) -> bool; // Offset: 0x24E4250 Flags: 0
    #[unity::class_method(23)] pub fn area(proc: &ProcInst, unit: &Unit, x: i32, z: i32, force: ForceType) -> bool; // Offset: 0x24E4270 Flags: 0
    #[unity::class_method(24)] pub fn die(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E4290 Flags: 0
    #[unity::class_method(25)] pub fn revive_before(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E42A0 Flags: 0
    #[unity::class_method(26)] pub fn revive_after(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E42B0 Flags: 0
    #[unity::class_method(27)] pub fn fixed(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E42C0 Flags: 0
    #[unity::class_method(28)] pub fn talk(proc: &ProcInst, from: &Unit, to: &Unit) -> bool; // Offset: 0x24E42D0 Flags: 0
    #[unity::class_method(29)] pub fn help_spot(proc: &ProcInst, unit: &Unit, x: i32, z: i32) -> bool; // Offset: 0x24E42F0 Flags: 0
    #[unity::class_method(30)] pub fn battle_before(proc: &ProcInst, from_unit: &Unit, to_unit: &Unit) -> bool; // Offset: 0x24E4310 Flags: 0
    #[unity::class_method(31)] pub fn battle_talk(proc: &ProcInst, from_unit: &Unit, to_unit: &Unit) -> bool; // Offset: 0x24E4330 Flags: 0
    #[unity::class_method(32)] pub fn battle_after(proc: &ProcInst, from_unit: &Unit, to_unit: &Unit) -> bool; // Offset: 0x24E4350 Flags: 0
    #[unity::class_method(33)] pub fn pickup(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E4370 Flags: 0
    #[unity::class_method(34)] pub fn unit_command_prepare(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E4380 Flags: 0
    #[unity::class_method(35)] pub fn unit_command_interrupt(proc: &ProcInst, unit: &Unit, command: &Il2CppString) -> bool; // Offset: 0x24E4390 Flags: 0
    #[unity::class_method(36)] pub fn engage_before(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E43E0 Flags: 0
    #[unity::class_method(37)] pub fn engage_after(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E43F0 Flags: 0
    #[unity::class_method(38)] pub fn target_select(proc: &ProcInst, unit: &Unit) -> bool; // Offset: 0x24E4400 Flags: 0
    #[unity::class_method(39)] pub fn poke(proc: &ProcInst, kind: MapInspectorKind, unit: &Unit, x: i32, z: i32) -> bool; // Offset: 0x24E4410 Flags: 0
    #[unity::class_method(40)] pub fn poke2(proc: &ProcInst, inspector: &PokeInspector) -> bool; // Offset: 0x24E4420 Flags: 0
    #[unity::class_method(41)] pub fn startup(proc: &ProcInst); // Offset: 0x24E4430 Flags: 0
    #[unity::class_method(42)] pub fn cleanup(proc: &ProcInst); // Offset: 0x24E4490 Flags: 0
    #[unity::class_method(43)] pub fn main(proc: &ProcInst); // Offset: 0x24E44F0 Flags: 0
    #[unity::class_method(44)] pub fn opening(proc: &ProcInst); // Offset: 0x24E4550 Flags: 0
    #[unity::class_method(45)] pub fn ending(proc: &ProcInst); // Offset: 0x24E45B0 Flags: 0
    #[unity::class_method(46)] pub fn game_over(proc: &ProcInst); // Offset: 0x24E4610 Flags: 0
    #[unity::class_method(47)] pub fn map_dispos(proc: &ProcInst); // Offset: 0x24E4670 Flags: 0
    #[unity::class_method(48)] pub fn map_opening(proc: &ProcInst); // Offset: 0x24E46D0 Flags: 0
    #[unity::class_method(49)] pub fn map_ending(proc: &ProcInst); // Offset: 0x24E4730 Flags: 0
    #[unity::class_method(50)] pub fn ctor(&self); // Offset: 0x24E3DE0 Flags: 0
}

#[unity::from_offset("App", "EventSequence", "MapOpening")]
fn eventsequence_mapopening(parent: &ProcInst, method_info: OptionalMethod);

#[unity::from_offset("App", "EventSequence", "AddCoroutine")]
pub fn eventsequence_addcoroutine(
    this: &ProcInst,
    func: &'static DynValue,
    args: *const u8,
    method_info: OptionalMethod,
) -> &'static EventSequence;

#[unity::from_offset("App", "EventSequence", "TryCreateBind")]
fn eventsequence_trycreatebind(
    parent: &ProcInst,
    func: &'static DynValue,
    pre_call: *const u8,
    post_call: *const u8,
    args: *const u8,
    method_info: OptionalMethod,
) -> bool;
