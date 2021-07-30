use itertools::iproduct;
use rand::Rng;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
pub struct LifeGame {
    cell_field: Vec<Vec<CellState>>,
}

impl LifeGame {
    pub fn new(height: usize, width: usize) -> Self {
        LifeGame {
            cell_field: vec![vec![CellState::Dead; width]; height],
        }
    }

    pub fn new_with_random_state(height: usize, width: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut cell_field = vec![vec![CellState::Dead; width]; height];
        for (x, y) in iproduct!(0..width, 0..height) {
            cell_field[y][x] = if rng.gen() {
                CellState::Alive
            } else {
                CellState::Dead
            };
        }
        LifeGame { cell_field }
    }

    pub fn height(&self) -> usize {
        self.cell_field.len()
    }

    pub fn width(&self) -> usize {
        self.cell_field[0].len()
    }

    pub fn next(&mut self) {
        let mut next_lifegame = LifeGame::new(self.height(), self.width());
        for (x, y) in iproduct!(0..self.width(), 0..self.height()) {
            let next_cell_state = match (
                self.get_cell_state(x, y),
                self.surrounding_alive_cells(x, y),
            ) {
                (CellState::Alive, 2 | 3) => CellState::Alive,
                (CellState::Alive, _) => CellState::Dead,
                (CellState::Dead, 3) => CellState::Alive,
                (CellState::Dead, _) => CellState::Dead,
            };
            next_lifegame.set_cell_state(next_cell_state, x, y);
        }
        *self = next_lifegame;
    }

    pub fn get_cell_state<T>(&self, x: T, y: T) -> CellState
    where
        T: TryInto<i32>,
        <T as std::convert::TryInto<i32>>::Error: std::fmt::Debug,
    {
        let (x, y) = self.get_row_point(x, y);
        self.cell_field[y][x]
    }

    pub fn set_cell_state<T>(&mut self, state: CellState, x: T, y: T)
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
        for (move_x, move_y) in iproduct!(-1..=1, -1..=1) {
            if move_x == 0 && move_y == 0 {
                continue;
            }
            if self.get_cell_state(x + move_x, y + move_y) == CellState::Alive {
                count += 1;
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
        let (height, width) = (self.height() as i32, self.width() as i32);

        let (x, y) = ((x + width) % width, (y + height) % height);

        (x as usize, y as usize)
    }
}
