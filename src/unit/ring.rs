use crate::gamedata::ring::RingData;
use crate::unit::Unit;

#[unity::class("App", "UnitRing")]
pub struct UnitRing {
    base: [u8;0x10],
    pub data: &'static RingData,
    pub owner: Option<&'static Unit>,
    pub stock_count: u8
}
