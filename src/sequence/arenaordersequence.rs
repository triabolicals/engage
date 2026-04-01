use unity::system::List;
use super::*;
use crate::{
    battle::BattleCalculator,
    unit::{UnitRing, Unit},
    god::GodUnit,
};

#[unity::class("App", "ArenaOrderSequence")]
pub struct ArenaOrderSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub loaded: bool,
    pub unit_list: &'static List<Unit>,
    gods_info_list: *const u8,
    pub is_emblem_battle: bool,
    pub is_special_battle: bool, 
    pub training_type: i32,
    pub training_unit: &'static Unit,
    pub battle_unit: &'static Unit,
    pub battle_emblem: &'static GodUnit,
    pub emblem_type: i32,
    pub bond_exp: i32,
    pub calculator: &'static BattleCalculator,
    pub sim_calculator: &'static BattleCalculator,
    exp_unit_select_root: *const u8,
    bond_unit_select_root: *const u8,
    bond_emblem_select_root: *const u8,
    bond_level_select_root: *const u8,
    pub next_label: i32,
    pub is_back_bond_select_emblem: bool,
    arena_objects: *const u8,
    pub god_unit: Option<&'static GodUnit>,
    pub ring: Option<&'static UnitRing>,
}
impl Bindable for ArenaOrderSequence {}

impl ArenaOrderSequence {
    #[unity::class_method(7)] pub fn get_next_level_cap(unit: &Unit, god: &GodUnit, is_cap_over: bool, is_level_cap_count: bool) -> i32; // Offset: 0x1CA01E0 Flags: 0
    #[unity::class_method(8)] pub fn is_level_cap_talk(unit: &Unit, god: &GodUnit) -> bool; // Offset: 0x1CA0280 Flags: 0

    #[unity::class_method(29)] pub fn create_bind<B>(proc: &B) where B: Bindable; // Offset: 0x1CA5330 Flags: 0
    #[unity::class_method(23)] pub fn get_bond_exp(&self) -> i32; // Offset: 0x1CA52D0 Flags: 0
    #[unity::class_method(24)] pub fn set_bond_exp(&self, value: i32); // Offset: 0x1CA52E0 Flags: 0
    #[unity::class_method(25)] pub fn get_calculator(&self) -> &'static BattleCalculator; // Offset: 0x1CA52F0 Flags: 0
    #[unity::class_method(27)] pub fn get_sim_calculator(&self) -> &'static BattleCalculator; // Offset: 0x1CA5310 Flags: 0
    #[unity::class_method(34)] pub fn disp_title_bar(&self); // Offset: 0x1CA70A0 Flags: 0
    #[unity::class_method(48)] pub fn setup_training(&self); // Offset: 0x1CA9E80 Flags: 0
    #[unity::class_method(49)] pub fn start_training(&self); // Offset: 0x1CAA3A0 Flags: 0
    #[unity::class_method(50)] pub fn finish_training(&self); // Offset: 0x1CAA4B0 Flags: 0

