use uuid::Uuid;

pub trait HasUUID {
  fn get_uuid(&self) -> String;
  fn set_uuid(&mut self, uuid: String);
  fn is_valid_uuid(&self) -> bool;
  fn generate_uuid(&mut self) {
    self.set_uuid(Uuid::new_v4().to_string());
  }
}

pub trait HasShortDescription {
  fn get_short_description(&self) -> &str;
  fn set_short_description(&mut self, description: &str);
}

pub trait HasLongDescription {
  fn get_long_description(&self) -> &str;
  fn set_long_description(&mut self, description: &str);
  fn add_additional_description(&mut self, description: &str);
  fn remove_additional_description(&mut self, description: &str);
}

pub trait HasMaterial {
  fn get_material(&self) -> &str;
  fn set_material(&mut self, material: &str);
}

pub trait HasName {
  fn get_name(&self) -> &str;
  fn set_name(&mut self, name: &str);
}

pub trait HasValue {
  fn get_value(&self) -> i32;
  fn set_value(&mut self, value: i32);
}

pub trait HasWeight {
  fn get_weight(&self) -> f32;
  fn set_weight(&mut self, weight: f32);
}

pub trait CanBeDamaged {
  fn take_damage(&mut self, damage: i32);
}
