pub mod mobs;
use mobs::Mob;

pub fn create_mob() -> mobs::Mob {
  let name = format!("{} Van {}", "Xinthral", "Horton");
  let race = format!("{}", "Elf");
  let class = format!("{}", "Rogue");
  let level = 0;
  Mob::new(&name, &race, &class, level)
}

