pub struct Combatant {
  mob: Mob,
  health: u32,
  attack_power: u32,
  defense_power: u32,
}

impl Combatant {
  pub fn new(name: &str, health: u32, attack_power: u32, defense_power: u32) -> Combatant {
    Combatant {
      mob: Mob::new(name),
      health,
      attack_power,
      defense_power,
    }
  }
  pub fn attack(&self, target: &mut Combatant) {
    let damage = self.calculate_damage(target.defense_power);
    target.health -= damage;
    println!("{} attacks {} for {} damage!", self.mob.name, target.mob.name, damage);
  }
  fn calculate_damage(&self, defense_power: u32) -> u32 {
    (self.attack_power * (1.0 - (defense_power as f32 / 100.0))) as u32
  }
  pub fn is_alive(&self) -> bool {
    self.health > 0
  }
  fn log_attack(&self, damage: u32, attacker: &Mob) {
    println!("{} takes {} damage from {}!", self.name, damage, attacker.name);
  }
  pub fn log_defend(&self, damage: u32, attacker: &Mob) {
    println!("{} defends against {}'s attack!", self.name, attacker.name);
  }
}