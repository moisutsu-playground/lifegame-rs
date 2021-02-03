use lifegame_rs::{LifeGame, State};
use std::{thread, time};

fn main() {
    let sleep_time = time::Duration::from_millis(500);
    let mut life_game = LifeGame::new(10, 10);
    life_game.set_cell_state(State::Alive, 5, 5);
    life_game.set_cell_state(State::Alive, 5, 6);
    life_game.set_cell_state(State::Alive, 5, 7);
    loop {
        life_game.display();
        life_game.next();
        thread::sleep(sleep_time);
    }
}
