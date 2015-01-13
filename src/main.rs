use std::io;
use models::monster::Monster;

mod models;

fn main() {
  let mut monster = Monster::new(20, 40);
  println!("Welcome to your doom!");

  loop {
    println!("The monster has {} health.", monster.health)
    let input = io::stdin().read_line().ok().expect("Failed to read line");
    let command = input.as_slice().trim();
    match command {
      "exit" => break,
      "attack" => attack_monster(&mut monster),
      _ => println!("Unknown command: {}", command)
    }
  }
}

fn attack_monster(monster: &mut Monster) {
  (*monster).attack(10);
  println!("You attack the monster for {} damage.", 10);
}
