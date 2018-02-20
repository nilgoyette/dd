
use item::item::Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArmorProficiency {
    Light,
    Medium,
    Heavy,
    Shield
}

impl ArmorProficiency {
    pub fn all() -> Vec<ArmorProficiency> {
        vec![ArmorProficiency::Light, ArmorProficiency::Medium,
             ArmorProficiency::Heavy, ArmorProficiency::Shield]
    }
}

pub struct Armor {
    pub item: Item,
    pub proficiency: ArmorProficiency,
    pub ac: usize,
    pub min_strength: Option<usize>,
    pub stealth_disadvantage: bool
}

impl Armor {
    pub fn light(
        name: &str,
        weight: f32,
        cost: usize,
        ac: usize,
        stealth_disadvantage: bool
    ) -> Armor {
        let proficiency = ArmorProficiency::Light; 
        Armor {
            item: Item::new(name, weight, cost),
            proficiency, ac, min_strength: None, stealth_disadvantage
        }
    }

    pub fn medium(
        name: &str,
        weight: f32,
        cost: usize,
        ac: usize,
        stealth_disadvantage: bool
    ) -> Armor {
        let proficiency = ArmorProficiency::Medium;
        Armor {
            item: Item::new(name, weight, cost),
            proficiency, ac, min_strength: None, stealth_disadvantage
        }
    }

    pub fn heavy(
        name: &str,
        weight: f32,
        cost: usize,
        ac: usize,
        min_strength: Option<usize>
    ) -> Armor {
        let proficiency = ArmorProficiency::Heavy;
        Armor {
            item: Item::new(name, weight, cost),
            proficiency, ac, min_strength, stealth_disadvantage: true
        }
    }

    pub fn base_ac(&self, dex_mod: isize) -> isize {
        let ac = self.ac as isize;
        match self.proficiency {
            ArmorProficiency::Light => ac + dex_mod,
            ArmorProficiency::Medium => ac + dex_mod.max(2),
            ArmorProficiency::Heavy => ac,
            ArmorProficiency::Shield => 0
        }
    }

    pub fn ac_modifier(&self) -> isize {
        match self.proficiency {
            ArmorProficiency::Shield => 2,
            _ => 0
        }
    }
}
