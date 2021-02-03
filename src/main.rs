use lifegame_rs::{LifeGame, State};
use std::{thread, time};

fn main() {
    let sleep_time = time::Duration::from_millis(100);
    let mut life_game = LifeGame::new(10, 10);
    life_game.set_cell_state(State::Alive, 4, 5);
    life_game.set_cell_state(State::Alive, 5, 5);
    life_game.set_cell_state(State::Alive, 6, 5);
    life_game.set_cell_state(State::Alive, 4, 6);
    life_game.set_cell_state(State::Alive, 5, 7);
    loop {
        print!("{}[2J", 27 as char);
        life_game.display();
        life_game.next();
        thread::sleep(sleep_time);
    }
}
