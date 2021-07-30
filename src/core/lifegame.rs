use itertools::iproduct;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
pub struct LifeGame {
    cell_field: Vec<Vec<State>>,
    pub height: usize,
    pub width: usize,
}

impl LifeGame {
    pub fn new(height: usize, width: usize) -> Self {
        LifeGame {
            cell_field: vec![vec![State::Dead; width]; height],
            height,
            width,
        }
    }

    pub fn next(&mut self) {
        let mut new_lifegame = self.clone();
        for (x, y) in iproduct!(0..self.width, 0..self.height) {
            match self.get_cell_state(x, y) {
                State::Alive => match self.surrounding_alive_cells(x, y) {
                    2 | 3 => (),
                    _ => new_lifegame.set_cell_state(State::Dead, x, y),
                },
                State::Dead => {
                    if self.surrounding_alive_cells(x, y) == 3 {
                        new_lifegame.set_cell_state(State::Alive, x, y);
                    }
                }
            }
        }
        *self = new_lifegame;
    }

    pub fn get_cell_state<T>(&self, x: T, y: T) -> State
    where
        T: TryInto<i32>,
        <T as std::convert::TryInto<i32>>::Error: std::fmt::Debug,
    {
        let (x, y) = self.get_row_point(x, y);
        self.cell_field[y][x]
    }

    pub fn set_cell_state<T>(&mut self, state: State, x: T, y: T)
    where
        T: TryInto<i32>,
        <T as std::convert::TryInto<i32>>::Error: std::fmt::Debug,
    {
        let (x, y) = self.get_row_point(x, y);
        self.cell_field[y][x] = state;
    }

    fn surrounding_alive_cells<T>(&self, x: T, y: T) -> i32
    where
        T: TryInto<i32>,
        <T as std::convert::TryInto<i32>>::Error: std::fmt::Debug,
    {
        let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
        let mut count = 0;
        for move_x in -1..=1 {
            for move_y in -1..=1 {
                if move_x == 0 && move_y == 0 {
                    continue;
                }
                if self.get_cell_state(x + move_x, y + move_y) == State::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_row_point<T>(&self, x: T, y: T) -> (usize, usize)
    where
        T: TryInto<i32>,
        <T as std::convert::TryInto<i32>>::Error: std::fmt::Debug,
    {
        let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
        let x = if x >= 0 {
            x as usize % self.width
        } else {
            self.width - -x as usize
        };
        let y = if y >= 0 {
            y as usize % self.height
        } else {
            self.height - -y as usize
        };
        (x, y)
    }
}
