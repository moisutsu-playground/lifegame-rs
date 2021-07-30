use lifegame_rs::{CellState, DisplayInCui, LifeGame};
use std::{thread, time};

fn main() {
    let sleep_time = time::Duration::from_millis(100);
    let mut lifegame = LifeGame::new(10, 10);
    lifegame.set_cell_state(CellState::Alive, 4, 5);
    lifegame.set_cell_state(CellState::Alive, 5, 5);
    lifegame.set_cell_state(CellState::Alive, 6, 5);
    lifegame.set_cell_state(CellState::Alive, 4, 6);
    lifegame.set_cell_state(CellState::Alive, 5, 7);
    loop {
        print!("{}[2J", 27 as char);
        lifegame.display();
        lifegame.next();
        thread::sleep(sleep_time);
    }
}
