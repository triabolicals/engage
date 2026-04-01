use num_derive::FromPrimitive;

#[repr(i32)]
#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, FromPrimitive, Ord)]
pub enum MapPanelDeployMode {
    Hide = 0, // Attr: 17
    MoveThinly = 1, // Attr: 17
    Move = 2, // Attr: 17
    MoveFree = 3, // Attr: 17
    Attack = 4, // Attr: 17
    Destroy = 5, // Attr: 17
    Rod = 6, // Attr: 17
    Dance = 7, // Attr: 17
    Engage = 8, // Attr: 17
    Direct = 9, // Attr: 17
    UnitCommand = 10, // Attr: 17
    UnitMenu = 11, // Attr: 17
    Target = 12, // Attr: 17
    Talk = 13, // Attr: 17
    TrickThinly = 14, // Attr: 17
    Trick = 15, // Attr: 17
    Warp = 16, // Attr: 17
    Rewarp = 17, // Attr: 17
    Interference = 18, // Attr: 17
    Torch = 19, // Attr: 17
    Creation = 20, // Attr: 17
    Cannon = 21, // Attr: 17
    FireCannon = 22, // Attr: 17
    FullBulletCharge = 23, // Attr: 17
}