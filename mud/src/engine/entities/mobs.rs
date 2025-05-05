

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum MobClass {
  Warrior,
  Rogue,
  Mage,
  Druid,
}

impl Display for MobClass {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub struct Mob {
  pub name: String,
  pub race: String,
  pub class: String,
  pub level: u8,
}

impl Mob {
  pub fn new(name: &str, race: &str, class: &str, level: u8) -> Mob {
    Mob {
      name: name.to_string(),
      race: race.to_string(),
      class: match class {
        "Warrior" => format!("{}", MobClass::Warrior).to_string(),
        "Rogue" => format!("{}", MobClass::Rogue).to_string(),
        "Mage" => format!("{}", MobClass::Mage).to_string(),
        "Druid" => format!("{}", MobClass::Druid).to_string(),
        _ => panic!("Invalid mob class"),
      },
      level,
    }
  }
  pub fn attack(&self, target: &Mob) {
    println!("{} attacks {}!", self.name, target.name);
  }
  pub fn defend(&self, damage: u32, attacker: &Mob) {
    println!("{} defends against {}'s attack!", self.name, attacker.name);
    self.take_damage(damage, attacker);
  }
  fn take_damage(&self, damage: u32, attacker: &Mob) {
    println!("{} takes {} damage from {}!", self.name, damage, attacker.name);
  }
}