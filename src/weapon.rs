
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeaponProficiency {
    //Simple,
    Club,
    Dagger,
    GreatClub,
    HardAxe,
    Javelin,
    LightHammer,
    Mace,
    QuarterStaff,
    Sickle,
    Spear,
    Yklwa,
    LightCrossbow,
    Dart,
    ShortBow,
    Sling,

    //Martial,
    BattleAxe,
    Flail,
    Glaive,
    GreatAxe,
    GreatSword,
    Halberd,
    Lance,
    LongSword,
    Maul,
    Morningstar,
    Pike,
    Rapier,
    Scimitar,
    ShortSword,
    Trident,
    WarPick,
    WarHammer,
    Whip,
    Blowgun,
    HandCrossbow,
    HeanvyCrossbow,
    Longbow,
    Net
}

impl WeaponProficiency {
    pub fn simple() -> Vec<WeaponProficiency> {
        vec![WeaponProficiency::Club,
             WeaponProficiency::Dagger,
             WeaponProficiency::GreatClub,
             WeaponProficiency::HardAxe,
             WeaponProficiency::Javelin,
             WeaponProficiency::LightHammer,
             WeaponProficiency::Mace,
             WeaponProficiency::QuarterStaff,
             WeaponProficiency::Sickle,
             WeaponProficiency::Spear,
             WeaponProficiency::Yklwa,
             WeaponProficiency::LightCrossbow,
             WeaponProficiency::Dart,
             WeaponProficiency::ShortBow,
             WeaponProficiency::Sling]
    }

    pub fn martial() -> Vec<WeaponProficiency> {
        vec![WeaponProficiency::BattleAxe,
             WeaponProficiency::Flail,
             WeaponProficiency::Glaive,
             WeaponProficiency::GreatAxe,
             WeaponProficiency::GreatSword,
             WeaponProficiency::Halberd,
             WeaponProficiency::Lance,
             WeaponProficiency::LongSword,
             WeaponProficiency::Maul,
             WeaponProficiency::Morningstar,
             WeaponProficiency::Pike,
             WeaponProficiency::Rapier,
             WeaponProficiency::Scimitar,
             WeaponProficiency::ShortSword,
             WeaponProficiency::Trident,
             WeaponProficiency::WarPick,
             WeaponProficiency::WarHammer,
             WeaponProficiency::Whip,
             WeaponProficiency::Blowgun,
             WeaponProficiency::HandCrossbow,
             WeaponProficiency::HeanvyCrossbow,
             WeaponProficiency::Longbow,
             WeaponProficiency::Net]
    }

    pub fn all() -> Vec<WeaponProficiency> {
        let mut weapons = Vec::with_capacity(38);
        weapons.extend(WeaponProficiency::simple());
        weapons.extend(WeaponProficiency::martial());
        weapons
    }
}
