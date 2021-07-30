use lifegame_rs::{DisplayInCui, LifeGame};
use std::{thread, time};

fn main() {
    let sleep_time = time::Duration::from_millis(100);
    let mut lifegame = LifeGame::new_with_random_state(50, 100);
    loop {
        clear_terminal();
        lifegame.display();
        lifegame.next();
        thread::sleep(sleep_time);
    }
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}
