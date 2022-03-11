// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => match self.level {
                level if level >= 10 => Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                }),
                _ => Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                }),
            },
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana > mana_cost => {
                let new_mana = mana - mana_cost;
                self.mana = Some(new_mana);
                mana_cost * 2
            }
            Some(mana) => 0,
            None => {
                if self.health < mana_cost {
                    self.health = 0;
                } else {
                    self.health = self.health - mana_cost;
                }
                0
            }
        }
    }
}
