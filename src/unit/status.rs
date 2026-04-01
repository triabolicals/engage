use crate::bit::BitField64Methods;
use super::*;

#[unity::class("", "StatusField")]
#[nested_from_type(Unit)]
pub struct UnitStatusField { pub value: i64, }

impl BitField64Methods for UnitStatusField {}

#[allow(non_upper_case_globals)] 
impl UnitStatusField {
    pub const Fixed: i64 = 1;
    pub const MoveNotAllow: i64 = 2;
    pub const MustSortie: i64 = 4;
    pub const NeverSortie: i64 = 8;
    pub const DontPosChange: i64 = 16;
    pub const DangerShowing: i64 = 32;
    pub const ChainGuard: i64 = 64;
    pub const DualGuard: i64 = 128;
    pub const Sortie: i64 = 256;
    pub const Dead: i64 = 512;
    pub const EscapeHere: i64 = 1024;
    pub const DiedHere: i64 = 2048;
    pub const JoinHere: i64 = 4096;
    pub const PureHide: i64 = 8192;
    pub const DisposHide: i64 = 16384;
    pub const ViewOut: i64 = 32768;
    pub const UnderRoof: i64 = 65536;
    pub const LockedUpdate: i64 = 131072;
    pub const Removing: i64 = 262144;
    pub const Remagicing: i64 = 524288;
    pub const Rerewarping: i64 = 1048576;
    pub const Guest: i64 = 2097152;
    pub const DisposGuset: i64 = 4194304;
    pub const Engaging: i64 = 8388608;
    pub const EngageAttack: i64 = 16777216;
    pub const EngageLinked: i64 = 33554432;
    pub const EngageAttacked: i64 = 67108864;
    pub const Vision: i64 = 134217728;
    pub const ExistDead: i64 = 268435456;
    pub const Continued: i64 = 536870912;
    pub const Defect: i64 = 1073741824;
    pub const BowCannon: i64 = 4294967296;
    pub const MagicCannon: i64 = 8589934592;
    pub const FireCannon: i64 = 17179869184;
    pub const IgnoreWholeSkill: i64 = 68719476736;
    pub const IgnoreEquipSkill: i64 = 137438953472;
    pub const IgnoreEquipEnhance: i64 = 274877906944;
    pub const RelayLeave: i64 = 549755813888;
    pub const IgnoreImmortal: i64 = 1099511627776;
    pub const IgnoreGodUnit: i64 = 2199023255552;
    pub const IgonreMapEnhance: i64 = 4398046511104;
    pub const BeforeSortied: i64 = 8796093022208;
    pub const IgnoreSupportedSkill: i64 = 17592186044416;
    pub const Summon: i64 = 35184372088832;
    pub const Lockon: i64 = 70368744177664;
    pub const Reacted: i64 = 140737488355328;
    pub const HoldHp: i64 = 281474976710656;
    pub const IgnoreMapHistory: i64 = 562949953421312;
    pub const ChangeEngaged: i64 = 1125899906842624;
    pub const LockedSupport: i64 = 2251799813685248;
    pub const HideImage: i64 = 8192;
    pub const HideRender: i64 = 122880;
    pub const NotTarget: i64 = 65536;
    pub const InitMapBegin: i64 = 1130298490224924;
    pub const InitPhaseBegin: i64 = 1125899906842816;
    pub const InitPhaseEnd: i64 = 1548112424075265;
    pub const InitMapEnd: i64 = 3874680177943037;
    pub const DeadMask: i64 = 268438016;
    pub const GuardMask: i64 = 192;
    pub const EngageMask: i64 = 125829248;
    pub const FixedMask: i64 = 70368744177857;
    pub const CannonMask: i64 = 30064771072;
    pub const ReactionMask: i64 = 1835008;
    pub const SaveMask: i64 = -1;
}