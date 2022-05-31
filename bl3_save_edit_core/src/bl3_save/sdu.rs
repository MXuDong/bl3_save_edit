use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct SaveSduSlotData {
    pub sdu: SaveSduSlot,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, Display, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum SaveSduSlot {
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Backpack.SDU_Backpack",
        to_string = "背包"
    )]
    Backpack,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_SniperRifle.SDU_SniperRifle",
        to_string = "狙击枪"
    )]
    Sniper,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Shotgun.SDU_Shotgun",
        to_string = "霰弹枪"
    )]
    Shotgun,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Pistol.SDU_Pistol",
        to_string = "手枪"
    )]
    Pistol,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Grenade.SDU_Grenade",
        to_string = "手榴弹"
    )]
    Grenade,
    #[strum(serialize = "/Game/Pickups/SDU/SDU_SMG.SDU_SMG", to_string = "SMG")]
    Smg,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_AssaultRifle.SDU_AssaultRifle",
        to_string = "步枪"
    )]
    Ar,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Heavy.SDU_Heavy",
        to_string = "重型武器"
    )]
    Heavy,
}

impl std::default::Default for SaveSduSlot {
    fn default() -> Self {
        Self::Backpack
    }
}

impl SaveSduSlot {
    pub fn maximum(&self) -> i32 {
        match self {
            SaveSduSlot::Backpack | SaveSduSlot::Sniper | SaveSduSlot::Heavy => 13,
            SaveSduSlot::Shotgun
            | SaveSduSlot::Pistol
            | SaveSduSlot::Grenade
            | SaveSduSlot::Smg
            | SaveSduSlot::Ar => 10,
        }
    }
}
