pub struct Monster {
  pub health: int,
  attack: uint
}

impl Monster {
  pub fn new(health: int, attack: uint) -> Monster {
    Monster { health: health, attack: attack }
  }

  pub fn attack(&mut self, damage: int) {
    self.health -= damage;
  }
}

#[test]
fn it_can_be_instantiated() {
  let monster = Monster::new(20, 10);
  assert!(monster.health == 20);
  assert!(monster.attack == 10);
}

#[test]
fn it_loses_health_when_attacked() {
  let mut monster = Monster::new(20, 10);
  monster.attack(5);
  assert!(monster.health == 15);
}