    /*
    #[unity::class_method(0)] pub fn get_selectable_unit(&self) -> List<&'static Unit>; // Offset: 0x1CA5130 Flags: 0
    #[unity::class_method(1)] pub fn set_selectable_unit(&self, value: List<&Unit>); // Offset: 0x1CA5140 Flags: 0
    #[unity::class_method(4)] pub fn setup_selectable_god_list(&self, unit: &Unit); // Offset: 0x1C9DE80 Flags: 0
    #[unity::class_method(5)] pub fn is_selectable_god(god: &GodUnit, unit: &Unit) -> bool; // Offset: 0x1C9E450 Flags: 0
    #[unity::class_method(6)] pub fn is_selectable_god_impl(god: &GodUnit) -> bool; // Offset: 0x1CA5170 Flags: 0
    #[unity::class_method(7)] pub fn get_next_level_cap(unit: &Unit, god: &GodUnit, is_cap_over: bool, is_level_cap_count: bool) -> i32; // Offset: 0x1CA01E0 Flags: 0
    #[unity::class_method(8)] pub fn is_level_cap_talk(unit: &Unit, god: &GodUnit) -> bool; // Offset: 0x1CA0280 Flags: 0
    #[unity::class_method(9)] pub fn get_is_emblem_battle(&self) -> bool; // Offset: 0x1CA51F0 Flags: 0
    #[unity::class_method(10)] pub fn set_is_emblem_battle(&self, value: bool); // Offset: 0x1CA5200 Flags: 0
    #[unity::class_method(11)] pub fn get_is_special_battle(&self) -> bool; // Offset: 0x1CA5210 Flags: 0
    #[unity::class_method(12)] pub fn set_is_special_battle(&self, value: bool); // Offset: 0x1CA5220 Flags: 0
    #[unity::class_method(15)] pub fn get_training_unit(&self) -> &'static Unit; // Offset: 0x1CA5250 Flags: 0
    #[unity::class_method(16)] pub fn set_training_unit(&self, value: &Unit); // Offset: 0x1CA5260 Flags: 0
    #[unity::class_method(17)] pub fn get_battle_unit(&self) -> &'static Unit; // Offset: 0x1CA5270 Flags: 0
    #[unity::class_method(18)] pub fn set_battle_unit(&self, value: &Unit); // Offset: 0x1CA5280 Flags: 0
    #[unity::class_method(19)] pub fn get_battle_emblem(&self) -> &'static GodUnit; // Offset: 0x1CA5290 Flags: 0
    #[unity::class_method(20)] pub fn set_battle_emblem(&self, value: &GodUnit); // Offset: 0x1CA52A0 Flags: 0
    #[unity::class_method(23)] pub fn get_bond_exp(&self) -> i32; // Offset: 0x1CA52D0 Flags: 0
    #[unity::class_method(24)] pub fn set_bond_exp(&self, value: i32); // Offset: 0x1CA52E0 Flags: 0
    #[unity::class_method(25)] pub fn get_calculator(&self) -> &'static BattleCalculator; // Offset: 0x1CA52F0 Flags: 0
    #[unity::class_method(26)] pub fn set_calculator(&self, value: &BattleCalculator); // Offset: 0x1CA5300 Flags: 0
    #[unity::class_method(27)] pub fn get_sim_calculator(&self) -> &'static BattleCalculator; // Offset: 0x1CA5310 Flags: 0
    #[unity::class_method(28)] pub fn set_sim_calculator(&self, value: &BattleCalculator); // Offset: 0x1CA5320 Flags: 0

    #[unity::class_method(31)] pub fn unload(&self); // Offset: 0x1CA6980 Flags: 0
    #[unity::class_method(32)] pub fn background_in(&self); // Offset: 0x1CA6F40 Flags: 0
    #[unity::class_method(33)] pub fn background_out(&self); // Offset: 0x1CA6FF0 Flags: 0
    #[unity::class_method(34)] pub fn disp_title_bar(&self); // Offset: 0x1CA70A0 Flags: 0
    #[unity::class_method(35)] pub fn create_skill_inheritance(&self); // Offset: 0x1CA7200 Flags: 0
    #[unity::class_method(36)] pub fn create_top_menu(&self); // Offset: 0x1CA7210 Flags: 0
    #[unity::class_method(47)] pub fn culculate(&self); // Offset: 0x1CA9C10 Flags: 0
    #[unity::class_method(48)] pub fn setup_training(&self); // Offset: 0x1CA9E80 Flags: 0
    #[unity::class_method(49)] pub fn start_training(&self); // Offset: 0x1CAA3A0 Flags: 0
    #[unity::class_method(50)] pub fn finish_training(&self); // Offset: 0x1CAA4B0 Flags: 0
    #[unity::class_method(51)] pub fn set_battle_unit_weapon(&self, unit: &Unit); // Offset: 0x1CA8E00 Flags: 0
    #[unity::class_method(52)] pub fn set_emblem_weapon(&self, unit: &Unit, emblem: &GodUnit, bond_level: i32); // Offset: 0x1CA94D0 Flags: 0
    #[unity::class_method(53)] pub fn exit(&self); // Offset: 0x1CAA930 Flags: 0
    #[unity::class_method(54)] pub fn ctor(&self); // Offset: 0x1CA67F0 Flags: 0
     */
}

impl AsMut<ProcInstFields> for ArenaOrderSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}
impl AsRef<ProcInstFields> for ArenaOrderSequence {
    fn as_ref(&self) -> &ProcInstFields { &self.proc }
}




#[unity::class("Combat", "ArenaCombatSequence")]
pub struct ArenaCombatSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub loaded: bool,
}


impl ArenaCombatSequence {
    pub fn ctor(&self, calc: &BattleCalculator, sim: &BattleCalculator, is_emblem: bool, is_special: bool, bond_exp: i32) {
        unsafe { combatarena_ctor(self, calc, sim, is_emblem, is_special, bond_exp, None);}
    }
}
#[unity::from_offset("Combat", "ArenaCombatSequence", ".ctor")]
fn combatarena_ctor(this: &ArenaCombatSequence, calc: &BattleCalculator, sim_calc: &BattleCalculator, is_emblem: bool, is_special: bool, bond_exp: i32, method_info: OptionalMethod);

#[unity::from_offset("App","ArenaOrderSequence", "CreateBind")]
fn arena_order_create_bind<P: Bindable>(parent: &P, method_info: OptionalMethod);

#[unity::from_offset("App", "ArenaOrderSequence", "StartTraining")]
fn arena_start_training(this: &ArenaOrderSequence,  method_info: OptionalMethod);


