use strum::{Display, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct AmmoPoolData {
    pub pool: AmmoPool,
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Display, EnumString, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum AmmoPool {
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Grenade.Resource_Ammo_Grenade",
        to_string = "手榴弹"
    )]
    Grenade,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Pistol.Resource_Ammo_Pistol",
        to_string = "手枪"
    )]
    Pistol,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Shotgun.Resource_Ammo_Shotgun",
        to_string = "霰弹枪"
    )]
    Shotgun,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_SMG.Resource_Ammo_SMG",
        to_string = "冲锋枪"
    )]
    Smg,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_AssaultRifle.Resource_Ammo_AssaultRifle",
        to_string = "步枪"
    )]
    Ar,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Sniper.Resource_Ammo_Sniper",
        to_string = "狙击枪"
    )]
    Sniper,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Heavy.Resource_Ammo_Heavy",
        to_string = "重型武器"
    )]
    Heavy,
}

impl std::default::Default for AmmoPool {
    fn default() -> Self {
        Self::Grenade
    }
}

impl AmmoPool {
    pub fn maximum(&self) -> i32 {
        match self {
            AmmoPool::Grenade => 13,
            AmmoPool::Pistol => 1200,
            AmmoPool::Shotgun => 280,
            AmmoPool::Smg => 2160,
            AmmoPool::Ar => 1680,
            AmmoPool::Sniper => 204,
            AmmoPool::Heavy => 51,
        }
    }
}
