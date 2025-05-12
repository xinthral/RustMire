
use std::str::FromStr;
use std::result;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MobClass {
  Druid,
  Mage,
  Rogue,
  Warrior,
}

impl FromStr for MobClass {

  type Err = &'static str;

  fn from_str(input: &str) -> result::Result<Self, Self::Err> {
    match input.to_lowercase().as_str() {
      "druid"   => Ok(MobClass::Druid),
      "mage"    => Ok(MobClass::Mage),
      "rogue"   => Ok(MobClass::Rogue),
      "warrior" => Ok(MobClass::Warrior),
      _         => Err("Invalid mob class"),
    }
  }
}

impl fmt::Display for MobClass {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

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
      class: match class.parse::<MobClass>() {
        Ok(class) => class.to_string(),
        Err(e) => panic!("Invalid mob class: {}", e),
      },
      level,
    }
  }

}

impl fmt::Display for Mob {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({:^3}) {} the {}", self.level, self.name, self.class)
  }
}

impl fmt::Debug for Mob {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Character Sheet\n {:<5}: {}\n {:<5}: {}\n {:<5}: {}\n {:<5}: {}", "Name", self.name, "Race", self.race, "Class", self.class, "Level", self.level)
  }
}